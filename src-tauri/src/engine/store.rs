//! Reading and writing settings: the only code that touches the registry.
//!
//! [`SettingStore`] is the seam between the engine and Windows. Production uses
//! [`WindowsRegistry`]; tests use [`MemoryStore`], which can also inject failures.

use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::engine::setting::{Hive, RegTarget, RegValue};
use crate::engine::system_store;
use crate::error::{AppError, AppResult};
use crate::system::devices::{self, AdapterKind};

const NO_GPU: &str = "no GPU driver found";
const NO_ADAPTER: &str = "no network adapter found";

pub trait SettingStore: Send + Sync {
    /// Current value, or `None` if the value (or its key) does not exist.
    fn read(&self, target: &RegTarget) -> AppResult<Option<RegValue>>;
    /// Sets the value, or deletes it when `value` is `None` (deleting a missing value is a no-op).
    fn write(&self, target: &RegTarget, value: Option<&RegValue>) -> AppResult<()>;
    /// The driver instance (`0008`, never a hardware identifier) a target resolves to right now,
    /// because the same target can land on other hardware from one call to the next. `None` for a
    /// target that's always one place, and an error when it needs a device and none is here:
    /// `read` can't tell that apart from an unset value, which is what undo has to.
    fn device_of(&self, _target: &RegTarget) -> AppResult<Option<String>> {
        Ok(None)
    }
    /// What a driver instance calls itself, for a person to read.
    fn device_name(&self, _target: &RegTarget, _device: &str) -> Option<String> {
        None
    }
    /// Runs many reads in a row. A store can keep what it resolves for the length of the pass, so
    /// nothing that decides a write may run inside one.
    fn pass(&self, reads: &mut dyn FnMut()) {
        reads();
    }
}

/// Something that applies one write and reports what the system holds afterwards.
/// Either in-process ([`LocalWriter`]) or through the elevated broker.
pub trait SettingWriter {
    fn write(
        &mut self,
        target: &RegTarget,
        value: Option<&RegValue>,
    ) -> AppResult<Option<RegValue>>;
}

pub struct LocalWriter<'a>(pub &'a dyn SettingStore);

impl SettingWriter for LocalWriter<'_> {
    fn write(
        &mut self,
        target: &RegTarget,
        value: Option<&RegValue>,
    ) -> AppResult<Option<RegValue>> {
        self.0.write(target, value)?;
        // Verification re-reads from the system rather than trusting the write call.
        self.0.read(target)
    }
}

/// The real registry, through `windows-registry`.
pub struct WindowsRegistry;

const ERROR_FILE_NOT_FOUND: u32 = 0x8007_0002;
const ERROR_ACCESS_DENIED: u32 = 0x8007_0005;

fn root(hive: Hive) -> &'static windows_registry::Key {
    match hive {
        Hive::CurrentUser => windows_registry::CURRENT_USER,
        _ => windows_registry::LOCAL_MACHINE,
    }
}

/// A key's first segment, and the rest if there is any.
fn split_device_key(key: &str) -> (&str, Option<&str>) {
    match key.split_once('\\') {
        Some((device, rest)) => (device, Some(rest)),
        None => (key, None),
    }
}

/// The same value under a real driver key.
fn under_driver(driver: &str, rest: Option<&str>, target: &RegTarget) -> RegTarget {
    let key = match rest {
        Some(rest) => format!(r"{driver}\{rest}"),
        None => driver.to_string(),
    };
    RegTarget {
        hive: Hive::LocalMachine,
        key: key.into(),
        value: target.value.clone(),
    }
}

/// That vendor's driver key, or `None` without one.
fn gpu_driver(target: &RegTarget) -> Option<&'static str> {
    let id = match split_device_key(&target.key).0 {
        "Nvidia" => 0x10de,
        "Amd" => 0x1002,
        "Intel" => 0x8086,
        _ => return None,
    };
    devices::gpu_key(id)
}

