//! The services the catalogue's tweaks switch, as Windows reports them now. Read-only: the handles
//! here can list and query, and a service's start type only ever changes through the tweak engine.

use std::collections::HashSet;

use serde::Serialize;
use windows::Win32::System::Services::{
    CloseServiceHandle, ENUM_SERVICE_STATUS_PROCESSW, ENUM_SERVICE_TYPE, EnumServicesStatusExW,
    OpenSCManagerW, OpenServiceW, QUERY_SERVICE_CONFIGW, QueryServiceConfig2W, QueryServiceConfigW,
    SC_ENUM_PROCESS_INFO, SC_HANDLE, SC_MANAGER_CONNECT, SC_MANAGER_ENUMERATE_SERVICE,
    SERVICE_CONFIG_DESCRIPTION, SERVICE_DESCRIPTIONW, SERVICE_DRIVER, SERVICE_QUERY_CONFIG,
    SERVICE_RUNNING, SERVICE_STATE_ALL, SERVICE_WIN32,
};
use windows::core::{HSTRING, PCWSTR};
use windows_registry::LOCAL_MACHINE;

const SERVICES: &str = r"SYSTEM\CurrentControlSet\Services";
/// `Type` flag of a per-user template, which never runs itself: Windows starts a copy named
/// `<name>_<session>` for each signed-in user.
const USER_SERVICE: u32 = 0x40;
/// Services can start between asking for the list's size and asking for the list.
const LIST_SLACK: usize = 4096;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ServiceStartType {
    Automatic,
    AutomaticDelayed,
    Manual,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceState {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    /// `None` when the Service Control Manager can't be asked.
    pub running: Option<bool>,
    pub start_type: ServiceStartType,
}

struct Handle(SC_HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        // SAFETY: an open handle this type owns, closed once.
        let _ = unsafe { CloseServiceHandle(self.0) };
    }
}

/// The state of each named service that's installed. One that isn't is left out.
pub fn states(names: &[String]) -> Vec<ServiceState> {
    // SAFETY: null names mean this PC's active database. The access asked for is read-only.
    let manager = unsafe {
        OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_CONNECT | SC_MANAGER_ENUMERATE_SERVICE,
        )
    }
    .ok()
    .map(Handle);
    let running = manager.as_ref().and_then(running_services);

    names
        .iter()
        .filter_map(|name| {
            let key = LOCAL_MACHINE.open(format!(r"{SERVICES}\{name}")).ok()?;
            let delayed = key.get_u32("DelayedAutostart").is_ok_and(|v| v != 0);
            let per_user = key.get_u32("Type").is_ok_and(|t| t & USER_SERVICE != 0);
            let (display_name, description) = manager
                .as_ref()
                .and_then(|manager| service_text(manager, name))
                .unwrap_or_default();
            Some(ServiceState {
                display_name: display_name.unwrap_or_else(|| name.clone()),
                description,
                running: running.as_ref().map(|set| is_running(name, per_user, set)),
                start_type: start_type(key.get_u32("Start").ok(), delayed),
                name: name.clone(),
            })
        })
        .collect()
}

/// `u64`s, so the structures Windows writes into the buffer are aligned for their pointers.
fn buffer_of(bytes: usize) -> Vec<u64> {
    vec![0; bytes.div_ceil(size_of::<u64>()).max(1)]
}

fn as_bytes(buffer: &mut [u64]) -> &mut [u8] {
    let length = size_of_val(buffer);
    // SAFETY: the same memory, and every byte pattern is a valid `u8`.
    unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr().cast(), length) }
}

/// Lowercase names of everything running now.
fn running_services(manager: &Handle) -> Option<HashSet<String>> {
    let kinds = ENUM_SERVICE_TYPE(SERVICE_WIN32.0 | SERVICE_DRIVER.0);
    let (mut needed, mut count) = (0, 0);
    // SAFETY: the out pointers are valid. Fails by design, reporting the size it needs.
    let _ = unsafe {
        EnumServicesStatusExW(
            manager.0,
            SC_ENUM_PROCESS_INFO,
            kinds,
            SERVICE_STATE_ALL,
            None,
            &mut needed,
            &mut count,
            None,
            PCWSTR::null(),
        )
    };
    let mut buffer = buffer_of(needed as usize + LIST_SLACK);
    // SAFETY: the buffer is writable for the length the slice carries.
    unsafe {
        EnumServicesStatusExW(
            manager.0,
            SC_ENUM_PROCESS_INFO,
            kinds,
            SERVICE_STATE_ALL,
            Some(as_bytes(&mut buffer)),
            &mut needed,
            &mut count,
            None,
            PCWSTR::null(),
        )
    }
    .ok()?;
    // SAFETY: on success the buffer starts with `count` rows, and the names they point at are
    // further inside it, read before it's dropped.
    let rows = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr().cast::<ENUM_SERVICE_STATUS_PROCESSW>(),
            count as usize,
        )
    };
    Some(
        rows.iter()
            .filter(|row| row.ServiceStatusProcess.dwCurrentState == SERVICE_RUNNING)
            // SAFETY: see above.
            .filter_map(|row| unsafe { row.lpServiceName.to_string() }.ok())
            .map(|name| name.to_ascii_lowercase())
            .collect(),
    )
}

