use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::AppResult;
use crate::logging::{LogState, sanitise_message};
use crate::system::privilege;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub is_elevated: bool,
    pub log_dir: String,
    pub debug_build: bool,
}

#[tauri::command]
pub fn get_app_info(log: State<'_, LogState>) -> AppInfo {
    AppInfo {
        name: "2VX Tweaks",
        version: env!("CARGO_PKG_VERSION"),
        is_elevated: privilege::is_elevated(),
        log_dir: log.log_dir.to_string_lossy().into_owned(),
        debug_build: cfg!(debug_assertions),
    }
}

/// Levels the frontend may log at. Deserialisation rejects anything else.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrontendLogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Forwards a frontend event to the backend log file.
#[tauri::command]
pub fn log_frontend_event(
    level: FrontendLogLevel,
    message: String,
    source: Option<String>,
) -> AppResult<()> {
    let message = sanitise_message(&message);
    let source = source
        .as_deref()
        .map(sanitise_message)
        .unwrap_or_else(|| "ui".to_owned());

    match level {
        FrontendLogLevel::Debug => tracing::debug!(target: "frontend", %source, "{message}"),
        FrontendLogLevel::Info => tracing::info!(target: "frontend", %source, "{message}"),
        FrontendLogLevel::Warn => tracing::warn!(target: "frontend", %source, "{message}"),
        FrontendLogLevel::Error => tracing::error!(target: "frontend", %source, "{message}"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_log_level() {
        assert!(serde_json::from_str::<FrontendLogLevel>("\"trace\"").is_err());
        assert!(serde_json::from_str::<FrontendLogLevel>("\"warn\"").is_ok());
    }
}
