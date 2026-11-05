use tauri::State;

use crate::engine::cleanup::{self, CleanupLocation};
use crate::engine::startup::{self, StartupInventory};
use crate::error::{AppError, AppResult};
use crate::links::{self, Link};
use crate::system::devices::{self, DeviceReport, SleepSample};
use crate::system::hardware::{self, SystemSnapshot};
use crate::system::metrics::{MetricsSample, MetricsSampler};
use crate::system::processes::{self, RunningProgram};

#[tauri::command]
pub async fn get_system_snapshot() -> AppResult<SystemSnapshot> {
    tauri::async_runtime::spawn_blocking(hardware::collect_snapshot)
        .await
        .map_err(|e| AppError::Internal(format!("snapshot task failed: {e}")))?
}

/// Drivers, adapters, interrupt modes, protections and the timer. Read-only.
#[tauri::command]
pub async fn get_device_report() -> AppResult<DeviceReport> {
    tauri::async_runtime::spawn_blocking(devices::collect_report)
        .await
        .map_err(|e| AppError::Internal(format!("device report task failed: {e}")))
}

/// How long a one-millisecond sleep really takes. Only sleeps; takes under a second.
#[tauri::command]
pub async fn measure_sleep() -> AppResult<Vec<SleepSample>> {
    tauri::async_runtime::spawn_blocking(devices::measure_sleep)
        .await
        .map_err(|e| AppError::Internal(format!("sleep test failed: {e}")))
}

/// Opens one page from the fixed list in `links.rs`.
#[tauri::command]
pub fn open_link(link: Link) -> AppResult<()> {
    links::open(link)
}

/// Running programs, for Game Mode's detected game. Read-only, with no arguments.
#[tauri::command]
pub async fn get_running_programs() -> AppResult<Vec<RunningProgram>> {
    tauri::async_runtime::spawn_blocking(processes::running_programs)
        .await
        .map_err(|e| AppError::Internal(format!("program list task failed: {e}")))
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
