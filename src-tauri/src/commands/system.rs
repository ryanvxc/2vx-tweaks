use std::sync::Arc;

use tauri::{State, WebviewWindow};

use crate::engine::cleanup::{self, CleanupLocation};
use crate::engine::service::Engine;
use crate::engine::startup::{self, StartupInventory};
use crate::error::{AppError, AppResult};
use crate::links::{self, Link, WindowsTool};
use crate::logging::LogState;
use crate::report;
use crate::system::devices::{self, DeviceReport, SleepSample};
use crate::system::hardware::{self, SystemSnapshot};
use crate::system::hardware_details::{self, HardwareDetails};
use crate::system::metrics::{MetricsSample, MetricsSampler};
use crate::system::ping::{self, PingResult, PingTarget};
use crate::system::processes::{self, RunningProgram};
use crate::system::services::{self, ServiceState};

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

/// CPU layout, memory modules, drives, GPU drivers and devices. Read-only, needs no elevation.
#[tauri::command]
pub async fn get_hardware_details() -> AppResult<HardwareDetails> {
    tauri::async_runtime::spawn_blocking(hardware_details::collect)
        .await
        .map_err(|e| AppError::Internal(format!("hardware details task failed: {e}")))
}

/// The services named by catalogue tweaks. Read-only: the list comes from the catalogue, not from
/// the caller.
#[tauri::command]
pub async fn get_service_states(engine: State<'_, Arc<Engine>>) -> AppResult<Vec<ServiceState>> {
    let names = engine.registry().service_names();
    tauri::async_runtime::spawn_blocking(move || services::states(&names))
        .await
        .map_err(|e| AppError::Internal(format!("service list task failed: {e}")))
}

/// One ICMP echo to one of two fixed targets. Only ever started by the user. A bufferbloat test
/// calls this once a second for each target for as long as it runs.
#[tauri::command]
pub async fn ping_target(target: PingTarget) -> AppResult<PingResult> {
    tauri::async_runtime::spawn_blocking(move || ping::ping(target))
        .await
        .map_err(|e| AppError::Internal(format!("ping task failed: {e}")))?
}

/// Saves the report to wherever the user picks in the save dialog. Returns the file's name, or
/// nothing if they cancel. Takes the text only: no path crosses this command.
#[tauri::command]
pub async fn export_report(window: WebviewWindow, text: String) -> AppResult<Option<String>> {
    report::check_size(&text)?;
    let owner = window.hwnd().ok().map(|hwnd| hwnd.0 as isize);
    tauri::async_runtime::spawn_blocking(move || report::export(owner, &text))
        .await
        .map_err(|e| AppError::Internal(format!("report task failed: {e}")))?
}

/// How long a one-millisecond sleep really takes. Only sleeps; takes under a second.
#[tauri::command]
pub async fn measure_sleep() -> AppResult<Vec<SleepSample>> {
    tauri::async_runtime::spawn_blocking(devices::measure_sleep)
        .await
        .map_err(|e| AppError::Internal(format!("sleep test failed: {e}")))
}

/// Off the main thread, because a tool can bring up a UAC prompt and the call waits for it, which
/// would freeze the window behind it.
async fn opening(open: impl FnOnce() -> AppResult<()> + Send + 'static) -> AppResult<()> {
    tauri::async_runtime::spawn_blocking(open)
        .await
        .map_err(|e| AppError::Internal(format!("open task failed: {e}")))?
}

/// Opens one page from the fixed list in `links.rs`.
#[tauri::command]
pub async fn open_link(link: Link) -> AppResult<()> {
    opening(move || links::open(link)).await
}

/// Opens one Windows tool from the fixed list in `links.rs`.
#[tauri::command]
pub async fn open_tool(tool: WindowsTool) -> AppResult<()> {
    opening(move || links::open_tool(tool)).await
}

/// Opens the app's own log folder. The folder comes from the backend, never from the caller.
#[tauri::command]
pub async fn open_log_folder(log: State<'_, LogState>) -> AppResult<()> {
    let log_dir = log.log_dir.clone();
    opening(move || links::open_log_folder(&log_dir)).await
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
