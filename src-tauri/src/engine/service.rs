//! The engine service: detect → plan → apply (journal, write, verify, roll back) → revert.
//!
//! This is the only code path that changes settings. Commands call it with tweak IDs, plan
//! IDs or snapshot IDs, never with targets or values.

use std::sync::{Mutex, MutexGuard};

use serde::Serialize;

use crate::engine::journal::{
    ChangeRecord, ChangeStatus, Journal, SnapshotKind, SnapshotRecord, SnapshotStatus, now_ms,
};
use crate::engine::plan::{
    PLAN_TTL_MS, Plan, PlanStore, PlannedChange, SkippedTweak, is_plan_id, new_plan_id,
};
use crate::engine::setting::{RegTarget, RegValue, describe};
use crate::engine::store::{LocalWriter, SettingStore, SettingWriter};
use crate::engine::{
    Compatibility, RestartRequirement, TweakContext, TweakMeta, TweakRegistry, check_compatibility,
};
use crate::error::{AppError, AppResult};

const MAX_TWEAKS_PER_PLAN: usize = 64;
pub const MAX_HISTORY: u32 = 200;

/// Supplies a writer for one batch of changes: in-process, or the elevated broker when
/// the batch touches HKLM and this process is not elevated.
pub trait Elevation: Send + Sync {
    fn writer<'a>(
        &'a self,
        store: &'a dyn SettingStore,
        needs_admin: bool,
    ) -> AppResult<Box<dyn SettingWriter + 'a>>;
}

/// Always writes in-process. Used by tests and when the app already runs elevated.
pub struct InProcess;

