//! Settings Windows keeps outside the registry, read and written through their own APIs.
//!
//! `store` routes the non-registry [`Hive`]s here. Each maps a target to a single value, so the
//! engine backs them up, verifies and reverts them exactly like registry values.

use std::cell::RefCell;

use windows::Win32::Foundation::{
    ERROR_FILE_NOT_FOUND, HLOCAL, LocalFree, VARIANT_BOOL, WIN32_ERROR,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
};
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerReadACValueIndex, PowerReadDCValueIndex, PowerSetActiveScheme,
    PowerWriteACValueIndex, PowerWriteDCValueIndex,
};
use windows::Win32::System::TaskScheduler::{IRegisteredTask, ITaskService, TaskScheduler};
use windows::Win32::System::Variant::VARIANT;
use windows::core::{BSTR, GUID};

use crate::engine::setting::{RegTarget, RegValue};
use crate::error::{AppError, AppResult};

const HR_FILE_NOT_FOUND: u32 = 0x8007_0002;
const HR_PATH_NOT_FOUND: u32 = 0x8007_0003;
const HR_ACCESS_DENIED: u32 = 0x8007_0005;

fn map_err(target: &RegTarget, err: windows::core::Error) -> AppError {
    match err.code().0 as u32 {
        HR_ACCESS_DENIED => AppError::ElevationRequired(format!("access denied on `{target}`")),
        _ => AppError::System(format!("`{target}`: {}", err.message())),
    }
}

thread_local! {
    /// Reading the catalogue touches every task: one connection per thread, not one per read.
    static TASK_SERVICE: RefCell<Option<ITaskService>> = const { RefCell::new(None) };
}

fn task_service() -> windows::core::Result<ITaskService> {
    TASK_SERVICE.with_borrow_mut(|cached| {
        if let Some(service) = cached {
            return Ok(service.clone());
        }
        // SAFETY: plain COM calls on interfaces this function owns. A thread that already chose
        // another apartment model reports an error here, and COM still works on it, so that's
        // ignored.
        let service = unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let service: ITaskService =
                CoCreateInstance(&TaskScheduler, None, CLSCTX_INPROC_SERVER)?;
            let none = VARIANT::default();
            service.Connect(&none, &none, &none, &none)?;
            service
        };
        *cached = Some(service.clone());
        Ok(service)
    })
}