/// The localised display name and description. The registry often holds a resource reference
/// instead, which only the Service Control Manager resolves.
fn service_text(manager: &Handle, name: &str) -> Option<(Option<String>, Option<String>)> {
    // SAFETY: `HSTRING` is NUL-terminated. The access asked for is read-only.
    let service =
        unsafe { OpenServiceW(manager.0, &HSTRING::from(name), SERVICE_QUERY_CONFIG) }.ok()?;
    let service = Handle(service);
    Some((display_name(&service), description(&service)))
}

fn display_name(service: &Handle) -> Option<String> {
    let mut needed = 0;
    // SAFETY: the out pointer is valid. Fails by design, reporting the size it needs.
    let _ = unsafe { QueryServiceConfigW(service.0, None, 0, &mut needed) };
    let mut buffer = buffer_of(needed as usize);
    let config = buffer.as_mut_ptr().cast::<QUERY_SERVICE_CONFIGW>();
    // SAFETY: the buffer is `needed` bytes at least, and aligned for the structure.
    unsafe { QueryServiceConfigW(service.0, Some(config), needed, &mut needed) }.ok()?;
    // SAFETY: on success the structure is filled in, its strings inside the buffer.
    let name = unsafe { (*config).lpDisplayName.to_string() }.ok()?;
    (!name.is_empty()).then_some(name)
}

fn description(service: &Handle) -> Option<String> {
    let mut needed = 0;
    // SAFETY: the out pointer is valid. Fails by design, reporting the size it needs.
    let _ =
        unsafe { QueryServiceConfig2W(service.0, SERVICE_CONFIG_DESCRIPTION, None, &mut needed) };
    let mut buffer = buffer_of(needed as usize);
    // SAFETY: the buffer is writable for the length the slice carries.
    unsafe {
        QueryServiceConfig2W(
            service.0,
            SERVICE_CONFIG_DESCRIPTION,
            Some(as_bytes(&mut buffer)),
            &mut needed,
        )
    }
    .ok()?;
    // SAFETY: on success the buffer starts with the structure, its string inside the buffer.
    let text = unsafe { (*buffer.as_ptr().cast::<SERVICE_DESCRIPTIONW>()).lpDescription };
    if text.is_null() {
        return None;
    }
    // SAFETY: not null, and NUL-terminated by the service manager.
    let text = unsafe { text.to_string() }.ok()?;
    (!text.is_empty()).then_some(text)
}

fn is_running(name: &str, per_user: bool, running: &HashSet<String>) -> bool {
    let name = name.to_ascii_lowercase();
    running.contains(&name)
        || per_user
            && running
                .iter()
                .any(|s| s.strip_prefix(&name).is_some_and(|r| r.starts_with('_')))
}

/// `Start` is 0 and 1 for drivers loaded at boot, 2 automatic, 3 manual and 4 disabled.
fn start_type(start: Option<u32>, delayed: bool) -> ServiceStartType {
    match start {
        Some(2) if delayed => ServiceStartType::AutomaticDelayed,
        Some(0..=2) => ServiceStartType::Automatic,
        Some(3) => ServiceStartType::Manual,
        Some(4) => ServiceStartType::Disabled,
        _ => ServiceStartType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_values_are_named() {
        use ServiceStartType::*;
        for (start, delayed, expected) in [
            (Some(0), false, Automatic),
            (Some(2), false, Automatic),
            (Some(2), true, AutomaticDelayed),
            (Some(3), true, Manual),
            (Some(4), false, Disabled),
            (Some(9), false, Unknown),
            (None, false, Unknown),
        ] {
            assert_eq!(start_type(start, delayed), expected);
        }
    }

    #[test]
    fn a_per_user_service_runs_as_its_session_copy() {
        let running = HashSet::from(["cdpusersvc_1a2b3".to_owned(), "spooler".to_owned()]);
        assert!(is_running("CDPUserSvc", true, &running));
        assert!(is_running("Spooler", false, &running));
        // Only templates have copies: this is some other service with a similar name.
        assert!(!is_running("CDPUserSvc", false, &running));
        assert!(!is_running("Fax", true, &running));
    }

    #[test]
    fn reads_this_pc_and_leaves_out_what_is_not_installed() {
        let states = states(&["Winmgmt".to_owned(), "2VXNoSuchService".to_owned()]);
        assert_eq!(states.len(), 1);
        assert_eq!(states[0].name, "Winmgmt");
        assert!(!states[0].display_name.is_empty());
    }
}
