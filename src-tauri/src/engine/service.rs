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

/// Above the largest preset (Privacy, 157). Nothing downstream is sized to a batch: the broker sends
/// one frame per change, the journal stores the id list as one text column, and a plan's expiry only
/// ends the review, so this bounds a single request rather than a resource.
const MAX_TWEAKS_PER_PLAN: usize = 512;
/// The Restore Centre lists the whole history. An index keeps each row's change count cheap.
pub const MAX_HISTORY: u32 = 5_000;

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
    /// Tweaks of the plan that weren't applied. Left out of the JSON when there are none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skipped: Vec<SkippedTweak>,
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
            Ok(Ok(j)) => (true, None, j.interrupted_count().unwrap_or(0)),
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
        let mut statuses = Vec::new();
        // Every adapter setting would look the adapter up again otherwise, about 30 times a pass.
        self.store.pass(&mut || {
            statuses = self
                .registry
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
                .collect();
        });
        statuses
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
                    detail: None,
                });
                continue;
            }
            // Every setting is read before any is planned: a tweak that can't be backed up exactly
            // is left out whole, never half-applied, and the rest of the batch still goes ahead.
            let current: AppResult<Vec<_>> = tweak
                .settings()
                .iter()
                .map(|setting| {
                    let device = self.store.device_of(&setting.target)?;
                    Ok((self.store.read(&setting.target)?, device))
                })
                .collect();
            let current = match current {
                Ok(current) => current,
                Err(e) => {
                    plan.skipped.push(SkippedTweak {
                        tweak_id: meta.id.into(),
                        reason: Compatibility::Unreadable,
                        detail: Some(e.to_string()),
                    });
                    continue;
                }
            };
            let changes: Vec<PlannedChange> = tweak
                .settings()
                .iter()
                .zip(current)
                .filter(|(setting, (current, _))| *current != setting.value)
                .map(|(setting, (current, device))| {
                    let change = PlannedChange::new(
                        meta.id,
                        setting.target.clone(),
                        current,
                        setting.value.clone(),
                    );
                    match device {
                        Some(device) => {
                            let name = self.device_label(&setting.target, &device);
                            change.on_device(device, &name)
                        }
                        None => change,
                    }
                })
                .collect();
            if !changes.is_empty() {
                plan.changes.extend(changes);
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
                skipped: plan.skipped,
            });
        }

        // The user confirmed a diff; refuse if the system no longer matches it.
        for change in &plan.changes {
            self.check_device(change)?;
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
            // The admin prompt stays up as long as the user leaves it, and a link can move in that time.
            if let Err(e) = self.check_device(change) {
                failure = Some(e.to_string());
                break;
            }
            let change_id = match journal.record_change(
                snapshot,
                seq,
                &change.tweak_id,
                &change.target,
                change.device.as_deref(),
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
            skipped: plan.skipped,
        })
    }

    /// What to call a driver instance. Its own name goes with the device, and the instance is
    /// then what's left to tell a person which one it was.
    fn device_label(&self, target: &RegTarget, device: &str) -> String {
        self.store
            .device_name(target, device)
            .unwrap_or_else(|| format!("driver {device}"))
    }

    /// Refuses a change whose target no longer lands on the device it was planned against: the user
    /// agreed to that device, and its before-value says nothing about another one.
    fn check_device(&self, change: &PlannedChange) -> AppResult<()> {
        match self.store.device_of(&change.target) {
            Ok(now) if now == change.device => Ok(()),
            _ => Err(AppError::Conflict(format!(
                "`{}` now points at another device; review the changes again",
                change.target
            ))),
        }
    }

    /// Whether undoing a row can reach the device it changed. Nothing is written otherwise, so it
    /// doesn't count towards asking for elevation.
    fn can_reach(&self, change: &ChangeRecord) -> bool {
        self.store
            .device_of(&change.target)
            .is_ok_and(|now| on_recorded_device(change.device.as_deref(), now.as_deref()))
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
        let needs_admin = !ctx.is_elevated
            && pending
                .iter()
                .any(|c| c.target.hive.needs_elevation() && self.can_reach(c));
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
            skipped: Vec::new(),
        })
    }

    /// Restores the before-value of each change, newest first as given, journaling every
    /// step in `snapshot`. A value changed by something else since 2VX set it is left alone
    /// and recorded as a conflict, and so is a change made on a device the target no longer
    /// lands on: its backup means nothing on another one, and closing the row would lose it.
    /// Each entry carries older rows for the same target that are closed along with it.
    /// Returns `true` if every change was undone.
    fn restore(
        &self,
        journal: &mut Journal,
        writer: &mut dyn SettingWriter,
        snapshot: i64,
        pending: &[(ChangeRecord, Vec<i64>)],
    ) -> AppResult<bool> {
        let mut all_done = true;
        for (seq, (change, superseded)) in pending.iter().enumerate() {
            let row = RevertRow {
                snapshot,
                seq,
                change,
                device: change.device.as_deref(),
            };
            let now = match self.store.device_of(&change.target) {
                Ok(now) => now,
                Err(e) => {
                    row.leave_open(journal, &None, ChangeStatus::Failed, &e.to_string())?;
                    all_done = false;
                    continue;
                }
            };
            if !on_recorded_device(change.device.as_deref(), now.as_deref()) {
                let old = self.device_label(&change.target, row.device.unwrap_or_default());
                row.leave_open(
                    journal,
                    &None,
                    ChangeStatus::Conflict,
                    &format!(
                        "changed on {old}, which isn't in use now; left as is until it's back"
                    ),
                )?;
                all_done = false;
                continue;
            }
            // A row from before devices were recorded gets one from here on.
            let row = RevertRow {
                device: row.device.or(now.as_deref()),
                ..row
            };
            let current = match self.store.read(&change.target) {
                Ok(v) => v,
                Err(e) => {
                    row.leave_open(journal, &None, ChangeStatus::Failed, &e.to_string())?;
                    all_done = false;
                    continue;
                }
            };
            if current == change.before {
                row.record(journal, &current, ChangeStatus::Unchanged)?;
                mark_reverted(journal, change.id, superseded)?;
                continue;
            }
            if current != change.after {
                row.leave_open(
                    journal,
                    &current,
                    ChangeStatus::Conflict,
                    &format!(
                        "changed by something else since 2VX set it (now {}); left as is",
                        describe(&current)
                    ),
                )?;
                all_done = false;
                continue;
            }
            let id = row.record(journal, &current, ChangeStatus::Intent)?;
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
    /// Per target and device only the newest record counts: it holds the value from before 2VX's
    /// latest change there. Older records for that pair are superseded and closed with it.
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
            match pending.iter_mut().find(|(newest, _)| {
                newest.target.same_as(&row.target) && same_device(&newest.device, &row.device)
            }) {
                Some((_, superseded)) => superseded.push(row.id),
                None => pending.push((row, Vec::new())),
            }
        }

        let ctx = (self.context)();
        let needs_admin = !ctx.is_elevated
            && pending
                .iter()
                .any(|(c, _)| c.target.hive.needs_elevation() && self.can_reach(c));
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
            skipped: Vec::new(),
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

/// One row of a revert snapshot.
struct RevertRow<'a> {
    snapshot: i64,
    seq: usize,
    change: &'a ChangeRecord,
    device: Option<&'a str>,
}

