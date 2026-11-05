//! Project policy guards (docs/security.md §"Commitments"). These read the source tree and
//! fail the build when a change would break a commitment, so it cannot slip in unnoticed.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap()
}

/// A source file without its tests, whose fixtures hold what the code must not do.
fn code(rel: &str) -> String {
    let text = read(rel);
    let end = text.find("#[cfg(test)]").unwrap_or(text.len());
    text[..end].to_string()
}

/// String literals in a `[ ... ]` list that follows `marker`.
fn quoted_list_after(text: &str, marker: &str) -> Vec<String> {
    let start = text
        .find(marker)
        .unwrap_or_else(|| panic!("`{marker}` not found"));
    let rest = &text[start..];
    let open = rest.find('[').unwrap();
    let close = rest[open..].find(']').unwrap() + open;
    rest[open + 1..close]
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// The variants of an enum, read from its source, so that one added later can't go unseen by a list
/// someone has to remember to update.
fn enum_variants<'a>(source: &'a str, marker: &str) -> Vec<&'a str> {
    let variants: Vec<&str> = source
        .lines()
        .skip_while(|line| !line.contains(marker))
        .skip(1)
        .take_while(|line| *line != "}")
        .filter_map(|line| line.strip_prefix("    "))
        .filter(|line| line.starts_with(|c: char| c.is_ascii_uppercase()))
        .map(|line| line.trim_end_matches([' ', '{', ',']))
        .collect();
    assert!(!variants.is_empty(), "`{marker}` not found");
    variants
}

/// Source with its string literals removed. The Library's tables hold real setting names, and some
/// contain a forbidden word (`POWERSHELL_TELEMETRY_OPTOUT`, or the "wmic" inside `ShowMicrosoftRewards`).
/// They are data, so only the code around them needs scanning.
fn without_strings(source: &str) -> String {
    let mut code = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut previous = ' ';
    while let Some(c) = chars.next() {
        // The quote in `'"'` and `'\"'` opens nothing.
        let in_char = previous == '\\' || (previous == '\'' && chars.peek() == Some(&'\''));
        if c != '"' || in_char {
            code.push(c);
            previous = c;
            continue;
        }
        let hashed = previous == '#';
        let raw = previous == 'r' || hashed;
        while let Some(s) = chars.next() {
            match s {
                '\\' if !raw => {
                    chars.next();
                }
                '"' if !hashed || chars.peek() == Some(&'#') => break,
                _ => {}
            }
        }
        previous = '"';
    }
    code
}

#[test]
fn blanking_strings_keeps_the_code() {
    let source =
        r#"name: "powershell", key: r"SOFTWARE\X", run: Command::new("cmd.exe"), s: "a\"b""#;
    let code = without_strings(source).to_ascii_lowercase();
    assert!(code.contains("command::new"));
    assert!(!code.contains("powershell") && !code.contains("cmd.exe"));
}

#[test]
fn a_quote_in_a_char_or_a_hashed_string_doesnt_flip_what_counts_as_code() {
    let source = r##"a.split('"'); Command::new("x"); b.split('\"'); run("cmd.exe"); c = r#"say "powershell" now"#; winexec();"##;
    let code = without_strings(source).to_ascii_lowercase();
    assert!(code.contains("command::new") && code.contains("winexec"));
    assert!(!code.contains("cmd.exe") && !code.contains("powershell"));
}

/// `startup.rs` names the programs that only launch something else, to say nothing about who made an
/// entry that runs through one. Strings there are data for the same reason as the Library's tables,
/// and this keeps it true that they're only ever compared with a program's name.
#[test]
fn launcher_names_are_only_compared() {
    let startup = code("src/engine/startup.rs");
    assert_eq!(
        startup.matches("LAUNCHERS").count(),
        2,
        "the table and its one use"
    );
    assert!(startup.contains("LAUNCHERS.iter().any(|l| name.eq_ignore_ascii_case(l))"));
}

