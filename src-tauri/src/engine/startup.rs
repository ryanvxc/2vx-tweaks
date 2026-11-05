//! What starts with Windows: the Run and RunOnce keys, the Startup folders and scheduled tasks, and
//! switching the first two off and back on. Tasks are listed only, because a task isn't one setting
//! 2VX could put back exactly.
//!
//! Switching off moves an entry out of the place Windows reads into a holding place of 2VX's
//! own, with its data and type intact, so switching on restores it exactly. Task Manager's
//! `StartupApproved` values are only ever read (docs/security.md, decision P0-8a). An entry can
//! only be named by the ID a fresh scan gives it.
//!
//! This is also where the app's own sign-in entry lives: one Run value, off by default.

use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use windows::Win32::System::Environment::ExpandEnvironmentStringsW;
use windows::core::HSTRING;
use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE};

use crate::engine::Hive;
use crate::engine::broker::BrokerSession;
use crate::engine::system_store;
use crate::error::{AppError, AppResult};
use crate::system::file_info;
use crate::system::folders::{self, Folder};
use crate::system::privilege;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StartupSource {
    /// `...\CurrentVersion\Run`: runs at every sign-in.
    RunKey,
    /// `...\CurrentVersion\RunOnce`: runs at the next sign-in, then Windows removes it.
    RunOnceKey,
    StartupFolder,
    /// A scheduled task with a sign-in or start-up trigger.
    Task,
}

impl StartupSource {
    fn label(self) -> &'static str {
        match self {
            StartupSource::RunKey => "Run key",
            StartupSource::RunOnceKey => "RunOnce key",
            StartupSource::StartupFolder => "Startup folder",
            StartupSource::Task => "Task",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StartupScope {
    CurrentUser,
    AllUsers,
}

/// Who the program says made it. Self-reported, so a label and never a verdict on safety.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StartupOrigin {
    Microsoft,
    ThirdParty,
    /// Nothing to read: a program that's no longer there, or one with no version information.
    Unknown,
}

/// The keys entries can be moved out of. A closed list, because it crosses the broker pipe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RunKey {
    Run,
    Run32,
    RunOnce,
}

impl RunKey {
    fn path(self) -> &'static str {
        match self {
            RunKey::Run => RUN,
            RunKey::Run32 => RUN_WOW64,
            RunKey::RunOnce => RUN_ONCE,
        }
    }

    /// Where a switched-off entry waits. HKLM entries wait in HKLM, so only an administrator
    /// can change what comes back.
    fn held(self) -> &'static str {
        match self {
            RunKey::Run => r"Software\2VX Tweaks\DisabledStartup\Run",
            RunKey::Run32 => r"Software\2VX Tweaks\DisabledStartup\Run32",
            RunKey::RunOnce => r"Software\2VX Tweaks\DisabledStartup\RunOnce",
        }
    }

    fn source(self) -> StartupSource {
        match self {
            RunKey::RunOnce => StartupSource::RunOnceKey,
            _ => StartupSource::RunKey,
        }
    }

    fn approved(self) -> Option<&'static str> {
        match self {
            RunKey::Run => Some("Run"),
            RunKey::Run32 => Some("Run32"),
            RunKey::RunOnce => None,
        }
    }
}