impl RevertRow<'_> {
    fn record(
        &self,
        journal: &mut Journal,
        current: &Option<RegValue>,
        status: ChangeStatus,
    ) -> AppResult<i64> {
        journal.record_change(
            self.snapshot,
            self.seq,
            &self.change.tweak_id,
            &self.change.target,
            self.device,
            current,
            &self.change.before,
            status,
        )
    }

    /// Records why nothing was restored. The change itself stays open, so undo can try it again.
    fn leave_open(
        &self,
        journal: &mut Journal,
        current: &Option<RegValue>,
        status: ChangeStatus,
        why: &str,
    ) -> AppResult<()> {
        let id = self.record(journal, current, status)?;
        journal.set_change_status(id, status, Some(why))
    }
}

/// `None` on the left is a row from before schema 2, which matches any device as it always did.
fn on_recorded_device(recorded: Option<&str>, now: Option<&str>) -> bool {
    recorded.is_none() || recorded == now
}

/// Rows on one target can only stand for each other on one device. A row with none recorded could
/// be either, and is grouped as it was before devices were recorded.
fn same_device(a: &Option<String>, b: &Option<String>) -> bool {
    a.is_none() || b.is_none() || a == b
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
        // If the target has moved, a rollback here would land on hardware the user never tweaked.
        // The row stays open as failed, for undo to finish once the device is back.
        let here = store
            .device_of(&change.target)
            .is_ok_and(|now| now == change.device);
        // A write that failed usually changed nothing; only write back when needed, so a
        // target that refuses writes does not turn a clean rollback into a failure.
        let untouched = here
            && store
                .read(&change.target)
                .is_ok_and(|current| current == change.from);
        let restored = here
            && (untouched
                || writer
                    .write(&change.target, change.from.as_ref())
                    .is_ok_and(|read_back| read_back == change.from));
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
        fn device_of(&self, t: &RegTarget) -> AppResult<Option<String>> {
            self.0.device_of(t)
        }
        fn device_name(&self, t: &RegTarget, device: &str) -> Option<String> {
            self.0.device_name(t, device)
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
    fn plan_skips_a_tweak_it_cannot_read_and_keeps_the_rest() {
        let store = Arc::new(MemoryStore::default());
        store.fail_reads_of(&GONE);
        let e = engine(store.clone(), true);
        let plan = e.plan(&ids(&["gaming.a", "gaming.b"])).unwrap();

        assert_eq!(plan.tweak_ids, ids(&["gaming.b"]));
        // `One` was readable, but half a tweak isn't planned.
        assert!(plan.changes.iter().all(|c| c.tweak_id == "gaming.b"));
        assert_eq!(plan.skipped.len(), 1);
        assert_eq!(plan.skipped[0].tweak_id, "gaming.a");
        assert_eq!(plan.skipped[0].reason, Compatibility::Unreadable);
        assert!(plan.skipped[0].detail.as_deref().unwrap().contains("Gone"));
    }

    /// `count` tweaks, each with a setting of its own.
    fn many(store: Arc<MemoryStore>, count: usize) -> (Engine, Vec<String>) {
        let mut registry = TweakRegistry::default();
        let mut tweak_ids = Vec::new();
        for i in 0..count {
            let id: &'static str = format!("gaming.many-{i}").leak();
            let key: &'static str = format!(r"Software\2VX-Test\Many\{i}").leak();
            let settings = vec![Setting::dword(Hive::CurrentUser, key, "Value", 1)].leak();
            registry.register(tweak(meta(id), settings)).unwrap();
            tweak_ids.push(id.to_string());
        }
        let e = Engine::new(
            registry,
            Box::new(Shared(store)),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, true)),
        );
        (e, tweak_ids)
    }

    #[test]
    fn a_plan_takes_the_largest_preset() {
        let store = Arc::new(MemoryStore::default());
        let (e, tweak_ids) = many(store.clone(), 157);
        let plan = e.plan(&tweak_ids).unwrap();
        assert_eq!(plan.tweak_ids.len(), 157);
        assert_eq!(plan.changes.len(), 157);

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.changes.len(), 157);
        assert!(
            out.changes
                .iter()
                .all(|c| c.status == ChangeStatus::Verified)
        );

        let snapshot = out.snapshot_id.unwrap();
        assert_eq!(
            e.revert(snapshot).unwrap().status,
            SnapshotStatus::Completed
        );
        let first = RegTarget::new(Hive::CurrentUser, r"Software\2VX-Test\Many\0", "Value");
        assert_eq!(store.get(&first), None);
    }

    #[test]
    fn a_plan_stops_at_the_limit() {
        let (e, mut tweak_ids) = many(Arc::new(MemoryStore::default()), MAX_TWEAKS_PER_PLAN + 1);
        let over = tweak_ids.pop().unwrap();
        assert_eq!(tweak_ids.len(), 512);
        assert_eq!(e.plan(&tweak_ids).unwrap().changes.len(), 512);

        tweak_ids.push(over);
        assert_eq!(tweak_ids.len(), 513);
        assert!(matches!(e.plan(&tweak_ids), Err(AppError::InvalidInput(_))));
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
                None,
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

    #[test]
    fn the_outcome_says_what_was_skipped() {
        let e = engine(Arc::new(MemoryStore::default()), true);
        let plan = e.plan(&ids(&["gaming.a", "gaming.future"])).unwrap();
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.skipped.len(), 1);
        assert_eq!(out.skipped[0].tweak_id, "gaming.future");

        // Nothing but skipped tweaks still says so.
        let plan = e.plan(&ids(&["gaming.future"])).unwrap();
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.snapshot_id, None);
        assert_eq!(out.skipped[0].tweak_id, "gaming.future");
    }

    #[test]
    fn an_outcome_with_nothing_skipped_leaves_the_field_out() {
        let e = engine(Arc::new(MemoryStore::default()), true);
        let plan = e.plan(&ids(&["gaming.b"])).unwrap();
        let out = e.apply(&plan.id).unwrap();
        let json = serde_json::to_value(&out).unwrap();
        assert!(json.get("skipped").is_none());
    }

    // A value on a device. Two adapters answer to one target, and the store keeps each one's values
    // apart, so these tests see what a moved cable or an unplugged adapter does to a backup.
    const LINK_SETTINGS: &[Setting] = &[Setting::string(Hive::Adapter, "Ethernet", "*EEE", "0")];
    const LINK: RegTarget = RegTarget::new(Hive::Adapter, "Ethernet", "*EEE");
    const GPU_SETTINGS: &[Setting] = &[Setting::dword(Hive::Gpu, "Nvidia", "RmTest", 0)];
    const GPU: RegTarget = RegTarget::new(Hive::Gpu, "Nvidia", "RmTest");
    const NIC1: &str = "0008";
    const NIC2: &str = "0010";

    fn admin(id: &'static str) -> TweakMeta {
        TweakMeta {
            permission: Permission::Administrator,
            ..meta(id)
        }
    }

    fn device_engine_with(
        store: Arc<MemoryStore>,
        elevation: Box<dyn Elevation>,
        elevated: bool,
    ) -> Engine {
        let mut registry = TweakRegistry::default();
        registry
            .register(tweak(admin("network.eee"), LINK_SETTINGS))
            .unwrap();
        registry
            .register(tweak(admin("gaming.gpu"), GPU_SETTINGS))
            .unwrap();
        Engine::new(
            registry,
            Box::new(Shared(store)),
            elevation,
            Journal::open_in_memory(),
            Box::new(move || ctx(26_100, elevated)),
        )
    }

    fn device_engine(store: Arc<MemoryStore>) -> Engine {
        device_engine_with(store, Box::new(InProcess), true)
    }

    fn text(value: &'static str) -> Option<RegValue> {
        Some(RegValue::String(value.into()))
    }

    fn apply_link(e: &Engine) -> i64 {
        let plan = e.plan(&ids(&["network.eee"])).unwrap();
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        out.snapshot_id.unwrap()
    }

    fn undoable(e: &Engine, id: &str) -> bool {
        e.tweak_statuses()
            .iter()
            .find(|s| s.meta.id == id)
            .unwrap()
            .undoable
    }

    fn value_on(store: &MemoryStore, device: &str, target: &RegTarget) -> Option<RegValue> {
        store.use_device(device);
        store.get(target)
    }

    /// Both adapters hold `1`, and the cable is in the first.
    fn two_adapters() -> Arc<MemoryStore> {
        let store = Arc::new(MemoryStore::default());
        store.use_device(NIC2);
        store.set(&LINK, text("1"));
        store.use_device(NIC1);
        store.set(&LINK, text("1"));
        store
    }

    #[test]
    fn a_plan_names_the_device_and_the_journal_keeps_it() {
        let e = device_engine(two_adapters());
        let plan = e.plan(&ids(&["network.eee"])).unwrap();
        assert_eq!(
            plan.changes[0].target_label,
            r"ADAPTER\Ethernet\*EEE on Device 0008"
        );
        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.changes[0].device.as_deref(), Some(NIC1));
    }

    #[test]
    fn a_plan_skips_a_tweak_whose_device_is_gone() {
        let store = two_adapters();
        store.remove_device();
        let e = device_engine(store);
        let plan = e.plan(&ids(&["network.eee"])).unwrap();
        assert!(plan.changes.is_empty());
        assert_eq!(plan.skipped[0].reason, Compatibility::Unreadable);
        assert!(plan.skipped[0].detail.is_some());
    }

    #[test]
    fn moving_the_cable_leaves_the_backup_open() {
        let store = two_adapters();
        let e = device_engine(store.clone());
        let snapshot = apply_link(&e);

        // Failure 1: the second adapter holds the before-value, which used to read as nothing to do.
        store.use_device(NIC2);
        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes[0].status, ChangeStatus::Conflict);
        assert!(out.changes[0].error.as_deref().unwrap().contains("0008"));
        assert_eq!(value_on(&store, NIC1, &LINK), text("0"), "not touched");
        assert_eq!(value_on(&store, NIC2, &LINK), text("1"), "not touched");
        assert!(undoable(&e, "network.eee"));
        assert_eq!(
            e.history(10).unwrap()[1].status,
            SnapshotStatus::Completed,
            "the apply is still open"
        );

        // Undoing the snapshot takes the same road.
        let out = e.revert(snapshot).unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes[0].status, ChangeStatus::Conflict);
        assert_eq!(value_on(&store, NIC1, &LINK), text("0"));

        // With the cable back, the backup still works.
        store.use_device(NIC1);
        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&LINK), text("1"));
        assert!(!undoable(&e, "network.eee"));
    }

    #[test]
    fn unplugging_an_adapter_is_a_failure_not_a_restored_value() {
        // Failure 2: the value didn't exist before, and neither does the adapter now.
        let store = Arc::new(MemoryStore::default());
        store.use_device(NIC1);
        let e = device_engine(store.clone());
        apply_link(&e);

        store.remove_device();
        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes[0].status, ChangeStatus::Failed);
        assert_eq!(out.changes[0].device.as_deref(), Some(NIC1));
        assert!(undoable(&e, "network.eee"));

        store.use_device(NIC1);
        assert_eq!(store.get(&LINK), text("0"));
        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&LINK), None);
    }

    #[test]
    fn applying_on_a_second_adapter_keeps_the_first_ones_backup() {
        // Failure 3: one target, two adapters, two backups.
        let store = two_adapters();
        store.set(&LINK, text("3"));
        let e = device_engine(store.clone());
        apply_link(&e);
        store.use_device(NIC2);
        apply_link(&e);

        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes.len(), 2);
        assert_eq!(out.changes[0].status, ChangeStatus::Verified);
        assert_eq!(out.changes[1].status, ChangeStatus::Conflict);
        assert_eq!(value_on(&store, NIC2, &LINK), text("1"));
        assert_eq!(value_on(&store, NIC1, &LINK), text("0"));
        assert!(undoable(&e, "network.eee"));

        let out = e.revert_tweak("network.eee").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&LINK), text("3"), "the first adapter's own value");
        assert!(!undoable(&e, "network.eee"));
    }

    #[test]
    fn an_adapter_never_tweaked_is_never_written() {
        // Failure 4: the second adapter already holds what 2VX set on the first.
        for before in [text("1"), None] {
            let store = Arc::new(MemoryStore::default());
            store.use_device(NIC2);
            store.set(&LINK, text("0"));
            store.use_device(NIC1);
            store.set(&LINK, before.clone());
            let e = device_engine(store.clone());
            apply_link(&e);

            store.use_device(NIC2);
            let out = e.revert_tweak("network.eee").unwrap();
            assert_eq!(out.status, SnapshotStatus::Failed);
            assert_eq!(out.changes[0].status, ChangeStatus::Conflict);
            assert_eq!(
                store.get(&LINK),
                text("0"),
                "the second adapter is untouched"
            );
            assert_eq!(value_on(&store, NIC1, &LINK), text("0"));

            let out = e.revert_tweak("network.eee").unwrap();
            assert_eq!(out.status, SnapshotStatus::Completed);
            assert_eq!(store.get(&LINK), before);
        }
    }

    #[test]
    fn a_gpu_swap_is_treated_the_same() {
        let store = Arc::new(MemoryStore::default());
        store.use_device("0000");
        store.set(&GPU, Some(RegValue::Dword(1)));
        let e = device_engine(store.clone());
        let plan = e.plan(&ids(&["gaming.gpu"])).unwrap();
        assert_eq!(e.apply(&plan.id).unwrap().status, SnapshotStatus::Completed);

        store.use_device("0001");
        store.set(&GPU, Some(RegValue::Dword(0)));
        let out = e.revert_tweak("gaming.gpu").unwrap();
        assert_eq!(out.changes[0].status, ChangeStatus::Conflict);
        assert_eq!(store.get(&GPU), Some(RegValue::Dword(0)));

        store.use_device("0000");
        let out = e.revert_tweak("gaming.gpu").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&GPU), Some(RegValue::Dword(1)));
    }

    /// A journal row as 0.3 left it: no device recorded, the snapshot finished.
    fn legacy_gpu_row(journal: &mut Journal, before: Option<RegValue>) {
        let tweaks = ids(&["gaming.gpu"]);
        let snapshot = journal
            .begin_snapshot(SnapshotKind::Apply, &tweaks, None)
            .unwrap();
        journal
            .record_change(
                snapshot,
                0,
                "gaming.gpu",
                &GPU,
                None,
                &before,
                &Some(RegValue::Dword(0)),
                ChangeStatus::Verified,
            )
            .unwrap();
        journal
            .finish_snapshot(snapshot, SnapshotStatus::Completed, None)
            .unwrap();
    }

    fn engine_on_journal(store: Arc<MemoryStore>, journal: Journal) -> Engine {
        let mut registry = TweakRegistry::default();
        registry
            .register(tweak(admin("gaming.gpu"), GPU_SETTINGS))
            .unwrap();
        Engine::new(
            registry,
            Box::new(Shared(store)),
            Box::new(InProcess),
            Ok(journal),
            Box::new(|| ctx(26_100, true)),
        )
    }

    #[test]
    fn a_legacy_row_matches_whatever_device_is_there() {
        let mut journal = Journal::open_in_memory().unwrap();
        legacy_gpu_row(&mut journal, Some(RegValue::Dword(1)));
        let store = Arc::new(MemoryStore::default());
        store.use_device("0000");
        store.set(&GPU, Some(RegValue::Dword(0)));
        let e = engine_on_journal(store.clone(), journal);

        let out = e.revert_tweak("gaming.gpu").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.get(&GPU), Some(RegValue::Dword(1)));
        assert_eq!(out.changes[0].device.as_deref(), Some("0000"));
    }

    #[test]
    fn a_legacy_row_is_superseded_by_a_newer_one_as_before() {
        let mut journal = Journal::open_in_memory().unwrap();
        legacy_gpu_row(&mut journal, Some(RegValue::Dword(1)));
        let store = Arc::new(MemoryStore::default());
        store.use_device("0000");
        store.set(&GPU, Some(RegValue::Dword(7)));
        let e = engine_on_journal(store.clone(), journal);

        let plan = e.plan(&ids(&["gaming.gpu"])).unwrap();
        e.apply(&plan.id).unwrap();
        let out = e.revert_tweak("gaming.gpu").unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(out.changes.len(), 1);
        assert_eq!(store.get(&GPU), Some(RegValue::Dword(7)));
        assert!(!undoable(&e, "gaming.gpu"));
    }

    #[test]
    fn a_legacy_row_without_its_device_stays_open() {
        let mut journal = Journal::open_in_memory().unwrap();
        legacy_gpu_row(&mut journal, None);
        let store = Arc::new(MemoryStore::default());
        store.remove_device();
        let e = engine_on_journal(store, journal);

        let out = e.revert_tweak("gaming.gpu").unwrap();
        assert_eq!(out.changes[0].status, ChangeStatus::Failed);
        assert!(undoable(&e, "gaming.gpu"));
    }

    #[test]
    fn apply_refuses_a_plan_made_on_another_device() {
        let store = two_adapters();
        let e = device_engine(store.clone());
        let plan = e.plan(&ids(&["network.eee"])).unwrap();
        store.use_device(NIC2);

        assert!(matches!(e.apply(&plan.id), Err(AppError::Conflict(_))));
        assert_eq!(store.get(&LINK), text("1"));
        assert!(e.history(10).unwrap().is_empty());
    }

    /// Stands in for the UAC prompt, and for what can happen while it's up.
    struct Prompt<F>(F);
    impl<F: Fn(bool) + Send + Sync> Elevation for Prompt<F> {
        fn writer<'a>(
            &'a self,
            store: &'a dyn SettingStore,
            needs_admin: bool,
        ) -> AppResult<Box<dyn SettingWriter + 'a>> {
            (self.0)(needs_admin);
            Ok(Box::new(LocalWriter(store)))
        }
    }

    #[test]
    fn the_device_is_checked_again_after_the_prompt() {
        let store = two_adapters();
        let during = store.clone();
        let prompt = Prompt(move |_| during.use_device(NIC2));
        let e = device_engine_with(store.clone(), Box::new(prompt), false);
        let plan = e.plan(&ids(&["network.eee"])).unwrap();

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::RolledBack);
        assert!(out.changes.is_empty(), "nothing was journaled or written");
        assert_eq!(value_on(&store, NIC1, &LINK), text("1"));
        assert_eq!(value_on(&store, NIC2, &LINK), text("1"));
    }

    /// Moves the link to the second adapter, and fails, when one target is written.
    struct MovesOnWrite {
        inner: Arc<MemoryStore>,
        trigger: RegTarget,
    }
    impl SettingStore for MovesOnWrite {
        fn read(&self, t: &RegTarget) -> AppResult<Option<RegValue>> {
            self.inner.read(t)
        }
        fn write(&self, t: &RegTarget, v: Option<&RegValue>) -> AppResult<()> {
            if t.same_as(&self.trigger) {
                self.inner.use_device(NIC2);
                return Err(AppError::System("refused".into()));
            }
            self.inner.write(t, v)
        }
        fn device_of(&self, t: &RegTarget) -> AppResult<Option<String>> {
            self.inner.device_of(t)
        }
    }

    #[test]
    fn a_rollback_never_lands_on_another_device() {
        const LATER: &[Setting] = &[Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\2VX-Test",
            "Later",
            1,
        )];
        let store = two_adapters();
        let mut registry = TweakRegistry::default();
        registry
            .register(tweak(admin("network.eee"), LINK_SETTINGS))
            .unwrap();
        registry
            .register(tweak(admin("power.later"), LATER))
            .unwrap();
        let e = Engine::new(
            registry,
            Box::new(MovesOnWrite {
                inner: store.clone(),
                trigger: LATER[0].target.clone(),
            }),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, true)),
        );
        let plan = e.plan(&ids(&["network.eee", "power.later"])).unwrap();
        assert_eq!(plan.changes[0].target, LINK);

        let out = e.apply(&plan.id).unwrap();
        assert_eq!(out.status, SnapshotStatus::Failed);
        assert_eq!(out.changes[0].status, ChangeStatus::Failed);
        assert_eq!(
            store.get(&LINK),
            text("1"),
            "the second adapter isn't written"
        );
        assert_eq!(value_on(&store, NIC1, &LINK), text("0"));

        // Back on the first adapter, undo finishes what the rollback couldn't.
        assert_eq!(
            e.revert_tweak("network.eee").unwrap().status,
            SnapshotStatus::Completed
        );
        assert_eq!(store.get(&LINK), text("1"));
    }

    #[test]
    fn no_prompt_when_nothing_can_be_written() {
        let store = two_adapters();
        let asked = Arc::new(Mutex::new(Vec::new()));
        let seen = asked.clone();
        let prompt = Prompt(move |needs_admin| seen.lock().unwrap().push(needs_admin));
        let e = device_engine_with(store.clone(), Box::new(prompt), false);
        let snapshot = apply_link(&e);
        assert_eq!(*asked.lock().unwrap(), vec![true]);

        store.use_device(NIC2);
        e.revert_tweak("network.eee").unwrap();
        e.revert(snapshot).unwrap();
        assert_eq!(*asked.lock().unwrap(), vec![true, false, false]);

        store.use_device(NIC1);
        e.revert_tweak("network.eee").unwrap();
        assert_eq!(*asked.lock().unwrap(), vec![true, false, false, true]);
    }

    /// Counts passes, to prove a status pass really is one.
    #[derive(Default)]
    struct Counting {
        inner: MemoryStore,
        passes: Arc<Mutex<u32>>,
    }
    impl SettingStore for Counting {
        fn read(&self, t: &RegTarget) -> AppResult<Option<RegValue>> {
            self.inner.read(t)
        }
        fn write(&self, t: &RegTarget, v: Option<&RegValue>) -> AppResult<()> {
            self.inner.write(t, v)
        }
        fn pass(&self, reads: &mut dyn FnMut()) {
            *self.passes.lock().unwrap() += 1;
            reads();
        }
    }

    #[test]
    fn statuses_read_in_one_pass() {
        let passes = Arc::new(Mutex::new(0));
        let store = Counting {
            passes: passes.clone(),
            ..Counting::default()
        };
        let mut registry = TweakRegistry::default();
        registry
            .register(tweak(meta("gaming.a"), USER_SETTINGS))
            .unwrap();
        registry.register(tweak(meta("gaming.b"), SECOND)).unwrap();
        let e = Engine::new(
            registry,
            Box::new(store),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, true)),
        );
        assert_eq!(e.tweak_statuses().len(), 2);
        assert_eq!(*passes.lock().unwrap(), 1);
    }
}
