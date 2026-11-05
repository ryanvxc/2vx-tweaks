use windows::Win32::Foundation::{CloseHandle, HANDLE, LUID};
use windows::Win32::Security::{
    AdjustTokenPrivileges, LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, SE_PRIVILEGE_ENABLED,
    SE_SHUTDOWN_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows::Win32::System::Shutdown::{
    EWX_REBOOT, ExitWindowsEx, SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MAJOR_APPLICATION,
    SHTDN_REASON_MINOR_RECONFIG,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::core::PCWSTR;

use crate::error::{AppError, AppResult};

/// `BatteryFlag` value meaning "no system battery".
const NO_SYSTEM_BATTERY: u8 = 128;
/// `BatteryFlag` value meaning "unknown status".
const UNKNOWN_STATUS: u8 = 255;

/// Whether the PC has a system battery (laptop, tablet), or `None` if Windows cannot tell.
pub fn has_battery() -> Option<bool> {
    let mut status = SYSTEM_POWER_STATUS::default();
    // SAFETY: `status` is a valid, writable SYSTEM_POWER_STATUS.
    unsafe { GetSystemPowerStatus(&mut status) }.ok()?;
    match status.BatteryFlag {
        UNKNOWN_STATUS => None,
        flag => Some(flag & NO_SYSTEM_BATTERY == 0),
    }
}

/// Asks Windows to restart. Open apps get the usual chance to save, so it isn't forced, and the
/// shutdown privilege every interactive user holds only has to be switched on for this process.
pub fn restart() -> AppResult<()> {
    let failed = |e: windows::core::Error| AppError::System(format!("couldn't restart: {e}"));
    let mut token = HANDLE::default();
    // SAFETY: the out pointers are valid for each call, and `token` is closed before returning.
    unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )
        .map_err(failed)?;
        let mut shutdown = LUID::default();
        let enabled = LookupPrivilegeValueW(PCWSTR::null(), SE_SHUTDOWN_NAME, &mut shutdown)
            .and_then(|()| {
                let privileges = TOKEN_PRIVILEGES {
                    PrivilegeCount: 1,
                    Privileges: [LUID_AND_ATTRIBUTES {
                        Luid: shutdown,
                        Attributes: SE_PRIVILEGE_ENABLED,
                    }],
                };
                AdjustTokenPrivileges(token, false, Some(&privileges), 0, None, None)
            });
        let _ = CloseHandle(token);
        enabled.map_err(failed)?;
        ExitWindowsEx(
            EWX_REBOOT,
            SHTDN_REASON_MAJOR_APPLICATION
                | SHTDN_REASON_MINOR_RECONFIG
                | SHTDN_REASON_FLAG_PLANNED,
        )
        .map_err(failed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn power_status_query_does_not_panic() {
        let _ = super::has_battery();
    }
}