/// Where an entry can be switched, for the backend's own use.
#[derive(Debug, Clone)]
enum Place {
    Key { hive: Hive, key: RunKey },
    UserFolder { file: String },
    ReadOnly,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupItem {
    /// Stable across switching: source location plus entry name.
    pub id: String,
    pub name: String,
    /// The command line or file path Windows starts. For a shortcut, the `.lnk` itself.
    pub command: String,
    /// The program itself, with environment variables expanded. For a shortcut, what it points at.
    pub path: String,
    pub source: StartupSource,
    /// `source` in words, so every screen says the same thing.
    pub source_label: &'static str,
    pub scope: StartupScope,
    /// Where the entry lives, for display (`HKCU\...\Run`, a folder path).
    pub location: String,
    /// `Some(false)` when it won't run: switched off here or in Task Manager.
    pub enabled: Option<bool>,
    /// Off in Task Manager, which is the place to turn it back on.
    pub off_in_task_manager: bool,
    /// 2VX can switch this entry.
    pub changeable: bool,
    pub origin: StartupOrigin,
    /// The program's own description of itself.
    pub description: Option<String>,
    pub publisher: Option<String>,
    #[serde(skip)]
    place: Place,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupInventory {
    pub items: Vec<StartupItem>,
    /// Locations that could not be read, with the reason. Missing keys are not errors.
    pub unreadable: Vec<String>,
}

/// The app's own sign-in entry. `OffInTaskManager` is the user's choice and stays theirs: Task
/// Manager is where it's undone, and `StartupApproved` is only ever read (decision P0-8a).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AutoLaunch {
    Off,
    On,
    OffInTaskManager,
}

const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const RUN_ONCE: &str = r"Software\Microsoft\Windows\CurrentVersion\RunOnce";
const RUN_WOW64: &str = r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run";
const APPROVED: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved";
const ERROR_FILE_NOT_FOUND: u32 = 0x8007_0002;
/// The registry's own limit on a value name.
const MAX_VALUE_NAME: usize = 16_383;
/// The one value behind "open 2VX at sign-in". There is no service and no scheduled task.
const AUTO_LAUNCH_VALUE: &str = "2VX Tweaks";

const KEYS: &[(Hive, RunKey)] = &[
    (Hive::CurrentUser, RunKey::Run),
    (Hive::CurrentUser, RunKey::RunOnce),
    (Hive::LocalMachine, RunKey::Run),
    (Hive::LocalMachine, RunKey::Run32),
    (Hive::LocalMachine, RunKey::RunOnce),
];

fn root(hive: Hive) -> &'static Key {
    match hive {
        Hive::CurrentUser => CURRENT_USER,
        _ => LOCAL_MACHINE,
    }
}

fn scope_of(hive: Hive) -> StartupScope {
    match hive {
        Hive::CurrentUser => StartupScope::CurrentUser,
        _ => StartupScope::AllUsers,
    }
}

pub fn inventory() -> StartupInventory {
    let mut out = StartupInventory {
        items: vec![],
        unreadable: vec![],
    };
    for &(hive, key) in KEYS {
        read_key(&mut out, hive, key, false);
        read_key(&mut out, hive, key, true);
    }

    if let Some(dir) = folders::known(Folder::Startup) {
        read_folder(&mut out, &dir, &dir, StartupScope::CurrentUser, false);
        if let Some(held) = held_folder() {
            read_folder(&mut out, &held, &dir, StartupScope::CurrentUser, true);
        }
    }
    if let Some(dir) = folders::known(Folder::CommonStartup) {
        read_folder(&mut out, &dir, &dir, StartupScope::AllUsers, false);
    }
    read_tasks(&mut out);
    out.items.sort_by_key(|i| i.name.to_lowercase());
    out
}

fn read_tasks(out: &mut StartupInventory) {
    let found = match system_store::start_tasks() {
        Ok(found) => found,
        Err(e) => {
            out.unreadable
                .push(format!("Task Scheduler: {}", e.message()));
            return;
        }
    };
    out.unreadable.extend(found.unreadable);
    for task in found.tasks {
        let path = task_executable(&task.program);
        let (origin, description, publisher) = describe(Path::new(&path));
        let folder = task.path.rsplit_once('\\').map_or("", |(folder, _)| folder);
        out.items.push(StartupItem {
            id: format!("Task Scheduler{}", task.path),
            command: task_command(&task.program, &task.arguments),
            path,
            source: StartupSource::Task,
            source_label: StartupSource::Task.label(),
            scope: if task.for_one_user {
                StartupScope::CurrentUser
            } else {
                StartupScope::AllUsers
            },
            location: format!("Task Scheduler{folder}"),
            enabled: Some(task.enabled),
            off_in_task_manager: false,
            changeable: false,
            origin,
            description,
            publisher,
            name: task.name,
            place: Place::ReadOnly,
        });
    }
}

fn task_command(program: &str, arguments: &str) -> String {
    if arguments.is_empty() {
        program.to_owned()
    } else {
        format!("{program} {arguments}")
    }
}