/// The registry target behind a `Gpu` target, or `None` without that vendor's driver key.
fn gpu_target(target: &RegTarget) -> Option<RegTarget> {
    let rest = split_device_key(&target.key).1;
    Some(under_driver(gpu_driver(target)?, rest, target))
}

/// The driver key of that kind's primary adapter, or `None` with no adapter of that kind.
fn adapter_driver(target: &RegTarget) -> Option<String> {
    let kind = match split_device_key(&target.key).0 {
        "Ethernet" => AdapterKind::Ethernet,
        "Wifi" => AdapterKind::Wifi,
        _ => return None,
    };
    devices::adapter_key(kind)
}

/// The registry target behind an `Adapter` target, or `None` with no adapter of that kind.
fn adapter_target(target: &RegTarget) -> Option<RegTarget> {
    let rest = split_device_key(&target.key).1;
    Some(under_driver(&adapter_driver(target)?, rest, target))
}

/// The `0008` at the end of a driver key.
fn instance_of(driver: &str) -> Option<String> {
    driver.rsplit('\\').next().map(str::to_owned)
}

fn hresult(err: &windows_registry::Result<()>) -> Option<u32> {
    err.as_ref().err().map(|e| e.code().0 as u32)
}

/// A read that's refused isn't a value that can't be backed up: the same read as an administrator
/// might work, and only the plain words say so.
fn read_err(target: &RegTarget, err: windows_result::Error) -> AppError {
    match err.code().0 as u32 {
        ERROR_ACCESS_DENIED => AppError::ElevationRequired(format!(
            "`{target}` can't be read without administrator rights"
        )),
        _ => AppError::System(format!("`{target}`: {}", err.message())),
    }
}

fn write_err(target: &RegTarget, err: windows_result::Error) -> AppError {
    match err.code().0 as u32 {
        ERROR_ACCESS_DENIED => {
            AppError::ElevationRequired(format!("access denied writing `{target}`"))
        }
        _ => AppError::System(format!("`{target}`: {}", err.message())),
    }
}

impl SettingStore for WindowsRegistry {
    fn read(&self, target: &RegTarget) -> AppResult<Option<RegValue>> {
        match target.hive {
            Hive::ScheduledTask => return system_store::read_task(target),
            Hive::PowerPlan => return system_store::read_power(target),
            Hive::Gpu => {
                return match gpu_target(target) {
                    Some(real) => self.read(&real),
                    None => Ok(None),
                };
            }
            Hive::Adapter => {
                return match adapter_target(target) {
                    Some(real) => self.read(&real),
                    None => Ok(None),
                };
            }
            Hive::CurrentUser | Hive::LocalMachine => {}
        }
        let key = match root(target.hive).open(target.key.as_ref()) {
            Ok(key) => key,
            Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(None),
            Err(e) => return Err(read_err(target, e)),
        };
        let ty = match key.get_type(target.value.as_ref()) {
            Ok(ty) => ty,
            Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(None),
            Err(e) => return Err(read_err(target, e)),
        };
        let name = target.value.as_ref();
        let value = match ty {
            windows_registry::Type::U32 => {
                RegValue::Dword(key.get_u32(name).map_err(|e| read_err(target, e))?)
            }
            windows_registry::Type::U64 => {
                RegValue::Qword(key.get_u64(name).map_err(|e| read_err(target, e))?)
            }
            windows_registry::Type::String => RegValue::String(
                key.get_string(name)
                    .map_err(|e| read_err(target, e))?
                    .into(),
            ),
            windows_registry::Type::ExpandString => {
                // The HSTRING getter returns the raw, unexpanded text, which is what we must restore.
                RegValue::ExpandString(
                    key.get_hstring(name)
                        .map_err(|e| read_err(target, e))?
                        .to_string_lossy()
                        .into(),
                )
            }
            other => {
                return Err(AppError::Unsupported(format!(
                    "`{target}` holds a {other:?} value, which cannot be backed up exactly"
                )));
            }
        };
        Ok(Some(value))
    }

