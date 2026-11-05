//! Saves the system report. The frontend hands over text and nothing else: the file is whatever the
//! user picks in the Windows save dialog, so no path ever crosses a command.

use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use windows::Win32::Foundation::{ERROR_CANCELLED, HWND};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoCreateInstance,
    CoInitializeEx, CoTaskMemFree, CoUninitialize,
};
use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
use windows::Win32::UI::Shell::{
    FOS_FORCEFILESYSTEM, FOS_NOCHANGEDIR, FOS_NOREADONLYRETURN, FOS_OVERWRITEPROMPT,
    FOS_PATHMUSTEXIST, FOS_STRICTFILETYPES, FileSaveDialog, IFileSaveDialog, SIGDN_FILESYSPATH,
};
use windows::core::{HRESULT, Result as WinResult, w};

use crate::error::{AppError, AppResult};

pub const MAX_REPORT_BYTES: usize = 2 * 1024 * 1024;

/// Checked before the dialog, so the user isn't asked where to save something that won't be.
pub fn check_size(text: &str) -> AppResult<()> {
    if text.len() > MAX_REPORT_BYTES {
        return Err(AppError::InvalidInput("the report is too large".into()));
    }
    Ok(())
}

/// Asks where to save and writes the file. Returns its name, or `None` if the user cancels. `owner`
/// is the main window's handle, which is an integer here because a raw `HWND` can't cross threads.
pub fn export(owner: Option<isize>, text: &str) -> AppResult<Option<String>> {
    check_size(text)?;
    let Some(path) = choose_file(owner)? else {
        return Ok(None);
    };
    replace_file(&path, text)?;
    Ok(path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned()))
}

/// Writes beside the file, then renames over it. The user agreed to swap the old file for this one,
/// not to lose it to a full disk or a file another program has open.
fn replace_file(path: &Path, text: &str) -> AppResult<()> {
    let temp = temp_beside(path)?;
    let replaced = write_new(&temp, text).and_then(|()| fs::rename(&temp, path));
    if replaced.is_err() {
        let _ = fs::remove_file(&temp);
    }
    Ok(replaced?)
}

/// In the same folder, because a rename can't cross drives.
fn temp_beside(path: &Path) -> AppResult<PathBuf> {
    let mut random = [0u8; 8];
    getrandom::fill(&mut random).map_err(|e| AppError::Internal(format!("random source: {e}")))?;
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{:016x}.tmp", u64::from_ne_bytes(random)));
    Ok(path.with_file_name(name))
}

fn write_new(path: &Path, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(text.as_bytes())?;
    // On disk before the old file goes, so the rename can't leave an empty one behind a crash.
    file.sync_all()
}

/// The file dialog needs a single-threaded apartment, and worker threads have joined the
/// multithreaded one for Task Scheduler, so it gets a thread of its own.
fn choose_file(owner: Option<isize>) -> AppResult<Option<PathBuf>> {
    let dialog = std::thread::spawn(move || {
        // SAFETY: this thread is new, and paired with CoUninitialize below when it succeeds.
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) }.ok()?;
        let owner = owner.map(|handle| HWND(handle as *mut _));
        let chosen = show_dialog(owner);
        // SAFETY: the dialog's interfaces were dropped inside `show_dialog`.
        unsafe { CoUninitialize() };
        chosen
    });
    let chosen = dialog
        .join()
        .map_err(|_| AppError::Internal("the save dialog stopped unexpectedly".into()))?;
    chosen.map_err(|e| AppError::System(format!("save dialog: {}", e.message())))
}

fn show_dialog(owner: Option<HWND>) -> WinResult<Option<PathBuf>> {
    let filters = [COMDLG_FILTERSPEC {
        pszName: w!("Text file"),
        pszSpec: w!("*.txt"),
    }];
    let options = FOS_STRICTFILETYPES
        | FOS_OVERWRITEPROMPT
        | FOS_PATHMUSTEXIST
        | FOS_FORCEFILESYSTEM
        | FOS_NOREADONLYRETURN
        | FOS_NOCHANGEDIR;
    // SAFETY: COM is initialised on this thread, and each call is on an interface it owns.
    unsafe {
        let dialog: IFileSaveDialog =
            CoCreateInstance(&FileSaveDialog, None, CLSCTX_INPROC_SERVER)?;
        dialog.SetFileTypes(&filters)?;
        dialog.SetDefaultExtension(w!("txt"))?;
        dialog.SetFileName(w!("2VX system report.txt"))?;
        dialog.SetOptions(dialog.GetOptions()? | options)?;
        match dialog.Show(owner) {
            Ok(()) => {}
            Err(e) if e.code() == HRESULT::from_win32(ERROR_CANCELLED.0) => return Ok(None),
            Err(e) => return Err(e),
        }
        let path = dialog.GetResult()?.GetDisplayName(SIGDN_FILESYSPATH)?;
        let chosen = OsString::from_wide(path.as_wide());
        CoTaskMemFree(Some(path.as_ptr().cast()));
        Ok(Some(chosen.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries(dir: &Path) -> Vec<String> {
        fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn a_report_can_be_new_or_replace_one() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("report.txt");
        replace_file(&path, "first").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "first");
        replace_file(&path, "second").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "second");
        assert_eq!(entries(dir.path()), ["report.txt"], "no temp file left");
    }

    #[test]
    fn a_failed_save_keeps_the_old_file() {
        use std::os::windows::fs::OpenOptionsExt;

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("report.txt");
        fs::write(&path, "old").unwrap();
        // Open with no sharing, as a program holding the file would be.
        let held = OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&path)
            .unwrap();

        assert!(replace_file(&path, "new").is_err());
        drop(held);

        assert_eq!(fs::read_to_string(&path).unwrap(), "old");
        assert_eq!(entries(dir.path()), ["report.txt"], "no temp file left");
    }

    #[test]
    fn a_save_into_a_missing_folder_fails_cleanly() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gone").join("report.txt");
        assert!(replace_file(&path, "new").is_err());
        assert!(entries(dir.path()).is_empty());
    }

    #[test]
    fn the_size_cap_is_checked_before_any_dialog() {
        let limit = "x".repeat(MAX_REPORT_BYTES);
        assert!(check_size(&limit).is_ok());
        let over = "x".repeat(MAX_REPORT_BYTES + 1);
        assert!(matches!(
            export(None, &over),
            Err(AppError::InvalidInput(_))
        ));
    }
}