/// No shell, no script host, no process spawning: the backend changes the system only
/// through typed primitives. There are two exceptions, each in one file: the elevated broker
/// relaunches this executable via ShellExecuteExW, and `links.rs` hands Windows one entry of a fixed
/// list via ShellExecuteW, which can't elevate (docs/security.md, revised 2026-11-05).
#[test]
fn backend_never_spawns_shells_or_processes() {
    let forbidden = [
        "std::process::Command",
        "Command::new",
        "powershell",
        "cmd.exe",
        "cmd /c",
        "reg.exe",
        "regedit",
        "bcdedit",
        "schtasks",
        "sc.exe",
        "wmic",
        "nsudo",
        "createprocess",
        "winexec",
        "shellexecutea",
        "shellexecuteexa",
        "mshta",
        "wscript",
        "cscript",
        "rundll32",
        "regsvr32",
    ];
    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap().to_ascii_lowercase();
        let table =
            file.parent().is_some_and(|dir| dir.ends_with("library")) && !file.ends_with("mod.rs");
        let data = table || file.ends_with("startup.rs");
        let text = if data { without_strings(&text) } else { text };
        for needle in forbidden {
            assert!(
                !text.contains(&needle.to_ascii_lowercase()),
                "{} contains forbidden `{needle}`",
                file.display()
            );
        }
        // `shellexecutew` isn't inside `shellexecuteexw`, so each only passes in its own file.
        assert_eq!(
            text.contains("shellexecuteexw"),
            file.ends_with("broker.rs"),
            "{}: ShellExecuteExW is the broker's alone",
            file.display()
        );
        assert_eq!(
            text.contains("shellexecutew"),
            file.ends_with("links.rs"),
            "{}: ShellExecuteW is links.rs's alone",
            file.display()
        );
        if !file.ends_with("broker.rs") {
            assert!(
                !text.contains("shellexecuteex") && !text.contains("runas"),
                "{} can elevate",
                file.display()
            );
        }
    }
}

/// `open_tool` starts programs, so its whole reach is this list. A tool is a bare name looked up in
/// System32 and never a path, with no way to choose an argument and no elevating verb.
#[test]
fn windows_tools_are_a_fixed_list_of_bare_names() {
    use twovx_tweaks_lib::links::WindowsTool;

    // Read from the source: `ALL` is kept by hand, so a variant left out of it would go unchecked.
    let tools = [
        "ResourceMonitor",
        "ReliabilityMonitor",
        "SystemInformation",
        "DirectxDiagnostic",
        "DiskCleanup",
        "EventViewer",
        "TaskManager",
        "DeviceManager",
        "ServicesConsole",
        "NetworkConnections",
    ];
    let links = code("src/links.rs");
    assert_eq!(
        enum_variants(&links, "pub enum WindowsTool {"),
        tools,
        "a new tool needs a review here and in docs/security.md"
    );
    let all = &links[links.find("pub const ALL").unwrap()..];
    let all = &all[all.find("= [").unwrap()..all.find("];").unwrap()];
    let listed: Vec<&str> = all
        .split("Self::")
        .skip(1)
        .map(|tool| tool.trim_end_matches(|c: char| !c.is_alphanumeric()))
        .collect();
    assert_eq!(listed, tools, "`WindowsTool::ALL` lists every tool");

    let files = WindowsTool::ALL.map(|tool| tool.command().file);
    assert_eq!(
        files,
        [
            "resmon.exe",
            "perfmon.exe",
            "msinfo32.exe",
            "dxdiag.exe",
            "cleanmgr.exe",
            "eventvwr.msc",
            "taskmgr.exe",
            "devmgmt.msc",
            "services.msc",
            "ncpa.cpl",
        ],
        "a new tool needs a review here and in docs/security.md"
    );
    for tool in WindowsTool::ALL {
        let command = tool.command();
        assert!(
            !command
                .file
                .contains(['\\', '/', ':', '"', ' ', '%', '*', '?']),
            "{} isn't a bare name",
            command.file
        );
        assert!(matches!(
            command.file.rsplit_once('.'),
            Some((_, "exe" | "msc" | "cpl"))
        ));
        assert!(matches!(command.verb, "open" | "cplopen"), "{tool:?}");
        assert!(matches!(command.arguments, None | Some("/rel")), "{tool:?}");
    }
    assert!(
        links.contains("GetSystemDirectoryW") && !links.contains("GetEnvironmentVariable"),
        "tools resolve under the real System32, not a variable"
    );
    // Script hosts run whatever they're handed, so they can't join the list however it's written.
    let lower = links.to_ascii_lowercase();
    for host in [
        "mshta",
        "wscript",
        "cscript",
        "rundll32",
        "regsvr32",
        "cmd",
        "pwsh",
        "msiexec",
        "bitsadmin",
        "certutil",
        "forfiles",
    ] {
        assert!(
            !lower.contains(&format!("{host}.exe")),
            "links.rs names `{host}`"
        );
    }
}

