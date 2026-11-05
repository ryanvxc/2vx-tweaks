//! The fixed allowlist of temporary-file and cache locations: measuring them and clearing them.
//!
//! Every location is resolved here from a known folder plus a constant path, so the frontend
//! can only name a location by ID. Neither pass follows a reparse point (junction, symlink),
//! and both use the same [`Scope`], so the size shown is what clearing removes. Deleting has
//! no undo, which is why the UI asks first (docs/security.md).

use std::fs::Metadata;
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use serde::{Deserialize, Serialize};
use windows::Win32::UI::Shell::{
    SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI, SHERB_NOSOUND, SHEmptyRecycleBinW, SHQUERYRBINFO,
    SHQueryRecycleBinW,
};
use windows::core::PCWSTR;

use crate::engine::broker::BrokerSession;
use crate::error::{AppError, AppResult};
use crate::system::folders::{self, Folder};
use crate::system::privilege;

const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
const RECYCLE_BIN: &str = "recycle-bin";
/// Temp folders keep anything this new: a running installer may still need it. Disk Cleanup
/// holds back its Temp files in the same way.
const SETTLED: Duration = Duration::from_secs(24 * 60 * 60);
/// Per location. Caches with more files than this are reported as "at least".
const MAX_ENTRIES: u64 = 250_000;
const MAX_TIME: Duration = Duration::from_secs(8);
/// Often enough for the UI to move, without a channel message per file.
const TICK_FILES: u64 = 256;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ScanStatus {
    Measured,
    /// The cap was hit; the numbers are a lower bound.
    Partial,
    /// The location does not exist on this PC (e.g. no NVIDIA driver).
    Missing,
    /// Reading needs administrator rights.
    Denied,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupLocation {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    /// Display form of what was measured.
    pub path: String,
    pub bytes: u64,
    pub files: u64,
    pub status: ScanStatus,
}

/// What a location gives up. Measuring and clearing both go by this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scope {
    Everything,
    /// Everything older than [`SETTLED`].
    Settled,
    /// Files with this extension directly in the folder. Subfolders hold other things:
    /// Quick Access pins sit below Recent, for one.
    TopLevel(&'static str),
}

impl Scope {
    fn descends(self) -> bool {
        !matches!(self, Scope::TopLevel(_))
    }

    fn takes(self, path: &Path, meta: &Metadata, now: SystemTime) -> bool {
        match self {
            Scope::Everything => true,
            Scope::Settled => settled(meta, now),
            Scope::TopLevel(ext) => path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case(ext)),
        }
    }

    fn takes_folder(self, meta: &Metadata, now: SystemTime) -> bool {
        self != Scope::Settled || settled(meta, now)
    }
}

/// Checks the creation time too: an installer unpacks files that keep an old modified time.
fn settled(meta: &Metadata, now: SystemTime) -> bool {
    let old = |t: std::io::Result<SystemTime>| {
        t.ok()
            .and_then(|t| now.duration_since(t).ok())
            .is_some_and(|age| age >= SETTLED)
    };
    old(meta.modified()) && old(meta.created())
}

struct Spec {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    paths: fn() -> Vec<PathBuf>,
    scope: Scope,
    /// Clearing goes through the elevated helper unless the app is already elevated.
    admin: bool,
}

fn local(rel: &[&str]) -> Vec<PathBuf> {
    folders::known(Folder::LocalAppData)
        .map(|base| {
            rel.iter()
                .map(|r| r.split('\\').fold(base.clone(), |p, part| p.join(part)))
                .collect()
        })
        .unwrap_or_default()
}

fn windows(rel: &str) -> Vec<PathBuf> {
    folders::known(Folder::Windows)
        .map(|w| vec![w.join(rel)])
        .unwrap_or_default()
}

