//! Windows restore points. 2VX lists them and makes one when asked. Restoring and deleting stay in
//! Windows' own tools (see `links::WindowsTool`): undoing a whole PC deserves Microsoft's wizard,
//! with its warnings and its own list of what will change.

use serde::{Deserialize, Serialize};
use windows::Win32::System::Variant::VARIANT;
use windows::core::{BSTR, Result as WinResult};
use windows_registry::LOCAL_MACHINE;

use crate::engine::broker::BrokerSession;
use crate::error::{AppError, AppResult};
use crate::system::privilege;
use crate::system::wmi::{self, Wmi};

const NAMESPACE: &str = r"ROOT\DEFAULT";
const CLASS: &str = "SystemRestore";
const SETTINGS: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore";
const POLICY: &str = r"SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore";
const DESCRIPTION: &str = "2VX Tweaks restore point";
/// `MODIFY_SETTINGS` and `BEGIN_SYSTEM_CHANGE`, the kind of call Microsoft's own sample makes.
const POINT_TYPE: i32 = 12;
const EVENT_TYPE: i32 = 100;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProtectionStatus {
    On,
    Off,
    /// Turned off by a policy, which only an administrator of this PC can change.
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePoint {
    pub sequence: u32,
    pub description: String,
    /// Unix milliseconds, when Windows gave a readable date.
    pub created_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreOverview {
    pub protection: ProtectionStatus,
    /// Newest first. `None` when Windows wouldn't list them, which isn't the same as having none.
    pub points: Option<Vec<RestorePoint>>,
}

/// Windows quietly skips a new point when one was made recently, and still reports success. So
/// this comes from comparing the list before and after, not from the method's return value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedPoint {
    pub created: bool,
}

fn protection_status() -> ProtectionStatus {
    let policy = LOCAL_MACHINE.open(POLICY).ok();
    let set = |name: &str| policy.as_ref().and_then(|k| k.get_u32(name).ok()) == Some(1);
    if set("DisableSR") || set("DisableConfig") {
        return ProtectionStatus::Blocked;
    }
    match LOCAL_MACHINE
        .open(SETTINGS)
        .and_then(|k| k.get_u32("RPSessionInterval"))
    {
        Ok(0) => ProtectionStatus::Off,
        Ok(_) => ProtectionStatus::On,
        Err(_) => ProtectionStatus::Unknown,
    }
}

fn read_points() -> WinResult<Vec<RestorePoint>> {
    let rows = Wmi::open(NAMESPACE)?.rows(&format!(
        "SELECT SequenceNumber, Description, CreationTime FROM {CLASS}"
    ))?;
    let mut points: Vec<RestorePoint> = rows
        .iter()
        .filter_map(|row| {
            Some(RestorePoint {
                sequence: u32::try_from(wmi::number(row, "SequenceNumber")?).ok()?,
                description: wmi::text(row, "Description").unwrap_or_default(),
                created_ms: wmi::text(row, "CreationTime").and_then(|t| dmtf_to_ms(&t)),
            })
        })
        .collect();
    points.sort_by_key(|p| std::cmp::Reverse(p.sequence));
    Ok(points)
}

/// Read-only, and needs no elevation.
pub fn overview() -> RestoreOverview {
    RestoreOverview {
        protection: protection_status(),
        points: wmi::on_com_thread(read_points).ok(),
    }
}

fn newest() -> WinResult<Option<u32>> {
    Ok(read_points()?.first().map(|p| p.sequence))
}

/// Makes a point in this process, which must be elevated. The broker's side of [`create`].
pub fn create_here() -> AppResult<CreatedPoint> {
    let (code, before, after) = wmi::on_com_thread(|| {
        let before = newest()?;
        let code = Wmi::open(NAMESPACE)?.call(
            CLASS,
            CLASS,
            "CreateRestorePoint",
            &[
                ("Description", VARIANT::from(BSTR::from(DESCRIPTION))),
                ("RestorePointType", VARIANT::from(POINT_TYPE)),
                ("EventType", VARIANT::from(EVENT_TYPE)),
            ],
        )?;
        Ok((code, before, newest()?))
    })
    .map_err(|e| AppError::System(format!("Windows couldn't make a restore point: {e}")))?;
    match code {
        0 => Ok(CreatedPoint {
            created: after > before,
        }),
        code => Err(AppError::System(format!(
            "Windows couldn't make a restore point (code {code:#x})"
        ))),
    }
}

/// Turns System Protection on for the system drive. The broker's side of [`enable`].
pub fn enable_here() -> AppResult<()> {
    let drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".into());
    let code = wmi::on_com_thread(|| {
        Wmi::open(NAMESPACE)?.call(
            CLASS,
            CLASS,
            "Enable",
            &[("Drive", VARIANT::from(BSTR::from(format!(r"{drive}\"))))],
        )
    })
    .map_err(|e| AppError::System(format!("Windows couldn't turn on System Protection: {e}")))?;
    match code {
        0 => Ok(()),
        code => Err(AppError::System(format!(
            "Windows couldn't turn on System Protection (code {code:#x})"
        ))),
    }
}

/// One UAC prompt, unless the app is already elevated.
pub fn create() -> AppResult<CreatedPoint> {
    if privilege::is_elevated() {
        create_here()
    } else {
        BrokerSession::launch()?.create_restore_point()
    }
}

/// One UAC prompt, unless the app is already elevated.
pub fn enable() -> AppResult<()> {
    if privilege::is_elevated() {
        enable_here()
    } else {
        BrokerSession::launch()?.enable_system_restore()
    }
}

/// `yyyymmddHHMMSS.ffffff+UUU`, where the last part is the offset from UTC in minutes.
fn dmtf_to_ms(text: &str) -> Option<i64> {
    let field = |from: usize, to: usize| text.get(from..to)?.parse::<i64>().ok();
    let (year, month, day) = (field(0, 4)?, field(4, 6)?, field(6, 8)?);
    let (hour, minute, second) = (field(8, 10)?, field(10, 12)?, field(12, 14)?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let offset = text
        .get(21..25)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let seconds = days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second;
    Some((seconds - offset * 60) * 1_000)
}

/// Days since 1970-01-01 in the proleptic Gregorian calendar (Howard Hinnant's algorithm).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmtf_dates_become_unix_milliseconds() {
        assert_eq!(dmtf_to_ms("19700101000000.000000-000"), Some(0));
        assert_eq!(
            dmtf_to_ms("20240229123000.000000-000"),
            Some(1_709_209_800_000)
        );
        // An hour ahead of UTC is an hour earlier in UTC.
        assert_eq!(dmtf_to_ms("19700101010000.000000+060"), Some(0));
        assert_eq!(dmtf_to_ms("not a date"), None);
        assert_eq!(dmtf_to_ms("20241301000000.000000-000"), None);
    }

    /// Reads only. It shows Windows' list comes back through the real provider.
    #[test]
    fn the_overview_reads_without_elevation() {
        let overview = overview();
        println!("{overview:?}");
        if let Some(points) = overview.points {
            assert!(points.windows(2).all(|p| p[0].sequence >= p[1].sequence));
            for point in points {
                assert!(point.created_ms.is_some(), "{point:?}");
            }
        }
    }
}