    fn write(&self, target: &RegTarget, value: Option<&RegValue>) -> AppResult<()> {
        match target.hive {
            Hive::ScheduledTask => return system_store::write_task(target, value),
            Hive::PowerPlan => return system_store::write_power(target, value),
            Hive::Gpu => {
                let real =
                    gpu_target(target).ok_or_else(|| AppError::Unsupported(NO_GPU.into()))?;
                return self.write(&real, value);
            }
            Hive::Adapter => {
                let real = adapter_target(target)
                    .ok_or_else(|| AppError::Unsupported(NO_ADAPTER.into()))?;
                return self.write(&real, value);
            }
            Hive::CurrentUser | Hive::LocalMachine => {}
        }
        let name = target.value.as_ref();
        match value {
            Some(value) => {
                let key = root(target.hive)
                    .create(target.key.as_ref())
                    .map_err(|e| write_err(target, e))?;
                let result = match value {
                    RegValue::Dword(v) => key.set_u32(name, *v),
                    RegValue::Qword(v) => key.set_u64(name, *v),
                    RegValue::String(s) => key.set_string(name, s.as_ref()),
                    RegValue::ExpandString(s) => key.set_expand_string(name, s.as_ref()),
                };
                result.map_err(|e| write_err(target, e))
            }
            None => {
                let key = match root(target.hive)
                    .options()
                    .read()
                    .write()
                    .open(target.key.as_ref())
                {
                    Ok(key) => key,
                    Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(()),
                    Err(e) => return Err(write_err(target, e)),
                };
                let result = key.remove_value(name);
                if hresult(&result) == Some(ERROR_FILE_NOT_FOUND) {
                    return Ok(());
                }
                result.map_err(|e| write_err(target, e))
            }
        }
    }

    fn device_of(&self, target: &RegTarget) -> AppResult<Option<String>> {
        let driver = match target.hive {
            Hive::Gpu => gpu_driver(target)
                .map(str::to_owned)
                .ok_or_else(|| AppError::Unsupported(NO_GPU.into()))?,
            Hive::Adapter => {
                adapter_driver(target).ok_or_else(|| AppError::Unsupported(NO_ADAPTER.into()))?
            }
            _ => return Ok(None),
        };
        Ok(instance_of(&driver))
    }

    fn device_name(&self, target: &RegTarget, device: &str) -> Option<String> {
        match target.hive {
            Hive::Gpu => devices::gpu_name(device),
            Hive::Adapter => devices::adapter_name(device),
            _ => None,
        }
    }

    fn pass(&self, reads: &mut dyn FnMut()) {
        devices::hold_adapters(reads);
    }
}

/// In-memory store for tests and fault injection. Keys compare case-insensitively.
#[derive(Default)]
pub struct MemoryStore {
    values: Mutex<BTreeMap<String, RegValue>>,
    faults: Mutex<Faults>,
    device: Mutex<Device>,
}

/// Where a `Gpu` or `Adapter` target lands, for tests that move hardware around.
#[derive(Default)]
enum Device {
    /// Hardware isn't part of the test: these targets are plain keys.
    #[default]
    NotModelled,
    Present(String),
    Missing,
}

#[derive(Default)]
struct Faults {
    /// Writes to this target fail.
    fail_write: Option<String>,
    /// Writes to this target "succeed" but the value is not changed, like a policy override.
    ignore_write: Option<String>,
    /// Reads of this target fail, like a REG_BINARY value the engine can't restore exactly.
    fail_read: Option<String>,
}

fn identity(target: &RegTarget) -> String {
    format!("{:?}|{}|{}", target.hive, target.key, target.value).to_ascii_lowercase()
}

fn resolves_to_a_device(target: &RegTarget) -> bool {
    matches!(target.hive, Hive::Gpu | Hive::Adapter)
}

