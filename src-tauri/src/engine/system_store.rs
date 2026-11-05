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
use windows::Win32::System::TaskScheduler::{
    IExecAction, ILogonTrigger, IRegisteredTask, ITaskFolder, ITaskService, TASK_ACTION_EXEC,
    TASK_ENUM_HIDDEN, TASK_TRIGGER_BOOT, TASK_TRIGGER_LOGON, TASK_TRIGGER_TYPE2, TaskScheduler,
};
use windows::Win32::System::Variant::VARIANT;
use windows::core::{BSTR, GUID, Interface};

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

/// A task that runs at sign-in or start-up, for the Autoruns list. Read only.
pub struct StartTask {
    pub name: String,
    /// `\Folder\Name`.
    pub path: String,
    pub enabled: bool,
    /// The program, as the task stores it.
    pub program: String,
    pub arguments: String,
    /// It runs for one account, rather than for whoever signs in.
    pub for_one_user: bool,
}

/// What a walk of the task folders found, and the folders it couldn't read.
#[derive(Default)]
pub struct StartTasks {
    pub tasks: Vec<StartTask>,
    pub unreadable: Vec<String>,
}

impl StartTasks {
    fn note<F: TaskFolder>(&mut self, folder: &F, e: &windows::core::Error) {
        let path = folder.path();
        let path = path.trim_end_matches('\\');
        self.unreadable
            .push(format!("Task Scheduler{path}: {}", e.message()));
    }
}

