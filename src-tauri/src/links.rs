//! Opens one entry from a fixed list: a page, a Windows tool, or the app's own log folder. The
//! frontend names an entry, never an address or a command, so this can't be turned into a way to
//! start something else. See `docs/security.md`.

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use windows::Win32::System::Com::{
    COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx, CoUninitialize,
};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::UI::Shell::{SE_ERR_ACCESSDENIED, ShellExecuteW};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HSTRING, PCWSTR};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Link {
    NvidiaDrivers,
    AmdDrivers,
    IntelDrivers,
    BufferbloatTest,
    BufferbloatFixes,
    GraphicsSettings,
    NetworkSettings,
    TimerDocs,
    ReportIssue,
    ProjectPage,
}

impl Link {
    const fn target(self) -> &'static str {
        match self {
            Self::NvidiaDrivers => "https://www.nvidia.com/en-us/drivers/",
            Self::AmdDrivers => "https://www.amd.com/en/support/download/drivers.html",
            Self::IntelDrivers => "https://www.intel.com/content/www/us/en/support/detect.html",
            Self::BufferbloatTest => "https://www.waveform.com/tools/bufferbloat",
            Self::BufferbloatFixes => {
                "https://www.bufferbloat.net/projects/bloat/wiki/What_can_I_do_about_Bufferbloat/"
            }
            Self::GraphicsSettings => "ms-settings:display-advancedgraphics",
            Self::NetworkSettings => "ms-settings:network-advancedsettings",
            Self::TimerDocs => {
                "https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod"
            }
            Self::ReportIssue => "https://github.com/ryanvxc/2vx-tweaks/issues/new",
            Self::ProjectPage => "https://github.com/ryanvxc/2vx-tweaks",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WindowsTool {
    ResourceMonitor,
    ReliabilityMonitor,
    SystemInformation,
    DirectxDiagnostic,
    DiskCleanup,
    EventViewer,
    TaskManager,
    DeviceManager,
    ServicesConsole,
    NetworkConnections,
}

/// How Windows is asked to open a tool. Every part is a constant, reviewed in `tests/policy.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolCommand {
    /// A bare file name, only ever looked for in System32.
    pub file: &'static str,
    pub arguments: Option<&'static str>,
    /// Named per tool because a null verb falls back to the first one a file type lists. Never an
    /// elevating verb: a tool that wants administrator rights asks Windows through its own manifest.
    pub verb: &'static str,
}

const fn opens(file: &'static str) -> ToolCommand {
    ToolCommand {
        file,
        arguments: None,
        verb: "open",
    }
}

impl WindowsTool {
    pub const ALL: [WindowsTool; 10] = [
        Self::ResourceMonitor,
        Self::ReliabilityMonitor,
        Self::SystemInformation,
        Self::DirectxDiagnostic,
        Self::DiskCleanup,
        Self::EventViewer,
        Self::TaskManager,
        Self::DeviceManager,
        Self::ServicesConsole,
        Self::NetworkConnections,
    ];

    pub const fn command(self) -> ToolCommand {
        match self {
            Self::ResourceMonitor => opens("resmon.exe"),
            Self::ReliabilityMonitor => ToolCommand {
                arguments: Some("/rel"),
                ..opens("perfmon.exe")
            },
            Self::SystemInformation => opens("msinfo32.exe"),
            Self::DirectxDiagnostic => opens("dxdiag.exe"),
            Self::DiskCleanup => opens("cleanmgr.exe"),
            Self::EventViewer => opens("eventvwr.msc"),
            Self::TaskManager => opens("taskmgr.exe"),
            Self::DeviceManager => opens("devmgmt.msc"),
            Self::ServicesConsole => opens("services.msc"),
            // Control Panel files have no `open` verb. This is the one that shows them.
            Self::NetworkConnections => ToolCommand {
                verb: "cplopen",
                ..opens("ncpa.cpl")
            },
        }
    }
}

/// Shell extensions want a single-threaded apartment, and worker threads have joined the
/// multithreaded one for Task Scheduler, so this gets a thread of its own, as the save dialog does.
/// The call can sit behind a UAC prompt, which is why the commands run this off the main thread.
fn shell_open(verb: &str, target: &HSTRING, arguments: Option<&str>, what: &str) -> AppResult<()> {
    let code = std::thread::scope(|scope| {
        scope
            .spawn(|| {
                // SAFETY: this thread is new, and paired with CoUninitialize below when it succeeds.
                let com = unsafe {
                    CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)
                };
                let code = shell_execute(verb, target, arguments);
                if com.is_ok() {
                    // SAFETY: pairs with the call above, and nothing COM outlives this line.
                    unsafe { CoUninitialize() };
                }
                code
            })
            .join()
    })
    .map_err(|_| AppError::Internal(format!("opening {what} stopped unexpectedly")))?;
    shell_result(code, what)
}

