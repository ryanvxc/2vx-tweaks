//! Least-privilege elevation broker.
//!
//! The app runs unelevated. When a batch touches HKLM, [`BrokerSession::launch`] starts a
//! short-lived elevated copy of this executable (one UAC prompt) that connects back over
//! a private named pipe and performs individual writes for the duration of the batch.
//!
//! Hardening:
//! - The pipe name carries 128 random bits and is created with `FILE_FLAG_FIRST_PIPE_INSTANCE`,
//!   so it cannot be pre-created by another process; remote clients are rejected.
//! - The server accepts exactly one client and checks it is the process it launched (PID).
//! - The elevated side checks the pipe server is the parent that launched it, and opens the
//!   pipe at `SECURITY_IDENTIFICATION` so the unelevated server cannot impersonate it.
//! - The elevated side writes only targets declared by a built-in tweak, after the same
//!   policy check the engine uses. It accepts no commands, paths or code.
//! - Clearing a location takes its allowlist ID. Moving a Run entry takes a key from a fixed
//!   list and a value name; the value itself never crosses the pipe, so the elevated side
//!   can only put back what it took out.
//! - The journal stays in the unelevated app; the broker holds no state and exits when done.

use std::fs::File;
use std::io::{self, Read, Write};
use std::os::windows::io::{FromRawHandle, RawHandle};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_SHARE_NONE, OPEN_EXISTING, PIPE_ACCESS_DUPLEX,
    SECURITY_IDENTIFICATION, SECURITY_SQOS_PRESENT,
};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, GetNamedPipeClientProcessId, GetNamedPipeServerProcessId,
    PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows::Win32::System::Threading::{GetProcessId, WaitForSingleObject};
use windows::Win32::UI::Shell::{
    SEE_MASK_NOASYNC, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::core::{HSTRING, PCWSTR, w};

use crate::engine::TweakRegistry;
use crate::engine::cleanup::{self, CleanupResult};
use crate::engine::plan::new_plan_id;
use crate::engine::policy;
use crate::engine::setting::{Hive, RegTarget, RegValue};
use crate::engine::startup::{self, RunKey};
use crate::engine::store::{LocalWriter, SettingStore, SettingWriter, WindowsRegistry};
use crate::error::{AppError, AppResult};

/// First argument that switches the executable into broker mode (see `main.rs`).
pub const BROKER_FLAG: &str = "--2vx-broker";
const PIPE_PREFIX: &str = r"\\.\pipe\2vx-broker-";
const MAX_FRAME: u32 = 64 * 1024;
const CONNECT_TIMEOUT_MS: u32 = 60_000;
const EXIT_TIMEOUT_MS: u32 = 5_000;
const ERROR_CANCELLED: u32 = 0x8007_04C7;
const ERROR_PIPE_CONNECTED: u32 = 0x8007_0217;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Request {
    Write {
        target: RegTarget,
        value: Option<RegValue>,
    },
    /// Clears one administrator location from the cleanup allowlist.
    Clean {
        id: String,
    },
    /// Moves one HKLM entry between its Run key and 2VX's holding key.
    MoveStartup {
        key: RunKey,
        name: String,
        enabled: bool,
    },
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "camelCase")]
pub enum Response {
    Written { read_back: Option<RegValue> },
    Cleaned { outcome: CleanupResult },
    Moved,
    Error { code: String, message: String },
}

/// Length-prefixed (u32 LE) JSON frames.
pub fn write_frame<W: Write, T: Serialize>(w: &mut W, value: &T) -> io::Result<()> {
    let body = serde_json::to_vec(value).map_err(io::Error::other)?;
    let len = u32::try_from(body.len())
        .ok()
        .filter(|&l| l <= MAX_FRAME)
        .ok_or_else(|| io::Error::other("frame too large"))?;
    w.write_all(&len.to_le_bytes())?;
    w.write_all(&body)?;
    w.flush()
}

pub fn read_frame<R: Read, T: DeserializeOwned>(r: &mut R) -> io::Result<T> {
    let mut len = [0u8; 4];
    r.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len);
    if len > MAX_FRAME {
        return Err(io::Error::other("frame too large"));
    }
    let mut body = vec![0u8; len as usize];
    r.read_exact(&mut body)?;
    serde_json::from_slice(&body).map_err(io::Error::other)
}

