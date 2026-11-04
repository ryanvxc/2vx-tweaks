use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

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

/// Whether this build can check for updates, and where. The check itself runs in the
/// updater plugin, which verifies every download against the public key in
/// `tauri.conf.json` and only accepts versions newer than this one.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupport {
    pub available: bool,
    /// Why updates are unavailable, for the Update Centre.
    pub reason: Option<&'static str>,
    pub endpoint: Option<String>,
    pub channel: &'static str,
}

/// Release builds, local or from the workflow, read the `main` channel endpoint from
/// `tauri.conf.json`. Development builds never update themselves.
fn update_support(debug_build: bool, endpoints: &[String]) -> UpdateSupport {
    let endpoint = endpoints
        .iter()
        .find(|e| e.starts_with("https://"))
        .cloned();
    let reason = if debug_build {
        Some("Development builds don't update.")
    } else if endpoint.is_none() {
        Some("This build has no update channel.")
    } else {
        None
    };
    UpdateSupport {
        available: reason.is_none(),
        reason,
        endpoint: endpoint.filter(|_| reason.is_none()),
        channel: "main",
    }
}

#[tauri::command]
pub fn get_update_support(app: AppHandle) -> UpdateSupport {
    let endpoints: Vec<String> = app
        .config()
        .plugins
        .0
        .get("updater")
        .and_then(|u| u.get("endpoints"))
        .and_then(|e| e.as_array())
        .map(|list| {
            list.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    update_support(cfg!(debug_assertions), &endpoints)
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
    fn updates_need_a_release_build_and_an_https_endpoint() {
        let url = "https://github.com/o/r/releases/download/updater-main/latest.json".to_string();
        assert!(!update_support(true, std::slice::from_ref(&url)).available);
        assert!(!update_support(false, &[]).available);
        assert!(!update_support(false, &["http://example.com/latest.json".into()]).available);
        let ok = update_support(false, std::slice::from_ref(&url));
        assert!(ok.available);
        assert_eq!(ok.endpoint, Some(url));
    }

    #[test]
    fn rejects_unknown_log_level() {
        assert!(serde_json::from_str::<FrontendLogLevel>("\"trace\"").is_err());
        assert!(serde_json::from_str::<FrontendLogLevel>("\"warn\"").is_ok());
    }
}