fn read_key(out: &mut StartupInventory, hive: Hive, run_key: RunKey, held: bool) {
    let location = format!(r"{}\{}", hive.short_name(), run_key.path());
    let path = if held { run_key.held() } else { run_key.path() };
    let key = match root(hive).open(path) {
        Ok(k) => k,
        Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return,
        Err(e) => {
            out.unreadable.push(format!("{location}: {}", e.message()));
            return;
        }
    };
    let values = match key.values() {
        Ok(v) => v,
        Err(e) => {
            out.unreadable.push(format!("{location}: {}", e.message()));
            return;
        }
    };
    for (name, value) in values {
        let Ok(command) = String::try_from(value) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let approved = run_key
            .approved()
            .filter(|_| !held)
            .and_then(|sub| approved_state(root(hive), sub, &name));
        let off_in_task_manager = approved == Some(false);
        let path = executable_of(&command);
        let (origin, description, publisher) = describe(Path::new(&path));
        out.items.push(StartupItem {
            id: format!("{location}\\{name}"),
            enabled: if held { Some(false) } else { approved },
            off_in_task_manager,
            changeable: !off_in_task_manager,
            name,
            command,
            path,
            source: run_key.source(),
            source_label: run_key.source().label(),
            scope: scope_of(hive),
            location: location.clone(),
            origin,
            description,
            publisher,
            place: Place::Key { hive, key: run_key },
        });
    }
}

/// `dir` is what's read; `home` is the Startup folder an entry belongs to, which is the same
/// folder unless the entry is switched off.
fn read_folder(
    out: &mut StartupInventory,
    dir: &Path,
    home: &Path,
    scope: StartupScope,
    held: bool,
) {
    let approved_root = match scope {
        StartupScope::CurrentUser => CURRENT_USER,
        StartupScope::AllUsers => LOCAL_MACHINE,
    };
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => {
            out.unreadable.push(format!("{}: {e}", dir.display()));
            return;
        }
    };
    for entry in entries.flatten() {
        let file = entry.file_name().to_string_lossy().into_owned();
        // desktop.ini is the folder's own metadata, not a startup item.
        if file.eq_ignore_ascii_case("desktop.ini") {
            continue;
        }
        let approved = approved_state(approved_root, "StartupFolder", &file);
        let off_in_task_manager = !held && approved == Some(false);
        // A shortcut has no publisher of its own. The program it opens does.
        let program = program_behind(&entry.path());
        let (origin, description, publisher) = describe(&program);
        out.items.push(StartupItem {
            id: home.join(&file).to_string_lossy().into_owned(),
            name: display_name(&file),
            command: entry.path().to_string_lossy().into_owned(),
            path: program.to_string_lossy().into_owned(),
            source: StartupSource::StartupFolder,
            source_label: StartupSource::StartupFolder.label(),
            scope,
            location: home.to_string_lossy().into_owned(),
            enabled: if held { Some(false) } else { approved },
            off_in_task_manager,
            // Moving out of the all-users folder needs a holding place only administrators
            // can write, which %ProgramData% doesn't give by default.
            changeable: scope == StartupScope::CurrentUser && !off_in_task_manager,
            origin,
            description,
            publisher,
            place: match scope {
                StartupScope::CurrentUser => Place::UserFolder { file },
                StartupScope::AllUsers => Place::ReadOnly,
            },
        });
    }
}

fn is_shortcut(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("lnk"))
}

/// What a Startup-folder file opens. Anything that isn't a shortcut opens itself, and so does a
/// shortcut with no plain target.
fn program_behind(path: &Path) -> PathBuf {
    is_shortcut(path)
        .then(|| file_info::shortcut_target(path))
        .flatten()
        .unwrap_or_else(|| path.to_path_buf())
}