/// What the elevated side does with one write request.
pub fn handle_write(
    registry: &TweakRegistry,
    store: &dyn SettingStore,
    target: &RegTarget,
    value: Option<&RegValue>,
) -> AppResult<Option<RegValue>> {
    policy::check_target(target)?;
    if !registry.declares_target(target) {
        return Err(AppError::Policy(format!(
            "`{target}` is not declared by any built-in tweak"
        )));
    }
    LocalWriter(store).write(target, value)
}

/// Serves requests until `Done` or the pipe closes.
pub fn serve<S: Read + Write>(
    stream: &mut S,
    registry: &TweakRegistry,
    store: &dyn SettingStore,
) -> io::Result<()> {
    loop {
        let request: Request = match read_frame(stream) {
            Ok(r) => r,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(e),
        };
        let response = match request {
            Request::Done => return Ok(()),
            Request::Write { target, value } => {
                handle_write(registry, store, &target, value.as_ref())
                    .map(|read_back| Response::Written { read_back })
            }
            Request::Clean { id } => {
                cleanup::clean_elevated(&id).map(|outcome| Response::Cleaned { outcome })
            }
            Request::MoveStartup { key, name, enabled } => {
                startup::move_value(Hive::LocalMachine, key, &name, enabled)
                    .map(|()| Response::Moved)
            }
        };
        let response = response.unwrap_or_else(|e| Response::Error {
            code: e.code().into(),
            message: e.to_string(),
        });
        write_frame(stream, &response)?;
    }
}

/// Entry point of the elevated process. Returns the process exit code.
pub fn run_elevated(args: &[String]) -> i32 {
    let [pipe, parent] = args else { return 2 };
    let Ok(parent_pid) = parent.parse::<u32>() else {
        return 2;
    };
    let well_formed = pipe
        .strip_prefix(PIPE_PREFIX)
        .is_some_and(crate::engine::plan::is_plan_id);
    if !well_formed {
        return 2;
    }
    if !crate::system::privilege::is_elevated() {
        return 3;
    }
    let mut stream = match connect_to_parent(pipe, parent_pid) {
        Ok(s) => s,
        Err(_) => return 4,
    };
    match serve(
        &mut stream,
        &TweakRegistry::with_builtin_tweaks(),
        &WindowsRegistry,
    ) {
        Ok(()) => 0,
        Err(_) => 5,
    }
}

fn connect_to_parent(pipe: &str, parent_pid: u32) -> AppResult<File> {
    let name = HSTRING::from(pipe);
    // SAFETY: plain Win32 calls with valid arguments; the handle is owned by the returned File
    // or closed on the error path.
    unsafe {
        let handle = CreateFileW(
            &name,
            GENERIC_READ.0 | GENERIC_WRITE.0,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            // Identification only: the unelevated server must not impersonate this token.
            SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
            None,
        )
        .map_err(|e| AppError::System(format!("open broker pipe: {e}")))?;
        let mut server_pid = 0u32;
        if GetNamedPipeServerProcessId(handle, &mut server_pid).is_err() || server_pid != parent_pid
        {
            let _ = CloseHandle(handle);
            return Err(AppError::Policy(
                "broker pipe is not owned by the launching app".into(),
            ));
        }
        Ok(File::from_raw_handle(handle.0 as RawHandle))
    }
}

/// A process handle that can be shared with the connect watchdog.
struct ProcessHandle(HANDLE);
// SAFETY: a process handle is a kernel object handle, valid from any thread.
unsafe impl Send for ProcessHandle {}
unsafe impl Sync for ProcessHandle {}
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        // SAFETY: owned handle from ShellExecuteExW, closed exactly once.
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

/// Creates the server end. Fails if any process already owns a pipe with this name.
fn create_server(name: &HSTRING) -> AppResult<File> {
    // SAFETY: valid name; the returned handle is owned by the File.
    let server = unsafe {
        CreateNamedPipeW(
            name,
            PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            MAX_FRAME + 4,
            MAX_FRAME + 4,
            0,
            None,
        )
    };
    if server.is_invalid() {
        return Err(AppError::System("could not create the broker pipe".into()));
    }
    // SAFETY: valid, owned pipe handle; the File closes it on every path.
    Ok(unsafe { File::from_raw_handle(server.0 as RawHandle) })
}

