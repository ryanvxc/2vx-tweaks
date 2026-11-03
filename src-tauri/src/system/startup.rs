//! Read-only inventory of what starts with Windows: the Run and RunOnce registry keys and
//! the Startup folders. Nothing here changes an entry; disabling is planned (backlog) and
//! will go through the engine with a journal like every other change.

use std::path::Path;

use serde::Serialize;

use super::folders::{self, Folder};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StartupSource {
    /// `...\CurrentVersion\Run`: runs at every sign-in.
    RunKey,
    /// `...\CurrentVersion\RunOnce`: runs at the next sign-in, then Windows removes it.
    RunOnceKey,
    StartupFolder,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StartupScope {
    CurrentUser,
    AllUsers,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupItem {
    /// Stable within one listing: source location plus entry name.
    pub id: String,
    pub name: String,
    /// The command line or file path Windows starts.
    pub command: String,
    pub source: StartupSource,
    pub scope: StartupScope,
    /// Where the entry lives, for display (`HKCU\...\Run`, a folder path).
    pub location: String,
    /// `Some(false)` when Task Manager's Startup tab has it disabled. Read from the
    /// `StartupApproved` values Task Manager keeps; `None` when it has no record.
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupInventory {
    pub items: Vec<StartupItem>,
    /// Locations that could not be read, with the reason. Missing keys are not errors.
    pub unreadable: Vec<String>,
}

const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const RUN_ONCE: &str = r"Software\Microsoft\Windows\CurrentVersion\RunOnce";
const RUN_WOW64: &str = r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run";
const APPROVED: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved";
const ERROR_FILE_NOT_FOUND: u32 = 0x8007_0002;

pub fn inventory() -> StartupInventory {
    use windows_registry::{CURRENT_USER, LOCAL_MACHINE};
    let mut out = StartupInventory {
        items: vec![],
        unreadable: vec![],
    };
    let keys = [
        (
            CURRENT_USER,
            "HKCU",
            RUN,
            StartupSource::RunKey,
            StartupScope::CurrentUser,
            Some("Run"),
        ),
        (
            CURRENT_USER,
            "HKCU",
            RUN_ONCE,
            StartupSource::RunOnceKey,
            StartupScope::CurrentUser,
            None,
        ),
        (
            LOCAL_MACHINE,
            "HKLM",
            RUN,
            StartupSource::RunKey,
            StartupScope::AllUsers,
            Some("Run"),
        ),
        (
            LOCAL_MACHINE,
            "HKLM",
            RUN_WOW64,
            StartupSource::RunKey,
            StartupScope::AllUsers,
            Some("Run32"),
        ),
        (
            LOCAL_MACHINE,
            "HKLM",
            RUN_ONCE,
            StartupSource::RunOnceKey,
            StartupScope::AllUsers,
            None,
        ),
    ];
    for (root, hive, path, source, scope, approved) in keys {
        let location = format!(r"{hive}\{path}");
        let key = match root.open(path) {
            Ok(k) => k,
            Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => continue,
            Err(e) => {
                out.unreadable.push(format!("{location}: {}", e.message()));
                continue;
            }
        };
        let values = match key.values() {
            Ok(v) => v,
            Err(e) => {
                out.unreadable.push(format!("{location}: {}", e.message()));
                continue;
            }
        };
        for (name, value) in values {
            let Ok(command) = String::try_from(value) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }
            out.items.push(StartupItem {
                id: format!("{location}\\{name}"),
                enabled: approved.and_then(|sub| approved_state(root, sub, &name)),
                name,
                command,
                source,
                scope,
                location: location.clone(),
            });
        }
    }

    for (folder, scope, approved_root) in [
        (Folder::Startup, StartupScope::CurrentUser, CURRENT_USER),
        (Folder::CommonStartup, StartupScope::AllUsers, LOCAL_MACHINE),
    ] {
        let Some(dir) = folders::known(folder) else {
            continue;
        };
        match std::fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let file = entry.file_name().to_string_lossy().into_owned();
                    // desktop.ini is the folder's own metadata, not a startup item.
                    if file.eq_ignore_ascii_case("desktop.ini") {
                        continue;
                    }
                    out.items.push(StartupItem {
                        id: entry.path().to_string_lossy().into_owned(),
                        name: display_name(&file),
                        command: entry.path().to_string_lossy().into_owned(),
                        source: StartupSource::StartupFolder,
                        scope,
                        location: dir.to_string_lossy().into_owned(),
                        enabled: approved_state(approved_root, "StartupFolder", &file),
                    });
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => out.unreadable.push(format!("{}: {e}", dir.display())),
        }
    }
    out.items.sort_by_key(|i| i.name.to_lowercase());
    out
}

/// Task Manager records its Startup-tab choice as a binary value whose first byte is even
/// when the entry is enabled and odd when disabled.
fn approved_state(root: &windows_registry::Key, sub: &str, name: &str) -> Option<bool> {
    let key = root.open(format!(r"{APPROVED}\{sub}")).ok()?;
    let value = key.get_value(name).ok()?;
    parse_approved(&value)
}

fn parse_approved(bytes: &[u8]) -> Option<bool> {
    bytes.first().map(|b| b % 2 == 0)
}

fn display_name(file: &str) -> String {
    Path::new(file)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| file.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_flag_parity() {
        assert_eq!(parse_approved(&[0x02, 0, 0]), Some(true));
        assert_eq!(parse_approved(&[0x06]), Some(true));
        assert_eq!(parse_approved(&[0x03, 1]), Some(false));
        assert_eq!(parse_approved(&[]), None);
    }

    #[test]
    fn shortcut_names_drop_the_extension() {
        assert_eq!(display_name("Discord.lnk"), "Discord");
        assert_eq!(display_name("noext"), "noext");
    }

    #[test]
    #[cfg(windows)]
    fn reads_the_real_inventory_without_failing() {
        let inv = inventory();
        assert!(inv.items.iter().all(|i| !i.name.is_empty()));
    }
}
