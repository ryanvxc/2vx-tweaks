//! What a program says about itself in its version resource. Self-reported and unverified:
//! good for naming a startup entry, never a statement that it can be trusted.

use std::path::Path;

use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
};
use windows::core::{HSTRING, PCWSTR};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileInfo {
    pub description: Option<String>,
    pub company: Option<String>,
}

/// US English, Unicode: the block most programs have when they list no translation.
const FALLBACK_BLOCK: &str = "040904b0";

pub fn read(path: &Path) -> Option<FileInfo> {
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
    fn a_missing_file_has_no_info() {
        assert_eq!(read(Path::new(r"Z:\no\such\file.exe")), None);
    }
}