/// A `#[tauri::command]`'s name and its parameters as (name, type), in every attribute form.
fn commands_in(source: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut found = vec![];
    let mut rest = source;
    while let Some(start) = rest.find("tauri::command") {
        rest = &rest[start..];
        let after = &rest[rest.find("fn ").unwrap() + 3..];
        let open = after.find('(').unwrap();
        let name = after[..open].trim();
        assert!(!name.contains('<'), "{name} has generics");
        let (params, end) = parameters(&after[open + 1..]);
        found.push((name.to_string(), params));
        rest = &after[open + 1 + end..];
    }
    found
}

/// Splits at the top level, up to the bracket that closes the list. Returns where it stopped.
fn parameters(list: &str) -> (Vec<(String, String)>, usize) {
    let mut params = vec![];
    let (mut depth, mut from) = (0, 0);
    for (at, c) in list.char_indices() {
        match c {
            '(' | '<' | '[' => depth += 1,
            ')' if depth == 0 => {
                params.extend(parameter(&list[from..at]));
                return (params, at);
            }
            ')' | '>' | ']' => depth -= 1,
            ',' if depth == 0 => {
                params.extend(parameter(&list[from..at]));
                from = at + 1;
            }
            _ => {}
        }
    }
    panic!("unclosed parameter list");
}

fn parameter(text: &str) -> Option<(String, String)> {
    let (name, kind) = text.trim().split_once(':')?;
    Some((name.trim().to_string(), kind.trim().to_string()))
}

#[test]
fn every_form_of_command_is_read() {
    let source = r#"
        #[tauri::command]
        pub fn plain(link: Link) -> AppResult<()> { work((1, 2)) }
        #[tauri::command(async)]
        pub fn spun(file: String, map: HashMap<String, (u8, u8)>) {}
        #[tauri::command(rename_all = "snake_case")]
        pub async fn renamed(
            engine: State<'_, Arc<Engine>>,
            on_progress: Channel<CleanupProgress>,
        ) -> AppResult<()> { Ok(()) }
    "#;
    let found = commands_in(source);
    let names: Vec<&str> = found.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(names, ["plain", "spun", "renamed"]);
    assert_eq!(found[1].1[0], ("file".to_string(), "String".to_string()));
    assert_eq!(found[1].1[1].1, "HashMap<String, (u8, u8)>");
    assert_eq!(found[2].1[0].1, "State<'_, Arc<Engine>>");
    assert_eq!(found[2].1.len(), 2);
}

/// What a command may take. A name is on this list with its type, so a `path`, `url`, `file`, `dir`,
/// `target: String` or `command` can't arrive under any spelling, and every `String` here is an ID
/// checked against a fixed set or a report the backend only ever hands to the save dialog.
const COMMAND_PARAMETERS: &[(&str, &str)] = &[
    ("engine", "State<'_, Arc<Engine>>"),
    ("log", "State<'_, LogState>"),
    ("sampler", "State<'_, MetricsSampler>"),
    ("app", "AppHandle"),
    ("window", "WebviewWindow"),
    ("on_progress", "Channel<CleanupProgress>"),
    ("enabled", "bool"),
    ("limit", "Option<u32>"),
    ("since_ms", "Option<u64>"),
    ("snapshot_id", "i64"),
    ("link", "Link"),
    ("tool", "WindowsTool"),
    ("target", "PingTarget"),
    ("level", "FrontendLogLevel"),
    ("id", "String"),
    ("ids", "Vec<String>"),
    ("app_id", "String"),
    ("plan_id", "String"),
    ("tweak_id", "String"),
    ("tweak_ids", "Vec<String>"),
    ("email", "String"),
    ("text", "String"),
    ("message", "String"),
    ("source", "Option<String>"),
];

