use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

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

#[cfg(test)]
mod tests {
    #[test]
    fn power_status_query_does_not_panic() {
        let _ = super::has_battery();
    }
}
