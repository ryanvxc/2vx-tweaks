fn main() {
    // Declaring the app's commands in the manifest makes each one deny-by-default:
    // the frontend can only call a command that a capability file explicitly allows.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_app_info",
            "log_frontend_event",
            "get_update_support",
            "get_system_snapshot",
            "get_metrics",
            "list_startup_items",
            "scan_cleanup_locations",
            "list_tweaks",
            "get_engine_status",
            "get_tweak_statuses",
            "plan_tweaks",
            "apply_plan",
            "revert_snapshot",
            "revert_tweak",
            "list_snapshots",
            "get_snapshot_changes",
        ]),
    ))
    .expect("failed to run tauri-build");
}