/// Waits for one client and checks it is `expected_pid`.
fn accept(pipe: &File, expected_pid: u32) -> AppResult<()> {
    let server = HANDLE(pipe_raw(pipe));
    // SAFETY: valid pipe handle owned by `pipe`.
    let connect = unsafe { ConnectNamedPipe(server, None) };
    if let Err(e) = connect
        && e.code().0 as u32 != ERROR_PIPE_CONNECTED
    {
        return Err(AppError::System(format!(
            "elevated helper did not connect: {e}"
        )));
    }
    let mut client_pid = 0u32;
    // SAFETY: valid, connected pipe handle.
    let pid_ok = unsafe { GetNamedPipeClientProcessId(server, &mut client_pid) }.is_ok();
    if !pid_ok || client_pid != expected_pid {
        return Err(AppError::Policy(
            "an unexpected process connected to the broker pipe".into(),
        ));
    }
    Ok(())
}

/// The unelevated side of one elevated batch.
pub struct BrokerSession {
    pipe: File,
    /// `None` only in tests that run both ends in one process.
    process: Option<Arc<ProcessHandle>>,
}

impl BrokerSession {
    /// Shows the UAC prompt and waits for the elevated helper to connect.
    pub fn launch() -> AppResult<Self> {
        let pipe_name = format!("{PIPE_PREFIX}{}", new_plan_id()?);
        let wide_name = HSTRING::from(pipe_name.as_str());
        let pipe = create_server(&wide_name)?;

        let exe = std::env::current_exe()?;
        let file = HSTRING::from(exe.as_os_str());
        let params = HSTRING::from(format!("{BROKER_FLAG} {pipe_name} {}", std::process::id()));
        let mut info = SHELLEXECUTEINFOW {
            cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC,
            lpVerb: w!("runas"),
            lpFile: PCWSTR(file.as_ptr()),
            lpParameters: PCWSTR(params.as_ptr()),
            nShow: SW_HIDE.0,
            ..Default::default()
        };
        // SAFETY: `info` is fully initialised and its strings outlive the call.
        if let Err(e) = unsafe { ShellExecuteExW(&mut info) } {
            return Err(if e.code().0 as u32 == ERROR_CANCELLED {
                AppError::ElevationRequired("the administrator prompt was declined".into())
            } else {
                AppError::System(format!("could not start the elevated helper: {e}"))
            });
        }
        if info.hProcess.is_invalid() {
            return Err(AppError::System("the elevated helper did not start".into()));
        }
        let process = Arc::new(ProcessHandle(info.hProcess));
        // SAFETY: valid process handle.
        let child_pid = unsafe { GetProcessId(process.0) };

        // ConnectNamedPipe blocks until a client connects. If the helper dies or never
        // connects, the watchdog connects itself so the wait ends and the PID check fails.
        let connected = Arc::new(AtomicBool::new(false));
        {
            let process = Arc::clone(&process);
            let connected = Arc::clone(&connected);
            let name = wide_name.clone();
            std::thread::spawn(move || {
                // SAFETY: the Arc keeps the process handle open for this wait.
                unsafe {
                    WaitForSingleObject(process.0, CONNECT_TIMEOUT_MS);
                }
                if !connected.load(Ordering::SeqCst) {
                    // SAFETY: opening and immediately closing a pipe client handle.
                    unsafe {
                        if let Ok(h) = CreateFileW(
                            &name,
                            GENERIC_READ.0,
                            FILE_SHARE_NONE,
                            None,
                            OPEN_EXISTING,
                            Default::default(),
                            None,
                        ) {
                            let _ = CloseHandle(h);
                        }
                    }
                }
            });
        }

        let accepted = accept(&pipe, child_pid);
        connected.store(true, Ordering::SeqCst);
        accepted?;
        tracing::info!(child_pid, "elevated helper connected");
        Ok(BrokerSession {
            pipe,
            process: Some(process),
        })
    }
}

fn pipe_raw(file: &File) -> *mut core::ffi::c_void {
    use std::os::windows::io::AsRawHandle;
    file.as_raw_handle()
}

impl BrokerSession {
    fn exchange(&mut self, request: &Request) -> AppResult<Response> {
        write_frame(&mut self.pipe, request)?;
        match read_frame::<_, Response>(&mut self.pipe)? {
            Response::Error { code, message } => Err(match code.as_str() {
                "policy" => AppError::Policy(message),
                "elevation_required" => AppError::ElevationRequired(message),
                _ => AppError::System(message),
            }),
            response => Ok(response),
        }
    }

