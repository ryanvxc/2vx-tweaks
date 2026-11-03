use tauri::State;

use crate::error::{AppError, AppResult};
use crate::system::hardware::{self, SystemSnapshot};
use crate::system::metrics::{MetricsSample, MetricsSampler};

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
