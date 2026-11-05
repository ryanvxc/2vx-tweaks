//! 2VX Tweaks backend.
//!
//! Layering (see `docs/architecture.md`):
//! - `commands`  — the only surface the frontend can reach. Thin, validated wrappers.
//! - `system`    — read-only system information and live metrics.
//! - `engine`    — optimisation engine contracts and the tweak registry.
//! - `account`   — the local account: an email in one file, never sent anywhere.
//! - `logging`   — structured file + console logging.
//! - `error`     — the single error type serialised to the frontend.

pub mod account;
pub mod commands;
pub mod engine;
pub mod error;
pub mod links;
pub mod logging;
pub mod report;
pub mod system;

use std::sync::Arc;

use tauri::Manager;

use crate::engine::broker::BrokerElevation;
use crate::engine::journal::Journal;
use crate::engine::service::Engine;
use crate::engine::store::WindowsRegistry;
use crate::engine::{TweakContext, TweakRegistry};
use crate::system::metrics::MetricsSampler;

fn tweak_context() -> TweakContext {
    TweakContext {
        os: system::os::read_os_info(),
        is_elevated: system::privilege::is_elevated(),
        has_battery: system::power::has_battery(),
        gpu_vendors: system::devices::gpu_vendors(),
        cpu_vendor: system::devices::cpu_vendor(),
    }
}

pub fn run() {
    tauri::Builder::default()
        // Update checks are the app's only automatic network traffic (docs/UPDATE_SYSTEM.md); the
        // only other traffic is a ping the user starts (`system/ping.rs`). Downloads are verified
        // against the bundled public key; installing needs the user's go-ahead.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let log_dir = app.path().app_log_dir()?;
            let guard = logging::init(&log_dir)?;
            app.manage(logging::LogState { guard, log_dir });

            tracing::info!(
                version = env!("CARGO_PKG_VERSION"),
                elevated = system::privilege::is_elevated(),
                "2VX Tweaks starting"
            );

            app.manage(MetricsSampler::start());
            // %LOCALAPPDATA%\com.twovx.tweaks: per-user and not shared with other accounts.
            let journal_path = app.path().app_local_data_dir()?.join("journal.db");
            app.manage(Arc::new(Engine::new(
                TweakRegistry::with_builtin_tweaks(),
                Box::new(WindowsRegistry),
                Box::new(BrokerElevation),
                Journal::open(&journal_path),
                Box::new(tweak_context),
            )));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_info,
            commands::app::log_frontend_event,
            commands::app::get_update_support,
            commands::account::get_account,
            commands::account::set_account,
            commands::account::clear_account,
            commands::system::get_system_snapshot,
            commands::system::get_device_report,
            commands::system::get_hardware_details,
            commands::system::get_service_states,
            commands::system::ping_target,
            commands::system::export_report,
            commands::system::measure_sleep,
            commands::system::open_link,
            commands::system::open_tool,
            commands::system::open_log_folder,
            commands::system::get_metrics,
            commands::system::get_running_programs,
            commands::system::list_startup_items,
            commands::system::scan_cleanup_locations,
            commands::maintenance::clean_locations,
            commands::maintenance::get_auto_launch,
            commands::maintenance::set_auto_launch,
            commands::maintenance::set_startup_enabled,
            commands::maintenance::list_removable_apps,
            commands::maintenance::uninstall_app,
            commands::maintenance::reinstall_app,
            commands::system::get_firmware_report,
            commands::system::get_timer_hold,
            commands::system::set_timer_hold,
            commands::maintenance::get_restore_points,
            commands::maintenance::create_restore_point,
            commands::maintenance::enable_system_restore,
            commands::maintenance::optimize_drives,
            commands::maintenance::run_repair,
            commands::maintenance::restart_pc,
            commands::engine::list_tweaks,
            commands::engine::list_retired_tweaks,
            commands::engine::get_engine_status,
            commands::engine::get_tweak_statuses,
            commands::engine::plan_tweaks,
            commands::engine::apply_plan,
            commands::engine::revert_snapshot,
            commands::engine::revert_tweak,
            commands::engine::revert_tweaks,
            commands::engine::list_snapshots,
            commands::engine::get_snapshot_changes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 2VX Tweaks");
}