impl MemoryStore {
    pub fn with(values: impl IntoIterator<Item = (RegTarget, RegValue)>) -> Self {
        let store = MemoryStore::default();
        for (target, value) in values {
            store.set(&target, Some(value));
        }
        store
    }

    /// `Gpu` and `Adapter` targets land on this driver instance from now on, each with values of its
    /// own, as when a cable moves to another adapter.
    pub fn use_device(&self, device: &str) {
        *self.device.lock().unwrap() = Device::Present(device.to_owned());
    }

    /// No hardware for those targets, as when a USB adapter is unplugged. Reads as unset, like the
    /// real store.
    pub fn remove_device(&self) {
        *self.device.lock().unwrap() = Device::Missing;
    }

    /// Where a value lives: per device, for targets that have one.
    fn slot(&self, target: &RegTarget) -> String {
        match &*self.device.lock().unwrap() {
            Device::Present(device) if resolves_to_a_device(target) => {
                format!("{}|{device}", identity(target))
            }
            _ => identity(target),
        }
    }

    fn device_missing(&self, target: &RegTarget) -> bool {
        resolves_to_a_device(target) && matches!(*self.device.lock().unwrap(), Device::Missing)
    }

    pub fn fail_writes_to(&self, target: &RegTarget) {
        self.faults.lock().unwrap().fail_write = Some(identity(target));
    }

    pub fn ignore_writes_to(&self, target: &RegTarget) {
        self.faults.lock().unwrap().ignore_write = Some(identity(target));
    }

    pub fn fail_reads_of(&self, target: &RegTarget) {
        self.faults.lock().unwrap().fail_read = Some(identity(target));
    }

    pub fn clear_faults(&self) {
        *self.faults.lock().unwrap() = Faults::default();
    }

    pub fn get(&self, target: &RegTarget) -> Option<RegValue> {
        if self.device_missing(target) {
            return None;
        }
        self.values.lock().unwrap().get(&self.slot(target)).cloned()
    }

    pub fn set(&self, target: &RegTarget, value: Option<RegValue>) {
        let slot = self.slot(target);
        let mut values = self.values.lock().unwrap();
        match value {
            Some(v) => values.insert(slot, v),
            None => values.remove(&slot),
        };
    }
}

impl SettingStore for MemoryStore {
    fn read(&self, target: &RegTarget) -> AppResult<Option<RegValue>> {
        if self.faults.lock().unwrap().fail_read.as_deref() == Some(identity(target).as_str()) {
            return Err(AppError::Unsupported(format!(
                "`{target}` holds a Bytes value, which cannot be backed up exactly"
            )));
        }
        Ok(self.get(target))
    }

    fn write(&self, target: &RegTarget, value: Option<&RegValue>) -> AppResult<()> {
        if self.device_missing(target) {
            return Err(AppError::Unsupported(NO_ADAPTER.into()));
        }
        let id = identity(target);
        let faults = self.faults.lock().unwrap();
        if faults.fail_write.as_deref() == Some(id.as_str()) {
            return Err(AppError::System(format!(
                "injected failure writing `{target}`"
            )));
        }
        if faults.ignore_write.as_deref() == Some(id.as_str()) {
            return Ok(());
        }
        drop(faults);
        self.set(target, value.cloned());
        Ok(())
    }

    fn device_of(&self, target: &RegTarget) -> AppResult<Option<String>> {
        if !resolves_to_a_device(target) {
            return Ok(None);
        }
        match &*self.device.lock().unwrap() {
            Device::NotModelled => Ok(None),
            Device::Present(device) => Ok(Some(device.clone())),
            Device::Missing => Err(AppError::Unsupported(NO_ADAPTER.into())),
        }
    }