/// `None` when the task isn't on this PC, which differs by Windows version.
fn registered_task(path: &str) -> windows::core::Result<Option<IRegisteredTask>> {
    let (folder, name) = path.rsplit_once('\\').unwrap_or(("", path));
    let folder = if folder.is_empty() { r"\" } else { folder };
    let service = task_service()?;
    // SAFETY: `service` is a live, connected interface.
    unsafe {
        let found = service
            .GetFolder(&BSTR::from(folder))
            .and_then(|f| f.GetTask(&BSTR::from(name)));
        match found {
            Ok(task) => Ok(Some(task)),
            Err(e) if matches!(e.code().0 as u32, HR_FILE_NOT_FOUND | HR_PATH_NOT_FOUND) => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

pub fn read_task(target: &RegTarget) -> AppResult<Option<RegValue>> {
    let Some(task) = registered_task(&target.key).map_err(|e| map_err(target, e))? else {
        return Ok(None);
    };
    // SAFETY: `task` is a live interface.
    let enabled = unsafe { task.Enabled() }.map_err(|e| map_err(target, e))?;
    Ok(Some(RegValue::Dword(u32::from(enabled.as_bool()))))
}

pub fn write_task(target: &RegTarget, value: Option<&RegValue>) -> AppResult<()> {
    let Some(RegValue::Dword(data)) = value else {
        return Err(AppError::Unsupported(format!(
            "`{target}` can only be turned on or off"
        )));
    };
    let Some(task) = registered_task(&target.key).map_err(|e| map_err(target, e))? else {
        return Err(AppError::Unsupported(format!(
            "`{target}` isn't on this PC"
        )));
    };
    // SAFETY: `task` is a live interface.
    unsafe { task.SetEnabled(VARIANT_BOOL::from(*data != 0)) }.map_err(|e| map_err(target, e))
}

fn power_ids(target: &RegTarget) -> AppResult<(GUID, GUID)> {
    let parsed = target.key.split_once('\\').and_then(|(subgroup, setting)| {
        Some((
            GUID::try_from(subgroup).ok()?,
            GUID::try_from(setting).ok()?,
        ))
    });
    parsed.ok_or_else(|| AppError::Policy(format!("malformed power setting `{target}`")))
}

fn win32(target: &RegTarget, code: WIN32_ERROR) -> AppResult<()> {
    code.ok().map_err(|e| map_err(target, e))
}

/// The active plan, which `PowerGetActiveScheme` allocates for the caller to free.
fn active_scheme(target: &RegTarget) -> AppResult<GUID> {
    let mut scheme: *mut GUID = std::ptr::null_mut();
    // SAFETY: the call fills `scheme`, which is read once and freed once.
    unsafe {
        win32(target, PowerGetActiveScheme(None, &mut scheme))?;
        let guid = *scheme;
        let _ = LocalFree(Some(HLOCAL(scheme.cast())));
        Ok(guid)
    }
}

pub fn read_power(target: &RegTarget) -> AppResult<Option<RegValue>> {
    let (subgroup, setting) = power_ids(target)?;
    let scheme = active_scheme(target)?;
    let mut data = 0u32;
    // SAFETY: the GUID pointers and `data` outlive the call.
    let code = unsafe {
        let (id, sub, set): (*const GUID, *const GUID, *const GUID) =
            (&scheme, &subgroup, &setting);
        if target.value == "DC" {
            WIN32_ERROR(PowerReadDCValueIndex(
                None,
                Some(id),
                Some(sub),
                Some(set),
                &mut data,
            ))
        } else {
            PowerReadACValueIndex(None, Some(id), Some(sub), Some(set), &mut data)
        }
    };
    if code == ERROR_FILE_NOT_FOUND {
        return Ok(None);
    }
    win32(target, code)?;
    Ok(Some(RegValue::Dword(data)))
}

pub fn write_power(target: &RegTarget, value: Option<&RegValue>) -> AppResult<()> {
    let Some(RegValue::Dword(data)) = value else {
        return Err(AppError::Unsupported(format!("`{target}` needs a number")));
    };
    let (subgroup, setting) = power_ids(target)?;
    let scheme = active_scheme(target)?;
    // SAFETY: the GUID pointers outlive both calls.
    unsafe {
        let (sub, set): (*const GUID, *const GUID) = (&subgroup, &setting);
        let code = if target.value == "DC" {
            WIN32_ERROR(PowerWriteDCValueIndex(
                None,
                &scheme,
                Some(sub),
                Some(set),
                *data,
            ))
        } else {
            PowerWriteACValueIndex(None, &scheme, Some(sub), Some(set), *data)
        };
        win32(target, code)?;
        // A new index only takes effect when the plan is made active again.
        win32(target, PowerSetActiveScheme(None, Some(&scheme)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::setting::Hive;

    // These read the real system and change nothing.
    const MAPS: RegTarget = RegTarget::new(
        Hive::ScheduledTask,
        r"\Microsoft\Windows\Maps\MapsUpdateTask",
        "Enabled",
    );

    #[test]
    fn reads_a_scheduled_task() {
        let state = read_task(&MAPS).unwrap();
        assert!(matches!(state, None | Some(RegValue::Dword(0 | 1))));
    }

    #[test]
    fn a_missing_task_is_none() {
        let gone = RegTarget::new(Hive::ScheduledTask, r"\2VX\Not A Task", "Enabled");
        assert_eq!(read_task(&gone).unwrap(), None);
    }

    #[test]
    fn reads_a_power_setting() {
        // Processor boost mode, on every plan.
        let target = RegTarget::new(
            Hive::PowerPlan,
            r"54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7",
            "AC",
        );
        assert!(matches!(
            read_power(&target).unwrap(),
            Some(RegValue::Dword(_))
        ));
    }

    #[test]
    fn rejects_a_malformed_power_key() {
        let target = RegTarget::new(Hive::PowerPlan, "not a guid", "AC");
        assert!(read_power(&target).is_err());
    }
}