/// Every task outside `\Microsoft` that starts with Windows or at sign-in. Windows' own tasks stay
/// out: there are hundreds, and they aren't what someone looking at autoruns is after. Tasks this
/// account can't read aren't returned by Task Scheduler at all, but a folder can fail outright, and
/// that costs only what's in it.
pub fn start_tasks() -> windows::core::Result<StartTasks> {
    let service = task_service()?;
    // SAFETY: `service` is a live, connected interface.
    let root = unsafe { service.GetFolder(&BSTR::from(r"\")) }?;
    let mut found = StartTasks::default();
    collect_start_tasks(&root, true, &mut found);
    Ok(found)
}

/// The little the walk asks of a folder, so that tests can hand it a tree with bad folders in it.
trait TaskFolder: Sized {
    fn path(&self) -> String;
    fn name(&self) -> windows::core::Result<String>;
    fn start_tasks(&self) -> windows::core::Result<Vec<StartTask>>;
    fn children(&self) -> windows::core::Result<Vec<windows::core::Result<Self>>>;
}

impl TaskFolder for ITaskFolder {
    fn path(&self) -> String {
        // SAFETY: a live interface.
        unsafe { self.Path() }.map_or_else(|_| String::new(), |path| path.to_string())
    }

    fn name(&self) -> windows::core::Result<String> {
        // SAFETY: a live interface.
        Ok(unsafe { self.Name() }?.to_string())
    }

    fn start_tasks(&self) -> windows::core::Result<Vec<StartTask>> {
        // SAFETY: live interfaces. Task Scheduler's collections are numbered from 1.
        unsafe {
            let tasks = self.GetTasks(TASK_ENUM_HIDDEN.0)?;
            let mut found = vec![];
            for index in 1..=tasks.Count()? {
                if let Ok(task) = tasks.get_Item(&VARIANT::from(index)) {
                    found.extend(start_task(&task));
                }
            }
            Ok(found)
        }
    }

    fn children(&self) -> windows::core::Result<Vec<windows::core::Result<Self>>> {
        // SAFETY: live interfaces. Task Scheduler's collections are numbered from 1.
        unsafe {
            let folders = self.GetFolders(0)?;
            Ok((1..=folders.Count()?)
                .map(|index| folders.get_Item(&VARIANT::from(index)))
                .collect())
        }
    }
}

fn collect_start_tasks<F: TaskFolder>(folder: &F, root: bool, found: &mut StartTasks) {
    match folder.start_tasks() {
        Ok(tasks) => found.tasks.extend(tasks),
        Err(e) => found.note(folder, &e),
    }
    let children = match folder.children() {
        Ok(children) => children,
        Err(e) => {
            found.note(folder, &e);
            return;
        }
    };
    for child in children {
        match child.and_then(|child| Ok((child.name()?, child))) {
            Ok((name, _)) if root && name.eq_ignore_ascii_case("Microsoft") => {}
            Ok((_, child)) => collect_start_tasks(&child, false, found),
            Err(e) => found.note(folder, &e),
        }
    }
}

/// `None` for a task with no start trigger or nothing to run, or one this account can't read.
fn start_task(task: &IRegisteredTask) -> Option<StartTask> {
    // SAFETY: live interfaces, valid out pointers, and collections numbered from 1.
    unsafe {
        let definition = task.Definition().ok()?;
        let triggers = definition.Triggers().ok()?;
        let mut count = 0;
        triggers.Count(&mut count).ok()?;
        let mut starts = false;
        let mut for_one_user = false;
        for index in 1..=count {
            let trigger = triggers.get_Item(index).ok()?;
            let mut kind = TASK_TRIGGER_TYPE2::default();
            trigger.Type(&mut kind).ok()?;
            if kind == TASK_TRIGGER_BOOT {
                starts = true;
            } else if kind == TASK_TRIGGER_LOGON {
                starts = true;
                let mut user = BSTR::new();
                if let Ok(logon) = trigger.cast::<ILogonTrigger>() {
                    let _ = logon.UserId(&mut user);
                }
                for_one_user |= !user.is_empty();
            }
        }
        if !starts {
            return None;
        }
        let actions = definition.Actions().ok()?;
        actions.Count(&mut count).ok()?;
        let mut program = None;
        for index in 1..=count {
            let action = actions.get_Item(index).ok()?;
            let mut kind = Default::default();
            action.Type(&mut kind).ok()?;
            if kind == TASK_ACTION_EXEC {
                program = action.cast::<IExecAction>().ok();
                break;
            }
        }
        let program = program?;
        let (mut path, mut arguments) = (BSTR::new(), BSTR::new());
        program.Path(&mut path).ok()?;
        let _ = program.Arguments(&mut arguments);
        Some(StartTask {
            name: task.Name().ok()?.to_string(),
            path: task.Path().ok()?.to_string(),
            enabled: task.Enabled().ok()?.as_bool(),
            program: path.to_string(),
            arguments: arguments.to_string(),
            for_one_user,
        })
    }
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

    /// A folder as the walk sees it. `None` stands for a call that fails.
    #[derive(Clone)]
    struct Fake {
        path: &'static str,
        tasks: Option<Vec<&'static str>>,
        children: Option<Vec<Option<Fake>>>,
    }

    impl Fake {
        fn new(path: &'static str, tasks: &[&'static str], children: Vec<Option<Fake>>) -> Self {
            Fake {
                path,
                tasks: Some(tasks.to_vec()),
                children: Some(children),
            }
        }
    }

    fn denied() -> windows::core::Error {
        windows::Win32::Foundation::E_ACCESSDENIED.into()
    }

    impl TaskFolder for Fake {
        fn path(&self) -> String {
            self.path.to_owned()
        }
        fn name(&self) -> windows::core::Result<String> {
            Ok(self.path.rsplit('\\').next().unwrap_or("").to_owned())
        }
        fn start_tasks(&self) -> windows::core::Result<Vec<StartTask>> {
            let names = self.tasks.as_ref().ok_or_else(denied)?;
            Ok(names
                .iter()
                .map(|name| StartTask {
                    name: (*name).to_owned(),
                    path: format!(r"{}\{name}", self.path),
                    enabled: true,
                    program: r"C:\x\run.exe".to_owned(),
                    arguments: String::new(),
                    for_one_user: false,
                })
                .collect())
        }
        fn children(&self) -> windows::core::Result<Vec<windows::core::Result<Self>>> {
            let children = self.children.as_ref().ok_or_else(denied)?;
            Ok(children
                .iter()
                .map(|child| child.clone().ok_or_else(denied))
                .collect())
        }
    }

    #[test]
    fn a_bad_task_folder_costs_only_itself() {
        let kept = Fake::new(r"\NoTasks\Kept", &["below"], vec![]);
        let no_tasks = Fake {
            tasks: None,
            ..Fake::new(r"\NoTasks", &[], vec![Some(kept)])
        };
        let no_folders = Fake {
            children: None,
            ..Fake::new(r"\NoFolders", &["beside"], vec![])
        };
        let root = Fake::new(
            r"\",
            &["top"],
            vec![
                Some(Fake::new(r"\Good", &["good"], vec![])),
                Some(no_tasks),
                Some(no_folders),
                None,
                Some(Fake::new(r"\Microsoft", &["windows"], vec![])),
                Some(Fake::new(r"\After", &["after"], vec![])),
            ],
        );

        let mut found = StartTasks::default();
        collect_start_tasks(&root, true, &mut found);

        let names: Vec<_> = found.tasks.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["top", "good", "below", "beside", "after"]);
        assert_eq!(found.unreadable.len(), 3, "{:?}", found.unreadable);
        assert!(found.unreadable[0].starts_with(r"Task Scheduler\NoTasks: "));
        assert!(found.unreadable[1].starts_with(r"Task Scheduler\NoFolders: "));
        // A child that can't be fetched has no name of its own, so its parent stands in.
        assert!(found.unreadable[2].starts_with("Task Scheduler: "));
    }

    #[test]
    fn only_the_top_microsoft_folder_is_left_out() {
        let theirs = Fake::new(r"\Vendor\Microsoft", &["theirs"], vec![]);
        let vendor = Fake::new(r"\Vendor", &[], vec![Some(theirs)]);
        let root = Fake::new(r"\", &[], vec![Some(vendor)]);
        let mut found = StartTasks::default();
        collect_start_tasks(&root, true, &mut found);
        assert_eq!(found.tasks.len(), 1);
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
