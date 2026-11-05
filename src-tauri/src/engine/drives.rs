//! Runs Windows' own drive optimisation on fixed drives. Windows chooses the work from the media,
//! as it does on its weekly schedule: TRIM on an SSD, defragmenting on a hard drive.

use serde::{Deserialize, Serialize};
use windows::core::Result as WinResult;

use crate::engine::broker::BrokerSession;
use crate::error::{AppError, AppResult};
use crate::system::privilege;
use crate::system::wmi::{self, Wmi};

const NAMESPACE: &str = r"ROOT\Microsoft\Windows\Storage";
const CLASS: &str = "MSFT_Volume";
/// `MSFT_Volume.DriveType`: a fixed drive. USB sticks and network shares stay out of it.
const FIXED: i64 = 3;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DriveOutcome {
    /// `C:`, never a path below it.
    pub drive: String,
    pub optimized: bool,
    pub problem: Option<String>,
}

/// The drive letter of a volume this can run on.
fn eligible(letter: Option<i64>, file_system: Option<&str>, kind: Option<i64>) -> Option<char> {
    let letter = char::from_u32(u32::try_from(letter?).ok()?)?;
    let usable = letter.is_ascii_alphabetic()
        && kind == Some(FIXED)
        && file_system.is_some_and(|f| matches!(f, "NTFS" | "ReFS"));
    usable.then_some(letter)
}

/// `MSFT_Volume.Optimize`'s documented return values.
fn problem(code: i32) -> Option<String> {
    match code {
        0 => None,
        1 => Some("Windows doesn't support this on the drive".into()),
        3 => Some("Windows ran out of time".into()),
        code => Some(format!("Windows reported a failure (code {code})")),
    }
}

fn volumes(wmi: &Wmi) -> WinResult<Vec<(char, String)>> {
    Ok(wmi
        // Not a narrower list of columns: WMI leaves `__PATH` out of those.
        .rows(&format!("SELECT * FROM {CLASS}"))?
        .iter()
        .filter_map(|row| {
            let letter = eligible(
                wmi::number(row, "DriveLetter"),
                wmi::text(row, "FileSystem").as_deref(),
                wmi::number(row, "DriveType"),
            )?;
            Some((letter, wmi::text(row, "__PATH")?))
        })
        .collect())
}

/// Runs in this process, which must be elevated. The broker's side of [`optimize`].
pub fn optimize_here() -> AppResult<Vec<DriveOutcome>> {
    wmi::on_com_thread(|| {
        let wmi = Wmi::open(NAMESPACE)?;
        let outcomes = volumes(&wmi)?
            .into_iter()
            .map(|(letter, path)| {
                let problem = match wmi.call(CLASS, &path, "Optimize", &[]) {
                    Ok(code) => problem(code),
                    Err(e) => Some(e.message()),
                };
                DriveOutcome {
                    drive: format!("{letter}:"),
                    optimized: problem.is_none(),
                    problem,
                }
            })
            .collect();
        Ok(outcomes)
    })
    .map_err(|e| AppError::System(format!("couldn't reach Windows' drive tools: {e}")))
}

/// One UAC prompt, unless the app is already elevated. A hard drive can take hours.
pub fn optimize() -> AppResult<Vec<DriveOutcome>> {
    if privilege::is_elevated() {
        optimize_here()
    } else {
        BrokerSession::launch()?.optimize_drives()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_fixed_lettered_drives_are_touched() {
        let c = i64::from(b'C');
        assert_eq!(eligible(Some(c), Some("NTFS"), Some(3)), Some('C'));
        assert_eq!(eligible(Some(c), Some("ReFS"), Some(3)), Some('C'));
        assert_eq!(eligible(Some(c), Some("FAT32"), Some(3)), None);
        assert_eq!(eligible(Some(c), Some("NTFS"), Some(2)), None, "removable");
        assert_eq!(eligible(Some(c), None, Some(3)), None);
        // The recovery and EFI partitions have no letter.
        assert_eq!(eligible(Some(0), Some("NTFS"), Some(3)), None);
        assert_eq!(eligible(None, Some("NTFS"), Some(3)), None);
    }

    #[test]
    fn return_values_read_as_problems() {
        assert_eq!(problem(0), None);
        assert!(problem(1).is_some());
        assert!(problem(4).unwrap().contains('4'));
    }

    /// Reads only. Every eligible volume has a path to call, without any elevation.
    #[test]
    fn volumes_are_listed_with_a_path() {
        let found = wmi::on_com_thread(|| volumes(&Wmi::open(NAMESPACE)?)).unwrap();
        assert!(!found.is_empty(), "this PC has a fixed NTFS drive");
        assert!(
            found.iter().all(|(_, path)| path.contains(CLASS)),
            "{found:?}"
        );
    }
}
