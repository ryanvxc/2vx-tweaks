//! Opens one entry from a fixed list of pages. The frontend names an entry, never an address, so
//! this can't be turned into a way to start something else. See `docs/security.md`.

use serde::Deserialize;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, w};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Link {
    NvidiaDrivers,
    AmdDrivers,
    IntelDrivers,
    BufferbloatTest,
    BufferbloatFixes,
    GraphicsSettings,
    NetworkSettings,
    TimerDocs,
}

impl Link {
    const fn target(self) -> &'static str {
        match self {
            Self::NvidiaDrivers => "https://www.nvidia.com/en-us/drivers/",
            Self::AmdDrivers => "https://www.amd.com/en/support/download/drivers.html",
            Self::IntelDrivers => "https://www.intel.com/content/www/us/en/support/detect.html",
            Self::BufferbloatTest => "https://www.waveform.com/tools/bufferbloat",
            Self::BufferbloatFixes => {
                "https://www.bufferbloat.net/projects/bloat/wiki/What_can_I_do_about_Bufferbloat/"
            }
            Self::GraphicsSettings => "ms-settings:display-advancedgraphics",
            Self::NetworkSettings => "ms-settings:network-advancedsettings",
            Self::TimerDocs => {
                "https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod"
            }
        }
    }
}

pub fn open(link: Link) -> AppResult<()> {
    let target = HSTRING::from(link.target());
    // SAFETY: both strings outlive the call, and no window handle is passed.
    let result = unsafe { ShellExecuteW(None, w!("open"), &target, None, None, SW_SHOWNORMAL) };
    // Documented: anything above 32 is success.
    if result.0 as isize > 32 {
        Ok(())
    } else {
        Err(AppError::Internal(format!(
            "couldn't open {}",
            link.target()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_link_is_https_or_windows_settings() {
        for link in [
            Link::NvidiaDrivers,
            Link::AmdDrivers,
            Link::IntelDrivers,
            Link::BufferbloatTest,
            Link::BufferbloatFixes,
            Link::GraphicsSettings,
            Link::NetworkSettings,
            Link::TimerDocs,
        ] {
            let target = link.target();
            assert!(
                target.starts_with("https://") || target.starts_with("ms-settings:"),
                "{target}"
            );
        }
    }
}
