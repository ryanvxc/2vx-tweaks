//! SQLite change journal: the durable record that makes every change reversible.
//!
//! Write-ahead discipline: a change row is committed with its exact before-value
//! (`intent`) *before* the system is touched, then updated once the write is verified.
//! If the app dies in between, the row survives and [`Journal::recover_interrupted`]
//! marks the snapshot so the user can revert it.

use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;

use crate::engine::setting::{RegTarget, RegValue, describe};
use crate::error::{AppError, AppResult};

/// Bump when adding a migration below.
const SCHEMA_VERSION: i64 = 2;

const MIGRATION_1: &str = r#"
CREATE TABLE snapshots (
    id            INTEGER PRIMARY KEY,
    kind          TEXT    NOT NULL CHECK (kind IN ('apply', 'revert')),
    tweak_ids     TEXT    NOT NULL,           -- JSON array
    reverts       INTEGER REFERENCES snapshots(id),
    status        TEXT    NOT NULL,
    error         TEXT,
    app_version   TEXT    NOT NULL,
    created_at_ms INTEGER NOT NULL,
    finished_at_ms INTEGER
);
CREATE TABLE changes (
    id            INTEGER PRIMARY KEY,
    snapshot_id   INTEGER NOT NULL REFERENCES snapshots(id),
    seq           INTEGER NOT NULL,
    tweak_id      TEXT    NOT NULL,
    target        TEXT    NOT NULL,           -- JSON RegTarget
    before_value  TEXT    NOT NULL,           -- JSON Option<RegValue>
    after_value   TEXT    NOT NULL,           -- JSON Option<RegValue>
    status        TEXT    NOT NULL,
    error         TEXT,
    updated_at_ms INTEGER NOT NULL,
    UNIQUE (snapshot_id, seq)
);
CREATE INDEX changes_by_snapshot ON changes(snapshot_id);
CREATE TABLE audit_log (
    id     INTEGER PRIMARY KEY,
    at_ms  INTEGER NOT NULL,
    event  TEXT    NOT NULL,
    detail TEXT    NOT NULL                   -- JSON
);
"#;

/// Which driver instance a change reached (`0008`), for targets that resolve to a device. NULL on
/// every other target, and on rows from before this column, which match any device as they always
/// did. In the wild those are `Hive::Gpu` rows from 0.3.
const MIGRATION_2: &str = "ALTER TABLE changes ADD COLUMN device TEXT;";

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SnapshotKind {
    Apply,
    Revert,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SnapshotStatus {
    InProgress,
    /// Every change was written and verified.
    Completed,
    /// A change failed; earlier changes were undone.
    RolledBack,
    /// A change failed and at least one earlier change could not be undone. Needs attention.
    Failed,
    /// The app stopped mid-operation. Revert it to restore the recorded before-values.
    Interrupted,
    /// Every change of this apply snapshot has been reverted.
    Reverted,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChangeStatus {
    /// Recorded, not yet written (or the write's outcome is unknown after a crash).
    Intent,
    /// Written and read back with the expected value.
    Verified,
    /// Undone during rollback of its own snapshot.
    RolledBack,
    /// Undone by a later revert snapshot.
    Reverted,
    /// The write failed or read back the wrong value.
    Failed,
    /// Revert skipped: the value was changed by something else since 2VX set it.
    Conflict,
    /// Nothing to do: the system already held the target value.
    Unchanged,
}

macro_rules! text_enum {
    ($ty:ty { $($variant:ident => $text:literal),+ $(,)? }) => {
        impl $ty {
            pub fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $text),+ }
            }
            fn parse(text: &str) -> AppResult<Self> {
                match text {
                    $($text => Ok(Self::$variant),)+
                    other => Err(AppError::Storage(format!("unknown {} `{other}`", stringify!($ty)))),
                }
            }
        }
    };
}

