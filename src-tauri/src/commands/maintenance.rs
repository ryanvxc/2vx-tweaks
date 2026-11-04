//! PC Cleaner actions. Each takes IDs, checked in the engine against a fixed list or a fresh
//! scan, and leaves a line in the audit log: none of this passes through the tweak journal.

use std::sync::Arc;

use serde_json::json;
use tauri::State;

use crate::engine::apps::{self, RemovableApp};
use crate::engine::cleanup::{self, CleanupResult};
use crate::engine::service::Engine;
use crate::engine::startup;
use crate::error::{AppError, AppResult};

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
) -> AppResult<Vec<CleanupResult>> {
    let results = worker(move || cleanup::clean(&ids)).await?;
    engine.audit("cleanup_finished", json!({ "results": results }));
    Ok(results)
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

#[tauri::command]
pub async fn reinstall_app(engine: State<'_, Arc<Engine>>, app_id: String) -> AppResult<()> {
    let name = worker(move || apps::reinstall(&app_id)).await?;
    engine.audit("app_restored", json!({ "app": name }));
    Ok(())
}
