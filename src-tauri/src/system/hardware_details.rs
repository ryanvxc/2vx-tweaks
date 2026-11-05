//! What the hardware says about itself: how the CPU is built, what's in the memory slots and what
//! each physical drive is. Read-only and needs no elevation. Every field is optional where Windows
//! may not report it, because nothing here is ever filled in with a guess. Serial numbers are never
//! read: the SMBIOS and storage parsers skip those fields by offset.

use std::collections::HashMap;
use std::mem::offset_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use serde::Serialize;
use windows::Win32::Foundation::{CloseHandle, ERROR_INSUFFICIENT_BUFFER, HANDLE};
use windows::Win32::Storage::FileSystem::{
    BusTypeAta, BusTypeAtapi, BusTypeFileBackedVirtual, BusTypeMmc, BusTypeNvme, BusTypeRAID,
    BusTypeSas, BusTypeSata, BusTypeScsi, BusTypeSd, BusTypeSpaces, BusTypeUfs, BusTypeUsb,
    BusTypeVirtual, CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_READ, FILE_SHARE_WRITE,
    OPEN_EXISTING, STORAGE_BUS_TYPE,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
    CoSetProxyBlanket, CoUninitialize, EOAC_NONE, RPC_C_AUTHN_LEVEL_CALL,
    RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::IO::DeviceIoControl;
use windows::Win32::System::Ioctl::{
    DEVICE_SEEK_PENALTY_DESCRIPTOR, DEVICE_TRIM_DESCRIPTOR, DISK_GEOMETRY_EX,
    IOCTL_DISK_GET_DRIVE_GEOMETRY_EX, IOCTL_STORAGE_QUERY_PROPERTY, PropertyStandardQuery,
    STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_ID, STORAGE_PROPERTY_QUERY, StorageDeviceProperty,
    StorageDeviceSeekPenaltyProperty, StorageDeviceTrimProperty,
};
use windows::Win32::System::SystemInformation::{
    CACHE_RELATIONSHIP, CacheData, CacheUnified, GetLogicalProcessorInformationEx,
    GetSystemFirmwareTable, RSMB, RelationAll, RelationCache, RelationProcessorCore,
    SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::System::Wmi::{
    IEnumWbemClassObject, IWbemClassObject, IWbemLocator, WBEM_FLAG_FORWARD_ONLY,
    WBEM_FLAG_RETURN_IMMEDIATELY, WbemLocator,
};
use windows::core::{BSTR, HSTRING, PCWSTR, Result as WinResult, w};
use windows_registry::LOCAL_MACHINE;

use crate::system::devices::{self, DeviceEntry, GpuDriver};
use crate::system::hardware::StorageKind;
use crate::system::processes;

/// More physical disks than a PC has. Numbers can have gaps, so all of them are tried.
const MAX_DRIVES: u32 = 32;
/// `\\.\PhysicalDriveN` opened with no access: enough to ask what it is, and nothing else.
const NO_ACCESS: u32 = 0;
/// The storage namespace answers slowly when a disk is spinning up.
const HEALTH_TIMEOUT_MS: i32 = 5000;
/// How long the page waits for disk health in all. `HEALTH_TIMEOUT_MS` only covers each row: WMI has
/// no limit on connecting or on starting a query, and CPU, memory and device data wait on this.
const HEALTH_LIMIT: Duration = Duration::from_secs(6);
/// A disk health query is running, so a stuck one stays one stuck thread.
static HEALTH_RUNNING: AtomicBool = AtomicBool::new(false);
/// From rpcdce.h, which the `windows` crate only carries behind another feature.
const RPC_C_AUTHN_WINNT: u32 = 10;
const RPC_C_AUTHZ_NONE: u32 = 0;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareDetails {
    pub cpu: CpuDetails,
    pub memory: MemoryDetails,
    pub drives: Vec<DriveDetails>,
    /// Whether Windows sends TRIM, which is system-wide rather than per drive.
    pub trim_enabled: Option<bool>,
    pub gpu_drivers: Vec<GpuDriver>,
    pub devices: Vec<DeviceEntry>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CpuDetails {
    pub performance_cores: Option<u32>,
    /// `0` on a CPU with one kind of core.
    pub efficient_cores: Option<u32>,
    pub logical_processors: Option<u32>,
    pub l2_cache_bytes: Option<u64>,
    pub l3_cache_bytes: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MemoryDetails {
    pub slots_used: Option<u32>,
    pub slots_total: Option<u32>,
    pub modules: Vec<MemoryModule>,
    /// Inferred from Windows' "Memory Compression" process.
    pub compression_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MemoryModule {
    /// The board's label, such as `DIMM_A2`.
    pub slot: Option<String>,
    pub capacity_bytes: Option<u64>,
    /// The speed it's running at, falling back to its rated speed.
    pub speed_mts: Option<u32>,
    pub rated_speed_mts: Option<u32>,
    pub memory_type: Option<String>,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveDetails {
    pub model: Option<String>,
    pub bus: Option<String>,
    pub media: StorageKind,
    pub size_bytes: Option<u64>,
    pub trim_supported: Option<bool>,
    /// As Windows reports it, when that works without elevation. Never a verdict of ours.
    pub health: Option<String>,
}

pub fn collect() -> HardwareDetails {
    HardwareDetails {
        cpu: cpu(),
        memory: memory(),
        drives: drives(),
        trim_enabled: trim_enabled(),
        gpu_drivers: devices::gpu_drivers(),
        devices: devices::device_list(),
    }
}

fn cpu() -> CpuDetails {
    let Some(buffer) = processor_records() else {
        return CpuDetails::default();
    };
    let (mut classes, mut logical, mut l2, mut l3) = (vec![], 0, 0, 0);
    let base = buffer.as_ptr().cast::<u8>();
    let length = size_of_val(buffer.as_slice());
    let mut offset = 0;
    while offset + size_of::<u64>() <= length {
        // SAFETY: `offset` is inside the buffer, on a record boundary Windows gave. Fields are read
        // through raw places because a record can be shorter than the Rust union, so no reference
        // to one is ever formed.
        unsafe {
            let record = base
                .add(offset)
                .cast::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>();
            let size = (&raw const (*record).Size).read_unaligned() as usize;
            if size == 0 {
                break;
            }
            let relationship = (&raw const (*record).Relationship).read_unaligned();
            if relationship == RelationProcessorCore {
                let core = &raw const (*record).Anonymous.Processor;
                classes.push((&raw const (*core).EfficiencyClass).read_unaligned());
                let mask = (&raw const (*core).GroupMask[0].Mask).read_unaligned();
                logical += mask.count_ones();
            } else if relationship == RelationCache {
                let cache: *const CACHE_RELATIONSHIP = &raw const (*record).Anonymous.Cache;
                let kind = (&raw const (*cache).Type).read_unaligned();
                let bytes = u64::from((&raw const (*cache).CacheSize).read_unaligned());
                match (&raw const (*cache).Level).read_unaligned() {
                    2 if kind == CacheUnified || kind == CacheData => l2 += bytes,
                    3 if kind == CacheUnified || kind == CacheData => l3 += bytes,
                    _ => {}
                }
            }
            offset += size;
        }
    }
    if classes.is_empty() {
        return CpuDetails::default();
    }
    let (performance, efficient) = split_cores(&classes);
    CpuDetails {
        performance_cores: Some(performance),
        efficient_cores: Some(efficient),
        logical_processors: Some(logical),
        l2_cache_bytes: (l2 > 0).then_some(l2),
        l3_cache_bytes: (l3 > 0).then_some(l3),
    }
}

/// `u64`s, so records are aligned as far as they're ever going to be.
fn processor_records() -> Option<Vec<u64>> {
    let mut size = 0;
    // SAFETY: the out pointer is valid. Fails by design, reporting the size it needs.
    let sized = unsafe { GetLogicalProcessorInformationEx(RelationAll, None, &mut size) };
    if sized.err().map(|e| e.code()) != Some(ERROR_INSUFFICIENT_BUFFER.to_hresult()) {
        return None;
    }
    let mut buffer = vec![0u64; (size as usize).div_ceil(size_of::<u64>())];
    // SAFETY: the buffer holds at least `size` bytes and the call writes no more.
    unsafe {
        GetLogicalProcessorInformationEx(RelationAll, Some(buffer.as_mut_ptr().cast()), &mut size)
    }
    .ok()?;
    Some(buffer)
}

/// Windows ranks cores by efficiency class, higher meaning more performance. The top class is the
/// performance cores and anything below it is efficient. One class means one kind of core.
fn split_cores(classes: &[u8]) -> (u32, u32) {
    let top = classes.iter().copied().max().unwrap_or(0);
    let performance = classes.iter().filter(|class| **class == top).count() as u32;
    (performance, classes.len() as u32 - performance)
}

fn memory() -> MemoryDetails {
    MemoryDetails {
        compression_active: processes::memory_compression_running(),
        ..smbios_table().map_or_else(MemoryDetails::default, |t| parse_memory(&t))
    }
}

/// The raw SMBIOS table, starting with Windows' own 8-byte header.
fn smbios_table() -> Option<Vec<u8>> {
    // SAFETY: no buffer, so the call only reports the size.
    let size = unsafe { GetSystemFirmwareTable(RSMB, 0, None) };
    if size == 0 {
        return None;
    }
    let mut table = vec![0; size as usize];
    // SAFETY: the buffer's length travels with the slice.
    let written = unsafe { GetSystemFirmwareTable(RSMB, 0, Some(&mut table)) };
    (written != 0 && written <= size).then_some(table)
}

/// One SMBIOS structure: the formatted fields, then its strings, numbered from 1.
struct Structure<'a> {
    kind: u8,
    fields: &'a [u8],
    strings: &'a [u8],
}

impl Structure<'_> {
    /// Fields are read by offset and checked against the structure's own length, because an older
    /// SMBIOS version ends a structure sooner.
    fn byte(&self, offset: usize) -> Option<u8> {
        self.fields.get(offset).copied()
    }

    fn word(&self, offset: usize) -> Option<u16> {
        let bytes = self.fields.get(offset..offset + 2)?;
        Some(u16::from_le_bytes(bytes.try_into().ok()?))
    }

    fn dword(&self, offset: usize) -> Option<u32> {
        let bytes = self.fields.get(offset..offset + 4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    }

    fn text(&self, offset: usize) -> Option<String> {
        let index = usize::from(self.byte(offset)?).checked_sub(1)?;
        let raw = self.strings.split(|b| *b == 0).nth(index)?;
        let text = String::from_utf8_lossy(raw).trim().to_owned();
        (!is_placeholder(&text)).then_some(text)
    }
}

/// What firmware writes when nobody filled the field in.
fn is_placeholder(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.is_empty()
        || [
            "unknown",
            "not specified",
            "none",
            "undefined",
            "default string",
        ]
        .contains(&text.as_str())
        || text.starts_with("to be filled")
}

fn structures(raw: &[u8]) -> Vec<Structure<'_>> {
    const END_OF_TABLE: u8 = 127;
    const WINDOWS_HEADER: usize = 8;
    let table = raw.get(WINDOWS_HEADER..).unwrap_or_default();
    let mut found = vec![];
    let mut offset = 0;
    while let Some(&[kind, length]) = table.get(offset..offset + 2).and_then(|h| h.first_chunk()) {
        let length = usize::from(length);
        let Some(fields) = table.get(offset..offset + length).filter(|_| length >= 4) else {
            break;
        };
        // The strings run up to a double NUL. A structure with none is just the double NUL.
        let rest = &table[offset + length..];
        let Some(end) = rest.windows(2).position(|pair| pair == [0, 0]) else {
            break;
        };
        found.push(Structure {
            kind,
            fields,
            strings: &rest[..end],
        });
        if kind == END_OF_TABLE {
            break;
        }
        offset += length + end + 2;
    }
    found
}

/// Reads Type 16 (memory arrays) and Type 17 (memory devices). Offsets are from the SMBIOS
/// specification. 0x18 and 0x19 of Type 17 are the serial number and asset tag, and no line here
/// reads them.
fn parse_memory(raw: &[u8]) -> MemoryDetails {
    const ARRAY: u8 = 16;
    const DEVICE: u8 = 17;
    const SYSTEM_MEMORY: u8 = 0x03;

    let structures = structures(raw);
    // Boards also list flash and video memory as arrays, whose devices aren't RAM sticks.
    let arrays: Vec<&Structure> = structures
        .iter()
        .filter(|s| s.kind == ARRAY && s.byte(0x05) == Some(SYSTEM_MEMORY))
        .collect();
    let handles: Vec<u16> = arrays.iter().filter_map(|s| s.word(0x02)).collect();
    let slots: Vec<&Structure> = structures
        .iter()
        .filter(|s| s.kind == DEVICE)
        .filter(|s| handles.is_empty() || s.word(0x04).is_some_and(|h| handles.contains(&h)))
        .collect();
    let declared: u32 = arrays
        .iter()
        .filter_map(|s| s.word(0x0D))
        .map(u32::from)
        .sum();
    let modules: Vec<MemoryModule> = slots
        .iter()
        .filter(|s| s.word(0x0C).is_some_and(|size| size != 0))
        .map(|s| memory_module(s))
        .collect();
    if slots.is_empty() {
        return MemoryDetails::default();
    }
    MemoryDetails {
        slots_used: Some(modules.len() as u32),
        // An array can declare 0 or fewer devices than it lists, so the larger number is the slots.
        slots_total: Some(declared.max(slots.len() as u32)),
        modules,
        compression_active: None,
    }
}

fn memory_module(device: &Structure) -> MemoryModule {
    let rated = speed(device, 0x15, 0x54);
    MemoryModule {
        slot: device.text(0x10),
        capacity_bytes: module_bytes(device),
        speed_mts: speed(device, 0x20, 0x58).or(rated),
        rated_speed_mts: rated,
        memory_type: device.byte(0x12).and_then(memory_type).map(str::to_owned),
        manufacturer: device.text(0x17),
        part_number: device.text(0x1A),
    }
}

fn module_bytes(device: &Structure) -> Option<u64> {
    const MIB: u64 = 1024 * 1024;
    const UNKNOWN: u16 = 0xFFFF;
    const USE_EXTENDED: u16 = 0x7FFF;
    const IN_KIB: u16 = 0x8000;
    match device.word(0x0C)? {
        UNKNOWN => None,
        USE_EXTENDED => Some(u64::from(device.dword(0x1C)? & 0x7FFF_FFFF) * MIB),
        size if size & IN_KIB != 0 => Some(u64::from(size & !IN_KIB) * 1024),
        size => Some(u64::from(size) * MIB),
    }
}

/// A word in MT/s, where 0 means unknown and 0xFFFF means the value is in the extended field.
fn speed(device: &Structure, field: usize, extended: usize) -> Option<u32> {
    match device.word(field)? {
        0 => None,
        0xFFFF => device.dword(extended).filter(|speed| *speed != 0),
        speed => Some(u32::from(speed)),
    }
}

fn memory_type(code: u8) -> Option<&'static str> {
    Some(match code {
        0x12 => "DDR",
        0x13 => "DDR2",
        0x18 => "DDR3",
        0x1A => "DDR4",
        0x1B => "LPDDR",
        0x1C => "LPDDR2",
        0x1D => "LPDDR3",
        0x1E => "LPDDR4",
        0x22 => "DDR5",
        0x23 => "LPDDR5",
        _ => return None,
    })
}

struct Disk(HANDLE);

impl Drop for Disk {
    fn drop(&mut self) {
        // SAFETY: an open handle this type owns, closed once.
        let _ = unsafe { CloseHandle(self.0) };
    }
}

fn drives() -> Vec<DriveDetails> {
    let mut health = disk_health();
    (0..MAX_DRIVES)
        .filter_map(|number| {
            let path = HSTRING::from(format!(r"\\.\PhysicalDrive{number}"));
            // SAFETY: `path` is NUL-terminated and the rest are plain flags. With no access asked
            // for, the handle can only make the queries below.
            let disk = unsafe {
                CreateFileW(
                    &path,
                    NO_ACCESS,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    None,
                    OPEN_EXISTING,
                    FILE_FLAGS_AND_ATTRIBUTES(0),
                    None,
                )
            }
            .ok()
            .map(Disk)?;
            let (model, bus) = query(&disk, StorageDeviceProperty, 1024)
                .map_or((None, None), |raw| device_descriptor(&raw));
            Some(DriveDetails {
                model,
                bus: bus.and_then(bus_name).map(str::to_owned),
                media: media(&disk),
                size_bytes: size_bytes(&disk),
                trim_supported: flag(
                    &disk,
                    StorageDeviceTrimProperty,
                    size_of::<DEVICE_TRIM_DESCRIPTOR>(),
                    offset_of!(DEVICE_TRIM_DESCRIPTOR, TrimEnabled),
                ),
                health: health.remove(&number),
            })
        })
        .collect()
}

fn query(disk: &Disk, property: STORAGE_PROPERTY_ID, capacity: usize) -> Option<Vec<u8>> {
    let request = STORAGE_PROPERTY_QUERY {
        PropertyId: property,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut answer = vec![0u8; capacity];
    let mut returned = 0;
    // SAFETY: both buffers are valid for the sizes passed, and the call is synchronous.
    unsafe {
        DeviceIoControl(
            disk.0,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&raw const request).cast()),
            size_of_val(&request) as u32,
            Some(answer.as_mut_ptr().cast()),
            capacity as u32,
            Some(&mut returned),
            None,
        )
    }
    .ok()?;
    answer.truncate(returned as usize);
    Some(answer)
}

/// One boolean from a small descriptor, read as a byte: the call fills it from a driver, and a Rust
/// `bool` holding anything but 0 or 1 is undefined behaviour.
fn flag(disk: &Disk, property: STORAGE_PROPERTY_ID, size: usize, offset: usize) -> Option<bool> {
    query(disk, property, size)?.get(offset).map(|b| *b != 0)
}

fn media(disk: &Disk) -> StorageKind {
    let seek_penalty = flag(
        disk,
        StorageDeviceSeekPenaltyProperty,
        size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>(),
        offset_of!(DEVICE_SEEK_PENALTY_DESCRIPTOR, IncursSeekPenalty),
    );
    match seek_penalty {
        Some(true) => StorageKind::Hdd,
        Some(false) => StorageKind::Ssd,
        None => StorageKind::Unknown,
    }
}

fn size_bytes(disk: &Disk) -> Option<u64> {
    let mut geometry = DISK_GEOMETRY_EX::default();
    let mut returned = 0;
    // SAFETY: `geometry` is valid for the size passed, and the call is synchronous.
    unsafe {
        DeviceIoControl(
            disk.0,
            IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
            None,
            0,
            Some((&raw mut geometry).cast()),
            size_of_val(&geometry) as u32,
            Some(&mut returned),
            None,
        )
    }
    .ok()?;
    u64::try_from(geometry.DiskSize)
        .ok()
        .filter(|size| *size > 0)
}

/// The model and bus from a `STORAGE_DEVICE_DESCRIPTOR`. Reads the vendor and product offsets and
/// nothing else: `SerialNumberOffset` sits between them and the bus type and is stepped over.
fn device_descriptor(raw: &[u8]) -> (Option<String>, Option<STORAGE_BUS_TYPE>) {
    let dword = |offset: usize| -> Option<u32> {
        let bytes = raw.get(offset..offset + 4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    };
    let text = |offset: usize| -> Option<String> {
        let start = dword(offset).filter(|start| *start != 0)? as usize;
        let bytes = raw.get(start..)?;
        let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
        let text = String::from_utf8_lossy(&bytes[..end]).trim().to_owned();
        (!text.is_empty()).then_some(text)
    };
    let vendor = text(offset_of!(STORAGE_DEVICE_DESCRIPTOR, VendorIdOffset));
    let product = text(offset_of!(STORAGE_DEVICE_DESCRIPTOR, ProductIdOffset));
    let model = match (vendor, product) {
        // SATA and NVMe drives often repeat the vendor in the product, or report it as ATA.
        (Some(vendor), Some(product)) if !product.starts_with(&vendor) && vendor != "ATA" => {
            Some(format!("{vendor} {product}"))
        }
        (vendor, product) => product.or(vendor),
    };
    let bus = dword(offset_of!(STORAGE_DEVICE_DESCRIPTOR, BusType))
        .map(|bus| STORAGE_BUS_TYPE(bus as i32));
    (model, bus)
}

// The `windows` crate spells these constants as Windows does.
#[allow(non_upper_case_globals)]
fn bus_name(bus: STORAGE_BUS_TYPE) -> Option<&'static str> {
    Some(match bus {
        BusTypeNvme => "NVMe",
        BusTypeSata => "SATA",
        BusTypeUsb => "USB",
        BusTypeSas => "SAS",
        BusTypeScsi => "SCSI",
        BusTypeAta | BusTypeAtapi => "ATA",
        BusTypeRAID => "RAID",
        BusTypeSd => "SD",
        BusTypeMmc => "MMC",
        BusTypeUfs => "UFS",
        BusTypeSpaces => "Storage Spaces",
        BusTypeVirtual | BusTypeFileBackedVirtual => "Virtual",
        _ => return None,
    })
}

fn trim_enabled() -> Option<bool> {
    LOCAL_MACHINE
        .open(r"SYSTEM\CurrentControlSet\Control\FileSystem")
        .and_then(|key| key.get_u32("DisableDeleteNotification"))
        .ok()
        .map(|disabled| disabled == 0)
}

/// Health by disk number. Empty when the storage namespace can't be asked, or doesn't answer in time,
/// which leaves each drive's health `None` rather than making one up.
fn disk_health() -> HashMap<u32, String> {
    within(HEALTH_LIMIT, &HEALTH_RUNNING, || {
        read_disk_health().unwrap_or_default()
    })
    .unwrap_or_default()
}

/// Runs `work` on its own thread and waits up to `limit` for it. Windows offers no way to stop a
/// stuck WMI call, so a late `work` runs out on its thread with nobody waiting, and until it does
/// `running` turns later calls away at once, rather than leaving a thread behind for each.
fn within<T: Send + 'static>(
    limit: Duration,
    running: &'static AtomicBool,
    work: impl FnOnce() -> T + Send + 'static,
) -> Option<T> {
    if running.swap(true, Ordering::AcqRel) {
        return None;
    }
    let (answer, answered) = mpsc::channel();
    let spawned = std::thread::Builder::new().spawn(move || {
        let result = work();
        running.store(false, Ordering::Release);
        // Nobody listening just means it took too long.
        let _ = answer.send(result);
    });
    if spawned.is_err() {
        running.store(false, Ordering::Release);
        return None;
    }
    answered.recv_timeout(limit).ok()
}

/// Runs on a thread of its own, so COM starts and ends with it.
fn read_disk_health() -> WinResult<HashMap<u32, String>> {
    // SAFETY: this thread is new, and paired with CoUninitialize below when it succeeds.
    let com = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    let health = query_disk_health();
    if com.is_ok() {
        // SAFETY: pairs with the call above, and every interface was dropped inside the query.
        unsafe { CoUninitialize() };
    }
    health
}

fn query_disk_health() -> WinResult<HashMap<u32, String>> {
    // SAFETY: plain COM calls on interfaces this function owns.
    unsafe {
        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
        let none = BSTR::new();
        let namespace = BSTR::from(r"ROOT\Microsoft\Windows\Storage");
        let storage = locator.ConnectServer(&namespace, &none, &none, &none, 0, &none, None)?;
        CoSetProxyBlanket(
            &storage,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            PCWSTR::null(),
            RPC_C_AUTHN_LEVEL_CALL,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOAC_NONE,
        )?;
        let rows = storage.ExecQuery(
            &BSTR::from("WQL"),
            &BSTR::from("SELECT DeviceId, HealthStatus FROM MSFT_PhysicalDisk"),
            WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
            None,
        )?;
        Ok(health_rows(&rows))
    }
}

fn health_rows(rows: &IEnumWbemClassObject) -> HashMap<u32, String> {
    let mut health = HashMap::new();
    loop {
        let mut row: [Option<IWbemClassObject>; 1] = [None];
        let mut returned = 0;
        // SAFETY: the out pointers are valid for the call. `Next` stops at the timeout.
        let _ = unsafe { rows.Next(HEALTH_TIMEOUT_MS, &mut row, &mut returned) };
        let [Some(row)] = row else {
            return health;
        };
        if let Some((disk, status)) = health_row(&row) {
            health.insert(disk, status.to_owned());
        }
    }
}

fn health_row(row: &IWbemClassObject) -> Option<(u32, &'static str)> {
    let mut id = VARIANT::default();
    let mut status = VARIANT::default();
    // SAFETY: `row` is a live object and the names are NUL-terminated constants.
    unsafe {
        row.Get(w!("DeviceId"), 0, &mut id, None, None).ok()?;
        row.Get(w!("HealthStatus"), 0, &mut status, None, None)
            .ok()?;
    }
    let disk = BSTR::try_from(&id).ok()?.to_string().parse().ok()?;
    Some((disk, health_label(i32::try_from(&status).ok()?)?))
}

/// `MSFT_PhysicalDisk.HealthStatus`. 5 is "Unknown", which says nothing.
fn health_label(status: i32) -> Option<&'static str> {
    match status {
        0 => Some("Healthy"),
        1 => Some("Warning"),
        2 => Some("Unhealthy"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_quick_answer_comes_through() {
        static RUNNING: AtomicBool = AtomicBool::new(false);
        assert_eq!(within(Duration::from_secs(5), &RUNNING, || 7), Some(7));
    }

    #[test]
    fn a_stuck_query_can_only_hold_things_up_once() {
        static RUNNING: AtomicBool = AtomicBool::new(false);
        let (release, released) = mpsc::channel::<()>();
        let short = Duration::from_millis(20);

        // Stuck until the test lets it go, and the caller still gets its turn back.
        let stuck = within(short, &RUNNING, move || {
            let _ = released.recv();
            1
        });
        assert_eq!(stuck, None);
        assert!(RUNNING.load(Ordering::Acquire));

        // A second call isn't left waiting, and starts nothing.
        let started = std::time::Instant::now();
        assert_eq!(within(Duration::from_secs(30), &RUNNING, || 2), None);
        assert!(started.elapsed() < Duration::from_secs(5));

        // Once it finally ends, the next call runs.
        release.send(()).unwrap();
        while RUNNING.load(Ordering::Acquire) {
            std::thread::yield_now();
        }
        assert_eq!(within(Duration::from_secs(5), &RUNNING, || 3), Some(3));
    }

    #[test]
    fn the_real_health_query_comes_back_within_its_limit() {
        let started = std::time::Instant::now();
        let _ = disk_health();
        assert!(started.elapsed() < HEALTH_LIMIT + Duration::from_secs(2));
    }

    #[test]
    fn the_top_efficiency_class_is_the_performance_cores() {
        assert_eq!(split_cores(&[1, 1, 0, 0, 0, 0]), (2, 4));
        assert_eq!(split_cores(&[0, 0, 0, 0]), (4, 0));
        // Three kinds of core, as on a low-power-island CPU.
        assert_eq!(split_cores(&[2, 2, 1, 1, 0]), (2, 3));
        assert_eq!(split_cores(&[]), (0, 0));
    }

    /// One structure: a type, a length, a handle, then `fields` (so its first byte is offset 4),
    /// then its strings.
    fn structure(kind: u8, handle: u16, fields: &[u8], strings: &[&str]) -> Vec<u8> {
        let mut out = vec![kind, (fields.len() + 4) as u8];
        out.extend(handle.to_le_bytes());
        out.extend(fields);
        for text in strings {
            out.extend(text.as_bytes());
            out.push(0);
        }
        if strings.is_empty() {
            out.push(0);
        }
        out.push(0);
        out
    }

    fn table(structures: &[Vec<u8>]) -> Vec<u8> {
        let mut raw = vec![0u8; 8];
        for s in structures {
            raw.extend(s);
        }
        raw.extend(structure(127, 0xFFFF, &[], &[]));
        raw
    }

    fn array(handle: u16, usage: u8, devices: u16) -> Vec<u8> {
        let mut fields = vec![0u8; 0x0F - 4];
        fields[0x05 - 4] = usage;
        fields[0x0D - 4..0x0F - 4].copy_from_slice(&devices.to_le_bytes());
        structure(16, handle, &fields, &[])
    }

    /// A Type 17 device whose serial number and asset tag point at strings that must never appear.
    fn device(array: u16, size: u16, kind: u8, speed: u16, configured: u16) -> Vec<u8> {
        let mut fields = vec![0u8; 0x22 - 4];
        fields[0..2].copy_from_slice(&array.to_le_bytes());
        fields[0x0C - 4..0x0E - 4].copy_from_slice(&size.to_le_bytes());
        fields[0x10 - 4] = 1;
        fields[0x12 - 4] = kind;
        fields[0x15 - 4..0x17 - 4].copy_from_slice(&speed.to_le_bytes());
        fields[0x17 - 4] = 2;
        fields[0x18 - 4] = 3;
        fields[0x19 - 4] = 4;
        fields[0x1A - 4] = 5;
        fields[0x20 - 4..0x22 - 4].copy_from_slice(&configured.to_le_bytes());
        structure(
            17,
            0x40,
            &fields,
            &["DIMM_A2", "G.Skill", "SERIAL123", "TAG456", "F5-6000 "],
        )
    }

    #[test]
    fn smbios_gives_slots_and_modules_but_never_serials() {
        let raw = table(&[
            array(0x30, 0x03, 4),
            array(0x31, 0x09, 1),
            device(0x30, 16 * 1024, 0x22, 6000, 5600),
            device(0x30, 0, 0x02, 0, 0),
            // Flash on the other array isn't a RAM stick.
            device(0x31, 16, 0x09, 0, 0),
        ]);
        let memory = parse_memory(&raw);
        assert_eq!(memory.slots_used, Some(1));
        assert_eq!(memory.slots_total, Some(4));
        assert_eq!(
            memory.modules,
            [MemoryModule {
                slot: Some("DIMM_A2".into()),
                capacity_bytes: Some(16 * 1024 * 1024 * 1024),
                speed_mts: Some(5600),
                rated_speed_mts: Some(6000),
                memory_type: Some("DDR5".into()),
                manufacturer: Some("G.Skill".into()),
                part_number: Some("F5-6000".into()),
            }]
        );
        let json = serde_json::to_string(&memory).unwrap();
        assert!(!json.contains("SERIAL123") && !json.contains("TAG456"));
    }

    #[test]
    fn module_sizes_follow_the_encoding() {
        let bytes = |size: u16, extended: u32| {
            let mut fields = vec![0u8; 0x20 - 4];
            fields[0x0C - 4..0x0E - 4].copy_from_slice(&size.to_le_bytes());
            fields[0x1C - 4..].copy_from_slice(&extended.to_le_bytes());
            let raw = table(&[structure(17, 1, &fields, &[])]);
            module_bytes(&structures(&raw)[0])
        };
        assert_eq!(bytes(2048, 0), Some(2048 * 1024 * 1024));
        assert_eq!(bytes(0x8000 | 512, 0), Some(512 * 1024));
        assert_eq!(bytes(0x7FFF, 64 * 1024), Some(64 * 1024 * 1024 * 1024));
        assert_eq!(bytes(0xFFFF, 0), None);
    }

    #[test]
    fn a_truncated_or_empty_table_is_no_memory_rather_than_a_panic() {
        assert_eq!(parse_memory(&[]), MemoryDetails::default());
        let mut raw = table(&[array(0x30, 0x03, 2), device(0x30, 1024, 0x1A, 3200, 3200)]);
        raw.truncate(raw.len() - 30);
        assert_eq!(parse_memory(&raw).slots_used, None);
    }

    #[test]
    fn firmware_placeholders_are_not_names() {
        for text in ["", "Unknown", "NOT SPECIFIED", "To Be Filled By O.E.M."] {
            assert!(is_placeholder(text), "{text}");
        }
        assert!(!is_placeholder("Corsair"));
    }

    /// A descriptor laid out as Windows returns it, serial number included.
    fn descriptor(vendor: &str, product: &str, bus: i32) -> Vec<u8> {
        let mut raw = vec![0u8; offset_of!(STORAGE_DEVICE_DESCRIPTOR, RawDeviceProperties)];
        let put = |raw: &mut Vec<u8>, field: usize, text: &str| {
            let start = raw.len() as u32;
            raw[field..field + 4].copy_from_slice(&start.to_le_bytes());
            raw.extend(text.as_bytes());
            raw.push(0);
        };
        put(
            &mut raw,
            offset_of!(STORAGE_DEVICE_DESCRIPTOR, VendorIdOffset),
            vendor,
        );
        put(
            &mut raw,
            offset_of!(STORAGE_DEVICE_DESCRIPTOR, ProductIdOffset),
            product,
        );
        put(
            &mut raw,
            offset_of!(STORAGE_DEVICE_DESCRIPTOR, SerialNumberOffset),
            "S4EWNX0R",
        );
        let field = offset_of!(STORAGE_DEVICE_DESCRIPTOR, BusType);
        raw[field..field + 4].copy_from_slice(&bus.to_le_bytes());
        raw
    }

    #[test]
    fn a_drive_descriptor_gives_model_and_bus_but_never_the_serial() {
        let (model, bus) = device_descriptor(&descriptor("", "Samsung SSD 990 PRO", 17));
        assert_eq!(model.as_deref(), Some("Samsung SSD 990 PRO"));
        assert_eq!(bus.and_then(bus_name), Some("NVMe"));

        let (model, _) = device_descriptor(&descriptor("WDC", "WD20EZRZ", 11));
        assert_eq!(model.as_deref(), Some("WDC WD20EZRZ"));
        // ATA is the transport, not a maker.
        let (model, _) = device_descriptor(&descriptor("ATA", "ST2000DM008", 11));
        assert_eq!(model.as_deref(), Some("ST2000DM008"));

        assert_eq!(device_descriptor(&[]), (None, None));
    }

    #[test]
    fn only_a_real_health_status_is_named() {
        assert_eq!(health_label(0), Some("Healthy"));
        assert_eq!(health_label(2), Some("Unhealthy"));
        assert_eq!(health_label(5), None);
    }

    #[test]
    fn reads_this_pc_without_failing() {
        let details = collect();
        assert!(details.cpu.logical_processors.is_some_and(|n| n > 0));
        assert!(details.cpu.performance_cores.is_some_and(|n| n > 0));
    }
}