/// Task Manager records its Startup-tab choice as a binary value whose first byte is even
/// when the entry is enabled and odd when disabled.
fn approved_state(root: &Key, sub: &str, name: &str) -> Option<bool> {
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

/// The program a command line starts: the quoted part, or everything up to `.exe`.
fn program_in(command: &str) -> &str {
    let command = command.trim();
    if let Some(rest) = command.strip_prefix('"') {
        return rest.split('"').next().unwrap_or(rest);
    }
    match command.to_ascii_lowercase().find(".exe") {
        Some(end) => &command[..end + 4],
        None => command.split_whitespace().next().unwrap_or(command),
    }
}

fn executable_of(command: &str) -> String {
    resolved(program_in(command))
}

/// A task keeps its program apart from its arguments, so spaces in it are part of the path and
/// there's no command line to take it out of.
fn task_executable(program: &str) -> String {
    resolved(program.trim().trim_matches('"'))
}

fn resolved(program: &str) -> String {
    let program = expand(program);
    // Only a bare name is looked for, and in one place. `X:app.exe` would join to where it says
    // and the check below would go there, which can be a network drive.
    if !is_bare_name(Path::new(&program)) {
        return program;
    }
    // A bare name is usually a Windows program, found through PATH when it runs.
    folders::known(Folder::Windows)
        .map(|w| w.join("System32").join(&program))
        .filter(|p| p.exists())
        .map_or(program, |p| p.to_string_lossy().into_owned())
}

fn is_bare_name(path: &Path) -> bool {
    let mut parts = path.components();
    matches!(
        (parts.next(), parts.next()),
        (Some(Component::Normal(_)), None)
    )
}

fn expand(text: &str) -> String {
    let source = HSTRING::from(text);
    let mut buffer = vec![0u16; 1024];
    // SAFETY: `source` is NUL-terminated and `buffer` is writable for its whole length.
    let written = unsafe { ExpandEnvironmentStringsW(&source, Some(&mut buffer)) } as usize;
    if written == 0 || written > buffer.len() {
        return text.to_string();
    }
    String::from_utf16_lossy(&buffer[..written - 1])
}

/// Programs that only start something else. Their publisher says who made the launcher, not the
/// entry: a script run through a Windows shell isn't Microsoft's, and Discord isn't GitHub's. The
/// names are only ever compared with a program's name, never started (`is_launcher`).
const LAUNCHERS: [&str; 8] = [
    "cmd.exe",
    "cscript.exe",
    "mshta.exe",
    "powershell.exe",
    "pwsh.exe",
    "rundll32.exe",
    "update.exe",
    "wscript.exe",
];

fn is_launcher(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| LAUNCHERS.iter().any(|l| name.eq_ignore_ascii_case(l)))
}

fn describe(path: &Path) -> (StartupOrigin, Option<String>, Option<String>) {
    if is_launcher(path) {
        return (StartupOrigin::Unknown, None, None);
    }
    let info = file_info::read(path).unwrap_or_default();
    let origin = match &info.company {
        Some(c) if c.starts_with("Microsoft") => StartupOrigin::Microsoft,
        Some(_) => StartupOrigin::ThirdParty,
        None => StartupOrigin::Unknown,
    };
    (origin, info.description, info.company)
}

/// Switches one entry. The ID is only compared with a fresh scan, never used as a location.
pub fn set_enabled(id: &str, enabled: bool) -> AppResult<StartupItem> {
    let item = inventory()
        .items
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::Conflict("this startup entry has changed; refresh".into()))?;
    if !item.changeable {
        return Err(AppError::Policy(
            "2VX can't switch this startup entry".into(),
        ));
    }
    if (item.enabled != Some(false)) == enabled {
        return Ok(item);
    }
    match &item.place {
        Place::Key {
            hive: Hive::LocalMachine,
            key,
        } if !privilege::is_elevated() => {
            BrokerSession::launch()?.move_startup(*key, &item.name, enabled)?;
        }
        Place::Key { hive, key } => move_value(*hive, *key, &item.name, enabled)?,
        Place::UserFolder { file } => move_shortcut(file, enabled)?,
        Place::ReadOnly => unreachable!("read-only entries aren't changeable"),
    }
    Ok(item)
}

/// Moves one value between its Run key and the holding key. The elevated helper runs this for
/// HKLM, which is why it reads the value here rather than being handed it.
pub fn move_value(hive: Hive, key: RunKey, name: &str, enabled: bool) -> AppResult<()> {
    if name.is_empty() || name.len() > MAX_VALUE_NAME {
        return Err(AppError::InvalidInput("bad startup entry name".into()));
    }
    let (from, to) = if enabled {
        (key.held(), key.path())
    } else {
        (key.path(), key.held())
    };
    transfer(root(hive), from, to, name, enabled)
}