text_enum!(SnapshotKind { Apply => "apply", Revert => "revert" });
text_enum!(SnapshotStatus {
    InProgress => "in_progress",
    Completed => "completed",
    RolledBack => "rolled_back",
    Failed => "failed",
    Interrupted => "interrupted",
    Reverted => "reverted",
});
text_enum!(ChangeStatus {
    Intent => "intent",
    Verified => "verified",
    RolledBack => "rolled_back",
    Reverted => "reverted",
    Failed => "failed",
    Conflict => "conflict",
    Unchanged => "unchanged",
});

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRecord {
    pub id: i64,
    pub kind: SnapshotKind,
    pub tweak_ids: Vec<String>,
    /// For a revert snapshot: the apply snapshot it undid.
    pub reverts: Option<i64>,
    pub status: SnapshotStatus,
    pub error: Option<String>,
    pub app_version: String,
    pub created_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub change_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRecord {
    pub id: i64,
    pub snapshot_id: i64,
    pub seq: i64,
    pub tweak_id: String,
    pub target: RegTarget,
    /// Display forms, as in `PlannedChange`, so history needs no value formatting in the UI.
    pub target_label: String,
    /// The driver instance this change reached, which undo compares with where the target resolves
    /// to now. Not sent to the UI. `None` for a fixed location and for rows from before schema 2.
    #[serde(skip)]
    pub device: Option<String>,
    pub before: Option<RegValue>,
    pub before_label: String,
    pub after: Option<RegValue>,
    pub after_label: String,
    pub status: ChangeStatus,
    pub error: Option<String>,
    pub updated_at_ms: i64,
}

pub struct Journal {
    conn: Connection,
}

pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn to_json<T: Serialize>(value: &T) -> AppResult<String> {
    serde_json::to_string(value).map_err(|e| AppError::Internal(e.to_string()))
}

fn from_json<T: serde::de::DeserializeOwned>(text: &str) -> AppResult<T> {
    serde_json::from_str(text).map_err(|e| AppError::Storage(format!("corrupt journal entry: {e}")))
}

impl Journal {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Self::init(Connection::open(path)?)
    }

    pub fn open_in_memory() -> AppResult<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> AppResult<Self> {
        // FULL sync: an `intent` row must be on disk before the registry write that follows it.
        conn.execute_batch(
            "PRAGMA journal_mode = WAL; PRAGMA synchronous = FULL; PRAGMA foreign_keys = ON;",
        )?;
        let mut journal = Journal { conn };
        journal.check_integrity()?;
        journal.migrate()?;
        Ok(journal)
    }

    fn check_integrity(&self) -> AppResult<()> {
        let result: String = self
            .conn
            .query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        if result != "ok" {
            return Err(AppError::Storage(format!(
                "integrity check failed: {result}"
            )));
        }
        Ok(())
    }

    fn migrate(&mut self) -> AppResult<()> {
        let version: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if version > SCHEMA_VERSION {
            return Err(AppError::Storage(format!(
                "journal schema v{version} is newer than this app (v{SCHEMA_VERSION}); update 2VX Tweaks"
            )));
        }
        let tx = self.conn.transaction()?;
        if version < 1 {
            tx.execute_batch(MIGRATION_1)?;
        }
        if version < 2 {
            tx.execute_batch(MIGRATION_2)?;
        }
        tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        tx.commit()?;
        Ok(())
    }

    /// Marks snapshots left `in_progress` by a crash as `interrupted`. Returns how many.
    pub fn recover_interrupted(&mut self) -> AppResult<usize> {
        let count = self.conn.execute(
            "UPDATE snapshots SET status = ?1, error = 'The app stopped before this finished.', finished_at_ms = ?2
             WHERE status = ?3",
            params![
                SnapshotStatus::Interrupted.as_str(),
                now_ms(),
                SnapshotStatus::InProgress.as_str()
            ],
        )?;
        if count > 0 {
            self.audit(
                "recovered_interrupted",
                serde_json::json!({ "snapshots": count }),
            )?;
        }
        Ok(count)
    }

    /// Counted here rather than from `snapshots`: engine status runs often and the history is long.
    pub fn interrupted_count(&self) -> AppResult<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM snapshots WHERE status = ?1",
            [SnapshotStatus::Interrupted.as_str()],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    pub fn begin_snapshot(
        &mut self,
        kind: SnapshotKind,
        tweak_ids: &[String],
        reverts: Option<i64>,
    ) -> AppResult<i64> {
        self.conn.execute(
            "INSERT INTO snapshots (kind, tweak_ids, reverts, status, app_version, created_at_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                kind.as_str(),
                to_json(&tweak_ids)?,
                reverts,
                SnapshotStatus::InProgress.as_str(),
                env!("CARGO_PKG_VERSION"),
                now_ms()
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn finish_snapshot(
        &mut self,
        id: i64,
        status: SnapshotStatus,
        error: Option<&str>,
    ) -> AppResult<()> {
        self.conn.execute(
            "UPDATE snapshots SET status = ?1, error = ?2, finished_at_ms = ?3 WHERE id = ?4",
            params![status.as_str(), error, now_ms(), id],
        )?;
        Ok(())
    }

    pub fn set_snapshot_status(&mut self, id: i64, status: SnapshotStatus) -> AppResult<()> {
        self.conn.execute(
            "UPDATE snapshots SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    /// Records a change before it is made. Committed immediately (autocommit).
    #[allow(clippy::too_many_arguments)]
    pub fn record_change(
        &mut self,
        snapshot_id: i64,
        seq: usize,
        tweak_id: &str,
        target: &RegTarget,
        device: Option<&str>,
        before: &Option<RegValue>,
        after: &Option<RegValue>,
        status: ChangeStatus,
    ) -> AppResult<i64> {
        self.conn.execute(
            "INSERT INTO changes (snapshot_id, seq, tweak_id, target, device, before_value, after_value, status, updated_at_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                snapshot_id,
                seq as i64,
                tweak_id,
                to_json(target)?,
                device,
                to_json(before)?,
                to_json(after)?,
                status.as_str(),
                now_ms()
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn set_change_status(
        &mut self,
        change_id: i64,
        status: ChangeStatus,
        error: Option<&str>,
    ) -> AppResult<()> {
        self.conn.execute(
            "UPDATE changes SET status = ?1, error = ?2, updated_at_ms = ?3 WHERE id = ?4",
            params![status.as_str(), error, now_ms(), change_id],
        )?;
        Ok(())
    }

    pub fn snapshot(&self, id: i64) -> AppResult<Option<SnapshotRecord>> {
        self.conn
            .query_row(
                &format!("{SNAPSHOT_SELECT} WHERE s.id = ?1"),
                [id],
                snapshot_row,
            )
            .optional()?
            .map(|raw| raw.decode())
            .transpose()
    }

    /// Most recent first.
    pub fn snapshots(&self, limit: u32) -> AppResult<Vec<SnapshotRecord>> {
        let mut stmt = self
            .conn
            .prepare(&format!("{SNAPSHOT_SELECT} ORDER BY s.id DESC LIMIT ?1"))?;
        let rows = stmt.query_map([limit], snapshot_row)?;
        rows.map(|r| r?.decode()).collect()
    }

    /// In the order they were made.
    pub fn changes(&self, snapshot_id: i64) -> AppResult<Vec<ChangeRecord>> {
        self.query_changes(
            &format!("{CHANGE_SELECT} WHERE snapshot_id = ?1 ORDER BY seq"),
            [snapshot_id],
        )
    }

    /// Changes 2VX made for `tweak_id` and has not undone yet, newest first. These are the
    /// rows a per-tweak revert restores.
    pub fn undoable_changes(&self, tweak_id: &str) -> AppResult<Vec<ChangeRecord>> {
        self.query_changes(
            &format!(
                "{CHANGE_SELECT} WHERE tweak_id = ?1 AND {UNDOABLE} ORDER BY snapshot_id DESC, seq DESC"
            ),
            [tweak_id],
        )
    }

    /// IDs of every tweak with at least one undoable change.
    pub fn undoable_tweaks(&self) -> AppResult<std::collections::BTreeSet<String>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT DISTINCT tweak_id FROM changes WHERE {UNDOABLE}"
        ))?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<Result<_, _>>()?)
    }

    /// Whether an apply snapshot still has changes left to undo.
    pub fn has_undoable_changes(&self, snapshot_id: i64) -> AppResult<bool> {
        let count: i64 = self.conn.query_row(
            &format!("SELECT COUNT(*) FROM changes WHERE snapshot_id = ?1 AND {UNDOABLE}"),
            [snapshot_id],
            |r| r.get(0),
        )?;
        Ok(count > 0)
    }

    fn query_changes(
        &self,
        sql: &str,
        params: impl rusqlite::Params,
    ) -> AppResult<Vec<ChangeRecord>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
                r.get::<_, String>(7)?,
                r.get::<_, Option<String>>(8)?,
                r.get::<_, i64>(9)?,
                r.get::<_, Option<String>>(10)?,
            ))
        })?;
        rows.map(|row| {
            let (
                id,
                snapshot_id,
                seq,
                tweak_id,
                target,
                before,
                after,
                status,
                error,
                updated_at_ms,
                device,
            ) = row?;
            let target: RegTarget = from_json(&target)?;
            let before = from_json(&before)?;
            let after = from_json(&after)?;
            Ok(ChangeRecord {
                id,
                snapshot_id,
                seq,
                tweak_id,
                target_label: target.to_string(),
                target,
                device,
                before_label: describe(&before),
                before,
                after_label: describe(&after),
                after,
                status: ChangeStatus::parse(&status)?,
                error,
                updated_at_ms,
            })
        })
        .collect()
    }

    pub fn audit(&mut self, event: &str, detail: serde_json::Value) -> AppResult<()> {
        self.conn.execute(
            "INSERT INTO audit_log (at_ms, event, detail) VALUES (?1, ?2, ?3)",
            params![now_ms(), event, detail.to_string()],
        )?;
        Ok(())
    }
}

