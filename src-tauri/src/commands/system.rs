use tauri::State;

use crate::error::{AppError, AppResult};
use crate::system::cleanup::{self, CleanupLocation};
use crate::system::hardware::{self, SystemSnapshot};
use crate::system::metrics::{MetricsSample, MetricsSampler};
use crate::system::startup::{self, StartupInventory};

#[tauri::command]
pub async fn get_system_snapshot() -> AppResult<SystemSnapshot> {
    tauri::async_runtime::spawn_blocking(hardware::collect_snapshot)
        .await
        .map_err(|e| AppError::Internal(format!("snapshot task failed: {e}")))?
}

/// Returns samples newer than `since_ms` (Unix ms), or the full history when omitted.
#[tauri::command]
pub fn get_metrics(
    sampler: State<'_, MetricsSampler>,
    since_ms: Option<u64>,
) -> Vec<MetricsSample> {
    sampler.since(since_ms)
}

/// Run keys and Startup folders. Read-only.
#[tauri::command]
pub async fn list_startup_items() -> AppResult<StartupInventory> {
    tauri::async_runtime::spawn_blocking(startup::inventory)
        .await
        .map_err(|e| AppError::Internal(format!("startup task failed: {e}")))
}

/// Sizes of the fixed cleanup allowlist. Read-only; takes a few seconds on large caches.
#[tauri::command]
pub async fn scan_cleanup_locations() -> AppResult<Vec<CleanupLocation>> {
    tauri::async_runtime::spawn_blocking(cleanup::scan)
        .await
        .map_err(|e| AppError::Internal(format!("cleanup scan failed: {e}")))
}
