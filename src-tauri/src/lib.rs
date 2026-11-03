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

use tauri::Manager;

use crate::engine::TweakRegistry;
use crate::system::metrics::MetricsSampler;

pub fn run() {
    tauri::Builder::default()
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
            app.manage(TweakRegistry::with_builtin_tweaks());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_info,
            commands::app::log_frontend_event,
            commands::system::get_system_snapshot,
            commands::system::get_metrics,
            commands::engine::list_tweaks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 2VX Tweaks");
}
