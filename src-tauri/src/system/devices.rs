//! What the registry and the kernel report about this PC's devices, protections and timer. Used
//! by Hardware, Network and Advanced. Read-only: every registry write in the app goes through the
//! tweak engine, and nothing here is one of its targets.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use std::time::Instant;

use serde::Serialize;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    CM_DEVNODE_STATUS_FLAGS, CM_Get_DevNode_PropertyW, CM_Get_DevNode_Status,
    CM_LOCATE_DEVNODE_NORMAL, CM_Locate_DevNodeW, CM_PROB, CM_PROB_DISABLED, CR_SUCCESS,
    DN_HAS_PROBLEM,
};
use windows::Win32::Devices::Properties::{DEVPROP_TYPE_UINT32, DEVPROPTYPE};
use windows::Win32::Foundation::{DEVPROPKEY, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{
    FreeMibTable, GetIfTable2, MIB_IF_ROW2, MIB_IF_TABLE2,
};
use windows::Win32::NetworkManagement::Ndis::{
    MediaConnectStateConnected, MediaConnectStateUnknown,
};
use windows::Win32::System::Threading::Sleep;
use windows::core::{GUID, HSTRING};
use windows_registry::{Key, LOCAL_MACHINE};

const CLASS: &str = r"SYSTEM\CurrentControlSet\Control\Class";
const ENUM: &str = r"SYSTEM\CurrentControlSet\Enum";
const DISPLAY_CLASS: &str = "{4d36e968-e325-11ce-bfc1-08002be10318}";
const NET_CLASS: &str = "{4d36e972-e325-11ce-bfc1-08002be10318}";
const MEDIA_CLASS: &str = "{4d36e96c-e325-11ce-bfc1-08002be10318}";
const USB_CLASS: &str = "{36fc9e60-c465-11cf-8056-444553540000}";
const BLUETOOTH_CLASS: &str = "{e0cbf06c-cd8b-4647-bb8a-263b43f0f974}";
const MOUSE_CLASS: &str = "{4d36e96f-e325-11ce-bfc1-08002be10318}";
const KEYBOARD_CLASS: &str = "{4d36e96b-e325-11ce-bfc1-08002be10318}";
const DISK_CLASS: &str = "{4d36e967-e325-11ce-bfc1-08002be10318}";

/// `Characteristics` flag for a real adapter, as opposed to a VPN or a virtual switch.
const NCF_PHYSICAL: u32 = 0x4;
/// Standard interface types from the IANA ifType table, which `*IfType` uses.
const IF_TYPE_ETHERNET: u32 = 6;
const IF_TYPE_WIFI: u32 = 71;
const SLEEP_SAMPLES: usize = 16;
/// `DEVPKEY_PciDevice_InterruptSupport` from pciprop.h, which the `windows` crate doesn't carry.
const INTERRUPT_SUPPORT: DEVPROPKEY = DEVPROPKEY {
    fmtid: GUID::from_u128(0x3ab22e31_8264_4b4e_9af5_a8d2d8e33e62),
    pid: 14,
};
/// Bits of that property, from `DevProp_PciDevice_InterruptType_*`.
const INTERRUPT_LINE_BASED: u32 = 1;
const INTERRUPT_MSI: u32 = 2;
const INTERRUPT_MSI_X: u32 = 4;

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
    /// Who published the driver, as it says of itself.
    pub provider: Option<String>,
    /// Windows' fallback driver, so the GPU has no driver of its own. No judgement of any other
    /// driver: nothing here can say that one is out of date.
    pub is_basic_driver: bool,
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
    /// `None` when Windows has no interface for it or doesn't know its media state.
    pub connected: Option<bool>,
    /// The faster direction, while connected.
    pub link_speed_bps: Option<u64>,
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
    /// What the device can do, beside `enabled`, which is what it's configured for.
    pub interrupt_support: Option<InterruptSupport>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InterruptSupport {
    pub line_based: bool,
    pub msi: bool,
    pub msi_x: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeviceClass {
    Display,
    Net,
    Media,
    Usb,
    Bluetooth,
    Mouse,
    Keyboard,
    DiskDrive,
}

/// One device that's here now. Names only: instance paths can hold serial numbers, so they stay
/// in this module.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceEntry {
    pub name: String,
    pub class: DeviceClass,
    /// Windows reports a problem with it. A device the user turned off isn't one.
    pub problem: bool,
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

/// Each driver instance of a class as (`0000`, key).
fn class_instances(class: &str) -> Vec<(String, Key)> {
    let Ok(root) = LOCAL_MACHINE.open(format!(r"{CLASS}\{class}")) else {
        return Vec::new();
    };
    let Ok(names) = root.keys() else {
        return Vec::new();
    };
    names
        // `Properties` sits beside 0000, 0001… and standard users can't open it.
        .filter(|name| name.chars().all(|c| c.is_ascii_digit()))
        .filter_map(|name| {
            let key = root.open(&name).ok()?;
            Some((name, key))
        })
        .collect()
}

/// The device's node in the PnP tree, if it's here now. This is what "present" means everywhere in
/// the app: `Enum` and `Control\Class` both keep what they knew about hardware after it's removed,
/// and only the PnP manager knows what's connected. Without the phantom flag it finds live nodes.
fn devnode(instance: &str) -> Option<u32> {
    let mut node = 0;
    // SAFETY: the out pointer is valid and HSTRING is NUL-terminated.
    let status = unsafe {
        CM_Locate_DevNodeW(
            &mut node,
            &HSTRING::from(instance),
            CM_LOCATE_DEVNODE_NORMAL,
        )
    };
    (status == CR_SUCCESS).then_some(node)
}

/// Every device under one enumerator as (`<enumerator>\<device>\<instance>`, key), removed ones
/// included: see [`devnode`].
fn enumerator_devices(enumerator: &str) -> Vec<(String, Key)> {
    let Ok(root) = LOCAL_MACHINE.open(format!(r"{ENUM}\{enumerator}")) else {
        return Vec::new();
    };
    let Ok(ids) = root.keys() else {
        return Vec::new();
    };
    let mut devices = Vec::new();
    for id in ids {
        let Ok(device) = root.open(&id) else { continue };
        let Ok(instances) = device.keys() else {
            continue;
        };
        for instance in instances {
            if let Ok(key) = device.open(&instance) {
                devices.push((format!(r"{enumerator}\{id}\{instance}"), key));
            }
        }
    }
    devices
}

fn all_devices() -> Vec<(String, Key)> {
    let Ok(root) = LOCAL_MACHINE.open(ENUM) else {
        return Vec::new();
    };
    let Ok(enumerators) = root.keys() else {
        return Vec::new();
    };
    enumerators
        .flat_map(|name| enumerator_devices(&name))
        .collect()
}

/// Driver keys (`{class}\0000`, lowercase) of a class's devices that are here now. `None` when that
/// can't be told: every PC has a display device, so finding none means the walk failed.
fn present_drivers(class: &str) -> Option<HashSet<String>> {
    let drivers: HashSet<String> = all_devices()
        .into_iter()
        .filter(|(_, key)| {
            key.get_string("ClassGUID")
                .is_ok_and(|c| c.eq_ignore_ascii_case(class))
        })
        .filter(|(instance, _)| devnode(instance).is_some())
        .filter_map(|(_, key)| key.get_string("Driver").ok())
        .map(|driver| driver.to_ascii_lowercase())
        .collect();
    (!drivers.is_empty()).then_some(drivers)
}

/// A failed read allows the device, as with `has_battery`: it must not hide hardware.
fn is_present(present: Option<&HashSet<String>>, class: &str, instance: &str) -> bool {
    present.is_none_or(|set| set.contains(&format!(r"{class}\{instance}").to_ascii_lowercase()))
}

pub(crate) fn gpu_drivers() -> Vec<GpuDriver> {
    let present = present_drivers(DISPLAY_CLASS);
    class_instances(DISPLAY_CLASS)
        .iter()
        .filter(|(instance, _)| is_present(present.as_ref(), DISPLAY_CLASS, instance))
        .filter_map(|(_, key)| {
            let name = key.get_string("DriverDesc").ok()?;
            let provider = key.get_string("ProviderName").ok();
            let matching_id = key.get_string("MatchingDeviceId").unwrap_or_default();
            Some(GpuDriver {
                version: key
                    .get_string("DriverVersion")
                    .ok()
                    .map(|v| driver_label(provider.as_deref().unwrap_or_default(), &v)),
                date: key.get_string("DriverDate").ok().and_then(|d| iso_date(&d)),
                provider,
                is_basic_driver: is_basic_driver(&name, &matching_id),
                name,
            })
        })
        .collect()
}

/// The fallback binds by PCI class (`pci\cc_0300`) rather than by a vendor's hardware ID, which
/// holds in every display language. The name covers a build that reports another ID.
fn is_basic_driver(name: &str, matching_id: &str) -> bool {
    matching_id.to_ascii_lowercase().starts_with(r"pci\cc_")
        || name.to_ascii_lowercase().contains("basic display")
}

/// NVIDIA's own number is the last five digits of the Windows one: 32.0.16.1714 is 617.14.
fn driver_label(provider: &str, version: &str) -> String {
    if !provider.to_ascii_lowercase().contains("nvidia") {
        return version.to_owned();
    }
    let digits: String = version.split('.').skip(2).collect();
    // `get`, because this text is the driver's and a byte offset needn't be a character boundary.
    let last = digits
        .len()
        .checked_sub(5)
        .and_then(|start| digits.get(start..));
    match last {
        Some(last) if last.bytes().all(|b| b.is_ascii_digit()) => {
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
    physical_adapters()
        .into_iter()
        .map(|(_, adapter)| adapter)
        .collect()
}

thread_local! {
    /// The adapter list for one pass of reads on this thread. A pass only reads, and everything that
    /// decides a write looks the adapter up itself, so a link that moves mid-pass costs a stale
    /// status and no more.
    static HELD_ADAPTERS: RefCell<Option<Vec<(String, NetworkAdapter)>>> =
        const { RefCell::new(None) };
}

/// Runs a pass of reads against one reading of the adapters, which costs a walk of the class key
/// and a table of interfaces each time.
pub fn hold_adapters(pass: &mut dyn FnMut()) {
    with_adapters(physical_adapters(), pass);
}

fn with_adapters(adapters: Vec<(String, NetworkAdapter)>, pass: &mut dyn FnMut()) {
    HELD_ADAPTERS.with_borrow_mut(|held| *held = Some(adapters));
    pass();
    HELD_ADAPTERS.with_borrow_mut(|held| *held = None);
}

/// This kind's primary adapter as its full key below HKLM: the one that's connected, or else the
/// first listed. Read fresh, not cached like the GPU's: the elevated helper resolves it again in
/// its own process, and a link can change while the app runs. Only a pass holds on to it.
pub fn adapter_key(kind: AdapterKind) -> Option<String> {
    let instance = adapter_instance(kind)?;
    Some(format!(r"{CLASS}\{NET_CLASS}\{instance}"))
}

fn adapter_instance(kind: AdapterKind) -> Option<String> {
    HELD_ADAPTERS
        .with_borrow(|held| {
            let adapters = held.as_ref()?;
            Some(primary_adapter(adapters, kind).map(str::to_owned))
        })
        .unwrap_or_else(|| primary_adapter(&physical_adapters(), kind).map(str::to_owned))
}

/// What a network driver instance calls itself. Still there for a while after the device is gone.
pub fn adapter_name(instance: &str) -> Option<String> {
    driver_name(NET_CLASS, instance)
}

/// What a display driver instance calls itself.
pub fn gpu_name(instance: &str) -> Option<String> {
    driver_name(DISPLAY_CLASS, instance)
}

fn driver_name(class: &str, instance: &str) -> Option<String> {
    // The instance comes back from the journal, so it's checked before it becomes a key path.
    if !is_driver_instance(instance) {
        return None;
    }
    LOCAL_MACHINE
        .open(format!(r"{CLASS}\{class}\{instance}"))
        .ok()?
        .get_string("DriverDesc")
        .ok()
}

fn is_driver_instance(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

fn primary_adapter(adapters: &[(String, NetworkAdapter)], kind: AdapterKind) -> Option<&str> {
    let of_kind = || adapters.iter().filter(|(_, a)| a.kind == kind);
    of_kind()
        .find(|(_, a)| a.connected == Some(true))
        .or_else(|| of_kind().next())
        .map(|(instance, _)| instance.as_str())
}

/// Each real adapter that's here now, as (`0000`, adapter).
fn physical_adapters() -> Vec<(String, NetworkAdapter)> {
    let interfaces = interfaces();
    class_instances(NET_CLASS)
        .into_iter()
        .filter_map(|(instance, key)| {
            let if_type = key.get_u32("*IfType").ok().or_else(|| {
                key.get_string("*IfType")
                    .ok()
                    .and_then(|t| t.parse::<u32>().ok())
            })?;
            let name = key.get_string("DriverDesc").ok()?;
            let characteristics = key.get_u32("Characteristics").unwrap_or(0);
            let kind = adapter_kind(if_type, characteristics, &name)?;
            let interface = key
                .get_string("NetCfgInstanceId")
                .ok()
                .and_then(|id| interface_id(&id))
                .and_then(|id| interfaces.get(&id));
            // An older driver may not record its device; an interface still proves it's running.
            let present = match key.get_string("DeviceInstanceID") {
                Ok(device) => devnode(&device).is_some(),
                Err(_) => interface.is_some(),
            };
            present.then(|| {
                let adapter = NetworkAdapter {
                    kind,
                    driver_version: key.get_string("DriverVersion").ok(),
                    connected: interface.and_then(|i| i.connected),
                    link_speed_bps: interface.and_then(|i| i.link_speed_bps),
                    name,
                };
                (instance, adapter)
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Interface {
    connected: Option<bool>,
    link_speed_bps: Option<u64>,
}

/// `NetCfgInstanceId` is `{GUID}`.
fn interface_id(raw: &str) -> Option<u128> {
    let bare = raw.strip_prefix('{')?.strip_suffix('}')?;
    GUID::try_from(bare).ok().map(|id| id.to_u128())
}

/// Link state of every network interface, by interface GUID.
fn interfaces() -> HashMap<u128, Interface> {
    // Filter drivers appear as interfaces of their own, on top of the adapter.
    const FILTER_INTERFACE: u8 = 1 << 1;

    let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
    // SAFETY: the out pointer is valid, and a successful call's table is freed below.
    if unsafe { GetIfTable2(&mut table) } != NO_ERROR || table.is_null() {
        return HashMap::new();
    }
    // SAFETY: `Table` is declared with one row but holds `NumEntries` of them, all inside the
    // allocation, which stays alive until FreeMibTable.
    let rows = unsafe {
        let first = (&raw const (*table).Table).cast::<MIB_IF_ROW2>();
        std::slice::from_raw_parts(first, (*table).NumEntries as usize)
    };
    let found = rows
        .iter()
        .filter(|row| row.InterfaceAndOperStatusFlags._bitfield & FILTER_INTERFACE == 0)
        .map(|row| {
            let connected = (row.MediaConnectState != MediaConnectStateUnknown)
                .then_some(row.MediaConnectState == MediaConnectStateConnected);
            let interface = Interface {
                connected,
                link_speed_bps: connected
                    .filter(|connected| *connected)
                    .and_then(|_| link_speed(row.TransmitLinkSpeed, row.ReceiveLinkSpeed)),
            };
            (row.InterfaceGuid.to_u128(), interface)
        })
        .collect();
    // SAFETY: the table came from GetIfTable2 and `rows` isn't used past this point.
    unsafe { FreeMibTable(table.cast()) };
    found
}

/// Unknown speeds read as all ones, and some drivers report zero until they've negotiated.
fn link_speed(transmit: u64, receive: u64) -> Option<u64> {
    [transmit, receive]
        .into_iter()
        .filter(|speed| *speed != 0 && *speed != u64::MAX)
        .max()
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

/// PCI vendor IDs of the display adapters with a driver installed.
pub fn gpu_vendors() -> Vec<u16> {
    let mut vendors: Vec<u16> = display_drivers().iter().map(|(_, id)| *id).collect();
    vendors.dedup();
    vendors
}

/// This vendor's driver key under `Control\Class`, so a setting for one GPU never lands on another.
pub fn gpu_key(vendor: u16) -> Option<&'static str> {
    display_drivers()
        .iter()
        .find(|(_, id)| *id == vendor)
        .map(|(key, _)| key.as_str())
}

/// Read once: the catalogue resolves this for every GPU tweak, and adapters don't change while the
/// app runs.
fn display_drivers() -> &'static [(String, u16)] {
    static DRIVERS: OnceLock<Vec<(String, u16)>> = OnceLock::new();
    DRIVERS.get_or_init(read_display_drivers)
}

/// Each display adapter that's here now as (key, PCI vendor ID). A driver key outlives its GPU, so
/// without the presence check a removed card would keep its vendor's tweaks compatible.
fn read_display_drivers() -> Vec<(String, u16)> {
    let class = format!(r"{CLASS}\{DISPLAY_CLASS}");
    let present = present_drivers(DISPLAY_CLASS);
    let mut drivers: Vec<_> = class_instances(DISPLAY_CLASS)
        .iter()
        .filter(|(instance, _)| is_present(present.as_ref(), DISPLAY_CLASS, instance))
        .filter_map(|(instance, key)| {
            let vendor = pci_vendor(&key.get_string("MatchingDeviceId").ok()?)?;
            Some((format!(r"{class}\{instance}"), vendor))
        })
        .collect();
    drivers.sort();
    drivers
}

/// From `MatchingDeviceId` (`pci\ven_10de&...`).
fn pci_vendor(matching_id: &str) -> Option<u16> {
    let id = matching_id.to_ascii_lowercase();
    u16::from_str_radix(id.strip_prefix(r"pci\ven_")?.get(..4)?, 16).ok()
}

/// The CPUID vendor string, such as `GenuineIntel`.
pub fn cpu_vendor() -> Option<String> {
    LOCAL_MACHINE
        .open(r"HARDWARE\DESCRIPTION\System\CentralProcessor\0")
        .and_then(|key| key.get_string("VendorIdentifier"))
        .ok()
}

fn msi_devices() -> Vec<MsiDevice> {
    let mut devices: Vec<MsiDevice> = enumerator_devices("PCI")
        .into_iter()
        .filter_map(|(instance, key)| {
            let class = key.get_string("ClassGUID").unwrap_or_default();
            let service = key.get_string("Service").unwrap_or_default();
            let group = msi_group(&class, &service)?;
            let node = devnode(&instance)?;
            Some(MsiDevice {
                group,
                name: device_name(&key),
                enabled: msi_enabled(&key),
                interrupt_support: interrupt_support(node),
            })
        })
        .collect();
    devices.sort_by_key(|d| d.group as u8);
    devices
}

fn interrupt_support(node: u32) -> Option<InterruptSupport> {
    let mut kind = DEVPROPTYPE::default();
    let mut bits = 0u32;
    let mut size = size_of::<u32>() as u32;
    // SAFETY: every pointer is valid for the call and `size` is the buffer's real size.
    let status = unsafe {
        CM_Get_DevNode_PropertyW(
            node,
            &INTERRUPT_SUPPORT,
            &mut kind,
            Some((&raw mut bits).cast()),
            &mut size,
            0,
        )
    };
    (status == CR_SUCCESS && kind == DEVPROP_TYPE_UINT32).then(|| interrupt_flags(bits))
}

fn interrupt_flags(bits: u32) -> InterruptSupport {
    InterruptSupport {
        line_based: bits & INTERRUPT_LINE_BASED != 0,
        msi: bits & INTERRUPT_MSI != 0,
        msi_x: bits & INTERRUPT_MSI_X != 0,
    }
}

/// The devices shown on Hardware, across every enumerator: mice sit under HID, disks under SCSI.
pub(crate) fn device_list() -> Vec<DeviceEntry> {
    let mut devices: Vec<DeviceEntry> = all_devices()
        .into_iter()
        .filter_map(|(instance, key)| {
            let class = device_class(&key.get_string("ClassGUID").ok()?)?;
            let node = devnode(&instance)?;
            let name = device_name(&key);
            (!name.is_empty()).then(|| DeviceEntry {
                name,
                class,
                problem: device_problem(node),
            })
        })
        .collect();
    devices.sort_by(|a, b| (a.class as u8, &a.name).cmp(&(b.class as u8, &b.name)));
    devices
}

fn device_class(guid: &str) -> Option<DeviceClass> {
    match guid.to_ascii_lowercase().as_str() {
        DISPLAY_CLASS => Some(DeviceClass::Display),
        NET_CLASS => Some(DeviceClass::Net),
        MEDIA_CLASS => Some(DeviceClass::Media),
        USB_CLASS => Some(DeviceClass::Usb),
        BLUETOOTH_CLASS => Some(DeviceClass::Bluetooth),
        MOUSE_CLASS => Some(DeviceClass::Mouse),
        KEYBOARD_CLASS => Some(DeviceClass::Keyboard),
        DISK_CLASS => Some(DeviceClass::DiskDrive),
        _ => None,
    }
}

fn device_problem(node: u32) -> bool {
    let mut status = CM_DEVNODE_STATUS_FLAGS::default();
    let mut code = CM_PROB::default();
    // SAFETY: both out pointers are valid for the call.
    let read = unsafe { CM_Get_DevNode_Status(&mut status, &mut code, node, 0) };
    read == CR_SUCCESS && is_fault(status.0, code.0)
}

fn is_fault(status: u32, code: u32) -> bool {
    status & DN_HAS_PROBLEM.0 != 0 && code != CM_PROB_DISABLED.0
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
    fn a_version_that_isnt_ascii_keeps_its_text() {
        // Five bytes back lands inside the `é`.
        assert_eq!(driver_label("NVIDIA", "x.y.é1234"), "x.y.é1234");
        assert_eq!(driver_label("NVIDIA", "32.0.16.é171"), "32.0.16.é171");
    }

    #[test]
    fn a_driver_instance_is_digits_only() {
        assert!(is_driver_instance("0008"));
        assert!(!is_driver_instance(""));
        assert!(!is_driver_instance(r"0008\Ndi"));
        assert!(!is_driver_instance(".."));
        assert_eq!(adapter_name(r"..\.."), None);
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
    fn a_driver_key_without_its_device_is_not_present() {
        let present = HashSet::from([format!(r"{DISPLAY_CLASS}\0001")]);
        assert!(is_present(Some(&present), DISPLAY_CLASS, "0001"));
        assert!(!is_present(Some(&present), DISPLAY_CLASS, "0000"));
        // A walk that failed doesn't hide hardware.
        assert!(is_present(None, DISPLAY_CLASS, "0000"));
    }

    #[test]
    fn a_vendor_comes_from_its_hardware_id() {
        assert_eq!(pci_vendor(r"PCI\VEN_10DE&DEV_2D04"), Some(0x10de));
        assert_eq!(pci_vendor(r"pci\cc_0300"), None);
    }

    #[test]
    fn the_fallback_driver_is_flagged_in_any_language() {
        assert!(is_basic_driver("Microsoft Basic Display Adapter", ""));
        assert!(is_basic_driver(
            "Microsoft Standardanzeigeadapter",
            r"pci\cc_0300"
        ));
        assert!(!is_basic_driver(
            "NVIDIA GeForce RTX 5060 Ti",
            r"pci\ven_10de&dev_2d04"
        ));
    }

    fn adapter(kind: AdapterKind, connected: Option<bool>) -> NetworkAdapter {
        NetworkAdapter {
            name: "Test".into(),
            kind,
            driver_version: None,
            connected,
            link_speed_bps: None,
        }
    }

    #[test]
    fn a_pass_answers_from_its_one_reading() {
        let list = vec![(
            "0042".to_string(),
            adapter(AdapterKind::Ethernet, Some(true)),
        )];
        let mut seen = None;
        let mut wifi = Some("unset".to_string());
        with_adapters(list, &mut || {
            seen = adapter_key(AdapterKind::Ethernet);
            wifi = adapter_key(AdapterKind::Wifi);
        });
        assert!(seen.unwrap().ends_with(r"\0042"));
        assert_eq!(
            wifi, None,
            "a kind the list doesn't have isn't looked up again"
        );
        // Gone with the pass, whatever this PC has.
        assert!(HELD_ADAPTERS.with_borrow(Option::is_none));
    }

    #[test]
    fn the_primary_adapter_is_connected_or_else_first_listed() {
        let idle = adapter(AdapterKind::Ethernet, Some(false));
        let linked = adapter(AdapterKind::Ethernet, Some(true));
        let wifi = adapter(AdapterKind::Wifi, Some(true));
        let list = [
            ("0001".to_owned(), idle.clone()),
            ("0002".to_owned(), linked),
            ("0003".to_owned(), wifi),
        ];
        assert_eq!(primary_adapter(&list, AdapterKind::Ethernet), Some("0002"));
        assert_eq!(primary_adapter(&list, AdapterKind::Wifi), Some("0003"));
        assert_eq!(
            primary_adapter(&list[..1], AdapterKind::Ethernet),
            Some("0001")
        );
        assert_eq!(primary_adapter(&list[..1], AdapterKind::Wifi), None);
    }

    #[test]
    fn link_speed_skips_what_the_driver_does_not_know() {
        assert_eq!(link_speed(1_000, 2_500), Some(2_500));
        assert_eq!(link_speed(u64::MAX, 1_000), Some(1_000));
        assert_eq!(link_speed(0, u64::MAX), None);
    }

    #[test]
    fn interface_ids_are_read_from_their_braces() {
        assert_eq!(
            interface_id("{01234567-89ab-cdef-0123-456789abcdef}"),
            Some(0x01234567_89ab_cdef_0123_456789abcdef)
        );
        assert_eq!(interface_id("not a guid"), None);
    }

    #[test]
    fn interrupt_bits_are_named() {
        assert_eq!(
            interrupt_flags(INTERRUPT_LINE_BASED | INTERRUPT_MSI_X),
            InterruptSupport {
                line_based: true,
                msi: false,
                msi_x: true
            }
        );
    }

    #[test]
    fn a_device_turned_off_is_not_a_fault() {
        assert!(is_fault(DN_HAS_PROBLEM.0, 28));
        assert!(!is_fault(DN_HAS_PROBLEM.0, CM_PROB_DISABLED.0));
        assert!(!is_fault(0, 28));
    }

    #[test]
    fn device_classes_ignore_case() {
        assert_eq!(
            device_class("{E0CBF06C-CD8B-4647-BB8A-263B43F0F974}"),
            Some(DeviceClass::Bluetooth)
        );
        assert_eq!(device_class("{00000000-0000-0000-0000-000000000000}"), None);
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