    pub fn clean(&mut self, id: &str) -> AppResult<CleanupResult> {
        match self.exchange(&Request::Clean { id: id.into() })? {
            Response::Cleaned { outcome } => Ok(outcome),
            _ => Err(unexpected_reply()),
        }
    }

    pub fn move_startup(&mut self, key: RunKey, name: &str, enabled: bool) -> AppResult<()> {
        let request = Request::MoveStartup {
            key,
            name: name.into(),
            enabled,
        };
        match self.exchange(&request)? {
            Response::Moved => Ok(()),
            _ => Err(unexpected_reply()),
        }
    }
}

fn unexpected_reply() -> AppError {
    AppError::System("the elevated helper sent an unexpected reply".into())
}

impl SettingWriter for BrokerSession {
    fn write(
        &mut self,
        target: &RegTarget,
        value: Option<&RegValue>,
    ) -> AppResult<Option<RegValue>> {
        let request = Request::Write {
            target: target.clone(),
            value: value.cloned(),
        };
        match self.exchange(&request)? {
            Response::Written { read_back } => Ok(read_back),
            _ => Err(unexpected_reply()),
        }
    }
}

impl Drop for BrokerSession {
    fn drop(&mut self) {
        let _ = write_frame(&mut self.pipe, &Request::Done);
        if let Some(process) = &self.process {
            // SAFETY: valid process handle kept alive by the Arc.
            unsafe {
                WaitForSingleObject(process.0, EXIT_TIMEOUT_MS);
            }
        }
    }
}

/// Production elevation: in-process when already elevated or when no HKLM target is involved.
pub struct BrokerElevation;