/// A command that took a path could read or write anywhere. The report's file is the one the user
/// picks in the dialog, and the log folder comes from the backend's own state.
#[test]
fn commands_never_take_a_path() {
    let mut files = vec![];
    rust_sources(&root().join("src/commands"), &mut files);
    let mut names = vec![];
    for file in files {
        let source = fs::read_to_string(&file).unwrap();
        for (name, params) in commands_in(&source) {
            for (param, kind) in &params {
                assert!(
                    COMMAND_PARAMETERS.contains(&(param.as_str(), kind.as_str())),
                    "{name} in {} takes `{param}: {kind}`, which isn't on the reviewed list",
                    file.display()
                );
            }
            names.push(name);
        }
    }
    // Every command was seen above: none is written in a form the parser doesn't know.
    let mut manifest = quoted_list_after(&read("build.rs"), ".commands(&");
    names.sort();
    manifest.sort();
    assert_eq!(names, manifest, "commands in src/commands and in build.rs");

    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        let in_commands = file.parent().is_some_and(|dir| dir.ends_with("commands"));
        assert!(
            in_commands || !text.contains("tauri::command"),
            "{}: commands live in src/commands",
            file.display()
        );
        for short in ["#[command", "use tauri::command", "tauri::{command"] {
            assert!(!text.contains(short), "{}: `{short}`", file.display());
        }
    }

    let commands = read("src/commands/system.rs");
    assert!(commands.contains("export_report(window: WebviewWindow, text: String)"));
    assert!(commands.contains("let log_dir = log.log_dir.clone();"));
    assert!(commands.contains("links::open_log_folder(&log_dir)"));
    let report = code("src/report.rs");
    assert!(report.contains("MAX_REPORT_BYTES: usize = 2 * 1024 * 1024"));
    assert!(report.contains("FileSaveDialog"));
}

/// Whether `name` appears as a call of its own. `connect(` has to be whole: Task Scheduler has a
/// `.Connect(` of its own, and `disconnect(` is no socket.
fn calls(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(at, _)| {
        !text[..at]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.')
    })
}

#[test]
fn a_call_is_told_from_a_longer_name() {
    assert!(calls("let s = socket(af, kind, 0);", "socket("));
    assert!(calls("winsock::connect(s, name, size)", "connect("));
    assert!(!calls("service.connect(&none)", "connect("));
    assert!(!calls("disconnect(x); my_socket(y)", "connect("));
    assert!(!calls("my_socket(y)", "socket("));
}

/// Besides the updater, the app's traffic is ICMP echoes to two fixed targets, only ever started by
/// the user: one call sends one, and a bufferbloat test makes one call a second to each target until
/// it ends. Nothing else in the backend can open a connection.
#[test]
fn ping_is_the_only_traffic_besides_the_updater() {
    let ping = code("src/system/ping.rs");
    assert_eq!(
        ping.matches("IcmpSendEcho2(").count(),
        1,
        "one call, in one place"
    );
    assert!(ping.contains("const INTERNET: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);"));
    assert!(
        !ping.to_ascii_lowercase().contains("http"),
        "ICMP echo only"
    );
    let target = &ping[ping.find("pub enum PingTarget {").unwrap()..];
    let target = &target[..target.find('}').unwrap()];
    let variants: Vec<&str> = target
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|line| !line.starts_with("//"))
        .collect();
    assert_eq!(variants, ["Gateway,", "Internet,"], "the target is a name");
    // `Cargo.toml` turns on the whole WinSock feature for these two: an address family and an
    // address type for reading the route table. The rest of it stays out.
    assert!(ping.contains("use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_INET};"));
    assert_eq!(
        ping.matches("WinSock").count(),
        1,
        "one import, in one place"
    );

    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = without_strings(&fs::read_to_string(&file).unwrap()).to_ascii_lowercase();
        assert_eq!(
            text.contains("icmpsendecho"),
            file.ends_with("ping.rs"),
            "{}: ICMP belongs to system/ping.rs",
            file.display()
        );
        assert_eq!(
            text.contains("networking::winsock"),
            file.ends_with("ping.rs"),
            "{}: WinSock belongs to system/ping.rs",
            file.display()
        );
        for needle in [
            "tcpstream",
            "tcplistener",
            "udpsocket",
            "wsasocket",
            "wsastartup",
            "wsaconnect",
            "getaddrinfo",
            "icmp6",
            "winhttp",
            "wininet",
            "internetopen",
            "urldownload",
        ] {
            assert!(
                !text.contains(needle),
                "{} contains `{needle}`",
                file.display()
            );
        }
        for call in ["socket(", "connect("] {
            assert!(!calls(&text, call), "{} calls `{call}`", file.display());
        }
    }
}

