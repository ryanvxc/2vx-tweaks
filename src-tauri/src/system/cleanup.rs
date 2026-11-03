//! Read-only measurement of a fixed allowlist of temporary-file and cache locations.
//!
//! The scan only reads sizes. It never follows reparse points (junctions, symlinks), stops
//! at a file-count cap, and resolves every location itself: the frontend cannot name a
//! path. Cleaning is planned and will reuse this allowlist (docs/security.md).

use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Serialize;
use windows::Win32::UI::Shell::{SHQUERYRBINFO, SHQueryRecycleBinW};

use super::folders::{self, Folder};

const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
/// Per location. Caches with more files than this are reported as "at least".
const MAX_ENTRIES: u64 = 250_000;
const MAX_TIME: Duration = Duration::from_secs(8);

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

struct Spec {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    paths: fn() -> Vec<PathBuf>,
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

const SPECS: &[Spec] = &[
    Spec {
        id: "user-temp",
        label: "Your temporary files",
        description: "Files apps leave in your Temp folder. Apps that are running may still use some of them.",
        paths: || vec![std::env::temp_dir()],
    },
    Spec {
        id: "windows-temp",
        label: "Windows temporary files",
        description: "Windows' own Temp folder, used by installers and services.",
        paths: || {
            folders::known(Folder::Windows)
                .map(|w| vec![w.join("Temp")])
                .unwrap_or_default()
        },
    },
    Spec {
        id: "directx-shader-cache",
        label: "DirectX shader cache",
        description: "Compiled shaders Windows keeps so games load faster. Rebuilt automatically when needed.",
        paths: || local(&["D3DSCache"]),
    },
    Spec {
        id: "nvidia-shader-cache",
        label: "NVIDIA shader cache",
        description: "The NVIDIA driver's DirectX and OpenGL shader caches.",
        paths: || local(&[r"NVIDIA\DXCache", r"NVIDIA\GLCache"]),
    },
    Spec {
        id: "amd-shader-cache",
        label: "AMD shader cache",
        description: "The AMD driver's DirectX and Vulkan shader caches.",
        paths: || local(&[r"AMD\DxCache", r"AMD\DxcCache", r"AMD\VkCache"]),
    },
    Spec {
        id: "edge-cache",
        label: "Microsoft Edge cache",
        description: "Web pages and images Edge stored for your default profile.",
        paths: || {
            local(&[
                r"Microsoft\Edge\User Data\Default\Cache",
                r"Microsoft\Edge\User Data\Default\Code Cache",
            ])
        },
    },
    Spec {
        id: "chrome-cache",
        label: "Google Chrome cache",
        description: "Web pages and images Chrome stored for your default profile.",
        paths: || {
            local(&[
                r"Google\Chrome\User Data\Default\Cache",
                r"Google\Chrome\User Data\Default\Code Cache",
            ])
        },
    },
    Spec {
        id: "error-reports",
        label: "Windows error reports",
        description: "Crash and problem reports Windows saved for your account.",
        paths: || local(&[r"Microsoft\Windows\WER"]),
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
                walk(path, &mut total, Instant::now());
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
fn walk(root: &Path, tally: &mut Tally, started: Instant) {
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
                stack.push(entry.path());
            } else {
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
        id: "recycle-bin",
        label: "Recycle Bin",
        description: "Files you deleted that Windows keeps until you empty the bin.",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_a_tree_and_skips_nothing_real() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("a.bin"), [0u8; 100]).unwrap();
        std::fs::write(dir.path().join("sub").join("b.bin"), [0u8; 50]).unwrap();
        let mut t = Tally::default();
        walk(dir.path(), &mut t, Instant::now());
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
