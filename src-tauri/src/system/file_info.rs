//! What a program says about itself in its version resource. Self-reported and unverified:
//! good for naming a startup entry, never a statement that it can be trusted.

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use windows::Win32::Storage::FileSystem::{
    GetDriveTypeW, GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, WIN32_FIND_DATAW,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, IPersistFile,
    STGM_READ,
};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::core::{HSTRING, Interface, PCWSTR};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileInfo {
    pub description: Option<String>,
    pub company: Option<String>,
}

/// US English, Unicode: the block most programs have when they list no translation.
const FALLBACK_BLOCK: &str = "040904b0";
/// What `GetDriveTypeW` says of a disk that's in this PC. The crate's constants sit behind a
/// feature that brings in nothing else this app uses.
const DRIVE_REMOVABLE: u32 = 2;
const DRIVE_FIXED: u32 = 3;

/// What a `.lnk` file points at, as stored in it. Never calls `Resolve`, which searches the disk for
/// a moved target and can rewrite the shortcut. `None` for a broken file, and for an advertised
/// (installer-managed) shortcut, which has no plain target.
pub fn shortcut_target(link: &Path) -> Option<PathBuf> {
    let mut target = [0u16; 1024];
    let mut find = WIN32_FIND_DATAW::default();
    // SAFETY: plain COM calls on interfaces this function owns, and the buffers are valid for the
    // call. A thread that already chose another apartment model reports an error here and COM
    // still works on it, so that's ignored.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let shortcut: IShellLinkW =
            CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER).ok()?;
        let file: IPersistFile = shortcut.cast().ok()?;
        file.Load(&HSTRING::from(link.as_os_str()), STGM_READ)
            .ok()?;
        shortcut.GetPath(&mut target, &mut find, 0).ok()?;
    }
    let length = target.iter().position(|c| *c == 0).unwrap_or(target.len());
    (length > 0).then(|| OsString::from_wide(&target[..length]).into())
}

/// `C:\` for a path that starts on a drive letter. Everything else has no drive to ask about: a
/// share, a device path, or a relative name that Windows would go looking for.
fn drive_root(path: &Path) -> Option<String> {
    match path.to_str()?.as_bytes() {
        [letter, b':', b'\\' | b'/', ..] if letter.is_ascii_alphabetic() => {
            Some(format!("{}:\\", char::from(*letter)))
        }
        _ => None,
    }
}

fn is_local_drive(path: &Path) -> bool {
    drive_root(path).is_some_and(|root| {
        // SAFETY: the HSTRING is NUL-terminated and outlives the call. This asks Windows about the
        // drive letter; it opens nothing on the drive.
        let kind = unsafe { GetDriveTypeW(&HSTRING::from(root)) };
        matches!(kind, DRIVE_FIXED | DRIVE_REMOVABLE)
    })
}

/// `None` for anything not on a drive in this PC. The path comes from a shortcut, a task or a Run
/// value, which anyone can point at a share: opening it there would send this account's credentials
/// to that host, and an offline one blocks the scan, per entry, for as long as it takes to time out.
pub fn read(path: &Path) -> Option<FileInfo> {
    if !is_local_drive(path) {
        return None;
    }
    let name = HSTRING::from(path.as_os_str());
    // SAFETY: `name` outlives the calls and `data` is `size` bytes; VerQueryValueW returns
    // pointers into `data`, read before it's dropped.
    unsafe {
        let size = GetFileVersionInfoSizeW(&name, None);
        if size == 0 {
            return None;
        }
        let mut data = vec![0u8; size as usize];
        GetFileVersionInfoW(&name, None, size, data.as_mut_ptr().cast()).ok()?;

        let mut raw = std::ptr::null_mut();
        let mut len = 0u32;
        let translated = VerQueryValueW(
            data.as_ptr().cast(),
            &HSTRING::from(r"\VarFileInfo\Translation"),
            &mut raw,
            &mut len,
        )
        .as_bool();
        let block = if translated && len >= 4 {
            let ids = raw.cast::<[u16; 2]>().read_unaligned();
            format!("{:04x}{:04x}", ids[0], ids[1])
        } else {
            FALLBACK_BLOCK.to_string()
        };
        Some(FileInfo {
            description: field(&data, &block, "FileDescription"),
            company: field(&data, &block, "CompanyName"),
        })
    }
}

/// SAFETY: `data` must be a block filled by `GetFileVersionInfoW`.
unsafe fn field(data: &[u8], block: &str, name: &str) -> Option<String> {
    let path = HSTRING::from(format!(r"\StringFileInfo\{block}\{name}"));
    let mut raw = std::ptr::null_mut();
    let mut len = 0u32;
    // SAFETY: per this function's contract; the result points into `data`.
    let found = unsafe { VerQueryValueW(data.as_ptr().cast(), &path, &mut raw, &mut len) };
    if !found.as_bool() || len == 0 {
        return None;
    }
    // SAFETY: on success `raw` is a NUL-terminated UTF-16 string inside `data`.
    let text = unsafe { PCWSTR(raw.cast()).to_string() }.ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn reads_a_windows_binary() {
        let windir = crate::system::folders::known(crate::system::folders::Folder::Windows)
            .expect("Windows folder resolves");
        let info = read(&windir.join("explorer.exe")).expect("explorer has version info");
        assert!(info.company.is_some_and(|c| c.contains("Microsoft")));
    }

    #[test]
    fn only_a_path_on_a_drive_letter_has_a_drive() {
        assert_eq!(
            drive_root(Path::new(r"C:\Windows\explorer.exe")).as_deref(),
            Some(r"C:\")
        );
        assert_eq!(drive_root(Path::new("d:/x.exe")).as_deref(), Some(r"d:\"));
        for path in [
            r"\\server\share\app.exe",
            "//server/share/app.exe",
            r"\\?\UNC\server\share\app.exe",
            r"\\?\C:\app.exe",
            r"\\.\C:\app.exe",
            r"C:app.exe",
            r"\Windows\app.exe",
            "app.exe",
            r"é:\app.exe",
            "",
        ] {
            assert_eq!(drive_root(Path::new(path)), None, "{path}");
        }
    }

    #[test]
    fn a_share_is_never_opened() {
        // `.invalid` can't resolve, so this can't reach a host even if the check were wrong.
        let share = Path::new(r"\\2vx-test.invalid\share\app.exe");
        assert!(!is_local_drive(share));
        assert_eq!(read(share), None);
    }

    #[test]
    #[cfg(windows)]
    fn the_system_drive_is_local() {
        let windir = crate::system::folders::known(crate::system::folders::Folder::Windows)
            .expect("Windows folder resolves");
        assert!(is_local_drive(&windir));
    }

    #[test]
    fn a_missing_file_has_no_info() {
        assert_eq!(read(Path::new(r"Z:\no\such\file.exe")), None);
    }
}