/// Copy, verify, then remove the original, so a failure at any step leaves the entry where it
/// was. When `restoring`, a program that registered itself again meanwhile keeps its new entry.
fn transfer(root: &Key, from: &str, to: &str, name: &str, restoring: bool) -> AppResult<()> {
    let failed = |e: windows_result::Error| AppError::System(e.message());
    let source = root.create(from).map_err(failed)?;
    let value = source.get_value(name).map_err(failed)?;
    let target = root.create(to).map_err(failed)?;
    let existed = target.get_value(name).is_ok();
    let keep_theirs = existed && restoring;
    if !keep_theirs {
        target.set_value(name, &value).map_err(failed)?;
        if target.get_value(name).ok().as_ref() != Some(&value) {
            let _ = target.remove_value(name);
            return Err(AppError::System(format!(
                "`{name}` didn't read back after moving it"
            )));
        }
    }
    if let Err(e) = source.remove_value(name) {
        if !existed {
            let _ = target.remove_value(name);
        }
        return Err(failed(e));
    }
    Ok(())
}

/// `%LOCALAPPDATA%\com.twovx.tweaks\disabled-startup`, next to the journal.
fn held_folder() -> Option<PathBuf> {
    folders::known(Folder::LocalAppData)
        .map(|base| base.join("com.twovx.tweaks").join("disabled-startup"))
}

/// Whether 2VX opens at sign-in. The value existing is half the answer: the path in it may be from
/// an older install, and turning this on again rewrites it. The other half is Task Manager's, since
/// Windows skips a value that's off there.
pub fn auto_launch() -> AutoLaunch {
    let present = CURRENT_USER
        .open(RUN)
        .and_then(|key| key.get_value(AUTO_LAUNCH_VALUE))
        .is_ok();
    auto_launch_state(
        present,
        approved_state(CURRENT_USER, "Run", AUTO_LAUNCH_VALUE),
    )
}

fn auto_launch_state(present: bool, approved: Option<bool>) -> AutoLaunch {
    match (present, approved) {
        (false, _) => AutoLaunch::Off,
        (true, Some(false)) => AutoLaunch::OffInTaskManager,
        (true, _) => AutoLaunch::On,
    }
}

/// Quoted, with no arguments: a path with a space must not be split into a program and a rest.
fn auto_launch_command(exe: &Path) -> String {
    format!("\"{}\"", exe.display())
}

/// Adds or removes the Run value. Either way a copy switched off in Autoruns goes too, or it could
/// come back beside the new one.
pub fn set_auto_launch(enabled: bool) -> AppResult<()> {
    let failed = |e: windows_result::Error| AppError::System(e.message());
    // Writing the value again wouldn't change what Windows does, and removing it would lose the
    // entry the user switched off.
    if auto_launch() == AutoLaunch::OffInTaskManager {
        return Err(AppError::Conflict(
            "this is off in Task Manager, so turn it back on there".into(),
        ));
    }
    remove_auto_launch(RunKey::Run.held()).map_err(failed)?;
    if enabled {
        let command = auto_launch_command(&std::env::current_exe()?);
        CURRENT_USER
            .create(RUN)
            .and_then(|key| key.set_string(AUTO_LAUNCH_VALUE, command))
            .map_err(failed)
    } else {
        remove_auto_launch(RUN).map_err(failed)
    }
}

/// Removing what isn't there is fine.
fn remove_auto_launch(path: &str) -> windows_result::Result<()> {
    let key = match CURRENT_USER.options().read().write().open(path) {
        Ok(key) => key,
        Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(()),
        Err(e) => return Err(e),
    };
    match key.remove_value(AUTO_LAUNCH_VALUE) {
        Err(e) if e.code().0 as u32 != ERROR_FILE_NOT_FOUND => Err(e),
        _ => Ok(()),
    }
}

fn move_shortcut(file: &str, enabled: bool) -> AppResult<()> {
    let unavailable = || AppError::System("the Startup folder isn't available".into());
    let home = folders::known(Folder::Startup).ok_or_else(unavailable)?;
    let held = held_folder().ok_or_else(unavailable)?;
    std::fs::create_dir_all(&held)?;
    move_file(&home, &held, file, enabled)
}