const SPECS: &[Spec] = &[
    Spec {
        id: "user-temp",
        label: "Clear Temp Files",
        description: "Files apps left in your Temp folder. Anything from the last day stays.",
        // Not `env::temp_dir()`: that reads TMP, which can point anywhere.
        paths: || local(&["Temp"]),
        scope: Scope::Settled,
        admin: false,
    },
    Spec {
        id: "windows-temp",
        label: "Clear Windows Temp Files",
        description: "Windows' own Temp folder. Anything from the last day stays.",
        paths: || windows("Temp"),
        scope: Scope::Settled,
        admin: true,
    },
    Spec {
        id: "prefetch",
        label: "Clear Prefetch",
        description: "Windows rebuilds these, and apps can start slower until it has.",
        paths: || windows("Prefetch"),
        scope: Scope::TopLevel("pf"),
        admin: true,
    },
    Spec {
        id: "directx-shader-cache",
        label: "Clear DirectX Shader Cache",
        description: "Compiled shaders. Games rebuild them, so first loads take longer.",
        paths: || local(&["D3DSCache"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "nvidia-shader-cache",
        label: "Clear NVIDIA Shader Cache",
        description: "The driver's shader caches. Games rebuild them, so first loads take longer.",
        paths: || local(&[r"NVIDIA\DXCache", r"NVIDIA\GLCache"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "amd-shader-cache",
        label: "Clear AMD Shader Cache",
        description: "The driver's shader caches. Games rebuild them, so first loads take longer.",
        paths: || local(&[r"AMD\DxCache", r"AMD\DxcCache", r"AMD\VkCache"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "edge-cache",
        label: "Clear Edge Cache",
        description: "Pages and images Edge stored for your default profile.",
        paths: || {
            local(&[
                r"Microsoft\Edge\User Data\Default\Cache",
                r"Microsoft\Edge\User Data\Default\Code Cache",
                r"Microsoft\Edge\User Data\Default\GPUCache",
            ])
        },
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "chrome-cache",
        label: "Clear Chrome Cache",
        description: "Pages and images Chrome stored for your default profile.",
        paths: || {
            local(&[
                r"Google\Chrome\User Data\Default\Cache",
                r"Google\Chrome\User Data\Default\Code Cache",
                r"Google\Chrome\User Data\Default\GPUCache",
            ])
        },
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "brave-cache",
        label: "Clear Brave Cache",
        description: "Pages and images Brave stored for your default profile.",
        paths: || {
            local(&[
                r"BraveSoftware\Brave-Browser\User Data\Default\Cache",
                r"BraveSoftware\Brave-Browser\User Data\Default\Code Cache",
                r"BraveSoftware\Brave-Browser\User Data\Default\GPUCache",
            ])
        },
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "internet-cache",
        label: "Clear Internet Cache",
        description: "Web files Windows stores for apps that use its browser engine.",
        paths: || local(&[r"Microsoft\Windows\INetCache"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "package-caches",
        label: "Clear Package Manager Caches",
        description: "npm, pip and Yarn downloads. They download again when needed.",
        paths: || local(&["npm-cache", r"pip\Cache", r"Yarn\Cache"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "recent-files",
        label: "Clear Recent Files List",
        description: "Shortcuts behind Recent files. The files themselves and your pins stay.",
        paths: || folders::known(Folder::Recent).into_iter().collect(),
        scope: Scope::TopLevel("lnk"),
        admin: false,
    },
    Spec {
        id: "error-reports",
        label: "Clear Error Reports",
        description: "Crash and problem reports Windows saved for your account.",
        paths: || local(&[r"Microsoft\Windows\WER"]),
        scope: Scope::Everything,
        admin: false,
    },
    Spec {
        id: "crash-dumps",
        label: "Clear Crash Dumps",
        description: "Memory dumps apps left when they crashed.",
        paths: || local(&["CrashDumps"]),
        scope: Scope::Everything,
        admin: false,
    },
];

pub fn scan() -> Vec<CleanupLocation> {
    let mut out: Vec<CleanupLocation> = SPECS.iter().map(measure).collect();
    out.push(recycle_bin());
    out
}

fn measure(spec: &Spec) -> CleanupLocation {
    let paths = (spec.paths)();
    let mut total = Tally::default();
    let mut any = false;
    let mut denied = false;
    for path in &paths {
        match std::fs::symlink_metadata(path) {
            Ok(meta)
                if meta.is_dir() && meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0 =>
            {
                any = true;
                walk(path, spec.scope, &mut total, Instant::now());
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => denied = true,
            Err(_) => {}
        }
    }
    let status = if !any && denied || (any && total.denied_root) {
        ScanStatus::Denied
    } else if !any {
        ScanStatus::Missing
    } else if total.truncated {
        ScanStatus::Partial
    } else {
        ScanStatus::Measured
    };
    CleanupLocation {
        id: spec.id,
        label: spec.label,
        description: spec.description,
        path: paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("; "),
        bytes: total.bytes,
        files: total.files,
        status,
    }
}

#[derive(Default)]
struct Tally {
    bytes: u64,
    files: u64,
    entries: u64,
    truncated: bool,
    denied_root: bool,
}

/// Iterative walk, never following reparse points.
fn walk(root: &Path, scope: Scope, tally: &mut Tally, started: Instant) {
    let now = SystemTime::now();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                if dir == root && e.kind() == std::io::ErrorKind::PermissionDenied {
                    tally.denied_root = true;
                }
                continue;
            }
        };
        for entry in entries.flatten() {
            tally.entries += 1;
            if tally.entries > MAX_ENTRIES || started.elapsed() > MAX_TIME {
                tally.truncated = true;
                return;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                continue;
            }
            if meta.is_dir() {
                if scope.descends() {
                    stack.push(entry.path());
                }
            } else if scope.takes(&entry.path(), &meta, now) {
                tally.files += 1;
                tally.bytes += meta.len();
            }
        }
    }
}

fn recycle_bin() -> CleanupLocation {
    let mut info = SHQUERYRBINFO {
        cbSize: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: `info` is a correctly sized, writable SHQUERYRBINFO; a null root queries
    // every drive's Recycle Bin.
    let ok = unsafe { SHQueryRecycleBinW(None, &mut info) }.is_ok();
    CleanupLocation {
        id: RECYCLE_BIN,
        label: "Empty Recycle Bin",
        description: "Files you deleted. Emptying the bin removes them for good.",
        path: "All drives".into(),
        bytes: if ok { info.i64Size.max(0) as u64 } else { 0 },
        files: if ok {
            info.i64NumItems.max(0) as u64
        } else {
            0
        },
        status: if ok {
            ScanStatus::Measured
        } else {
            ScanStatus::Missing
        },
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CleanupStatus {
    Cleared,
    /// The administrator prompt was refused, so nothing ran. Not a clean with 0 bytes to show.
    ElevationDeclined,
    Failed,
}

/// What clearing one location did.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CleanupResult {
    pub id: String,
    pub freed_bytes: u64,
    pub removed_files: u64,
    /// Files left behind, nearly always because something has them open.
    pub skipped_files: u64,
    pub status: CleanupStatus,
    /// Why this location wasn't cleared at all.
    pub error: Option<String>,
}

impl CleanupResult {
    fn new(id: &str) -> Self {
        CleanupResult {
            id: id.into(),
            freed_bytes: 0,
            removed_files: 0,
            skipped_files: 0,
            status: CleanupStatus::Cleared,
            error: None,
        }
    }

    fn failed(id: &str, error: &AppError) -> Self {
        CleanupResult {
            status: match error {
                AppError::ElevationRequired(_) => CleanupStatus::ElevationDeclined,
                _ => CleanupStatus::Failed,
            },
            error: Some(error.to_string()),
            ..Self::new(id)
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CleanupPhase {
    Started,
    /// Still going. Only locations cleared in this process report this: the elevated helper
    /// answers once, when it's done.
    Clearing,
    Finished,
    Declined,
    Failed,
}

/// One step of a run, sent to the UI as it happens.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CleanupProgress {
    pub id: String,
    pub phase: CleanupPhase,
    /// Freed from this location so far.
    pub freed_bytes: u64,
    /// Freed across the whole run so far.
    pub total_freed_bytes: u64,
}

/// Keeps the run's total, which no single location knows.
struct Reporter<'a> {
    send: &'a mut dyn FnMut(CleanupProgress),
    total: u64,
}

impl Reporter<'_> {
    fn emit(&mut self, id: &str, phase: CleanupPhase, freed_bytes: u64) {
        (self.send)(CleanupProgress {
            id: id.into(),
            phase,
            freed_bytes,
            total_freed_bytes: self.total + freed_bytes,
        });
    }

    fn started(&mut self, id: &str) {
        self.emit(id, CleanupPhase::Started, 0);
    }

    fn finished(&mut self, result: &CleanupResult) {
        let phase = match result.status {
            CleanupStatus::Cleared => CleanupPhase::Finished,
            CleanupStatus::ElevationDeclined => CleanupPhase::Declined,
            CleanupStatus::Failed => CleanupPhase::Failed,
        };
        self.emit(&result.id, phase, result.freed_bytes);
        self.total += result.freed_bytes;
    }
}

/// The elevated side of a run. A seam so tests can refuse the prompt without showing one.
trait Helper {
    fn clean(&mut self, id: &str) -> AppResult<CleanupResult>;
}

impl Helper for BrokerSession {
    fn clean(&mut self, id: &str) -> AppResult<CleanupResult> {
        BrokerSession::clean(self, id)
    }
}

fn spec(id: &str) -> Option<&'static Spec> {
    SPECS.iter().find(|s| s.id == id)
}

/// Clears the given locations. Every ID is checked before anything is deleted.
pub fn clean(
    ids: &[String],
    on_progress: &mut dyn FnMut(CleanupProgress),
) -> AppResult<Vec<CleanupResult>> {
    clean_with(
        ids,
        privilege::is_elevated(),
        BrokerSession::launch,
        on_progress,
    )
}

fn clean_with<H: Helper>(
    ids: &[String],
    elevated: bool,
    mut launch: impl FnMut() -> AppResult<H>,
    on_progress: &mut dyn FnMut(CleanupProgress),
) -> AppResult<Vec<CleanupResult>> {
    if ids.is_empty() {
        return Err(AppError::InvalidInput("choose a location to clear".into()));
    }
    if let Some(id) = ids
        .iter()
        .find(|id| *id != RECYCLE_BIN && spec(id).is_none())
    {
        return Err(AppError::InvalidInput(format!(
            "unknown cleanup location `{id}`"
        )));
    }

    let mut reporter = Reporter {
        send: on_progress,
        total: 0,
    };
    let mut results = vec![];
    let mut for_helper = vec![];
    for id in ids {
        let spec = spec(id);
        if let Some(s) = spec.filter(|s| s.admin && !elevated) {
            for_helper.push(s.id);
            continue;
        }
        reporter.started(id);
        let result = match spec {
            None => empty_recycle_bin(),
            Some(s) => clear(s, &mut |freed| {
                reporter.emit(s.id, CleanupPhase::Clearing, freed);
            }),
        };
        reporter.finished(&result);
        results.push(result);
    }

    // Launched on the first location that needs it, which is the one showing as started while the
    // prompt is up. A refusal stays in `session`, so it covers the rest without asking again.
    let mut session = None;
    for id in for_helper {
        reporter.started(id);
        let result = match session.get_or_insert_with(&mut launch) {
            Ok(session) => session
                .clean(id)
                .unwrap_or_else(|e| CleanupResult::failed(id, &e)),
            Err(e) => CleanupResult::failed(id, e),
        };
        reporter.finished(&result);
        results.push(result);
    }
    Ok(results)
}

/// What the elevated helper runs: only the locations that need it, by ID.
pub fn clean_elevated(id: &str) -> AppResult<CleanupResult> {
    let spec = spec(id).filter(|s| s.admin).ok_or_else(|| {
        AppError::Policy(format!("`{id}` is not an administrator cleanup location"))
    })?;
    Ok(clear(spec, &mut |_| {}))
}

fn clear(spec: &Spec, tick: &mut dyn FnMut(u64)) -> CleanupResult {
    let mut out = CleanupResult::new(spec.id);
    let now = SystemTime::now();
    for root in (spec.paths)() {
        // The root must be a real folder: a junction here would point somewhere else entirely.
        let plain = std::fs::symlink_metadata(&root)
            .is_ok_and(|m| m.is_dir() && m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT == 0);
        if plain {
            clear_tree(&root, spec.scope, now, &mut out, tick);
        }
    }
    tracing::info!(
        location = spec.id,
        freed = out.freed_bytes,
        removed = out.removed_files,
        skipped = out.skipped_files,
        "location cleared"
    );
    out
}

/// Removes what `scope` takes below `root`, keeping `root` itself. A file that won't go is
/// counted and left. Reparse points are left too: neither followed nor removed. `tick` gets the
/// bytes freed so far every [`TICK_FILES`] files, so a large cache doesn't look stuck.
fn clear_tree(
    root: &Path,
    scope: Scope,
    now: SystemTime,
    out: &mut CleanupResult,
    tick: &mut dyn FnMut(u64),
) {
    let mut stack = vec![root.to_path_buf()];
    let mut seen = vec![];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(meta) = entry.metadata() else {
                out.skipped_files += 1;
                continue;
            };
            if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                continue;
            }
            let path = entry.path();
            if meta.is_dir() {
                if scope.descends() {
                    stack.push(path.clone());
                    seen.push((path, meta));
                }
            } else if scope.takes(&path, &meta, now) {
                match std::fs::remove_file(&path) {
                    Ok(()) => {
                        out.freed_bytes += meta.len();
                        out.removed_files += 1;
                        if out.removed_files.is_multiple_of(TICK_FILES) {
                            tick(out.freed_bytes);
                        }
                    }
                    Err(_) => out.skipped_files += 1,
                }
            }
        }
    }
    // Parents come first in `seen`, so this goes deepest first. A folder that still holds
    // anything simply refuses.
    for (dir, meta) in seen.iter().rev() {
        if scope.takes_folder(meta, now) {
            let _ = std::fs::remove_dir(dir);
        }
    }
}

fn empty_recycle_bin() -> CleanupResult {
    let before = recycle_bin();
    let mut out = CleanupResult::new(RECYCLE_BIN);
    // Windows reports an error for a bin that's already empty.
    if before.files == 0 {
        return out;
    }
    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;
    // SAFETY: no window and a null root are documented: every drive, with no UI.
    match unsafe { SHEmptyRecycleBinW(None, PCWSTR::null(), flags) } {
        Ok(()) => {
            out.freed_bytes = before.bytes;
            out.removed_files = before.files;
        }
        Err(e) => {
            out.status = CleanupStatus::Failed;
            out.error = Some(format!("Couldn't empty the Recycle Bin: {}", e.message()));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::fs::{File, FileTimes};
    use std::os::windows::fs::FileTimesExt;

    use super::*;

    fn cleared(root: &Path, scope: Scope) -> CleanupResult {
        let mut out = CleanupResult::new("test");
        clear_tree(root, scope, SystemTime::now(), &mut out, &mut |_| {});
        out
    }

    /// These tests only ever name administrator locations with `elevated` false, so everything
    /// goes to the fake helper and nothing on this PC is cleared.
    const ADMIN_IDS: [&str; 2] = ["windows-temp", "prefetch"];

    struct FakeHelper;

    impl Helper for FakeHelper {
        fn clean(&mut self, id: &str) -> AppResult<CleanupResult> {
            Ok(CleanupResult {
                freed_bytes: 100,
                ..CleanupResult::new(id)
            })
        }
    }

    fn run(
        launch: impl FnMut() -> AppResult<FakeHelper>,
    ) -> (Vec<CleanupResult>, Vec<CleanupProgress>) {
        let ids = ADMIN_IDS.map(String::from);
        let mut events = vec![];
        let results = clean_with(&ids, false, launch, &mut |p| events.push(p)).unwrap();
        (results, events)
    }

    #[test]
    fn refusing_the_prompt_is_not_a_success() {
        let mut prompts = 0;
        let (results, events) = run(|| {
            prompts += 1;
            Err(AppError::ElevationRequired("declined".into()))
        });
        assert_eq!(prompts, 1, "one refusal covers the whole run");
        for result in &results {
            assert_eq!(result.status, CleanupStatus::ElevationDeclined);
            assert_eq!(result.freed_bytes, 0);
            assert!(result.error.is_some());
        }
        let phases: Vec<_> = events.iter().map(|e| e.phase).collect();
        assert_eq!(
            phases,
            [
                CleanupPhase::Started,
                CleanupPhase::Declined,
                CleanupPhase::Started,
                CleanupPhase::Declined
            ]
        );
    }

    #[test]
    fn a_helper_that_fails_to_start_is_a_failure_not_a_refusal() {
        let (results, _) = run(|| Err(AppError::System("no pipe".into())));
        assert!(results.iter().all(|r| r.status == CleanupStatus::Failed));
    }

    #[test]
    fn progress_carries_each_location_and_the_running_total() {
        let (results, events) = run(|| Ok(FakeHelper));
        assert!(results.iter().all(|r| r.status == CleanupStatus::Cleared));
        let done: Vec<_> = events
            .iter()
            .filter(|e| e.phase == CleanupPhase::Finished)
            .map(|e| (e.id.as_str(), e.freed_bytes, e.total_freed_bytes))
            .collect();
        assert_eq!(done, [("windows-temp", 100, 100), ("prefetch", 100, 200)]);
    }

    #[test]
    fn large_locations_tick_while_they_clear() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..TICK_FILES * 2 {
            std::fs::write(dir.path().join(format!("{i}.bin")), [0u8; 10]).unwrap();
        }
        let mut ticks = vec![];
        let mut out = CleanupResult::new("test");
        let now = SystemTime::now();
        clear_tree(dir.path(), Scope::Everything, now, &mut out, &mut |freed| {
            ticks.push(freed);
        });
        assert_eq!(ticks, [TICK_FILES * 10, TICK_FILES * 20]);
    }

    fn age(path: &Path, by: Duration) {
        let then = SystemTime::now() - by;
        let times = FileTimes::new().set_modified(then).set_created(then);
        File::options()
            .write(true)
            .open(path)
            .unwrap()
            .set_times(times)
            .unwrap();
    }

    #[test]
    fn clearing_empties_the_tree_and_keeps_the_root() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("a.bin"), [0u8; 100]).unwrap();
        std::fs::write(dir.path().join("sub").join("b.bin"), [0u8; 50]).unwrap();
        let out = cleared(dir.path(), Scope::Everything);
        assert_eq!((out.removed_files, out.freed_bytes), (2, 150));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
        assert!(dir.path().exists());
    }

    #[test]
    fn temp_folders_keep_recent_files() {
        let dir = tempfile::tempdir().unwrap();
        let fresh = dir.path().join("fresh.tmp");
        let stale = dir.path().join("stale.tmp");
        let unpacked = dir.path().join("unpacked.tmp");
        for path in [&fresh, &stale, &unpacked] {
            std::fs::write(path, [0u8; 10]).unwrap();
        }
        age(&stale, SETTLED * 2);
        // An old modified time on a new file, as an installer leaves behind.
        File::options()
            .write(true)
            .open(&unpacked)
            .unwrap()
            .set_modified(SystemTime::now() - SETTLED * 2)
            .unwrap();

        let mut tally = Tally::default();
        walk(dir.path(), Scope::Settled, &mut tally, Instant::now());
        assert_eq!(tally.files, 1, "the size shown is what clearing removes");

        let out = cleared(dir.path(), Scope::Settled);
        assert_eq!(out.removed_files, 1);
        assert!(fresh.exists() && unpacked.exists() && !stale.exists());
    }

    #[test]
    fn top_level_scope_stays_out_of_subfolders() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("AutomaticDestinations")).unwrap();
        let pin = dir.path().join("AutomaticDestinations").join("pin.lnk");
        std::fs::write(&pin, [0u8; 10]).unwrap();
        std::fs::write(dir.path().join("recent.lnk"), [0u8; 10]).unwrap();
        std::fs::write(dir.path().join("keep.ini"), [0u8; 10]).unwrap();
        let out = cleared(dir.path(), Scope::TopLevel("lnk"));
        assert_eq!(out.removed_files, 1);
        assert!(pin.exists() && dir.path().join("keep.ini").exists());
    }

    /// Needs Developer Mode or elevation to make the link; without either it proves nothing.
    #[test]
    fn links_are_not_followed() {
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("precious.txt"), [1u8; 10]).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let link = dir.path().join("link");
        if std::os::windows::fs::symlink_dir(outside.path(), &link).is_err() {
            return;
        }
        let out = cleared(dir.path(), Scope::Everything);
        assert_eq!(out.removed_files, 0);
        assert!(outside.path().join("precious.txt").exists());
    }

    #[test]
    fn an_unknown_id_stops_the_run_before_anything_is_deleted() {
        let ids = ["crash-dumps".to_string(), r"C:\Windows".to_string()];
        let mut none = |_| panic!("nothing starts");
        assert!(matches!(
            clean(&ids, &mut none),
            Err(AppError::InvalidInput(_))
        ));
        assert!(matches!(
            clean(&[], &mut none),
            Err(AppError::InvalidInput(_))
        ));
    }

    #[test]
    fn the_elevated_helper_takes_only_administrator_locations() {
        for id in ["user-temp", RECYCLE_BIN, "nope"] {
            assert!(matches!(clean_elevated(id), Err(AppError::Policy(_))));
        }
    }

    #[test]
    #[cfg(windows)]
    fn every_location_sits_below_a_known_folder() {
        let bases: Vec<PathBuf> = [Folder::LocalAppData, Folder::Windows, Folder::Recent]
            .into_iter()
            .filter_map(folders::known)
            .collect();
        for spec in SPECS {
            for path in (spec.paths)() {
                let allowed = bases.iter().any(|b| {
                    path.starts_with(b) && (path != *b || spec.scope != Scope::Everything)
                });
                assert!(allowed, "{} resolves to {}", spec.id, path.display());
            }
        }
    }

    #[test]
    fn measures_a_tree_and_skips_nothing_real() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("a.bin"), [0u8; 100]).unwrap();
        std::fs::write(dir.path().join("sub").join("b.bin"), [0u8; 50]).unwrap();
        let mut t = Tally::default();
        walk(dir.path(), Scope::Everything, &mut t, Instant::now());
        assert_eq!((t.files, t.bytes, t.truncated), (2, 150, false));
    }

    #[test]
    fn ids_are_unique() {
        let mut ids: Vec<_> = SPECS.iter().map(|s| s.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), SPECS.len());
    }

    #[test]
    #[cfg(windows)]
    fn real_scan_reports_every_location() {
        assert_eq!(scan().len(), SPECS.len() + 1);
    }
}