    fn device_name(&self, target: &RegTarget, device: &str) -> Option<String> {
        resolves_to_a_device(target).then(|| format!("Device {device}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trips every supported type through the real registry under a private HKCU test key.
    #[test]
    #[ignore = "writes to HKCU\\Software\\2VX-Test on the real registry"]
    fn windows_registry_round_trip() {
        let store = WindowsRegistry;
        let values = [
            ("Dword", RegValue::Dword(7)),
            ("Qword", RegValue::Qword(1 << 40)),
            ("Sz", RegValue::String("hello".into())),
            ("ExpandSz", RegValue::ExpandString("%TEMP%\\x".into())),
        ];
        for (name, value) in values {
            let target = RegTarget {
                hive: Hive::CurrentUser,
                key: r"Software\2VX-Test\Store".into(),
                value: name.into(),
            };
            store.write(&target, Some(&value)).unwrap();
            assert_eq!(store.read(&target).unwrap(), Some(value));
            store.write(&target, None).unwrap();
            assert_eq!(store.read(&target).unwrap(), None);
            // Deleting again is a no-op.
            store.write(&target, None).unwrap();
        }
        windows_registry::CURRENT_USER
            .remove_tree(r"Software\2VX-Test")
            .unwrap();
    }

    #[test]
    fn missing_key_reads_as_absent() {
        let target = RegTarget::new(
            Hive::CurrentUser,
            r"Software\2VX-Test-Does-Not-Exist\Nope",
            "Value",
        );
        assert_eq!(WindowsRegistry.read(&target).unwrap(), None);
    }

    #[test]
    fn a_driver_key_ends_in_its_instance() {
        assert_eq!(
            instance_of(r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972}\0008").as_deref(),
            Some("0008")
        );
    }

    #[test]
    fn only_device_targets_have_a_device() {
        let plain = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\Test", "V");
        let task = RegTarget::new(Hive::ScheduledTask, r"\Test", "Enabled");
        assert_eq!(WindowsRegistry.device_of(&plain).unwrap(), None);
        assert_eq!(WindowsRegistry.device_of(&task).unwrap(), None);
        assert_eq!(WindowsRegistry.device_name(&plain, "0000"), None);
        // A vendor or kind that isn't one is no device, whatever this PC has.
        let unknown = RegTarget::new(Hive::Adapter, "Bluetooth", "*EEE");
        assert!(WindowsRegistry.device_of(&unknown).is_err());
    }

    #[test]
    fn a_missing_device_is_an_error_where_a_read_only_says_unset() {
        let target = RegTarget::new(Hive::Adapter, "Ethernet", "*EEE");
        let store = MemoryStore::default();
        assert_eq!(store.device_of(&target).unwrap(), None, "not modelled");

        store.use_device("0008");
        store.write(&target, Some(&RegValue::Dword(1))).unwrap();
        assert_eq!(store.device_of(&target).unwrap().as_deref(), Some("0008"));

        store.use_device("0010");
        assert_eq!(store.read(&target).unwrap(), None, "values are per device");

        store.remove_device();
        assert_eq!(store.read(&target).unwrap(), None);
        assert!(store.device_of(&target).is_err());
        assert!(store.write(&target, None).is_err());

        store.use_device("0008");
        assert_eq!(store.read(&target).unwrap(), Some(RegValue::Dword(1)));
    }

    #[test]
    fn memory_store_is_case_insensitive_and_injects_faults() {
        let a = RegTarget::new(Hive::CurrentUser, r"Software\X", "V");
        let b = RegTarget::new(Hive::CurrentUser, r"SOFTWARE\x", "v");
        let store = MemoryStore::default();
        store.write(&a, Some(&RegValue::Dword(1))).unwrap();
        assert_eq!(store.read(&b).unwrap(), Some(RegValue::Dword(1)));

        store.fail_writes_to(&a);
        assert!(store.write(&b, Some(&RegValue::Dword(2))).is_err());
        store.clear_faults();
        store.ignore_writes_to(&a);
        let mut writer = LocalWriter(&store);
        assert_eq!(
            writer.write(&a, Some(&RegValue::Dword(3))).unwrap(),
            Some(RegValue::Dword(1))
        );
    }
}
