//! 2VX Tweaks backend.
//!
//! Layering (see `docs/architecture.md`):
//! - `commands`  — the only surface the frontend can reach. Thin, validated wrappers.
//! - `system`    — read-only system information and live metrics.
//! - `engine`    — optimisation engine contracts and the tweak registry.
//! - `logging`   — structured file + console logging.
//! - `error`     — the single error type serialised to the frontend.

pub mod commands;
pub mod engine;
pub mod error;
pub mod logging;
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
    }
}

pub fn run() {
    tauri::Builder::default()
        // Update checks are the app's only network traffic (docs/UPDATE_SYSTEM.md). Downloads
        // are verified against the bundled public key; installing needs the user's go-ahead.
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
            commands::system::get_system_snapshot,
            commands::system::get_metrics,
            commands::system::list_startup_items,
            commands::system::scan_cleanup_locations,
            commands::engine::list_tweaks,
            commands::engine::get_engine_status,
            commands::engine::get_tweak_statuses,
            commands::engine::plan_tweaks,
            commands::engine::apply_plan,
            commands::engine::revert_snapshot,
            commands::engine::revert_tweak,
            commands::engine::list_snapshots,
            commands::engine::get_snapshot_changes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 2VX Tweaks");
}
