//! What the registry and the kernel report about this PC's devices, protections and timer. Used
//! by Hardware, Network and Advanced. Read-only: every registry write in the app goes through the
//! tweak engine, and nothing here is one of its targets.

use std::time::Instant;

use serde::Serialize;
use windows::Win32::System::Threading::Sleep;
use windows_registry::{Key, LOCAL_MACHINE};

const CLASS: &str = r"SYSTEM\CurrentControlSet\Control\Class";
const ENUM: &str = r"SYSTEM\CurrentControlSet\Enum";
const DISPLAY_CLASS: &str = "{4d36e968-e325-11ce-bfc1-08002be10318}";
const NET_CLASS: &str = "{4d36e972-e325-11ce-bfc1-08002be10318}";
const MEDIA_CLASS: &str = "{4d36e96c-e325-11ce-bfc1-08002be10318}";
const USB_CLASS: &str = "{36fc9e60-c465-11cf-8056-444553540000}";

/// `Characteristics` flag for a real adapter, as opposed to a VPN or a virtual switch.
const NCF_PHYSICAL: u32 = 0x4;
/// Standard interface types from the IANA ifType table, which `*IfType` uses.
const IF_TYPE_ETHERNET: u32 = 6;
const IF_TYPE_WIFI: u32 = 71;
const SLEEP_SAMPLES: usize = 16;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceReport {
    pub gpus: Vec<GpuDriver>,
    /// Drive models, which the volumes in the snapshot don't carry.
    pub drives: Vec<String>,
    pub adapters: Vec<NetworkAdapter>,
    pub msi: Vec<MsiDevice>,
    pub protections: Vec<Protection>,
    pub timer: Option<TimerResolution>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuDriver {
    pub name: String,
    pub version: Option<String>,
    /// `YYYY-MM-DD`.
    pub date: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AdapterKind {
    Ethernet,
    Wifi,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAdapter {
    pub name: String,
    pub kind: AdapterKind,
    pub driver_version: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MsiGroup {
    Gpu,
    Audio,
    Network,
    Nvme,
    Sata,
    Usb,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MsiDevice {
    pub group: MsiGroup,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProtectionId {
    MemoryIntegrity,
    SecureBoot,
    SmartScreen,
    Uac,
    Firewall,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProtectionState {
    On,
    Off,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Protection {
    pub id: ProtectionId,
    pub state: ProtectionState,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerResolution {
    pub current_ms: f64,
    /// The largest interval, usually 15.625 ms.
    pub coarsest_ms: f64,
    /// The smallest interval, usually 0.5 ms.
    pub finest_ms: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepSample {
    pub resolution_ms: f64,
    pub slept_ms: f64,
    pub delta_ms: f64,
}

pub fn collect_report() -> DeviceReport {
    DeviceReport {
        gpus: gpu_drivers(),
        drives: drive_models(),
        adapters: network_adapters(),
        msi: msi_devices(),
        protections: protections(),
        timer: timer_resolution(),
    }
}

/// How long `Sleep(1)` really takes for a program that hasn't asked for a finer timer. Changes
/// nothing: it only sleeps.
pub fn measure_sleep() -> Vec<SleepSample> {
    (0..SLEEP_SAMPLES)
        .map(|_| {
            let resolution_ms = timer_resolution().map_or(0.0, |t| t.current_ms);
            let start = Instant::now();
            // SAFETY: Sleep takes a plain integer and has no failure mode.
            unsafe { Sleep(1) };
            let slept_ms = start.elapsed().as_secs_f64() * 1000.0;
            SleepSample {
                resolution_ms,
                slept_ms,
                delta_ms: slept_ms - 1.0,
            }
        })
        .collect()
}

fn class_instances(class: &str) -> Vec<Key> {
    let Ok(root) = LOCAL_MACHINE.open(format!(r"{CLASS}\{class}")) else {
        return Vec::new();
    };
    let Ok(names) = root.keys() else {
        return Vec::new();
    };
    names
        // `Properties` sits beside 0000, 0001… and standard users can't open it.
        .filter(|name| name.chars().all(|c| c.is_ascii_digit()))
        .filter_map(|name| root.open(name).ok())
        .collect()
}

fn gpu_drivers() -> Vec<GpuDriver> {
    class_instances(DISPLAY_CLASS)
        .iter()
        .filter_map(|key| {
            let name = key.get_string("DriverDesc").ok()?;
            let provider = key.get_string("ProviderName").unwrap_or_default();
            Some(GpuDriver {
                version: key
                    .get_string("DriverVersion")
                    .ok()
                    .map(|v| driver_label(&provider, &v)),
                date: key.get_string("DriverDate").ok().and_then(|d| iso_date(&d)),
                name,
            })
        })
        .collect()
}

/// NVIDIA's own number is the last five digits of the Windows one: 32.0.16.1714 is 617.14.
fn driver_label(provider: &str, version: &str) -> String {
    if !provider.to_ascii_lowercase().contains("nvidia") {
        return version.to_owned();
    }
    let digits: String = version.split('.').skip(2).collect();
    match digits.len().checked_sub(5).map(|start| &digits[start..]) {
        Some(last) if last.chars().all(|c| c.is_ascii_digit()) => {
            format!("{}.{}", &last[..3], &last[3..])
        }
        _ => version.to_owned(),
    }
}

/// `DriverDate` is `M-D-YYYY`.
fn iso_date(raw: &str) -> Option<String> {
    let mut parts = raw.split('-');
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    let year: u32 = parts.next()?.parse().ok()?;
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

fn drive_models() -> Vec<String> {
    let Ok(list) = LOCAL_MACHINE.open(r"SYSTEM\CurrentControlSet\Services\disk\Enum") else {
        return Vec::new();
    };
    let count = list.get_u32("Count").unwrap_or(0);
    (0..count)
        .filter_map(|i| {
            let path = list.get_string(i.to_string()).ok()?;
            LOCAL_MACHINE
                .open(format!(r"{ENUM}\{path}"))
                .ok()?
                .get_string("FriendlyName")
                .ok()
        })
        .collect()
}

fn network_adapters() -> Vec<NetworkAdapter> {
    class_instances(NET_CLASS)
        .iter()
        .filter_map(|key| {
            let if_type = key.get_u32("*IfType").ok().or_else(|| {
                key.get_string("*IfType")
                    .ok()
                    .and_then(|t| t.parse::<u32>().ok())
            })?;
            let name = key.get_string("DriverDesc").ok()?;
            let characteristics = key.get_u32("Characteristics").unwrap_or(0);
            Some(NetworkAdapter {
                kind: adapter_kind(if_type, characteristics, &name)?,
                driver_version: key.get_string("DriverVersion").ok(),
                name,
            })
        })
        .collect()
}

fn adapter_kind(if_type: u32, characteristics: u32, name: &str) -> Option<AdapterKind> {
    // Bluetooth networking reports itself as Ethernet.
    if characteristics & NCF_PHYSICAL == 0 || name.to_ascii_lowercase().contains("bluetooth") {
        return None;
    }
    match if_type {
        IF_TYPE_ETHERNET => Some(AdapterKind::Ethernet),
        IF_TYPE_WIFI => Some(AdapterKind::Wifi),
        _ => None,
    }
}

fn msi_devices() -> Vec<MsiDevice> {
    let Ok(pci) = LOCAL_MACHINE.open(format!(r"{ENUM}\PCI")) else {
        return Vec::new();
    };
    let Ok(ids) = pci.keys() else {
        return Vec::new();
    };
    let mut devices = Vec::new();
    for id in ids {
        let Ok(device) = pci.open(&id) else { continue };
        let Ok(instances) = device.keys() else {
            continue;
        };
        for instance in instances {
            let Ok(key) = device.open(&instance) else {
                continue;
            };
            // Enum remembers removed hardware; only a device that's here has a Control key.
            if key.open("Control").is_err() {
                continue;
            }
            let class = key.get_string("ClassGUID").unwrap_or_default();
            let service = key.get_string("Service").unwrap_or_default();
            let Some(group) = msi_group(&class, &service) else {
                continue;
            };
            devices.push(MsiDevice {
                group,
                name: device_name(&key),
                enabled: msi_enabled(&key),
            });
        }
    }
    devices.sort_by_key(|d| d.group as u8);
    devices
}

fn msi_group(class: &str, service: &str) -> Option<MsiGroup> {
    let class = class.to_ascii_lowercase();
    match service.to_ascii_lowercase().as_str() {
        "stornvme" => return Some(MsiGroup::Nvme),
        "storahci" => return Some(MsiGroup::Sata),
        // HD Audio controllers sit in the System class; the sound devices on them have no IRQ.
        "hdaudbus" => return Some(MsiGroup::Audio),
        _ => {}
    }
    match class.as_str() {
        DISPLAY_CLASS => Some(MsiGroup::Gpu),
        MEDIA_CLASS => Some(MsiGroup::Audio),
        NET_CLASS => Some(MsiGroup::Network),
        USB_CLASS => Some(MsiGroup::Usb),
        _ => None,
    }
}

/// `FriendlyName` when there is one.
fn device_name(key: &Key) -> String {
    key.get_string("FriendlyName")
        .or_else(|_| key.get_string("DeviceDesc"))
        .map(|name| display_name(&name))
        .unwrap_or_default()
}

/// Registry names are often `@oem12.inf,%key%;Actual name`. Some add a template and its arguments:
/// `@oem12.inf,%key%;%1 USB %2 Controller;(Intel(R),3.20)`.
fn display_name(raw: &str) -> String {
    let mut parts = raw.rsplit(';');
    let last = parts.next().unwrap_or_default();
    let args = last.strip_prefix('(').and_then(|a| a.strip_suffix(')'));
    match (args, parts.next()) {
        (Some(args), Some(template)) if template.contains("%1") => args
            .split(',')
            .enumerate()
            .fold(template.to_owned(), |name, (i, arg)| {
                name.replace(&format!("%{}", i + 1), arg)
            }),
        _ => last.to_owned(),
    }
}

fn msi_enabled(key: &Key) -> bool {
    key.open(r"Device Parameters\Interrupt Management\MessageSignaledInterruptProperties")
        .and_then(|k| k.get_u32("MSISupported"))
        .is_ok_and(|v| v == 1)
}

fn protections() -> Vec<Protection> {
    let hklm_dword =
        |key: &str, value: &str| LOCAL_MACHINE.open(key).and_then(|k| k.get_u32(value)).ok();
    let integrity = hklm_dword(
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        "Enabled",
    );
    let secure_boot = hklm_dword(
        r"SYSTEM\CurrentControlSet\Control\SecureBoot\State",
        "UEFISecureBootEnabled",
    );
    let uac = hklm_dword(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
        "EnableLUA",
    );
    let firewall = |profile: &str| {
        hklm_dword(
            &format!(
                r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\{profile}"
            ),
            "EnableFirewall",
        )
    };
    let smart_screen = LOCAL_MACHINE
        .open(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer")
        .and_then(|k| k.get_string("SmartScreenEnabled"))
        .ok();
    let smart_screen_policy = hklm_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "EnableSmartScreen",
    );

    vec![
        Protection {
            id: ProtectionId::MemoryIntegrity,
            // Off is the usual state on older hardware, so a missing value means off here.
            state: flag(Some(integrity.unwrap_or(0))),
        },
        Protection {
            id: ProtectionId::SecureBoot,
            state: flag(secure_boot),
        },
        Protection {
            id: ProtectionId::SmartScreen,
            state: smart_screen_state(smart_screen.as_deref(), smart_screen_policy),
        },
        Protection {
            id: ProtectionId::Uac,
            state: flag(uac),
        },
        Protection {
            id: ProtectionId::Firewall,
            state: all_on(&[firewall("StandardProfile"), firewall("PublicProfile")]),
        },
    ]
}

fn flag(value: Option<u32>) -> ProtectionState {
    match value {
        Some(0) => ProtectionState::Off,
        Some(_) => ProtectionState::On,
        None => ProtectionState::Unknown,
    }
}

fn all_on(values: &[Option<u32>]) -> ProtectionState {
    let states: Vec<_> = values.iter().map(|v| flag(*v)).collect();
    if states.contains(&ProtectionState::Off) {
        ProtectionState::Off
    } else if states.contains(&ProtectionState::Unknown) {
        ProtectionState::Unknown
    } else {
        ProtectionState::On
    }
}

fn smart_screen_state(setting: Option<&str>, policy: Option<u32>) -> ProtectionState {
    if policy == Some(0) || setting.is_some_and(|s| s.eq_ignore_ascii_case("off")) {
        ProtectionState::Off
    } else if policy.is_some() || setting.is_some() {
        ProtectionState::On
    } else {
        ProtectionState::Unknown
    }
}

#[link(name = "ntdll")]
unsafe extern "system" {
    /// Not in the Windows SDK headers but exported by ntdll on every supported build. Read-only.
    fn NtQueryTimerResolution(coarsest: *mut u32, finest: *mut u32, current: *mut u32) -> i32;
}

fn timer_resolution() -> Option<TimerResolution> {
    let (mut coarsest, mut finest, mut current) = (0u32, 0u32, 0u32);
    // SAFETY: all three pointers are valid for writes for the length of the call.
    let status = unsafe { NtQueryTimerResolution(&mut coarsest, &mut finest, &mut current) };
    (status == 0).then(|| TimerResolution {
        current_ms: hundred_ns_to_ms(current),
        coarsest_ms: hundred_ns_to_ms(coarsest),
        finest_ms: hundred_ns_to_ms(finest),
    })
}

fn hundred_ns_to_ms(value: u32) -> f64 {
    f64::from(value) / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_names_read_as_device_names() {
        assert_eq!(display_name("Plain name"), "Plain name");
        assert_eq!(
            display_name("@usbxhci.inf,%desc%;USB xHCI Compliant Host Controller"),
            "USB xHCI Compliant Host Controller"
        );
        assert_eq!(
            display_name(
                "@oem1.inf,%desc%;%1 USB %2 Controller - %3 (Microsoft);(Intel(R),3.20,1.20)"
            ),
            "Intel(R) USB 3.20 Controller - 1.20 (Microsoft)"
        );
        // Brackets alone aren't arguments.
        assert_eq!(display_name("@oem1.inf,%desc%;(Generic)"), "(Generic)");
    }

    #[test]
    fn audio_controllers_are_grouped_by_their_service() {
        assert_eq!(msi_group("{any}", "HDAudBus"), Some(MsiGroup::Audio));
    }

    #[test]
    fn nvidia_versions_become_nvidias_own_number() {
        assert_eq!(driver_label("NVIDIA", "32.0.16.1714"), "617.14");
        assert_eq!(driver_label("NVIDIA", "31.0.15.3623"), "536.23");
    }

    #[test]
    fn other_vendors_keep_the_windows_version() {
        assert_eq!(
            driver_label("Advanced Micro Devices, Inc.", "31.0.21001.45002"),
            "31.0.21001.45002"
        );
        assert_eq!(driver_label("NVIDIA", "bad"), "bad");
    }

    #[test]
    fn driver_dates_become_iso() {
        assert_eq!(iso_date("10-2-2026").as_deref(), Some("2026-10-02"));
        assert_eq!(iso_date("not a date"), None);
    }

    #[test]
    fn only_physical_adapters_count() {
        assert_eq!(
            adapter_kind(71, NCF_PHYSICAL, "Intel Wi-Fi 6"),
            Some(AdapterKind::Wifi)
        );
        assert_eq!(
            adapter_kind(6, NCF_PHYSICAL | 0x80, "Realtek PCIe GbE"),
            Some(AdapterKind::Ethernet)
        );
        assert_eq!(adapter_kind(6, 0, "Hyper-V Virtual Ethernet"), None);
        assert_eq!(
            adapter_kind(6, NCF_PHYSICAL, "Bluetooth Device (PAN)"),
            None
        );
        assert_eq!(adapter_kind(23, NCF_PHYSICAL, "PPP"), None);
    }

    #[test]
    fn storage_groups_come_from_the_service() {
        assert_eq!(msi_group("", "stornvme"), Some(MsiGroup::Nvme));
        assert_eq!(msi_group("", "storahci"), Some(MsiGroup::Sata));
        assert_eq!(
            msi_group("{4D36E968-E325-11CE-BFC1-08002BE10318}", "nvlddmkm"),
            Some(MsiGroup::Gpu)
        );
        assert_eq!(msi_group("{unknown}", "other"), None);
    }

    #[test]
    fn smart_screen_is_off_if_either_place_says_so() {
        assert_eq!(
            smart_screen_state(Some("Warn"), Some(0)),
            ProtectionState::Off
        );
        assert_eq!(smart_screen_state(Some("Off"), None), ProtectionState::Off);
        assert_eq!(smart_screen_state(Some("Warn"), None), ProtectionState::On);
        assert_eq!(smart_screen_state(None, None), ProtectionState::Unknown);
    }

    #[test]
    fn one_profile_off_means_the_firewall_is_off() {
        assert_eq!(all_on(&[Some(1), Some(0)]), ProtectionState::Off);
        assert_eq!(all_on(&[Some(1), None]), ProtectionState::Unknown);
        assert_eq!(all_on(&[Some(1), Some(1)]), ProtectionState::On);
    }

    #[test]
    fn timer_units_are_milliseconds() {
        assert!((hundred_ns_to_ms(156_250) - 15.625).abs() < 1e-9);
    }

    #[test]
    fn reads_this_pc_without_failing() {
        let report = collect_report();
        assert_eq!(report.protections.len(), 5);
        assert!(report.timer.is_some_and(|t| t.finest_ms <= t.coarsest_ms));
    }
}