impl crate::engine::service::Elevation for BrokerElevation {
    fn writer<'a>(
        &'a self,
        store: &'a dyn SettingStore,
        needs_admin: bool,
    ) -> AppResult<Box<dyn SettingWriter + 'a>> {
        if needs_admin && !crate::system::privilege::is_elevated() {
            Ok(Box::new(BrokerSession::launch()?))
        } else {
            Ok(Box::new(LocalWriter(store)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::engine::setting::Hive;
    use crate::engine::store::MemoryStore;
    use crate::engine::tests::{ADMIN_SETTINGS, meta, tweak};
    use crate::engine::{Permission, TweakMeta};

    /// In-memory duplex stream: reads from `input`, collects writes in `output`.
    struct Duplex {
        input: VecDeque<u8>,
        output: Vec<u8>,
    }
    impl Read for Duplex {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.input.read(buf)
        }
    }
    impl Write for Duplex {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn registry() -> TweakRegistry {
        let mut r = TweakRegistry::default();
        r.register(tweak(
            TweakMeta {
                permission: Permission::Administrator,
                ..meta("power.admin")
            },
            ADMIN_SETTINGS,
        ))
        .unwrap();
        r
    }

    #[test]
    fn frames_round_trip_and_reject_oversize() {
        let mut buf = Vec::new();
        write_frame(&mut buf, &Request::Done).unwrap();
        let back: Request = read_frame(&mut buf.as_slice()).unwrap();
        assert!(matches!(back, Request::Done));

        let mut huge = (MAX_FRAME + 1).to_le_bytes().to_vec();
        huge.extend([0u8; 8]);
        assert!(read_frame::<_, Request>(&mut huge.as_slice()).is_err());
    }

    #[test]
    fn writes_only_declared_unprotected_targets() {
        let store = MemoryStore::default();
        let r = registry();
        let declared = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\2VX-Test", "Admin");
        assert_eq!(
            handle_write(&r, &store, &declared, Some(&RegValue::Dword(2))).unwrap(),
            Some(RegValue::Dword(2))
        );

        let undeclared = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\Elsewhere", "X");
        assert!(matches!(
            handle_write(&r, &store, &undeclared, Some(&RegValue::Dword(1))),
            Err(AppError::Policy(_))
        ));
        let protected = RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\WinDefend",
            "Start",
        );
        assert!(matches!(
            handle_write(&r, &store, &protected, Some(&RegValue::Dword(4))),
            Err(AppError::Policy(_))
        ));
        assert_eq!(store.get(&undeclared), None);
    }

    #[test]
    fn serve_answers_each_request_until_done() {
        let mut input = Vec::new();
        let target = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\2VX-Test", "Admin");
        write_frame(
            &mut input,
            &Request::Write {
                target: target.clone(),
                value: Some(RegValue::Dword(2)),
            },
        )
        .unwrap();
        write_frame(
            &mut input,
            &Request::Write {
                target: RegTarget::new(Hive::LocalMachine, r"SOFTWARE\Other", "X"),
                value: None,
            },
        )
        .unwrap();
        write_frame(&mut input, &Request::Done).unwrap();
        // Anything after Done is ignored.
        write_frame(
            &mut input,
            &Request::Write {
                target: target.clone(),
                value: None,
            },
        )
        .unwrap();

        let mut stream = Duplex {
            input: input.into(),
            output: vec![],
        };
        let store = MemoryStore::default();
        serve(&mut stream, &registry(), &store).unwrap();

        let mut out = stream.output.as_slice();
        assert!(matches!(
            read_frame(&mut out).unwrap(),
            Response::Written {
                read_back: Some(RegValue::Dword(2))
            }
        ));
        assert!(
            matches!(read_frame(&mut out).unwrap(), Response::Error { ref code, .. } if code == "policy")
        );
        assert!(out.is_empty());
        assert_eq!(store.get(&target), Some(RegValue::Dword(2)));
    }

    /// Both ends over a real named pipe in one process: exercises the pipe flags, the
    /// identification-level client, both PID checks and the frame protocol. Only the UAC
    /// launch itself (ShellExecuteExW "runas") is not covered.
    #[test]
    fn real_pipe_session_round_trip() {
        let name = HSTRING::from(format!("{PIPE_PREFIX}{}", new_plan_id().unwrap()).as_str());
        let server = create_server(&name).unwrap();
        // FILE_FLAG_FIRST_PIPE_INSTANCE: nobody can create (or squat) the same name again.
        assert!(create_server(&name).is_err());

        let pid = std::process::id();
        let client_name = name.to_string_lossy();
        let client = std::thread::spawn(move || {
            let mut stream = connect_to_parent(&client_name, pid).unwrap();
            serve(&mut stream, &registry(), &MemoryStore::default()).unwrap();
        });
        accept(&server, pid).unwrap();

        let mut session = BrokerSession {
            pipe: server,
            process: None,
        };
        let target = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\2VX-Test", "Admin");
        assert_eq!(
            session.write(&target, Some(&RegValue::Dword(2))).unwrap(),
            Some(RegValue::Dword(2))
        );
        let undeclared = RegTarget::new(Hive::LocalMachine, r"SOFTWARE\Other", "X");
        assert!(matches!(
            session.write(&undeclared, None),
            Err(AppError::Policy(_))
        ));
        drop(session); // sends Done
        client.join().unwrap();
    }

    #[test]
    fn rejects_a_server_that_is_not_the_parent() {
        let name = HSTRING::from(format!("{PIPE_PREFIX}{}", new_plan_id().unwrap()).as_str());
        let _server = create_server(&name).unwrap();
        assert!(matches!(
            connect_to_parent(&name.to_string_lossy(), std::process::id() + 1),
            Err(AppError::Policy(_))
        ));
    }

    #[test]
    fn rejects_an_unexpected_client() {
        let name = HSTRING::from(format!("{PIPE_PREFIX}{}", new_plan_id().unwrap()).as_str());
        let server = create_server(&name).unwrap();
        let pid = std::process::id();
        let client_name = name.to_string_lossy();
        let client = std::thread::spawn(move || connect_to_parent(&client_name, pid));
        assert!(matches!(accept(&server, pid + 1), Err(AppError::Policy(_))));
        let _ = client.join();
    }

    #[test]
    fn elevated_entry_rejects_bad_arguments() {
        assert_eq!(run_elevated(&[]), 2);
        assert_eq!(run_elevated(&["x".into(), "1".into()]), 2);
        assert_eq!(
            run_elevated(&[format!("{PIPE_PREFIX}nothex"), "1".into()]),
            2
        );
        assert_eq!(
            run_elevated(&[format!("{PIPE_PREFIX}{}", "a".repeat(32)), "pid".into()]),
            2
        );
    }
}
