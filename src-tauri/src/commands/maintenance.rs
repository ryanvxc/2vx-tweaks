//! PC Cleaner actions. Each takes IDs, checked in the engine against a fixed list or a fresh
//! scan, and leaves a line in the audit log: none of this passes through the tweak journal.

use std::sync::Arc;

use serde_json::json;
use tauri::State;
use tauri::ipc::Channel;

use crate::engine::apps::{self, RemovableApp};
use crate::engine::cleanup::{self, CleanupProgress, CleanupResult};
use crate::engine::drives::{self, DriveOutcome};
use crate::engine::repair::{self, RepairOutcome, RepairTool};
use crate::engine::restore_points::{self, CreatedPoint, RestoreOverview};
use crate::engine::service::Engine;
use crate::engine::startup::{self, AutoLaunch};
use crate::error::{AppError, AppResult};
use crate::system::power;

async fn worker<T, F>(f: F) -> AppResult<T>
where
    T: Send + 'static,
    F: FnOnce() -> AppResult<T> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| AppError::Internal(format!("worker failed: {e}")))?
}

/// Clears locations from the fixed allowlist. Can't be undone; may show one UAC prompt.
#[tauri::command]
pub async fn clean_locations(
    engine: State<'_, Arc<Engine>>,
    ids: Vec<String>,
    on_progress: Channel<CleanupProgress>,
) -> AppResult<Vec<CleanupResult>> {
    let results = worker(move || {
        cleanup::clean(&ids, &mut |progress| {
            // A closed channel means nobody's watching. The run carries on and still returns.
            let _ = on_progress.send(progress);
        })
    })
    .await?;
    engine.audit("cleanup_finished", json!({ "results": results }));
    Ok(results)
}

/// Whether 2VX opens at sign-in. Off unless the user turned it on.
#[tauri::command]
pub async fn get_auto_launch() -> AppResult<AutoLaunch> {
    worker(|| Ok(startup::auto_launch())).await
}

/// Adds or removes the one Run value that opens 2VX at sign-in.
#[tauri::command]
pub async fn set_auto_launch(engine: State<'_, Arc<Engine>>, enabled: bool) -> AppResult<()> {
    worker(move || startup::set_auto_launch(enabled)).await?;
    engine.audit("auto_launch_switched", json!({ "enabled": enabled }));
    Ok(())
}

/// Switches one startup entry. May show one UAC prompt for an all-users entry.
#[tauri::command]
pub async fn set_startup_enabled(
    engine: State<'_, Arc<Engine>>,
    id: String,
    enabled: bool,
) -> AppResult<()> {
    let item = worker(move || startup::set_enabled(&id, enabled)).await?;
    engine.audit(
        "startup_switched",
        json!({ "entry": item.id, "enabled": enabled }),
    );
    Ok(())
}

/// The built-in apps 2VX can remove, with whether each is installed. Read-only.
#[tauri::command]
pub async fn list_removable_apps() -> AppResult<Vec<RemovableApp>> {
    worker(apps::list).await
}

#[tauri::command]
pub async fn uninstall_app(engine: State<'_, Arc<Engine>>, app_id: String) -> AppResult<()> {
    let name = worker(move || apps::uninstall(&app_id)).await?;
    engine.audit("app_removed", json!({ "app": name }));
    Ok(())
}

/// Windows' restore points and whether System Protection is on. Read-only.
#[tauri::command]
pub async fn get_restore_points() -> AppResult<RestoreOverview> {
    worker(|| Ok(restore_points::overview())).await
}

/// Asks Windows for a restore point. Shows one UAC prompt, and can take a minute.
#[tauri::command]
pub async fn create_restore_point(engine: State<'_, Arc<Engine>>) -> AppResult<CreatedPoint> {
    let outcome = worker(restore_points::create).await?;
    engine.audit("restore_point", json!({ "created": outcome.created }));
    Ok(outcome)
}

/// Turns on System Protection for the system drive. Shows one UAC prompt.
#[tauri::command]
pub async fn enable_system_restore(engine: State<'_, Arc<Engine>>) -> AppResult<()> {
    worker(restore_points::enable).await?;
    engine.audit("system_protection_on", json!({}));
    Ok(())
}

/// Windows' own TRIM or defragment on each fixed drive. Shows one UAC prompt.
#[tauri::command]
pub async fn optimize_drives(engine: State<'_, Arc<Engine>>) -> AppResult<Vec<DriveOutcome>> {
    let outcomes = worker(drives::optimize).await?;
    engine.audit("drives_optimized", json!({ "outcomes": outcomes }));
    Ok(outcomes)
}

/// Runs one of Windows' two repair tools. Shows one UAC prompt, and can take many minutes.
#[tauri::command]
pub async fn run_repair(
    engine: State<'_, Arc<Engine>>,
    tool: RepairTool,
) -> AppResult<RepairOutcome> {
    let outcome = worker(move || repair::run(tool)).await?;
    engine.audit(
        "repair_finished",
        json!({ "tool": tool, "succeeded": outcome.succeeded }),
    );
    Ok(outcome)
}

/// Restarts Windows. The page asks first, and only ever from a click.
#[tauri::command]
pub async fn restart_pc(engine: State<'_, Arc<Engine>>) -> AppResult<()> {
    engine.audit("restart_requested", json!({}));
    worker(power::restart).await
}

#[tauri::command]
pub async fn reinstall_app(engine: State<'_, Arc<Engine>>, app_id: String) -> AppResult<()> {
    let name = worker(move || apps::reinstall(&app_id)).await?;
    engine.audit("app_restored", json!({ "app": name }));
    Ok(())
}
