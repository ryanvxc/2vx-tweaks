use std::sync::Arc;

use tauri::State;

use crate::engine::TweakMeta;
use crate::engine::journal::{ChangeRecord, SnapshotRecord};
use crate::engine::plan::Plan;
use crate::engine::service::{Engine, EngineStatus, OperationOutcome, TweakStatus};
use crate::error::{AppError, AppResult};

/// Runs engine work off the async runtime: registry reads, journal writes and the UAC
/// prompt can block for a while.
async fn blocking<T, F>(engine: &Arc<Engine>, f: F) -> AppResult<T>
where
    T: Send + 'static,
    F: FnOnce(&Engine) -> AppResult<T> + Send + 'static,
{
    let engine = Arc::clone(engine);
    tauri::async_runtime::spawn_blocking(move || f(&engine))
        .await
        .map_err(|e| AppError::Internal(format!("engine task failed: {e}")))?
}

/// Lists registered tweaks. Read-only.
#[tauri::command]
pub fn list_tweaks(engine: State<'_, Arc<Engine>>) -> Vec<TweakMeta> {
    engine.registry().list().cloned().collect()
}

#[tauri::command]
pub async fn get_engine_status(engine: State<'_, Arc<Engine>>) -> AppResult<EngineStatus> {
    blocking(&engine, |e| Ok(e.status())).await
}

/// Current vs desired value of every setting of every tweak. Read-only.
#[tauri::command]
pub async fn get_tweak_statuses(engine: State<'_, Arc<Engine>>) -> AppResult<Vec<TweakStatus>> {
    blocking(&engine, |e| Ok(e.tweak_statuses())).await
}

/// Dry run for the given tweak IDs (validated against the registry). Changes nothing.
#[tauri::command]
pub async fn plan_tweaks(
    engine: State<'_, Arc<Engine>>,
    tweak_ids: Vec<String>,
) -> AppResult<Plan> {
    blocking(&engine, move |e| e.plan(&tweak_ids)).await
}

/// Applies a plan the user confirmed. May show one UAC prompt for HKLM changes.
#[tauri::command]
pub async fn apply_plan(
    engine: State<'_, Arc<Engine>>,
    plan_id: String,
) -> AppResult<OperationOutcome> {
    blocking(&engine, move |e| e.apply(&plan_id)).await
}

#[tauri::command]
pub async fn revert_snapshot(
    engine: State<'_, Arc<Engine>>,
    snapshot_id: i64,
) -> AppResult<OperationOutcome> {
    if snapshot_id <= 0 {
        return Err(AppError::InvalidInput(
            "snapshot id must be positive".into(),
        ));
    }
    blocking(&engine, move |e| e.revert(snapshot_id)).await
}

/// Undoes 2VX's recorded changes for one tweak (the toggle-off path). The ID is checked
/// against the registry; the values restored come from the journal, never the caller.
#[tauri::command]
pub async fn revert_tweak(
    engine: State<'_, Arc<Engine>>,
    tweak_id: String,
) -> AppResult<OperationOutcome> {
    blocking(&engine, move |e| e.revert_tweak(&tweak_id)).await
}

/// Most recent snapshots first.
#[tauri::command]
pub async fn list_snapshots(
    engine: State<'_, Arc<Engine>>,
    limit: Option<u32>,
) -> AppResult<Vec<SnapshotRecord>> {
    blocking(&engine, move |e| e.history(limit.unwrap_or(50))).await
}

#[tauri::command]
pub async fn get_snapshot_changes(
    engine: State<'_, Arc<Engine>>,
    snapshot_id: i64,
) -> AppResult<Vec<ChangeRecord>> {
    blocking(&engine, move |e| e.snapshot_changes(snapshot_id)).await
}