const CHANGE_SELECT: &str = "SELECT id, snapshot_id, seq, tweak_id, target, before_value, after_value, status, error, updated_at_ms, device FROM changes";

/// Rows of finished or interrupted apply snapshots that may still be in effect: the same
/// filter `Engine::revert` uses for a whole snapshot.
const UNDOABLE: &str = "status IN ('verified', 'intent', 'failed')
    AND snapshot_id IN (SELECT id FROM snapshots
        WHERE kind = 'apply' AND status IN ('completed', 'interrupted', 'failed'))";

const SNAPSHOT_SELECT: &str =
    "SELECT s.id, s.kind, s.tweak_ids, s.reverts, s.status, s.error, s.app_version,
        s.created_at_ms, s.finished_at_ms,
        (SELECT COUNT(*) FROM changes c WHERE c.snapshot_id = s.id)
     FROM snapshots s";

struct RawSnapshot {
    id: i64,
    kind: String,
    tweak_ids: String,
    reverts: Option<i64>,
    status: String,
    error: Option<String>,
    app_version: String,
    created_at_ms: i64,
    finished_at_ms: Option<i64>,
    change_count: i64,
}

fn snapshot_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<RawSnapshot> {
    Ok(RawSnapshot {
        id: r.get(0)?,
        kind: r.get(1)?,
        tweak_ids: r.get(2)?,
        reverts: r.get(3)?,
        status: r.get(4)?,
        error: r.get(5)?,
        app_version: r.get(6)?,
        created_at_ms: r.get(7)?,
        finished_at_ms: r.get(8)?,
        change_count: r.get(9)?,
    })
}