/// What a startup entry points at isn't ours to trust: it can be a share, where opening the file
/// sends this account's credentials to that host. Its version is only read from a drive in this PC,
/// and a program with a bare name is only looked for in System32.
#[test]
fn version_reads_stay_on_local_drives() {
    let info = code("src/system/file_info.rs");
    let read = &info[info.find("pub fn read(").unwrap()..];
    let guard = read.find("if !is_local_drive(path) {").unwrap();
    assert!(guard < read.find("GetFileVersionInfoSizeW(").unwrap());
    assert!(info.contains("matches!(kind, DRIVE_FIXED | DRIVE_REMOVABLE)"));

    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = without_strings(&fs::read_to_string(&file).unwrap()).to_ascii_lowercase();
        assert_eq!(
            text.contains("getfileversioninfo"),
            file.ends_with("file_info.rs"),
            "{}: version reads belong to system/file_info.rs",
            file.display()
        );
    }

    let startup = code("src/engine/startup.rs");
    assert!(startup.contains("if !is_bare_name(Path::new(&program)) {"));
    assert_eq!(
        startup.matches(".exists()").count(),
        2,
        "System32 for a bare name, and the holding folder"
    );
    assert!(startup.contains("target.exists()"));
}

/// `StartupApproved` belongs to Task Manager (decision P0-8a): read so the app can tell the truth
/// about an entry, and never written, even for the app's own.
#[test]
fn startup_approved_is_only_read() {
    let startup = code("src/engine/startup.rs");
    assert_eq!(
        startup.matches("APPROVED").count(),
        2,
        "the path and its one use"
    );
    assert!(startup.contains(r#"root.open(format!(r"{APPROVED}\{sub}"))"#));
}

/// The services page reads. A service changes only through a tweak's start type, in the engine,
/// which keeps backup, verify and undo. No handle that could do more is ever asked for.
#[test]
fn services_are_only_ever_read() {
    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = without_strings(&fs::read_to_string(&file).unwrap()).to_ascii_lowercase();
        for needle in [
            "changeserviceconfig",
            "controlservice",
            "startservice",
            "deleteservice",
            "createservice",
            "service_change_config",
            "service_start",
            "service_stop",
            "sc_manager_all_access",
            "sc_manager_create_service",
            "service_all_access",
        ] {
            assert!(
                !text.contains(needle),
                "{} contains `{needle}`",
                file.display()
            );
        }
    }
}

/// The sign-in entry is one Run value holding the app's own path. It can't become a service or a
/// task, and the rest of the Autoruns code only moves entries that were already there.
#[test]
fn auto_launch_is_one_run_value() {
    let startup = code("src/engine/startup.rs");
    assert!(startup.contains(r#"const AUTO_LAUNCH_VALUE: &str = "2VX Tweaks";"#));
    assert_eq!(startup.matches(".set_string(").count(), 1);
    assert!(startup.contains("key.set_string(AUTO_LAUNCH_VALUE, command)"));
    assert!(
        startup.contains("format!(\"\\\"{}\\\"\", exe.display())"),
        "quoted, with no arguments"
    );

    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = without_strings(&fs::read_to_string(&file).unwrap()).to_ascii_lowercase();
        for needle in ["registertask", "itaskfolder::create", "createfolder"] {
            assert!(
                !text.contains(needle),
                "{} contains `{needle}`",
                file.display()
            );
        }
    }
}

/// Rule 10's exception: documented NDIS keywords only, each off by a documented value and each
/// needing a restart. The list is repeated here so that adding to it takes a second, deliberate edit.
#[test]
fn adapter_tweaks_stay_on_the_documented_keywords() {
    use twovx_tweaks_lib::engine::policy::{ETHERNET_KEYWORDS, WIFI_KEYWORDS};
    use twovx_tweaks_lib::engine::{
        Evidence, Hive, RegValue, RestartRequirement, TweakRegistry, policy,
    };

    let ethernet = [
        "*EEE",
        "*FlowControl",
        "*InterruptModeration",
        "*WakeOnMagicPacket",
        "*WakeOnPattern",
        "*PMARPOffload",
        "*PMNSOffload",
        "*LsoV2IPv4",
        "*LsoV2IPv6",
        "*SelectiveSuspend",
    ];
    let wifi = [
        "*WakeOnMagicPacket",
        "*WakeOnPattern",
        "*PMARPOffload",
        "*PMNSOffload",
        "*PacketCoalescing",
    ];
    assert_eq!(ETHERNET_KEYWORDS, ethernet);
    assert_eq!(WIFI_KEYWORDS, wifi);

    let registry = TweakRegistry::with_builtin_tweaks();
    let mut found = 0;
    // Retired ones still count: undo can still write what they wrote.
    let retired = registry.retired().filter_map(|m| registry.undoable(m.id));
    for tweak in registry.iter().chain(retired) {
        for setting in tweak.settings() {
            if setting.target.hive != Hive::Adapter {
                continue;
            }
            found += 1;
            let meta = tweak.meta();
            let keywords: &[&str] = match setting.target.key.as_ref() {
                "Ethernet" => &ethernet,
                "Wifi" => &wifi,
                other => panic!("{other} isn't an adapter kind"),
            };
            assert!(keywords.contains(&setting.target.value.as_ref()));
            assert_eq!(setting.value, Some(RegValue::String("0".into())));
            assert_eq!(meta.evidence, Evidence::Documented, "{}", meta.id);
            assert_eq!(meta.restart, RestartRequirement::Reboot, "{}", meta.id);
            assert!(!meta.recommended, "{}", meta.id);
            assert!(
                meta.requires.is_some(),
                "{}: the driver has to list it",
                meta.id
            );
            assert!(
                meta.references
                    .iter()
                    .all(|r| r.starts_with("https://learn.microsoft.com/")),
                "{}",
                meta.id
            );
            policy::check_value(setting).unwrap();
        }
    }
    assert_eq!(found, ethernet.len() + wifi.len());

    // Only a driver key: a keyword can't be aimed at anything else, whatever the hive resolves to.
    let setting = read("src/engine/setting.rs");
    assert!(setting.contains("    Adapter,"));
    let devices = code("src/system/devices.rs");
    let resolve = &devices[devices.find("pub fn adapter_key").unwrap()..];
    assert!(resolve.contains(r#"format!(r"{CLASS}\{NET_CLASS}\{instance}")"#));
}

/// 2VX changes values, not keys. Undoing the classic right-click menu is the one exception, because
/// that key's presence is the setting. It stays one named key, and only an empty one is removed.
#[test]
fn only_the_classic_menu_key_is_ever_removed() {
    let store = code("src/engine/store.rs");
    assert_eq!(store.matches("remove_tree(").count(), 1);
    assert!(store.contains("keys.is_none() && values.is_none()"));
    let keys = &store[store.find("const PRESENCE_KEYS").unwrap()..];
    let keys = &keys[..keys.find("];").unwrap()];
    assert_eq!(keys.matches('"').count(), 2, "one key");
    assert!(
        keys.contains(
            r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32"
        )
    );
    for path in [
        "src/engine/service.rs",
        "src/engine/system_store.rs",
        "src/engine/broker.rs",
    ] {
        assert!(!code(path).contains("remove_tree("), "{path}");
    }
}

/// The UAC switches share a key with two harmless values, which pass one at a time.
#[test]
fn the_uac_key_opens_for_two_named_values_only() {
    let policy = code("src/engine/policy.rs");
    let allowed = &policy[policy.find("const ALLOWED_VALUES").unwrap()..];
    let allowed = &allowed[..allowed.find("];").unwrap()];
    assert!(allowed.contains("\"EnableFirstLogonAnimation\""));
    assert!(allowed.contains("\"VerboseStatus\""));
    assert_eq!(allowed.matches("Hive::").count(), 2);
}

/// Hardware details never carry a serial number. The parsers step over those fields by offset, so
/// this looks for any read of them, and `Resolve` is kept out because it can rewrite a shortcut.
#[test]
fn hardware_reads_stay_clear_of_identifiers() {
    let details = code("src/system/hardware_details.rs");
    assert!(
        !details.contains("SerialNumberOffset)"),
        "the storage descriptor's serial"
    );
    for offset in ["0x18", "0x19"] {
        for reader in ["byte", "word", "dword", "text"] {
            let read = format!("{reader}({offset})");
            assert!(
                !details.contains(&read),
                "SMBIOS Type 17 serial or asset tag"
            );
        }
    }

    let mut files = vec![];
    rust_sources(&root().join("src"), &mut files);
    for file in files {
        let text = without_strings(&fs::read_to_string(&file).unwrap()).to_ascii_lowercase();
        assert_eq!(
            text.contains("serialnumber"),
            file.ends_with("hardware_details.rs"),
            "{} touches a serial number",
            file.display()
        );
    }
    assert!(
        !read("src/system/file_info.rs").contains(".Resolve("),
        "reading a shortcut must not search the disk or rewrite it"
    );
}

/// The elevated side stays a short, typed list. A new request needs its own review here and in
/// docs/security.md, and none of them may carry a path, a command or a value to restore.
#[test]
fn broker_vocabulary_is_fixed() {
    let source = read("src/engine/broker.rs");
    assert_eq!(
        enum_variants(&source, "pub enum Request {"),
        ["Write", "Clean", "MoveStartup", "Done"]
    );
}

/// No network clients, analytics, crash reporters or remote-control crates. The updater
/// plugin is the one reviewed exception (docs/security.md, decision P0-8b revised); its own
/// guard is `updater_is_signed_https_and_opt_in`.
#[test]
fn no_network_or_telemetry_dependencies() {
    let manifest = read("Cargo.toml");
    let deps = &manifest[manifest.find("[dependencies]").unwrap()..];
    for name in [
        "reqwest",
        "ureq",
        "hyper",
        "isahc",
        "surf",
        "attohttpc",
        "curl",
        "tauri-plugin-http",
        "tauri-plugin-shell",
        "socket2",
        "sentry",
        "posthog",
        "segment",
        "machine-uid",
        "machineid",
    ] {
        assert!(
            !deps.lines().any(|l| l.trim_start().starts_with(name)),
            "dependency `{name}` needs a security review and a docs/security.md update first"
        );
    }
}

/// The WebView may only talk to the Tauri IPC bridge.
#[test]
fn csp_allows_no_remote_origins() {
    let conf = read("tauri.conf.json");
    let after_key = &conf[conf.find("\"csp\"").unwrap() + 5..];
    let value_start = after_key.find('"').unwrap() + 1;
    let csp = &after_key[value_start..value_start + after_key[value_start..].find('"').unwrap()];
    for directive in csp.split(';').filter(|d| !d.trim().is_empty()) {
        for source in directive.split_whitespace().skip(1) {
            let allowed = matches!(
                source,
                "'self'" | "'unsafe-inline'" | "data:" | "ipc:" | "http://ipc.localhost"
            );
            assert!(allowed, "CSP source `{source}` is not allowed");
        }
    }
}

/// Every command is declared (deny-by-default), granted, and registered — and nothing else.
#[test]
fn commands_manifest_capability_and_handler_agree() {
    let mut manifest = quoted_list_after(&read("build.rs"), ".commands(&");
    let mut granted: Vec<String> =
        quoted_list_after(&read("capabilities/default.json"), "\"permissions\"")
            .into_iter()
            .filter_map(|p| p.strip_prefix("allow-").map(|c| c.replace('-', "_")))
            .collect();
    let lib = read("src/lib.rs");
    let start = lib.find("generate_handler![").unwrap();
    let end = lib[start..].find(']').unwrap() + start;
    let mut handlers: Vec<String> = lib[start..end]
        .split(',')
        .filter_map(|s| s.trim().rsplit("::").next().map(str::to_string))
        .filter(|s| !s.is_empty() && !s.contains('['))
        .collect();
    manifest.sort();
    granted.sort();
    handlers.sort();
    assert_eq!(
        manifest, granted,
        "build.rs and capabilities/default.json disagree"
    );
    assert_eq!(
        manifest, handlers,
        "build.rs and lib.rs generate_handler! disagree"
    );
}

/// The capability grants no plugin permissions beyond core window chrome and the reviewed
/// updater set: check/download/install, and relaunch after an install.
#[test]
fn capability_grants_only_reviewed_plugins() {
    for permission in quoted_list_after(&read("capabilities/default.json"), "\"permissions\"") {
        let ok = permission.starts_with("allow-")
            || permission == "core:default"
            || permission.starts_with("core:window:allow-")
            || permission == "updater:default"
            || permission == "process:allow-restart";
        assert!(ok, "permission `{permission}` needs a security review");
    }
}

/// Updates are signed, fetched over HTTPS only, and the committed config has no endpoint:
/// the release workflow injects the repository's channel URL, so local builds never update.
#[test]
fn updater_is_signed_https_and_opt_in() {
    let conf: serde_json::Value = serde_json::from_str(&read("tauri.conf.json")).unwrap();
    let updater = &conf["plugins"]["updater"];
    let pubkey = updater["pubkey"].as_str().unwrap_or_default();
    assert!(pubkey.len() > 40, "updater public key missing");
    assert!(
        !pubkey.contains("PRIVATE") && !read("tauri.conf.json").contains("secret key"),
        "a private key must never be committed"
    );
    assert_eq!(
        updater["endpoints"],
        serde_json::json!([
            "https://github.com/ryanvxc/2vx-tweaks/releases/download/updater-main/latest.json"
        ]),
        "the one reviewed feed, over HTTPS"
    );
    assert_eq!(
        conf["bundle"]["createUpdaterArtifacts"],
        serde_json::json!(false),
        "only CI holds the signing key"
    );
    assert!(
        updater.get("dangerousInsecureTransportProtocol").is_none(),
        "insecure update transport"
    );
    let workflow = fs::read_to_string(root().join("../.github/workflows/release.yml")).unwrap();
    assert!(
        !workflow.contains("endpoints"),
        "tauri.conf.json is the one place that names the feed"
    );
    assert!(
        !workflow.contains("releases/latest/download"),
        "the feed stays on updater-main, the one reviewed URL"
    );
}

/// The owner's rule: every update can be downloaded by anyone, not only installed by the updater.
#[test]
fn every_release_publishes_an_installer_for_download() {
    let workflow = fs::read_to_string(root().join("../.github/workflows/release.yml")).unwrap();
    for needed in [
        "prerelease: false",
        "gh release upload \"$TAG\" 2VX-Tweaks-Setup.exe",
        "--notes-file release-body.md --latest",
    ] {
        assert!(workflow.contains(needed), "release.yml lost `{needed}`");
    }
    // It must stay a prerelease, or GitHub could show it as the latest release: it has no installer.
    assert!(
        workflow
            .contains("gh release create updater-main --repo \"$GITHUB_REPOSITORY\" --prerelease"),
        "updater-main must stay a prerelease"
    );
    // The check that stops the feed must run before the feed can go out.
    let check = workflow.find("Publish the installer for download").unwrap();
    let feed = workflow.find("Publish the feed to updater-main").unwrap();
    assert!(
        check < feed,
        "an update must never go out without its installer"
    );

    let readme = fs::read_to_string(root().join("../README.md")).unwrap();
    assert!(
        readme.contains("releases/latest/download/2VX-Tweaks-Setup.exe"),
        "the README's download link"
    );
}

/// The built-in registry validates (metadata gate, policy, conflicts) on every build.
#[test]
fn builtin_tweaks_pass_the_metadata_gate() {
    // `with_builtin_tweaks` panics or errors on an invalid tweak once tweaks are registered.
    let registry = twovx_tweaks_lib::engine::TweakRegistry::with_builtin_tweaks();
    for tweak in registry.iter() {
        for setting in tweak.settings() {
            twovx_tweaks_lib::engine::policy::check_target(&setting.target).unwrap();
        }
    }
}
