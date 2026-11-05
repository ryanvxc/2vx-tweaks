//! What Windows can say about the firmware, for the BIOS page. All of it reads: nothing in 2VX
//! writes firmware settings, because that takes a kernel driver (see `docs/security.md`).

use std::mem::size_of;

use serde::Serialize;
use windows::Win32::System::SystemInformation::{
    FIRMWARE_TYPE, FirmwareTypeBios, FirmwareTypeUefi, GetFirmwareType,
};
use windows::Win32::System::Threading::{IsProcessorFeaturePresent, PF_VIRT_FIRMWARE_ENABLED};
use windows::Win32::System::TpmBaseServices::{TPM_DEVICE_INFO, Tbsi_GetDeviceInfo};
use windows_registry::LOCAL_MACHINE;

use crate::system::devices;

const BIOS_KEY: &str = r"HARDWARE\DESCRIPTION\System\BIOS";
/// A display adapter's largest window at or above this means Resizable BAR is working: without it
/// the window is the classic 256 MB.
const LARGE_BAR_BYTES: u64 = 1 << 30;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BootMode {
    Uefi,
    Legacy,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareReport {
    pub boot_mode: BootMode,
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub release_date: Option<String>,
    pub board: Option<String>,
    /// CPU virtualisation switched on in the firmware.
    pub virtualization: Option<bool>,
    /// 2 for TPM 2.0, 1 for 1.2. `None` when no TPM answers.
    pub tpm_version: Option<u32>,
    /// `None` when no adapter's memory windows could be read.
    pub resizable_bar: Option<bool>,
}

fn boot_mode() -> BootMode {
    let mut kind = FIRMWARE_TYPE::default();
    // SAFETY: `kind` is valid for writes for the length of the call.
    match unsafe { GetFirmwareType(&mut kind) } {
        Ok(()) if kind == FirmwareTypeUefi => BootMode::Uefi,
        Ok(()) if kind == FirmwareTypeBios => BootMode::Legacy,
        _ => BootMode::Unknown,
    }
}

fn tpm_version() -> Option<u32> {
    let mut info = TPM_DEVICE_INFO::default();
    let size = u32::try_from(size_of::<TPM_DEVICE_INFO>()).ok()?;
    // SAFETY: `info` is valid for writes of `size` bytes for the length of the call.
    let status = unsafe { Tbsi_GetDeviceInfo(size, (&raw mut info).cast()) };
    (status == 0).then_some(info.tpmVersion).filter(|&v| v > 0)
}

/// CPUID leaf 1, ECX bit 31: this code is running inside a hypervisor, as it does with Hyper-V or
/// Memory Integrity on.
fn under_hypervisor() -> bool {
    let leaf = core::arch::x86_64::__cpuid(1);
    leaf.ecx & (1 << 31) != 0
}

/// A running hypervisor hides the firmware flag, and it can only run if the firmware allows it.
fn virtualization_enabled(hypervisor: bool, firmware_flag: bool) -> bool {
    hypervisor || firmware_flag
}

pub fn collect() -> FirmwareReport {
    let key = LOCAL_MACHINE.open(BIOS_KEY).ok();
    let text = |name: &str| {
        key.as_ref()
            .and_then(|k| k.get_string(name).ok())
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
    };
    let board = [text("BaseBoardManufacturer"), text("BaseBoardProduct")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    // SAFETY: a plain query with no pointers.
    let firmware_flag = unsafe { IsProcessorFeaturePresent(PF_VIRT_FIRMWARE_ENABLED) }.as_bool();
    FirmwareReport {
        boot_mode: boot_mode(),
        vendor: text("BIOSVendor"),
        version: text("BIOSVersion"),
        release_date: text("BIOSReleaseDate"),
        board: Some(board).filter(|b| !b.is_empty()),
        virtualization: Some(virtualization_enabled(under_hypervisor(), firmware_flag)),
        tpm_version: tpm_version(),
        resizable_bar: devices::largest_display_window().map(|bytes| bytes >= LARGE_BAR_BYTES),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hypervisor_means_virtualisation_is_on() {
        assert!(virtualization_enabled(true, false));
        assert!(virtualization_enabled(false, true));
        assert!(!virtualization_enabled(false, false));
    }

    /// Reads only. Every Windows PC can answer this one.
    #[test]
    fn the_report_reads_on_this_pc() {
        let report = collect();
        println!("{report:?}");
        assert_ne!(report.boot_mode, BootMode::Unknown);
    }
}
