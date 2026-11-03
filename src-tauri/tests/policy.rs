//! Project policy guards (docs/security.md §"Commitments"). These read the source tree and
//! fail the build when a change would break a commitment, so it cannot slip in unnoticed.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap()
}

/// String literals in a `[ ... ]` list that follows `marker`.
fn quoted_list_after(text: &str, marker: &str) -> Vec<String> {
    let start = text
        .find(marker)
        .unwrap_or_else(|| panic!("`{marker}` not found"));
    let rest = &text[start..];
    let open = rest.find('[').unwrap();
    let close = rest[open..].find(']').unwrap() + open;
    rest[open + 1..close]
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// No shell, no script host, no process spawning: the backend changes the system only
/// through typed primitives. The elevated broker relaunches this executable via
/// ShellExecuteExW, which is the single allowed exception.
#[test]
fn backend_never_spawns_shells_or_processes() {
    let forbidden = [
        "std::process::Command",
        "Command::new",
        "powershell",
        "cmd.exe",
        "cmd /c",
        "reg.exe",
        "regedit",
        "bcdedit",
        "schtasks",
        "sc.exe",
        "wmic",
        "nsudo",
        "createprocess",
        "winexec",
    ];
    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap().to_ascii_lowercase();
        for needle in forbidden {
            assert!(
                !text.contains(&needle.to_ascii_lowercase()),
                "{} contains forbidden `{needle}`",
                file.display()
            );
        }
        if !file.ends_with("broker.rs") {
            assert!(
                !text.contains("shellexecute"),
                "{} launches processes",
                file.display()
            );
        }
    }
}

/// No network clients, analytics, crash reporters or remote-control crates.
#[test]
fn no_network_or_telemetry_dependencies() {
    let manifest = read("Cargo.toml");
    let deps = &manifest[manifest.find("[dependencies]").unwrap()..];
    for name in [
        "reqwest",
        "ureq",
        "hyper",
        "isahc",
        "surf",
        "attohttpc",
        "curl",
        "tauri-plugin-http",
        "tauri-plugin-shell",
        "tauri-plugin-updater",
        "sentry",
        "posthog",
        "segment",
        "machine-uid",
        "machineid",
    ] {
        assert!(
            !deps.lines().any(|l| l.trim_start().starts_with(name)),
            "dependency `{name}` needs a security review and a docs/security.md update first"
        );
    }
}

/// The WebView may only talk to the Tauri IPC bridge.
#[test]
fn csp_allows_no_remote_origins() {
    let conf = read("tauri.conf.json");
    let after_key = &conf[conf.find("\"csp\"").unwrap() + 5..];
    let value_start = after_key.find('"').unwrap() + 1;
    let csp = &after_key[value_start..value_start + after_key[value_start..].find('"').unwrap()];
    for directive in csp.split(';').filter(|d| !d.trim().is_empty()) {
        for source in directive.split_whitespace().skip(1) {
            let allowed = matches!(
                source,
                "'self'" | "'unsafe-inline'" | "data:" | "ipc:" | "http://ipc.localhost"
            );
            assert!(allowed, "CSP source `{source}` is not allowed");
        }
    }
}

/// Every command is declared (deny-by-default), granted, and registered — and nothing else.
#[test]
fn commands_manifest_capability_and_handler_agree() {
    let mut manifest = quoted_list_after(&read("build.rs"), ".commands(&");
    let mut granted: Vec<String> =
        quoted_list_after(&read("capabilities/default.json"), "\"permissions\"")
            .into_iter()
            .filter_map(|p| p.strip_prefix("allow-").map(|c| c.replace('-', "_")))
            .collect();
    let lib = read("src/lib.rs");
    let start = lib.find("generate_handler![").unwrap();
    let end = lib[start..].find(']').unwrap() + start;
    let mut handlers: Vec<String> = lib[start..end]
        .split(',')
        .filter_map(|s| s.trim().rsplit("::").next().map(str::to_string))
        .filter(|s| !s.is_empty() && !s.contains('['))
        .collect();
    manifest.sort();
    granted.sort();
    handlers.sort();
    assert_eq!(
        manifest, granted,
        "build.rs and capabilities/default.json disagree"
    );
    assert_eq!(
        manifest, handlers,
        "build.rs and lib.rs generate_handler! disagree"
    );
}

/// The capability grants no plugin permissions beyond core window chrome.
#[test]
fn capability_grants_no_plugins() {
    for permission in quoted_list_after(&read("capabilities/default.json"), "\"permissions\"") {
        let ok = permission.starts_with("allow-")
            || permission == "core:default"
            || permission.starts_with("core:window:allow-");
        assert!(ok, "permission `{permission}` needs a security review");
    }
}

/// The built-in registry validates (metadata gate, policy, conflicts) on every build.
#[test]
fn builtin_tweaks_pass_the_metadata_gate() {
    // `with_builtin_tweaks` panics or errors on an invalid tweak once tweaks are registered.
    let registry = twovx_tweaks_lib::engine::TweakRegistry::with_builtin_tweaks();
    for tweak in registry.iter() {
        for setting in tweak.settings() {
            twovx_tweaks_lib::engine::policy::check_target(&setting.target).unwrap();
        }
    }
}