impl RawSnapshot {
    fn decode(self) -> AppResult<SnapshotRecord> {
        Ok(SnapshotRecord {
            id: self.id,
            kind: SnapshotKind::parse(&self.kind)?,
            tweak_ids: from_json(&self.tweak_ids)?,
            reverts: self.reverts,
            status: SnapshotStatus::parse(&self.status)?,
            error: self.error,
            app_version: self.app_version,
            created_at_ms: self.created_at_ms,
            finished_at_ms: self.finished_at_ms,
            change_count: self.change_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::setting::Hive;

    fn target() -> RegTarget {
        RegTarget::new(Hive::CurrentUser, r"Software\2VX-Test", "Value")
    }

    #[test]
    fn records_and_reads_back_changes() {
        let mut j = Journal::open_in_memory().unwrap();
        let s = j
            .begin_snapshot(SnapshotKind::Apply, &["gaming.test".into()], None)
            .unwrap();
        let c = j
            .record_change(
                s,
                0,
                "gaming.test",
                &target(),
                None,
                &None,
                &Some(RegValue::Dword(1)),
                ChangeStatus::Intent,
            )
            .unwrap();
        j.set_change_status(c, ChangeStatus::Verified, None)
            .unwrap();
        j.finish_snapshot(s, SnapshotStatus::Completed, None)
            .unwrap();

        let snap = j.snapshot(s).unwrap().unwrap();
        assert_eq!(snap.status, SnapshotStatus::Completed);
        assert_eq!(snap.tweak_ids, vec!["gaming.test".to_string()]);
        assert_eq!(snap.change_count, 1);
        let changes = j.changes(s).unwrap();
        assert_eq!(changes[0].before, None);
        assert_eq!(changes[0].after, Some(RegValue::Dword(1)));
        assert_eq!(changes[0].target_label, target().to_string());
        assert_eq!(changes[0].before_label, "(not set)");
        assert_eq!(changes[0].after_label, "1 (REG_DWORD)");
        assert_eq!(changes[0].status, ChangeStatus::Verified);
        assert!(j.snapshot(999).unwrap().is_none());
    }

    #[test]
    fn crash_mid_snapshot_is_recovered_as_interrupted() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.db");
        {
            let mut j = Journal::open(&path).unwrap();
            let s = j.begin_snapshot(SnapshotKind::Apply, &[], None).unwrap();
            j.record_change(
                s,
                0,
                "t",
                &target(),
                None,
                &None,
                &Some(RegValue::Dword(1)),
                ChangeStatus::Intent,
            )
            .unwrap();
            // Dropped without finishing: simulates the process dying after the intent row.
        }
        let mut j = Journal::open(&path).unwrap();
        assert_eq!(j.recover_interrupted().unwrap(), 1);
        let snap = &j.snapshots(10).unwrap()[0];
        assert_eq!(snap.status, SnapshotStatus::Interrupted);
        assert_eq!(j.changes(snap.id).unwrap()[0].status, ChangeStatus::Intent);
        // Idempotent.
        assert_eq!(j.recover_interrupted().unwrap(), 0);
        assert_eq!(j.interrupted_count().unwrap(), 1);
    }

    /// A journal as 0.3 wrote it: schema 1, with a `Gpu` row that says nothing about its device.
    fn version_1_journal(path: &Path) -> RegTarget {
        let gpu = RegTarget::new(Hive::Gpu, "Nvidia", "RmTest");
        let conn = Connection::open(path).unwrap();
        conn.execute_batch(MIGRATION_1).unwrap();
        conn.pragma_update(None, "user_version", 1).unwrap();
        conn.execute(
            "INSERT INTO snapshots (id, kind, tweak_ids, status, app_version, created_at_ms)
             VALUES (1, 'apply', '[\"gpu.test\"]', 'completed', '0.3.0', 0)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO changes (snapshot_id, seq, tweak_id, target, before_value, after_value, status, updated_at_ms)
             VALUES (1, 0, 'gpu.test', ?1, 'null', '{\"type\":\"dword\",\"data\":0}', 'verified', 0)",
            [to_json(&gpu).unwrap()],
        )
        .unwrap();
        gpu
    }

    #[test]
    fn a_version_1_journal_opens_and_keeps_its_rows() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.db");
        let gpu = version_1_journal(&path);

        let mut j = Journal::open(&path).unwrap();
        let version: i64 = j
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, SCHEMA_VERSION);

        let old = &j.undoable_changes("gpu.test").unwrap()[0];
        assert!(old.target.same_as(&gpu));
        assert_eq!(old.device, None, "a legacy row has no device");
        assert_eq!(old.status, ChangeStatus::Verified);
        assert_eq!(old.after, Some(RegValue::Dword(0)));
        assert!(j.undoable_tweaks().unwrap().contains("gpu.test"));

        // New rows carry their device beside it.
        let s = j
            .begin_snapshot(SnapshotKind::Apply, &["gpu.test".into()], None)
            .unwrap();
        j.record_change(
            s,
            0,
            "gpu.test",
            &gpu,
            Some("0001"),
            &None,
            &Some(RegValue::Dword(0)),
            ChangeStatus::Verified,
        )
        .unwrap();
        j.finish_snapshot(s, SnapshotStatus::Completed, None)
            .unwrap();
        assert_eq!(j.changes(s).unwrap()[0].device.as_deref(), Some("0001"));
        drop(j);

        // Opening again doesn't try to add the column a second time.
        let reopened = Journal::open(&path).unwrap();
        assert_eq!(reopened.undoable_changes("gpu.test").unwrap().len(), 2);
    }

