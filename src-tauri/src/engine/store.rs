//! Reading and writing settings: the only code that touches the registry.
//!
//! [`SettingStore`] is the seam between the engine and Windows. Production uses
//! [`WindowsRegistry`]; tests use [`MemoryStore`], which can also inject failures.

use std::collections::BTreeMap;
use std::sync::Mutex;

use crate::engine::setting::{Hive, RegTarget, RegValue};
use crate::engine::system_store;
use crate::error::{AppError, AppResult};

pub trait SettingStore: Send + Sync {
    /// Current value, or `None` if the value (or its key) does not exist.
    fn read(&self, target: &RegTarget) -> AppResult<Option<RegValue>>;
    /// Sets the value, or deletes it when `value` is `None` (deleting a missing value is a no-op).
    fn write(&self, target: &RegTarget, value: Option<&RegValue>) -> AppResult<()>;
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

/// The registry target behind a `Gpu` target, or `None` without that vendor's driver key.
fn gpu_target(target: &RegTarget) -> Option<RegTarget> {
    let (vendor, rest) = match target.key.split_once('\\') {
        Some((vendor, rest)) => (vendor, Some(rest)),
        None => (target.key.as_ref(), None),
    };
    let id = match vendor {
        "Nvidia" => 0x10de,
        "Amd" => 0x1002,
        "Intel" => 0x8086,
        _ => return None,
    };
    let driver = crate::system::devices::gpu_key(id)?;
    let key = match rest {
        Some(rest) => format!(r"{driver}\{rest}"),
        None => driver.to_string(),
    };
    Some(RegTarget {
        hive: Hive::LocalMachine,
        key: key.into(),
        value: target.value.clone(),
    })
}

fn hresult(err: &windows_registry::Result<()>) -> Option<u32> {
    err.as_ref().err().map(|e| e.code().0 as u32)
}

fn map_err(target: &RegTarget, err: windows_result::Error) -> AppError {
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
            Hive::CurrentUser | Hive::LocalMachine => {}
        }
        let key = match root(target.hive).open(target.key.as_ref()) {
            Ok(key) => key,
            Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(None),
            Err(e) => return Err(map_err(target, e)),
        };
        let ty = match key.get_type(target.value.as_ref()) {
            Ok(ty) => ty,
            Err(e) if e.code().0 as u32 == ERROR_FILE_NOT_FOUND => return Ok(None),
            Err(e) => return Err(map_err(target, e)),
        };
        let name = target.value.as_ref();
        let value = match ty {
            windows_registry::Type::U32 => {
                RegValue::Dword(key.get_u32(name).map_err(|e| map_err(target, e))?)
            }
            windows_registry::Type::U64 => {
                RegValue::Qword(key.get_u64(name).map_err(|e| map_err(target, e))?)
            }
            windows_registry::Type::String => {
                RegValue::String(key.get_string(name).map_err(|e| map_err(target, e))?.into())
            }
            windows_registry::Type::ExpandString => {
                // The HSTRING getter returns the raw, unexpanded text, which is what we must restore.
                RegValue::ExpandString(
                    key.get_hstring(name)
                        .map_err(|e| map_err(target, e))?
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
                let real = gpu_target(target)
                    .ok_or_else(|| AppError::Unsupported("no GPU driver found".into()))?;
                return self.write(&real, value);
            }
            Hive::CurrentUser | Hive::LocalMachine => {}
        }
        let name = target.value.as_ref();
        match value {
            Some(value) => {
                let key = root(target.hive)
                    .create(target.key.as_ref())
                    .map_err(|e| map_err(target, e))?;
                let result = match value {
                    RegValue::Dword(v) => key.set_u32(name, *v),
                    RegValue::Qword(v) => key.set_u64(name, *v),
                    RegValue::String(s) => key.set_string(name, s.as_ref()),
                    RegValue::ExpandString(s) => key.set_expand_string(name, s.as_ref()),
                };
                result.map_err(|e| map_err(target, e))
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
                    Err(e) => return Err(map_err(target, e)),
                };
                let result = key.remove_value(name);
                if hresult(&result) == Some(ERROR_FILE_NOT_FOUND) {
                    return Ok(());
                }
                result.map_err(|e| map_err(target, e))
            }
        }
    }
}

/// In-memory store for tests and fault injection. Keys compare case-insensitively.
#[derive(Default)]
pub struct MemoryStore {
    values: Mutex<BTreeMap<String, RegValue>>,
    faults: Mutex<Faults>,
}

#[derive(Default)]
struct Faults {
    /// Writes to this target fail.
    fail_write: Option<String>,
    /// Writes to this target "succeed" but the value is not changed, like a policy override.
    ignore_write: Option<String>,
}

fn identity(target: &RegTarget) -> String {
    format!("{:?}|{}|{}", target.hive, target.key, target.value).to_ascii_lowercase()
}

impl MemoryStore {
    pub fn with(values: impl IntoIterator<Item = (RegTarget, RegValue)>) -> Self {
        let store = MemoryStore::default();
        for (target, value) in values {
            store
                .values
                .lock()
                .unwrap()
                .insert(identity(&target), value);
        }
        store
    }

    pub fn fail_writes_to(&self, target: &RegTarget) {
        self.faults.lock().unwrap().fail_write = Some(identity(target));
    }

    pub fn ignore_writes_to(&self, target: &RegTarget) {
        self.faults.lock().unwrap().ignore_write = Some(identity(target));
    }

    pub fn clear_faults(&self) {
        *self.faults.lock().unwrap() = Faults::default();
    }

    pub fn get(&self, target: &RegTarget) -> Option<RegValue> {
        self.values.lock().unwrap().get(&identity(target)).cloned()
    }

    pub fn set(&self, target: &RegTarget, value: Option<RegValue>) {
        let mut values = self.values.lock().unwrap();
        match value {
            Some(v) => values.insert(identity(target), v),
            None => values.remove(&identity(target)),
        };
    }
}

impl SettingStore for MemoryStore {
    fn read(&self, target: &RegTarget) -> AppResult<Option<RegValue>> {
        Ok(self.get(target))
    }

    fn write(&self, target: &RegTarget, value: Option<&RegValue>) -> AppResult<()> {
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