fn move_file(home: &Path, held: &Path, file: &str, enabled: bool) -> AppResult<()> {
    let (from, to) = if enabled { (held, home) } else { (home, held) };
    // `file` came from reading this folder; this keeps it that way.
    if Path::new(file).file_name() != Some(std::ffi::OsStr::new(file)) {
        return Err(AppError::InvalidInput("bad startup entry name".into()));
    }
    let target = to.join(file);
    if target.exists() {
        return Err(AppError::Conflict(format!("{file} is already there")));
    }
    std::fs::rename(from.join(file), target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_launcher_says_nothing_about_who_made_the_entry() {
        for path in [
            r"C:\Windows\System32\cmd.exe",
            r"C:\Windows\System32\WindowsPowerShell\v1.0\PowerShell.EXE",
            r"C:\Users\me\AppData\Local\Discord\Update.exe",
        ] {
            let (origin, publisher, description) = describe(Path::new(path));
            assert_eq!(origin, StartupOrigin::Unknown);
            assert_eq!(publisher, None);
            assert_eq!(description, None);
        }
        assert!(!is_launcher(Path::new(
            "C:/Program Files/Example/example.exe"
        )));
    }

    #[test]
    fn approved_flag_parity() {
        assert_eq!(parse_approved(&[0x02, 0, 0]), Some(true));
        assert_eq!(parse_approved(&[0x06]), Some(true));
        assert_eq!(parse_approved(&[0x03, 1]), Some(false));
        assert_eq!(parse_approved(&[]), None);
    }

    #[test]
    fn the_sign_in_entry_quotes_its_path_and_takes_no_arguments() {
        assert_eq!(
            auto_launch_command(Path::new(r"C:\Program Files\2VX Tweaks\2vx-tweaks.exe")),
            r#""C:\Program Files\2VX Tweaks\2vx-tweaks.exe""#
        );
    }

    #[test]
    fn the_sign_in_entry_counts_as_on_only_if_windows_will_run_it() {
        assert_eq!(auto_launch_state(false, None), AutoLaunch::Off);
        assert_eq!(auto_launch_state(false, Some(false)), AutoLaunch::Off);
        assert_eq!(auto_launch_state(true, None), AutoLaunch::On);
        assert_eq!(auto_launch_state(true, Some(true)), AutoLaunch::On);
        assert_eq!(
            auto_launch_state(true, Some(false)),
            AutoLaunch::OffInTaskManager
        );
        assert_eq!(
            serde_json::to_value(AutoLaunch::OffInTaskManager).unwrap(),
            "offInTaskManager"
        );
    }

    #[test]
    fn every_source_has_a_label() {
        for (source, label) in [
            (StartupSource::RunKey, "Run key"),
            (StartupSource::RunOnceKey, "RunOnce key"),
            (StartupSource::StartupFolder, "Startup folder"),
            (StartupSource::Task, "Task"),
        ] {
            assert_eq!(source.label(), label);
        }
    }

    #[test]
    fn only_shortcuts_are_resolved() {
        assert!(is_shortcut(Path::new(r"C:\x\App.LNK")));
        assert!(!is_shortcut(Path::new(r"C:\x\app.exe")));
        // A file that isn't really a shortcut stands for itself rather than failing the scan.
        let dir = tempfile::tempdir().unwrap();
        let broken = dir.path().join("Broken.lnk");
        std::fs::write(&broken, b"not a shortcut").unwrap();
        assert_eq!(program_behind(&broken), broken);
    }

    #[test]
    fn task_commands_read_like_a_command_line() {
        assert_eq!(task_command(r"C:\x\up.exe", ""), r"C:\x\up.exe");
        assert_eq!(task_command(r"C:\x\up.exe", "/ua"), r"C:\x\up.exe /ua");
    }

    #[test]
    fn shortcut_names_drop_the_extension() {
        assert_eq!(display_name("Discord.lnk"), "Discord");
        assert_eq!(display_name("noext"), "noext");
    }

    #[test]
    fn finds_the_program_in_a_command_line() {
        assert_eq!(
            program_in(r#""C:\Program Files\Chat\chat.exe" --minimized"#),
            r"C:\Program Files\Chat\chat.exe"
        );
        assert_eq!(
            program_in(r"C:\Program Files\Sync\sync.exe /background"),
            r"C:\Program Files\Sync\sync.exe"
        );
        assert_eq!(program_in(r"C:\tools\run.bat now"), r"C:\tools\run.bat");
        assert_eq!(program_in("  "), "");
    }

    #[test]
    fn a_task_program_keeps_its_spaces() {
        assert_eq!(
            task_executable(r"C:\My Tools\run job.bat"),
            r"C:\My Tools\run job.bat"
        );
        assert_eq!(
            task_executable(r#""C:\My Tools\run job.bat""#),
            r"C:\My Tools\run job.bat"
        );
        let windir = std::env::var("SystemRoot").unwrap();
        assert_eq!(
            task_executable(r"%SystemRoot%\My Tools\job.exe"),
            format!(r"{windir}\My Tools\job.exe")
        );
    }

    #[test]
    fn only_a_bare_name_is_looked_for() {
        assert!(is_bare_name(Path::new("notepad.exe")));
        for path in [
            r"X:app.exe",
            "X:",
            r"\\host\share\app.exe",
            r"\app.exe",
            r"..\app.exe",
            r"tools\app.exe",
            r"C:\app.exe",
            "",
        ] {
            assert!(!is_bare_name(Path::new(path)), "{path}");
        }
        // Left as written, not joined to a folder and checked.
        assert_eq!(resolved(r"X:app.exe"), r"X:app.exe");
        assert_eq!(resolved(r"\\host\share\app.exe"), r"\\host\share\app.exe");
    }

    #[test]
    fn expands_environment_variables() {
        let windir = std::env::var("SystemRoot").unwrap();
        assert_eq!(expand(r"%SystemRoot%\x.exe"), format!(r"{windir}\x.exe"));
        assert_eq!(expand("plain"), "plain");
    }

    #[test]
    fn bad_names_are_refused_before_anything_moves() {
        let long = "x".repeat(MAX_VALUE_NAME + 1);
        for name in ["", long.as_str()] {
            assert!(matches!(
                move_value(Hive::CurrentUser, RunKey::Run, name, false),
                Err(AppError::InvalidInput(_))
            ));
        }
    }

    #[test]
    fn unknown_ids_are_refused() {
        assert!(matches!(
            set_enabled("not a real entry", false),
            Err(AppError::Conflict(_))
        ));
    }

    #[test]
    fn shortcuts_move_out_and_back() {
        let home = tempfile::tempdir().unwrap();
        let held = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join("App.lnk"), b"link").unwrap();

        move_file(home.path(), held.path(), "App.lnk", false).unwrap();
        assert!(!home.path().join("App.lnk").exists());
        assert!(held.path().join("App.lnk").exists());

        move_file(home.path(), held.path(), "App.lnk", true).unwrap();
        assert!(home.path().join("App.lnk").exists());
    }

    #[test]
    fn a_shortcut_name_cannot_climb_out_of_the_folder() {
        let home = tempfile::tempdir().unwrap();
        let held = tempfile::tempdir().unwrap();
        for name in [r"..\evil.lnk", "../evil.lnk", r"C:\Windows\notepad.exe"] {
            assert!(matches!(
                move_file(home.path(), held.path(), name, false),
                Err(AppError::InvalidInput(_))
            ));
        }
    }

    /// The move keeps the value's data and type, and a failed step leaves it in place.
    #[test]
    #[ignore = "writes to HKCU\\Software\\2VX-Test-Startup on the real registry"]
    fn values_keep_their_type_when_moved() {
        const BASE: &str = r"Software\2VX-Test-Startup";
        let (run, held) = (format!(r"{BASE}\Run"), format!(r"{BASE}\Held"));
        let key = CURRENT_USER.create(&run).unwrap();
        key.set_expand_string("App", r"%TEMP%\app.exe /quiet")
            .unwrap();
        let before = key.get_value("App").unwrap();

        transfer(CURRENT_USER, &run, &held, "App", false).unwrap();
        assert!(key.get_value("App").is_err());
        transfer(CURRENT_USER, &held, &run, "App", true).unwrap();
        assert_eq!(key.get_value("App").unwrap(), before);
        assert_eq!(key.get_value("App").unwrap().ty(), before.ty());

        // The program put itself back while its entry was held: its new entry stays.
        transfer(CURRENT_USER, &run, &held, "App", false).unwrap();
        key.set_string("App", "new.exe").unwrap();
        transfer(CURRENT_USER, &held, &run, "App", true).unwrap();
        assert_eq!(key.get_string("App").unwrap(), "new.exe");

        assert!(transfer(CURRENT_USER, &run, &held, "Missing", false).is_err());
        CURRENT_USER.remove_tree(BASE).unwrap();
    }

    #[test]
    #[cfg(windows)]
    fn reads_the_real_inventory_without_failing() {
        let inv = inventory();
        assert!(inv.items.iter().all(|i| !i.name.is_empty()));
    }
}
