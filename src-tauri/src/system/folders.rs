//! Known folder paths, resolved the way Windows does (redirected folders included).

use std::path::PathBuf;

use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::{
    FOLDERID_CommonStartup, FOLDERID_LocalAppData, FOLDERID_Startup, FOLDERID_Windows,
    KNOWN_FOLDER_FLAG, SHGetKnownFolderPath,
};
use windows::core::GUID;

#[derive(Debug, Clone, Copy)]
pub enum Folder {
    /// The current user's Startup folder.
    Startup,
    /// The all-users Startup folder.
    CommonStartup,
    LocalAppData,
    Windows,
}

pub fn known(folder: Folder) -> Option<PathBuf> {
    let id: GUID = match folder {
        Folder::Startup => FOLDERID_Startup,
        Folder::CommonStartup => FOLDERID_CommonStartup,
        Folder::LocalAppData => FOLDERID_LocalAppData,
        Folder::Windows => FOLDERID_Windows,
    };
    // SAFETY: `id` is a valid known-folder GUID; on success the returned buffer is a
    // NUL-terminated string the caller must free with CoTaskMemFree, done below.
    unsafe {
        let raw = SHGetKnownFolderPath(&id, KNOWN_FOLDER_FLAG(0), None).ok()?;
        let path = raw.to_string().ok().map(PathBuf::from);
        CoTaskMemFree(Some(raw.0 as *const _));
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn resolves_local_app_data() {
        let path = known(Folder::LocalAppData).expect("LocalAppData resolves");
        assert!(path.is_absolute());
    }
}