    #[test]
    fn a_failed_migration_leaves_the_version_alone() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.db");
        version_1_journal(&path);
        // With the column already there, the migration fails.
        Connection::open(&path)
            .unwrap()
            .execute_batch(MIGRATION_2)
            .unwrap();

        assert!(matches!(Journal::open(&path), Err(AppError::Storage(_))));
        let version: i64 = Connection::open(&path)
            .unwrap()
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, 1, "nothing half-done is recorded as done");
    }

    #[test]
    fn a_rows_device_never_reaches_the_ui() {
        let mut j = Journal::open_in_memory().unwrap();
        let s = j.begin_snapshot(SnapshotKind::Apply, &[], None).unwrap();
        j.record_change(
            s,
            0,
            "t",
            &target(),
            Some("0008"),
            &None,
            &None,
            ChangeStatus::Intent,
        )
        .unwrap();
        let json = serde_json::to_value(&j.changes(s).unwrap()[0]).unwrap();
        assert!(json.get("device").is_none());
    }

    #[test]
    fn refuses_a_newer_schema() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.db");
        drop(Journal::open(&path).unwrap());
        Connection::open(&path)
            .unwrap()
            .pragma_update(None, "user_version", SCHEMA_VERSION + 1)
            .unwrap();
        assert!(matches!(Journal::open(&path), Err(AppError::Storage(_))));
    }

    #[test]
    fn snapshots_are_listed_newest_first() {
        let mut j = Journal::open_in_memory().unwrap();
        let a = j.begin_snapshot(SnapshotKind::Apply, &[], None).unwrap();
        let b = j
            .begin_snapshot(SnapshotKind::Revert, &[], Some(a))
            .unwrap();
        let list = j.snapshots(10).unwrap();
        assert_eq!(list.iter().map(|s| s.id).collect::<Vec<_>>(), vec![b, a]);
        assert_eq!(list[0].reverts, Some(a));
        assert_eq!(list[0].kind, SnapshotKind::Revert);
    }
}