impl Elevation for InProcess {
    fn writer<'a>(
        &'a self,
        store: &'a dyn SettingStore,
        _: bool,
    ) -> AppResult<Box<dyn SettingWriter + 'a>> {
        Ok(Box::new(LocalWriter(store)))
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TweakState {
    Applied,
    NotApplied,
    /// Some but not all settings match.
    Partial,
    /// A setting could not be read (access denied, unsupported value type…).
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingStatus {
    pub target: RegTarget,
    pub target_label: String,
    pub current: Option<RegValue>,
    pub current_label: String,
    pub desired: Option<RegValue>,
    pub desired_label: String,
    pub matches: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakStatus {
    pub meta: TweakMeta,
    pub compatibility: Compatibility,
    pub state: TweakState,
    pub settings: Vec<SettingStatus>,
    /// 2VX has journaled changes for this tweak that it can undo (`revert_tweak`). `false`
    /// for a value set outside 2VX, which the app then leaves to the user.
    pub undoable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationOutcome {
    /// `None` when there was nothing to do.
    pub snapshot_id: Option<i64>,
    pub status: SnapshotStatus,
    pub error: Option<String>,
    pub changes: Vec<ChangeRecord>,
    pub restart: RestartRequirement,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineStatus {
    pub tweak_count: usize,
    /// Applying and reverting are disabled while the journal is unavailable.
    pub journal_available: bool,
    pub journal_error: Option<String>,
    /// Snapshots left unfinished by a crash and not yet reverted.
    pub interrupted_snapshots: usize,
}

pub struct Engine {
    registry: TweakRegistry,
    store: Box<dyn SettingStore>,
    elevation: Box<dyn Elevation>,
    /// `Err` holds why the journal is unavailable. The lock also serialises apply/revert.
    journal: Mutex<Result<Journal, String>>,
    plans: PlanStore,
    context: Box<dyn Fn() -> TweakContext + Send + Sync>,
}

impl Engine {
    pub fn new(
        registry: TweakRegistry,
        store: Box<dyn SettingStore>,
        elevation: Box<dyn Elevation>,
        journal: AppResult<Journal>,
        context: Box<dyn Fn() -> TweakContext + Send + Sync>,
    ) -> Self {
        let journal = journal
            .and_then(|mut j| {
                let recovered = j.recover_interrupted()?;
                if recovered > 0 {
                    tracing::warn!(recovered, "found interrupted snapshots from a previous run");
                }
                Ok(j)
            })
            .map_err(|e| {
                tracing::error!(error = %e, "change journal unavailable; applying is disabled");
                e.to_string()
            });
        Engine {
            registry,
            store,
            elevation,
            journal: Mutex::new(journal),
            plans: PlanStore::default(),
            context,
        }
    }

    pub fn registry(&self) -> &TweakRegistry {
        &self.registry
    }

    fn journal(&self) -> AppResult<MutexGuard<'_, Result<Journal, String>>> {
        let guard = self
            .journal
            .lock()
            .map_err(|_| AppError::Internal("journal lock poisoned".into()))?;
        if let Err(reason) = guard.as_ref() {
            return Err(AppError::Storage(reason.clone()));
        }
        Ok(guard)
    }

    pub fn status(&self) -> EngineStatus {
        let guard = self.journal.lock();
        let (available, error, interrupted) = match guard.as_deref() {
            Ok(Ok(j)) => (
                true,
                None,
                j.snapshots(MAX_HISTORY)
                    .map(|s| {
                        s.iter()
                            .filter(|s| s.status == SnapshotStatus::Interrupted)
                            .count()
                    })
                    .unwrap_or(0),
            ),
            Ok(Err(reason)) => (false, Some(reason.clone()), 0),
            Err(_) => (false, Some("journal lock poisoned".into()), 0),
        };
        EngineStatus {
            tweak_count: self.registry.len(),
            journal_available: available,
            journal_error: error,
            interrupted_snapshots: interrupted,
        }
    }

    pub fn tweak_statuses(&self) -> Vec<TweakStatus> {
        let ctx = (self.context)();
        // Without a journal nothing is undoable; the UI disables changes anyway.
        let undoable = self
            .journal()
            .ok()
            .and_then(|g| g.as_ref().ok().and_then(|j| j.undoable_tweaks().ok()))
            .unwrap_or_default();
        self.registry
            .iter()
            .map(|tweak| {
                let settings: Vec<SettingStatus> = tweak
                    .settings()
                    .iter()
                    .map(|s| {
                        let (current, error) = match self.store.read(&s.target) {
                            Ok(v) => (v, None),
                            Err(e) => (None, Some(e.to_string())),
                        };
                        SettingStatus {
                            target: s.target.clone(),
                            target_label: s.target.to_string(),
                            current_label: if error.is_some() {
                                "(unreadable)".into()
                            } else {
                                describe(&current)
                            },
                            matches: error.is_none() && current == s.value,
                            current,
                            desired: s.value.clone(),
                            desired_label: describe(&s.value),
                            error,
                        }
                    })
                    .collect();
                let state = if settings.iter().any(|s| s.error.is_some()) {
                    TweakState::Unknown
                } else if settings.iter().all(|s| s.matches) {
                    TweakState::Applied
                } else if settings.iter().any(|s| s.matches) {
                    TweakState::Partial
                } else {
                    TweakState::NotApplied
                };
                TweakStatus {
                    meta: tweak.meta().clone(),
                    compatibility: self.compatibility(tweak.meta(), &ctx),
                    state,
                    settings,
                    undoable: undoable.contains(tweak.meta().id),
                }
            })
            .collect()
    }

    /// [`check_compatibility`] plus what only the store can answer. An unreadable requirement
    /// doesn't block: its setting reports the read error instead.
    fn compatibility(&self, meta: &TweakMeta, ctx: &TweakContext) -> Compatibility {
        let compat = check_compatibility(meta, ctx);
        let missing = meta
            .requires
            .as_ref()
            .is_some_and(|t| matches!(self.store.read(t), Ok(None)));
        if !compat.blocks() && missing {
            Compatibility::NotInstalled
        } else {
            compat
        }
    }

    /// Dry run. Reads the system, changes nothing.
    pub fn plan(&self, tweak_ids: &[String]) -> AppResult<Plan> {
        if tweak_ids.is_empty() || tweak_ids.len() > MAX_TWEAKS_PER_PLAN {
            return Err(AppError::InvalidInput(format!(
                "select 1 to {MAX_TWEAKS_PER_PLAN} tweaks"
            )));
        }
        for id in tweak_ids {
            if self.registry.get(id).is_none() {
                return Err(AppError::InvalidInput(format!("unknown tweak `{id}`")));
            }
        }

        let ctx = (self.context)();
        let mut plan = Plan {
            id: new_plan_id()?,
            created_at_ms: now_ms(),
            expires_at_ms: now_ms() + PLAN_TTL_MS,
            tweak_ids: vec![],
            changes: vec![],
            already_applied: vec![],
            skipped: vec![],
            highest_risk: None,
            restart: RestartRequirement::None,
            needs_elevation: false,
        };
        // Registry order, not request order: plans are deterministic and duplicates collapse.
        for tweak in self
            .registry
            .iter()
            .filter(|t| tweak_ids.iter().any(|id| id == t.meta().id))
        {
            let meta = tweak.meta();
            let compat = self.compatibility(meta, &ctx);
            if compat.blocks() {
                plan.skipped.push(SkippedTweak {
                    tweak_id: meta.id.into(),
                    reason: compat,
                });
                continue;
            }
            let mut changed = false;
            for setting in tweak.settings() {
                let current = self.store.read(&setting.target)?;
                if current != setting.value {
                    plan.changes.push(PlannedChange::new(
                        meta.id,
                        setting.target.clone(),
                        current,
                        setting.value.clone(),
                    ));
                    changed = true;
                }
            }
            if changed {
                plan.tweak_ids.push(meta.id.into());
                plan.highest_risk = plan.highest_risk.max(Some(meta.risk));
                plan.restart = plan.restart.max(meta.restart);
            } else {
                plan.already_applied.push(meta.id.into());
            }
        }
        plan.needs_elevation =
            !ctx.is_elevated && plan.changes.iter().any(|c| c.target.hive.needs_elevation());
        self.plans.insert(plan.clone());
        Ok(plan)
    }

    /// Applies a confirmed plan: journal each before-value, write, read back, and roll the
    /// whole batch back if any step fails.
    pub fn apply(&self, plan_id: &str) -> AppResult<OperationOutcome> {
        if !is_plan_id(plan_id) {
            return Err(AppError::InvalidInput("malformed plan id".into()));
        }
        let mut guard = self.journal()?;
        let journal = guard.as_mut().expect("checked by journal()");
        let plan = self.plans.take(plan_id)?;
        if plan.changes.is_empty() {
            return Ok(OperationOutcome {
                snapshot_id: None,
                status: SnapshotStatus::Completed,
                error: None,
                changes: vec![],
                restart: RestartRequirement::None,
            });
        }

        // The user confirmed a diff; refuse if the system no longer matches it.
        for change in &plan.changes {
            let current = self.store.read(&change.target)?;
            if current != change.from {
                return Err(AppError::Conflict(format!(
                    "`{}` changed since the plan was made (now {}); review the changes again",
                    change.target,
                    describe(&current)
                )));
            }
        }

        let needs_admin = plan.changes.iter().any(|c| c.target.hive.needs_elevation());
        let mut writer = self.elevation.writer(self.store.as_ref(), needs_admin)?;
        let snapshot = journal.begin_snapshot(SnapshotKind::Apply, &plan.tweak_ids, None)?;
        journal.audit(
            "apply_started",
            serde_json::json!({ "snapshot": snapshot, "tweaks": plan.tweak_ids }),
        )?;

        // (change id, change, error) for everything that may have reached the system.
        let mut touched: Vec<(i64, &PlannedChange, Option<String>)> = vec![];
        let mut failure: Option<String> = None;
        for (seq, change) in plan.changes.iter().enumerate() {
            let change_id = match journal.record_change(
                snapshot,
                seq,
                &change.tweak_id,
                &change.target,
                &change.from,
                &change.to,
                ChangeStatus::Intent,
            ) {
                Ok(id) => id,
                // Nothing written for this change yet: safe to stop here.
                Err(e) => {
                    failure = Some(e.to_string());
                    break;
                }
            };
            let result = writer
                .write(&change.target, change.to.as_ref())
                .and_then(|read_back| {
                    if read_back == change.to {
                        Ok(())
                    } else {
                        Err(AppError::System(format!(
                            "`{}` reads back {} instead of {} (a policy may override it)",
                            change.target,
                            describe(&read_back),
                            describe(&change.to)
                        )))
                    }
                });
            let result = result
                .and_then(|()| journal.set_change_status(change_id, ChangeStatus::Verified, None));
            match result {
                Ok(()) => touched.push((change_id, change, None)),
                Err(e) => {
                    touched.push((change_id, change, Some(e.to_string())));
                    failure = Some(e.to_string());
                    break;
                }
            }
        }

        let (status, error) = match failure {
            None => (SnapshotStatus::Completed, None),
            Some(error) => {
                let clean = rollback(journal, self.store.as_ref(), writer.as_mut(), &touched);
                let status = if clean {
                    SnapshotStatus::RolledBack
                } else {
                    SnapshotStatus::Failed
                };
                (status, Some(error))
            }
        };
        drop(writer);
        journal.finish_snapshot(snapshot, status, error.as_deref())?;
        journal.audit(
            "apply_finished",
            serde_json::json!({ "snapshot": snapshot, "status": status.as_str() }),
        )?;
        tracing::info!(snapshot, status = status.as_str(), "apply finished");

        Ok(OperationOutcome {
            snapshot_id: Some(snapshot),
            status,
            error,
            changes: journal.changes(snapshot)?,
            restart: if status == SnapshotStatus::Completed {
                plan.restart
            } else {
                RestartRequirement::None
            },
        })
    }

    /// Restores the before-values of an apply snapshot. A value changed by something else
    /// since 2VX set it is left alone and reported as a conflict.
    pub fn revert(&self, snapshot_id: i64) -> AppResult<OperationOutcome> {
        let mut guard = self.journal()?;
        let journal = guard.as_mut().expect("checked by journal()");
        let original = journal
            .snapshot(snapshot_id)?
            .ok_or_else(|| AppError::InvalidInput(format!("unknown snapshot {snapshot_id}")))?;
        if original.kind != SnapshotKind::Apply {
            return Err(AppError::InvalidInput(
                "only apply snapshots can be reverted".into(),
            ));
        }
        if !matches!(
            original.status,
            SnapshotStatus::Completed | SnapshotStatus::Interrupted | SnapshotStatus::Failed
        ) {
            return Err(AppError::InvalidInput(format!(
                "snapshot {snapshot_id} is {}; nothing to revert",
                original.status.as_str()
            )));
        }

        // Intent and Failed rows may or may not have reached the system: the current
        // value decides below. Rolled-back and already-reverted rows are done.
        let mut pending: Vec<ChangeRecord> = journal
            .changes(snapshot_id)?
            .into_iter()
            .filter(|c| {
                matches!(
                    c.status,
                    ChangeStatus::Verified | ChangeStatus::Intent | ChangeStatus::Failed
                )
            })
            .collect();
        pending.reverse();
        let restart = original
            .tweak_ids
            .iter()
            .filter_map(|id| self.registry.get(id))
            .map(|t| t.meta().restart)
            .max()
            .unwrap_or(RestartRequirement::None);

        let ctx = (self.context)();
        let needs_admin =
            !ctx.is_elevated && pending.iter().any(|c| c.target.hive.needs_elevation());
        let mut writer = self.elevation.writer(self.store.as_ref(), needs_admin)?;
        let snapshot =
            journal.begin_snapshot(SnapshotKind::Revert, &original.tweak_ids, Some(snapshot_id))?;

        let pending: Vec<_> = pending.into_iter().map(|c| (c, Vec::new())).collect();
        let all_done = self.restore(journal, writer.as_mut(), snapshot, &pending)?;
        drop(writer);

        let (status, error) = if all_done {
            journal.set_snapshot_status(snapshot_id, SnapshotStatus::Reverted)?;
            (SnapshotStatus::Completed, None)
        } else {
            (
                SnapshotStatus::Failed,
                Some("some values could not be restored; see the change list".to_string()),
            )
        };
        journal.finish_snapshot(snapshot, status, error.as_deref())?;
        journal.audit(
            "revert_finished",
            serde_json::json!({ "snapshot": snapshot, "reverts": snapshot_id, "status": status.as_str() }),
        )?;
        tracing::info!(
            snapshot,
            reverts = snapshot_id,
            status = status.as_str(),
            "revert finished"
        );

        Ok(OperationOutcome {
            snapshot_id: Some(snapshot),
            status,
            error,
            changes: journal.changes(snapshot)?,
            restart: if status == SnapshotStatus::Completed {
                restart
            } else {
                RestartRequirement::None
            },
        })
    }

    /// Restores the before-value of each change, newest first as given, journaling every
    /// step in `snapshot`. A value changed by something else since 2VX set it is left alone
    /// and recorded as a conflict. Each entry carries older rows for the same target that
    /// are closed along with it. Returns `true` if every change was undone.
    fn restore(
        &self,
        journal: &mut Journal,
        writer: &mut dyn SettingWriter,
        snapshot: i64,
        pending: &[(ChangeRecord, Vec<i64>)],
    ) -> AppResult<bool> {
        let mut all_done = true;
        for (seq, (change, superseded)) in pending.iter().enumerate() {
            let current = match self.store.read(&change.target) {
                Ok(v) => v,
                Err(e) => {
                    journal
                        .record_change(
                            snapshot,
                            seq,
                            &change.tweak_id,
                            &change.target,
                            &None,
                            &change.before,
                            ChangeStatus::Failed,
                        )
                        .and_then(|id| {
                            journal.set_change_status(
                                id,
                                ChangeStatus::Failed,
                                Some(&e.to_string()),
                            )
                        })?;
                    all_done = false;
                    continue;
                }
            };
            if current == change.before {
                journal.record_change(
                    snapshot,
                    seq,
                    &change.tweak_id,
                    &change.target,
                    &current,
                    &change.before,
                    ChangeStatus::Unchanged,
                )?;
                mark_reverted(journal, change.id, superseded)?;
                continue;
            }
            if current != change.after {
                let id = journal.record_change(
                    snapshot,
                    seq,
                    &change.tweak_id,
                    &change.target,
                    &current,
                    &change.before,
                    ChangeStatus::Conflict,
                )?;
                journal.set_change_status(
                    id,
                    ChangeStatus::Conflict,
                    Some(&format!(
                        "changed by something else since 2VX set it (now {}); left as is",
                        describe(&current)
                    )),
                )?;
                all_done = false;
                continue;
            }
            let id = journal.record_change(
                snapshot,
                seq,
                &change.tweak_id,
                &change.target,
                &current,
                &change.before,
                ChangeStatus::Intent,
            )?;
            match writer.write(&change.target, change.before.as_ref()) {
                Ok(read_back) if read_back == change.before => {
                    journal.set_change_status(id, ChangeStatus::Verified, None)?;
                    mark_reverted(journal, change.id, superseded)?;
                }
                Ok(read_back) => {
                    journal.set_change_status(
                        id,
                        ChangeStatus::Failed,
                        Some(&format!(
                            "reads back {} after restore",
                            describe(&read_back)
                        )),
                    )?;
                    all_done = false;
                }
                Err(e) => {
                    journal.set_change_status(id, ChangeStatus::Failed, Some(&e.to_string()))?;
                    all_done = false;
                }
            }
        }
        Ok(all_done)
    }

    /// Undoes what 2VX changed for one tweak, across every apply snapshot that touched it.
    /// Per target only the newest record counts: it holds the value from before 2VX's latest
    /// change. Older records for that target are superseded and closed with it.
    pub fn revert_tweak(&self, tweak_id: &str) -> AppResult<OperationOutcome> {
        self.revert_tweaks(&[tweak_id.to_string()])
    }

    /// [`Self::revert_tweak`] for several tweaks as one batch, so one prompt covers them all.
    /// A tweak with nothing recorded is passed over unless none has anything.
    pub fn revert_tweaks(&self, tweak_ids: &[String]) -> AppResult<OperationOutcome> {
        let mut restart = RestartRequirement::None;
        for id in tweak_ids {
            let tweak = self
                .registry
                .get(id)
                .ok_or_else(|| AppError::InvalidInput(format!("unknown tweak `{id}`")))?;
            restart = restart.max(tweak.meta().restart);
        }
        let mut guard = self.journal()?;
        let journal = guard.as_mut().expect("checked by journal()");
        let mut rows = vec![];
        for id in tweak_ids {
            rows.extend(journal.undoable_changes(id)?);
        }
        if rows.is_empty() {
            return Err(AppError::InvalidInput(
                "2VX has no record of changing this tweak, so there is nothing to undo".into(),
            ));
        }
        let mut originals: Vec<i64> = rows.iter().map(|c| c.snapshot_id).collect();
        originals.sort_unstable();
        originals.dedup();
        let mut pending: Vec<(ChangeRecord, Vec<i64>)> = Vec::new();
        for row in rows {
            match pending
                .iter_mut()
                .find(|(newest, _)| newest.target.same_as(&row.target))
            {
                Some((_, superseded)) => superseded.push(row.id),
                None => pending.push((row, Vec::new())),
            }
        }

        let ctx = (self.context)();
        let needs_admin =
            !ctx.is_elevated && pending.iter().any(|(c, _)| c.target.hive.needs_elevation());
        let mut writer = self.elevation.writer(self.store.as_ref(), needs_admin)?;
        let reverts = (originals.len() == 1).then(|| originals[0]);
        let snapshot = journal.begin_snapshot(SnapshotKind::Revert, tweak_ids, reverts)?;
        let all_done = self.restore(journal, writer.as_mut(), snapshot, &pending)?;
        drop(writer);

        for original in originals {
            if !journal.has_undoable_changes(original)? {
                journal.set_snapshot_status(original, SnapshotStatus::Reverted)?;
            }
        }
        let (status, error) = if all_done {
            (SnapshotStatus::Completed, None)
        } else {
            (
                SnapshotStatus::Failed,
                Some("some values could not be restored; see the change list".to_string()),
            )
        };
        journal.finish_snapshot(snapshot, status, error.as_deref())?;
        journal.audit(
            "tweak_revert_finished",
            serde_json::json!({ "snapshot": snapshot, "tweaks": tweak_ids, "status": status.as_str() }),
        )?;
        tracing::info!(
            snapshot,
            tweaks = tweak_ids.len(),
            status = status.as_str(),
            "tweak revert finished"
        );

        Ok(OperationOutcome {
            snapshot_id: Some(snapshot),
            status,
            error,
            changes: journal.changes(snapshot)?,
            restart: if status == SnapshotStatus::Completed {
                restart
            } else {
                RestartRequirement::None
            },
        })
    }

    /// Records work that doesn't pass through the journal's snapshots: cleaning, startup
    /// switches, removing apps. That work is already done, so a failure here is only logged.
    pub fn audit(&self, event: &str, detail: serde_json::Value) {
        let Ok(mut guard) = self.journal.lock() else {
            return;
        };
        if let Ok(journal) = guard.as_mut()
            && let Err(e) = journal.audit(event, detail)
        {
            tracing::warn!(event, error = %e, "couldn't record an audit event");
        }
    }

    pub fn history(&self, limit: u32) -> AppResult<Vec<SnapshotRecord>> {
        let guard = self.journal()?;
        guard
            .as_ref()
            .expect("checked by journal()")
            .snapshots(limit.clamp(1, MAX_HISTORY))
    }

    pub fn snapshot_changes(&self, snapshot_id: i64) -> AppResult<Vec<ChangeRecord>> {
        let guard = self.journal()?;
        let journal = guard.as_ref().expect("checked by journal()");
        if journal.snapshot(snapshot_id)?.is_none() {
            return Err(AppError::InvalidInput(format!(
                "unknown snapshot {snapshot_id}"
            )));
        }
        journal.changes(snapshot_id)
    }
}

fn mark_reverted(journal: &mut Journal, id: i64, superseded: &[i64]) -> AppResult<()> {
    journal.set_change_status(id, ChangeStatus::Reverted, None)?;
    for &older in superseded {
        journal.set_change_status(older, ChangeStatus::Reverted, None)?;
    }
    Ok(())
}

/// Restores before-values in reverse order. Returns `true` if every change was undone.
fn rollback(
    journal: &mut Journal,
    store: &dyn SettingStore,
    writer: &mut dyn SettingWriter,
    touched: &[(i64, &PlannedChange, Option<String>)],
) -> bool {
    let mut clean = true;
    for (change_id, change, error) in touched.iter().rev() {
        // A write that failed usually changed nothing; only write back when needed, so a
        // target that refuses writes does not turn a clean rollback into a failure.
        let untouched = store
            .read(&change.target)
            .is_ok_and(|current| current == change.from);
        let restored = untouched
            || writer
                .write(&change.target, change.from.as_ref())
                .is_ok_and(|read_back| read_back == change.from);
        let status = if restored {
            ChangeStatus::RolledBack
        } else {
            ChangeStatus::Failed
        };
        let note = match (restored, error) {
            (true, e) => e.clone(),
            (false, Some(e)) => Some(format!("{e}; rollback also failed")),
            (false, None) => Some("rollback failed".to_string()),
        };
        clean &= restored;
        if let Err(e) = journal.set_change_status(*change_id, status, note.as_deref()) {
            tracing::error!(error = %e, change_id, "could not record rollback");
            clean = false;
        }
    }
    clean
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::engine::setting::Hive;
    use crate::engine::store::MemoryStore;
    use crate::engine::tests::{ADMIN_SETTINGS, USER_SETTINGS, ctx, meta, tweak};
    use crate::engine::{Permission, RiskLevel, Setting, TweakMeta};

    /// Shares a MemoryStore between the engine and the test so the test can inspect and poke it.
    struct Shared(Arc<MemoryStore>);
    impl SettingStore for Shared {
        fn read(&self, t: &RegTarget) -> AppResult<Option<RegValue>> {
            self.0.read(t)
        }
        fn write(&self, t: &RegTarget, v: Option<&RegValue>) -> AppResult<()> {
            self.0.write(t, v)
        }
    }

    const ONE: RegTarget = RegTarget::new(Hive::CurrentUser, r"Software\2VX-Test\A", "One");
    const GONE: RegTarget = RegTarget::new(Hive::CurrentUser, r"Software\2VX-Test\A", "Gone");
    const ADMIN: RegTarget = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\2VX-Test", "Admin");
    const SECOND: &[Setting] = &[Setting::dword(
        Hive::CurrentUser,
        r"Software\2VX-Test\B",
        "Two",
        2,
    )];
    const TWO: RegTarget = RegTarget::new(Hive::CurrentUser, r"Software\2VX-Test\B", "Two");
    const FUTURE: &[Setting] = &[Setting::dword(
        Hive::CurrentUser,
        r"Software\2VX-Test\F",
        "F",
        1,
    )];

    fn engine(store: Arc<MemoryStore>, elevated: bool) -> Engine {
        let mut registry = TweakRegistry::default();
        registry
            .register(tweak(meta("gaming.a"), USER_SETTINGS))
            .unwrap();
        registry
            .register(tweak(
                TweakMeta {
                    restart: RestartRequirement::Reboot,
                    ..meta("gaming.b")
                },
                SECOND,
            ))
            .unwrap();
        registry
            .register(tweak(
                TweakMeta {
                    permission: Permission::Administrator,
                    risk: RiskLevel::Moderate,
                    ..meta("power.admin")
                },
                ADMIN_SETTINGS,
            ))
            .unwrap();
        registry
            .register(tweak(
                TweakMeta {
                    supported_builds: crate::engine::BuildRange {
                        min_build: 99_999,
                        max_build: None,
                    },
                    ..meta("gaming.future")
                },
                FUTURE,
            ))
            .unwrap();
        Engine::new(
            registry,
            Box::new(Shared(store)),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(move || ctx(26_100, elevated)),
        )
    }

    fn ids(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn plan_is_a_side_effect_free_diff() {
        let store = Arc::new(MemoryStore::with([(GONE, RegValue::Dword(5))]));
        let e = engine(store.clone(), false);
        let plan = e
            .plan(&ids(&["gaming.a", "power.admin", "gaming.future"]))
            .unwrap();

        assert_eq!(plan.tweak_ids, ids(&["gaming.a", "power.admin"]));
        assert_eq!(plan.changes.len(), 3);
        assert_eq!(plan.changes[0].target, ONE);
        assert_eq!(plan.changes[0].from, None);
        assert_eq!(plan.changes[1].target, GONE);
        assert_eq!(plan.changes[1].to, None);
        assert_eq!(plan.changes[2].target, ADMIN);
        assert_eq!(plan.skipped[0].tweak_id, "gaming.future");
        assert_eq!(plan.highest_risk, Some(RiskLevel::Moderate));
        assert!(plan.needs_elevation);
        // Nothing was written.
        assert_eq!(store.get(&ONE), None);
        assert_eq!(store.get(&GONE), Some(RegValue::Dword(5)));
    }

    #[test]
    fn plan_validates_ids() {
        let e = engine(Arc::new(MemoryStore::default()), false);
        assert!(matches!(e.plan(&[]), Err(AppError::InvalidInput(_))));
        assert!(matches!(
            e.plan(&ids(&["nope.nope"])),
            Err(AppError::InvalidInput(_))
        ));
    }

    #[test]
    fn apply_then_revert_restores_exact_before_values() {
        let store = Arc::new(MemoryStore::with([(
            GONE,
            RegValue::String("keep me".into()),
        )]));
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.a", "gaming.b"])).unwrap();
        assert_eq!(plan.restart, RestartRequirement::Reboot);

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.restart, RestartRequirement::Reboot);
        assert!(
            out.changes
                .iter()
                .all(|c| c.status == ChangeStatus::Verified)
        );
        assert_eq!(store.get(&ONE), Some(RegValue::Dword(1)));
        assert_eq!(store.get(&GONE), None);
        assert_eq!(store.get(&TWO), Some(RegValue::Dword(2)));
        assert!(
            e.tweak_statuses()
                .iter()
                .filter(|s| s.meta.id.starts_with("gaming.") && s.meta.id != "gaming.future")
                .all(|s| s.state == TweakState::Applied)
        );

        // Plans are single use.
        assert!(e.apply(&plan.id).is_err());

        let snapshot = out.snapshot_id.unwrap();
        let rev = e.revert(snapshot).unwrap();
        assert_eq!(rev.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&ONE), None);
        assert_eq!(store.get(&GONE), Some(RegValue::String("keep me".into())));
        assert_eq!(store.get(&TWO), None);

        let history = e.history(10).unwrap();
        assert_eq!(history[0].kind, SnapshotKind::Revert);
        assert_eq!(history[1].status, SnapshotStatus::Reverted);
        assert!(e.revert(snapshot).is_err(), "already reverted");
    }

    #[test]
    fn revert_tweak_undoes_one_tweak_of_a_batch() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let undoable = |e: &Engine, id: &str| {
            e.tweak_statuses()
                .iter()
                .find(|s| s.meta.id == id)
                .unwrap()
                .undoable
        };
        assert!(!undoable(&e, "gaming.a"));
        assert!(e.revert_tweak("gaming.a").is_err(), "nothing recorded yet");
        assert!(e.revert_tweak("gaming.nope").is_err());

        let plan = e.plan(&ids(&["gaming.a", "gaming.b"])).unwrap();
        let batch = e.apply(&plan.id).unwrap().snapshot_id.unwrap();
        assert!(undoable(&e, "gaming.a") && undoable(&e, "gaming.b"));

        let out = e.revert_tweak("gaming.b").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.restart, RestartRequirement::Reboot);
        assert_eq!(store.get(&TWO), None);
        assert_eq!(
            store.get(&ONE),
            Some(RegValue::Dword(1)),
            "other tweak kept"
        );
        assert!(!undoable(&e, "gaming.b"));
        assert_eq!(e.history(10).unwrap()[1].status, SnapshotStatus::Completed);

        // Undoing the last tweak of the batch closes the batch too.
        e.revert_tweak("gaming.a").unwrap();
        assert_eq!(store.get(&ONE), None);
        let history = e.history(10).unwrap();
        assert_eq!(history[0].reverts, Some(batch));
        assert_eq!(history[2].status, SnapshotStatus::Reverted);
        assert!(e.revert(batch).is_err(), "already reverted");
    }

    #[test]
    fn revert_tweak_restores_the_value_from_before_the_latest_apply() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        let first = e.apply(&plan.id).unwrap().snapshot_id.unwrap();
        // Something else sets its own value, then the user applies the tweak again.
        store.write(&TWO, Some(&RegValue::Dword(7))).unwrap();
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        e.apply(&plan.id).unwrap();

        let out = e.revert_tweak("gaming.b").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.changes.len(), 1, "superseded record is not replayed");
        assert_eq!(store.get(&TWO), Some(RegValue::Dword(7)));
        let history = e.history(10).unwrap();
        assert!(
            history
                .iter()
                .filter(|s| s.kind == SnapshotKind::Apply)
                .all(|s| s.status == SnapshotStatus::Reverted)
        );
        assert!(e.revert(first).is_err());
    }

    #[test]
    fn revert_tweak_leaves_a_value_changed_by_others() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        e.apply(&plan.id).unwrap();
        store.write(&TWO, Some(&RegValue::Dword(9))).unwrap();

        let out = e.revert_tweak("gaming.b").unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.restart, RestartRequirement::None);
        assert_eq!(out.changes[0].status, ChangeStatus::Conflict);
        assert_eq!(store.get(&TWO), Some(RegValue::Dword(9)));
    }

    #[test]
    fn failed_write_rolls_back_the_whole_batch() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.a", "gaming.b"])).unwrap();
        store.fail_writes_to(&TWO);

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::RolledBack);
        assert!(out.error.is_some());
        assert_eq!(store.get(&ONE), None, "earlier change undone");
        let statuses: Vec<_> = out.changes.iter().map(|c| c.status).collect();
        // `Gone` is already absent, so the batch is One + Two.
        assert_eq!(statuses, vec![ChangeStatus::RolledBack; 2]);
    }

    /// First write stores a wrong value, every later write is refused.
    struct Corrupting {
        inner: MemoryStore,
        writes: Mutex<u32>,
    }
    impl SettingStore for Corrupting {
        fn read(&self, t: &RegTarget) -> AppResult<Option<RegValue>> {
            self.inner.read(t)
        }
        fn write(&self, t: &RegTarget, _: Option<&RegValue>) -> AppResult<()> {
            let mut writes = self.writes.lock().unwrap();
            *writes += 1;
            if *writes > 1 {
                return Err(AppError::System("refused".into()));
            }
            self.inner.write(t, Some(&RegValue::Dword(77)))
        }
    }

    #[test]
    fn unrestorable_value_marks_the_snapshot_failed() {
        let mut registry = TweakRegistry::default();
        registry.register(tweak(meta("gaming.b"), SECOND)).unwrap();
        let store = Corrupting {
            inner: MemoryStore::default(),
            writes: Mutex::new(0),
        };
        let e = Engine::new(
            registry,
            Box::new(store),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, true)),
        );
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes[0].status, ChangeStatus::Failed);
        assert!(
            out.changes[0]
                .error
                .as_deref()
                .unwrap()
                .contains("rollback also failed")
        );
        // Still revertible later from the recorded before-value.
        assert_eq!(e.history(1).unwrap()[0].status, SnapshotStatus::Failed);
    }

    #[test]
    fn readback_mismatch_counts_as_failure() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        store.ignore_writes_to(&TWO);
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::RolledBack);
        assert!(out.error.unwrap().contains("reads back"));
    }

    #[test]
    fn stale_plan_is_refused_without_changes() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        store.set(&TWO, Some(RegValue::Dword(7)));
        assert!(matches!(e.apply(&plan.id), Err(AppError::Conflict(_))));
        assert_eq!(store.get(&TWO), Some(RegValue::Dword(7)));
        assert!(
            e.history(10).unwrap().is_empty(),
            "no snapshot for a refused plan"
        );
    }

    #[test]
    fn revert_leaves_values_changed_by_others() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.a", "gaming.b"])).unwrap();
        let snapshot = e.apply(&plan.id).unwrap().snapshot_id.unwrap();
        store.set(&TWO, Some(RegValue::Dword(42))); // the user changed it elsewhere

        let rev = e.revert(snapshot).unwrap();
        assert_eq!(rev.status, SnapshotStatus::Failed);
        assert_eq!(store.get(&TWO), Some(RegValue::Dword(42)), "not clobbered");
        assert_eq!(store.get(&ONE), None, "other values still restored");
        assert!(
            rev.changes
                .iter()
                .any(|c| c.status == ChangeStatus::Conflict)
        );
        // The apply snapshot stays revertible once the conflict is resolved.
        store.set(&TWO, Some(RegValue::Dword(2)));
        assert_eq!(
            e.revert(snapshot).unwrap().status,
            SnapshotStatus::Completed
        );
        assert_eq!(store.get(&TWO), None);
    }

    #[test]
    fn interrupted_apply_can_be_reverted_after_restart() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.db");
        let store = Arc::new(MemoryStore::default());
        {
            // Simulate a crash: intent recorded and value written, but never verified or finished.
            let mut j = Journal::open(&path).unwrap();
            let s = j
                .begin_snapshot(SnapshotKind::Apply, &ids(&["gaming.b"]), None)
                .unwrap();
            j.record_change(
                s,
                0,
                "gaming.b",
                &TWO,
                &None,
                &Some(RegValue::Dword(2)),
                ChangeStatus::Intent,
            )
            .unwrap();
            store.set(&TWO, Some(RegValue::Dword(2)));
        }
        let mut registry = TweakRegistry::default();
        registry.register(tweak(meta("gaming.b"), SECOND)).unwrap();
        let e = Engine::new(
            registry,
            Box::new(Shared(store.clone())),
            Box::new(InProcess),
            Journal::open(&path),
            Box::new(|| ctx(26_100, true)),
        );
        assert_eq!(e.status().interrupted_snapshots, 1);
        let snapshot = e.history(10).unwrap()[0].id;
        assert_eq!(
            e.revert(snapshot).unwrap().status,
            SnapshotStatus::Completed
        );
        assert_eq!(store.get(&TWO), None);
    }

    #[test]
    fn unavailable_journal_blocks_apply_but_not_detection() {
        let e = Engine::new(
            TweakRegistry::default(),
            Box::new(MemoryStore::default()),
            Box::new(InProcess),
            Err(AppError::Storage("disk full".into())),
            Box::new(|| ctx(26_100, true)),
        );
        let status = e.status();
        assert!(!status.journal_available);
        assert_eq!(
            status.journal_error.as_deref(),
            Some("change journal error: disk full")
        );
        assert!(e.tweak_statuses().is_empty());
        assert!(matches!(
            e.apply(&"0".repeat(32)),
            Err(AppError::Storage(_))
        ));
        assert!(matches!(e.history(10), Err(AppError::Storage(_))));
    }

    #[test]
    fn empty_plan_applies_as_no_op() {
        let store = Arc::new(MemoryStore::with([(TWO, RegValue::Dword(2))]));
        let e = engine(store, true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        assert_eq!(plan.already_applied, ids(&["gaming.b"]));
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.snapshot_id, None);
    }
}
