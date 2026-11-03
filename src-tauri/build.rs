fn main() {
    // Declaring the app's commands in the manifest makes each one deny-by-default:
    // the frontend can only call a command that a capability file explicitly allows.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_app_info",
            "log_frontend_event",
            "get_system_snapshot",
            "get_metrics",
            "list_tweaks",
        ]),
    ))
    .expect("failed to run tauri-build");
}