fn shell_execute(verb: &str, target: &HSTRING, arguments: Option<&str>) -> isize {
    let verb = HSTRING::from(verb);
    let arguments = arguments.map(HSTRING::from);
    let arguments = arguments
        .as_ref()
        .map_or(PCWSTR::null(), |a| PCWSTR(a.as_ptr()));
    // SAFETY: every string outlives the call, and no window handle is passed.
    let result = unsafe { ShellExecuteW(None, &verb, target, arguments, None, SW_SHOWNORMAL) };
    result.0 as isize
}

fn shell_result(code: isize, what: &str) -> AppResult<()> {
    match code {
        // Documented: anything above 32 is success.
        33.. => Ok(()),
        // The tool asked for administrator rights through its manifest, and the user said no.
        code if code == SE_ERR_ACCESSDENIED as isize => Err(AppError::ElevationRequired(
            "the administrator prompt was declined".into(),
        )),
        _ => Err(AppError::Internal(format!("couldn't open {what}"))),
    }
}

pub fn open(link: Link) -> AppResult<()> {
    let target = link.target();
    shell_open("open", &HSTRING::from(target), None, target)
}

/// The real System32, from Windows: PATH or the working directory could put another file first.
fn system_directory() -> AppResult<PathBuf> {
    let mut buffer = [0u16; 512];
    // SAFETY: the buffer is writable for its whole length, which the call receives.
    let length = unsafe { GetSystemDirectoryW(Some(&mut buffer)) } as usize;
    if length == 0 || length >= buffer.len() {
        return Err(AppError::Internal(
            "couldn't find the Windows system folder".into(),
        ));
    }
    Ok(OsString::from_wide(&buffer[..length]).into())
}

pub fn open_tool(tool: WindowsTool) -> AppResult<()> {
    let command = tool.command();
    let path = system_directory()?.join(command.file);
    shell_open(
        command.verb,
        &HSTRING::from(path.as_path()),
        command.arguments,
        command.file,
    )
}

/// The caller passes the directory `logging` was set up with. No command takes a path.
pub fn open_log_folder(log_dir: &Path) -> AppResult<()> {
    shell_open("open", &HSTRING::from(log_dir), None, "the log folder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_link_is_https_or_windows_settings() {
        for link in [
            Link::NvidiaDrivers,
            Link::AmdDrivers,
            Link::IntelDrivers,
            Link::BufferbloatTest,
            Link::BufferbloatFixes,
            Link::GraphicsSettings,
            Link::NetworkSettings,
            Link::TimerDocs,
            Link::ReportIssue,
            Link::ProjectPage,
        ] {
            let target = link.target();
            assert!(
                target.starts_with("https://") || target.starts_with("ms-settings:"),
                "{target}"
            );
        }
    }

    #[test]
    fn a_declined_prompt_is_not_an_internal_error() {
        assert!(shell_result(33, "x").is_ok());
        assert!(matches!(
            shell_result(5, "x"),
            Err(AppError::ElevationRequired(_))
        ));
        for code in [0, 2, 3, 8, 26, 31, 32] {
            assert!(matches!(
                shell_result(code, "x"),
                Err(AppError::Internal(_))
            ));
        }
    }

    #[test]
    fn tools_are_looked_for_in_system32() {
        let directory = system_directory().unwrap();
        assert!(directory.is_absolute());
        assert!(directory.ends_with("System32") || directory.ends_with("system32"));
    }

    #[test]
    fn tools_arrive_as_names() {
        let tool: WindowsTool = serde_json::from_str("\"directxDiagnostic\"").unwrap();
        assert_eq!(tool, WindowsTool::DirectxDiagnostic);
        assert!(serde_json::from_str::<WindowsTool>(r#""C:\\Windows\\notepad.exe""#).is_err());
    }
}
