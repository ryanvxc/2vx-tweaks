# Security and safety

## Threat model

2VX modifies operating-system configuration. The two real risks are (1) the app damaging the user's system and
(2) the app's privileges being abused through its UI (e.g. injected script calling commands). Both are designed
against.

## Boundary: frontend ↔ backend

- The WebView has **no** shell, fs or opener plugins. It can only call app commands. `open_link` and `open_tool` are two
  of those commands: each takes an entry from a fixed list in `links.rs` (https pages and Windows Settings pages, or
  Windows tools), never an address or a command. `open_log_folder` takes nothing, and `export_report` takes only the
  report's text: the file is whatever the user picks in the Windows save dialog.
- Each command is declared in `src-tauri/build.rs` (`AppManifest::commands`), so it is **denied by default** until
  `capabilities/default.json` grants it. Adding a command without a capability entry makes it unreachable.
- Commands accept typed arguments only (enums, numbers, IDs validated against the registry). They never accept
  free-form paths, registry keys, process names or shell strings.
- CSP: `default-src 'self'`, scripts from `'self'` only, IPC origins allowed. `'unsafe-inline'` styles are
  required by Recharts and Radix positioning.
- `freezePrototype` is disabled because Recharts breaks with it (see `architecture.md`).

## Privilege

- The app runs unelevated (`asInvoker`) and shows its privilege level in the title bar.
- A batch that touches HKLM uses the **elevation broker** (`engine/broker.rs`): the app relaunches its own executable
  with `ShellExecuteExW("runas")` and `--2vx-broker <pipe> <parent-pid>`, which shows **one UAC prompt per batch**.
  The elevated copy connects back, performs individual writes and exits when the batch ends.
  - Pipe: `\\.\pipe\2vx-broker-<128 random bits>`, `FILE_FLAG_FIRST_PIPE_INSTANCE` (can't be pre-created or
    squatted), `PIPE_REJECT_REMOTE_CLIENTS`, one instance.
  - The app accepts exactly one client and checks its PID is the process it launched. The broker checks the pipe
    server's PID is the parent it was told about.
  - The broker opens the pipe at `SECURITY_IDENTIFICATION`, so the unelevated app can't impersonate the admin token.
  - The broker's vocabulary is `Write { target, value }`, `Clean { id }`, `MoveStartup { key, name, enabled }` and
    `Done` (guard: `broker_vocabulary_is_fixed`). It writes a target only if it passes the policy denylist **and** is
    declared by a built-in tweak. `Clean` takes an ID from the table in `engine/cleanup.rs` and refuses any location
    that doesn't need admin. `MoveStartup` takes a fixed Run key and a value name, and reads the value itself, so a
    command line never crosses the pipe. The broker accepts no commands, paths or code, holds no state and keeps no
    journal (the app journals every tweak).
  - A declined UAC prompt returns `elevation_required`; nothing is changed.
  - Residual risk: another process running as the same user could start the broker itself, but that still needs the
    user to accept a UAC prompt, and it can only touch targets that 2VX tweaks declare.
- Never SYSTEM, never TrustedInstaller, no Windows service, no scheduled task.
- Note: when launched from an elevated terminal, dev builds inherit elevation (the title bar shows "Administrator"),
  and HKLM writes then happen in-process.

## Change safety (engine)

- **Declarative tweaks**: tweaks list settings; only `engine/service.rs` writes, through `SettingStore`.
- **Write-ahead journal**: every before-value is committed to SQLite before the write, so a crash never loses the
  information needed to undo. Interrupted snapshots are flagged on the next start and can be reverted.
- **Read-back verification**: a write counts only if reading the value back gives the expected result (catches
  policy overrides). Any failure rolls the whole batch back in reverse order.
- **Stale-plan refusal**: apply refuses if any target changed since the plan was shown.
- **Non-clobbering revert**: values changed by something else since 2VX set them are left alone and reported.
- **Backups stay with their device**: `Hive::Adapter` and `Hive::Gpu` targets name a kind of device, and can land on
  different hardware from one call to the next. The journal records the driver instance each change reached, and
  nothing is restored anywhere else (rule 10).
- **Policy denylist** (`engine/policy.rs`), checked at registration, planning and in the broker: Defender, Firewall,
  Windows Update, UAC/system policies, VBS/HVCI/Device Guard, LSA, Secure Boot, SmartScreen, Windows Security,
  SAM/SECURITY/BCD hives, and the configuration of protected services (WinDefend, mpssvc, BFE, wuauserv,
  WaaSMedicSvc, UsoSvc, BITS, TrustedInstaller, EventLog and others).

## Commitments (enforced by `src-tauri/tests/policy.rs`)

2VX does not and will not include:

| Commitment | Enforcement |
| --- | --- |
| No telemetry, analytics or crash upload | No direct network, analytics or crash-reporting crates in `Cargo.toml` (`tauri-plugin-updater` is the one reviewed exception); CSP allows only the IPC bridge |
| No hardware fingerprinting or device IDs | No machine-ID crates; nothing leaves the machine. Hardware details never read a serial number, asset tag or device ID (`hardware_reads_stay_clear_of_identifiers`) |
| No remote kill switch or remote content | Tweaks are compiled in, never downloaded. The only network client is the signed updater (decision P0-8b, superseded): it fetches the release feed and, on request, a signed installer. The only other traffic is ping, which the user starts: ICMP echoes with ping's default filler, to the default gateway or 1.1.1.1, from one file, with no HTTP. One call sends one echo, and a bufferbloat test makes one call a second to each target for its 40 seconds (`ping_is_the_only_traffic_besides_the_updater`). Listing startup entries opens nothing remote: a program's version details are read only from a drive in this PC, because opening a file on a share is traffic too, and would hand that host this account's credentials (`version_reads_stay_on_local_drives`) |
| No background persistence | No service or scheduled task is created; nothing runs when the app is closed. If the user turns on "open at sign-in", which is off by default, one `HKCU` Run value opens the normal, visible window at sign-in (`auto_launch_is_one_run_value`). If the user turned that off in Task Manager, Settings says so and 2VX leaves it: `StartupApproved` is only ever read (`startup_approved_is_only_read`) |
| No kernel drivers | None bundled; not in scope |
| No unrestricted command execution | The backend source may not spawn processes or shells (`Command::new`, `ShellExecuteA`, PowerShell, cmd, reg, sc, schtasks, bcdedit, wmic, mshta, wscript, cscript, rundll32, regsvr32…). The only elevated launch is the broker's self-relaunch. `links.rs` is the only file that may call `ShellExecuteW`, on its fixed list of pages, on a fixed list of ten Windows tools resolved under the real System32 directory with constant arguments, and on the app's own log folder, so Hardware, Network and Advanced can point to driver downloads, Windows Settings and Windows' own tools for what 2VX won't do itself. It never takes an address or a command from the frontend, and never uses an elevating verb (`windows_tools_are_a_fixed_list_of_bare_names`) |
| No unnecessary privilege | `asInvoker`, per-batch broker, never SYSTEM or TrustedInstaller |
| No protected changes, and evidence on everything else | Metadata gate (https references, stated evidence tier) and the policy denylist |
| Every IPC command is deliberate | `build.rs` manifest, capability grants and `generate_handler!` must list the same commands, and every parameter of every command is on a reviewed list by name and type (`commands_never_take_a_path`); plugin permissions limited to window chrome, `updater:default` and `process:allow-restart` |

Changing any of these requires updating this document and the guard test in the same change, with a reason.

Revised 2026-11-05: `backend_never_spawns_shells_or_processes` scans `engine/tweaks/library/<tab>.rs` with string literals
removed, because real setting names contain the banned words (`POWERSHELL_TELEMETRY_OPTOUT`, and "wmic" inside
`ShowMicrosoftRewards`). The code in those files, and every other file in full, is scanned as before.

Revised 2026-11-05: the same scan now covers `engine/startup.rs`, because its `LAUNCHERS` table names `powershell.exe`,
`cmd.exe` and others to say nothing about who made an entry that runs through one. Those names are data: they're only
ever compared with a program's name, and `launcher_names_are_only_compared` pins that. The string filter now also leaves
`'"'` and `r#"..."#` alone, which would otherwise have flipped what it took for code in this file.

Revised 2026-11-05 after an independent review found guards weaker than this document said. Each only got stricter:

- `commands_never_take_a_path` looked for "path" and "url" in the signature and skipped `#[tauri::command(async)]`, so
  `file: String` passed. It now reads every attribute form, holds every parameter to a list of names with their types,
  keeps commands in `src/commands`, and checks it saw all of them against `build.rs`.
- `windows_tools_are_a_fixed_list_of_bare_names` read `WindowsTool::ALL`, which is kept by hand, so a new variant went
  unseen. It now parses the enum from source, and bans script hosts by name.
- `ping_is_the_only_traffic_besides_the_updater` now also bans `socket(`, `connect(`, `WSAStartup`, `WSAConnect`,
  `getaddrinfo` and `Icmp6*`, and pins `system/ping.rs`'s WinSock import to `AF_INET` and `SOCKADDR_INET`, the only reason
  `Cargo.toml` enables that feature. The Latency revision below always said socket APIs were banned. Now they are.
- `backend_never_spawns_shells_or_processes` now also bans `ShellExecuteA` and the script hosts.
- `a_plan_takes_the_largest_preset` repeated one ID, which collapses to one tweak. It's now 157 tweaks applied and undone
  as one batch, with a test at the 512 limit and one past it.
- `version_reads_stay_on_local_drives` and `startup_approved_is_only_read` are new.

Revised 2026-11-05 at the owner's request, each with its guard tightened in the same change:

- **Windows tools.** "No unrestricted command execution" now allows `open_tool`, because the redesign links to Resource
  Monitor, Reliability Monitor, System Information, DirectX Diagnostic, Disk Cleanup, Event Viewer, Task Manager, Device
  Manager, Services and Network Connections rather than rebuilding them. The list is pinned in the guard, and each tool
  is a bare name opened from `GetSystemDirectoryW`, never found through `PATH` or the current folder. `ncpa.cpl` uses
  `cplopen` because Control Panel files have no `open` verb. 2VX never asks for elevation here, but `perfmon.exe` and
  `mmc.exe`, which hosts the `.msc` consoles, are `highestAvailable` in their own manifests, so Windows can show its own
  prompt for those. The guard now also keeps
  `ShellExecuteW` to `links.rs` and `ShellExecuteExW` to `broker.rs`, where before either name was allowed in both.
- **Latency.** The network row now allows `ping_target`, because a real round-trip time is the honest way to show
  latency, and ICMP echo is the smallest thing that can measure it. The user starts every ping. A target is a name
  (`gateway` or `internet`), so a command can't choose an address; the gateway comes from the local routing table and
  1.1.1.1 is a constant. Nothing identifies the PC beyond the source address any packet has. The guard pins the call to
  one site in `system/ping.rs`, and bans socket and HTTP APIs across the backend. A bufferbloat test is a run of these
  echoes, one a second to each target for 40 seconds, and it only starts when the user asks.
- **Sign-in.** "No background persistence" now allows one Run value, because people want their settings app ready after
  sign-in, and off by default keeps that their decision. It opens the normal window, which can be seen and closed, and
  still runs nothing once it's closed. There's no tray, service or task behind it (`RegisterTask` is banned outright).
- **Adapter Tuner.** Rule 10 below now allows `Hive::Adapter`, because adapter settings live under a driver key whose
  index differs per PC, so scripts that hardcode one write to the wrong device. Only NDIS standardized keywords from
  Microsoft's documentation are reachable, checked in `policy::check_target` so the broker enforces the same list.
  `commands_never_take_a_path` is new, and holds for `export_report` too.

None of these touch rule 1 or the denylist in `engine/policy.rs`.

## Rules for system changes

1. Never disable Defender, Firewall, Windows Update, UAC, SmartScreen, VBS/HVCI, Secure Boot or similar protections.
2. Never bulk-disable services. Service changes are individual, explained and reversible. A preset is a short, fixed
   list of reviewed tweaks (`lib/service-presets.ts`): it shows every change first, and leaves out anything protected.
   Revised 2026-11-04 for the PC Cleaner's Services tab, with no guard change, because each entry is still its own
   tweak with its own warning, journal entry and undo. `get_service_states` only reads, through read-only handles, and
   only the services catalogue tweaks name. The service control APIs are banned (`services_are_only_ever_read`).
3. Every setting has references in `TweakMeta.references` and states its evidence: documented, found in a Windows or
   driver file, or community. Revised 2026-11-05 at the owner's request (the library brief says to keep legitimate
   tweaks unless there's a concrete technical reason not to). Tiers show in the details, community tweaks stay out of
   presets, and values that no Windows or driver file contains were left out. Rule 1 and the denylist are unchanged.
4. Back up before every change. If any step fails, roll back the earlier steps, then report the error (implemented
   once in the engine, see § Change safety).
5. Deletion only inside a fixed allowlist of locations (temp, caches). Scan first, show results, delete on confirmation.
   Each location is cleared by the same rule it was measured with (`engine/cleanup.rs`), reparse points are never
   followed, and the temp folders keep anything from the last 24 hours. Files are deleted, not moved to the Recycle
   Bin, because moving a cache there frees nothing. The confirmation says so.
6. Boot configuration (`bcdedit`) and security settings need a `High` risk confirmation with explicit acknowledgement.
7. Never apply anything silently. Presets show the full list of changes before applying.
8. No performance claims without measurement on the user's machine.
9. Apps are removed only from a fixed table (`engine/apps.rs`), through `PackageManager`, for the current account.
   Microsoft Store, App Installer, Windows Security, Edge, Xbox identity components and frameworks aren't in it.
10. Scheduled tasks are only turned off and back on, through Task Scheduler's COM API. The engine never creates or
    deletes a task or changes what it runs (`scheduled_tasks_are_only_turned_off`), and tasks on security paths are
    denied. Power plan values go through the power API and act on the active plan. GPU values go only to a display
    adapter key of the named vendor. Adapter values go only to the driver key of this PC's primary adapter of a named
    kind (`Ethernet` or `Wifi`), and only under a documented NDIS keyword that the driver itself lists under
    `Ndi\Params`. A tweak only ever sets the strings `0` or `1` (`check_value`). Undo puts back whatever was there, which
    can be any value the driver uses, such as `3` for a Realtek `*FlowControl`, so the broker can't hold a write to
    `check_value`: it checks the target, and that a tweak declares it. They ask for a restart, because read-back proves the
    registry changed and not that the driver took it. All four use the same journal, broker and declared targets as the
    registry.
    Revised 2026-11-05 at the owner's request (`adapter_tweaks_stay_on_the_documented_keywords`).

    The same `Adapter` or `Gpu` target can land on another device from one call to the next, when a cable moves, a USB
    adapter comes out or a card is swapped. So each change records the driver instance it reached (`changes.device`, an
    index like `0008` with no hardware identifier in it). Apply refuses a plan once its target has moved off the device
    it was planned on, and checks again after the UAC prompt. Undo restores a value only on the device that was changed:
    anywhere else it changes nothing, reports a conflict and keeps the row open, so the backup still works when that
    device is back. A rollback follows the same rule. A missing device is a failure rather than an unset value, and rows
    are grouped by target and device, so a newer change on one adapter can't close another's backup. Rows from before
    this column have no device and match any, as they always did. Those are GPU rows from 0.3, which shipped no adapter
    tweaks. Revert still restores a value only if it holds what 2VX wrote, and leaves anything else alone and reports it.

    Known limit: the app checks the device, then the broker resolves the target again in its own process, and `Write`
    carries no device, which keeps the broker's vocabulary as it was. A link that moves in the moment between the two
    reaches the new adapter under the old one's row, and undo can't find that change afterwards.

## Decisions (backlog P0-8, decided by the owner on 2026-11-03)

**(a) Disabling startup entries** (P1-5): ~~remove the `Run`/`RunOnce` value and journal it~~ **revised 2026-11-04**:
the value moves unchanged to `Software\2VX Tweaks\DisabledStartup` in the same hive, and a shortcut in the user's Startup
folder moves to `%LOCALAPPDATA%\com.twovx.tweaks\disabled-startup`. Reason: restoring an HKLM value from the journal would
mean sending a command line to the elevated side, and the broker must never take a value from the unelevated app.
Each move is copy, verify, remove, with rollback. The held copy is the backup and the switch is the undo, so these
moves are audit-logged but not listed in the Restore Centre. If a program registered itself again meanwhile, its new
entry wins. The all-users Startup folder is read-only. Task Manager's `StartupApproved` values are read but never
written (undocumented, `REG_BINARY`), so an entry turned off there can only be turned back on there. Consequence to
tell users: a disabled entry disappears from Task Manager's Startup tab rather than showing "Disabled". Guard added:
`broker_vocabulary_is_fixed`.

**(b) Updates** (P1-11): ~~no in-app updater~~ **superseded for 0.2** at the owner's request (the redesign
brief asked for an in-app Tauri updater). The updater is allowed under these conditions, all enforced by
`tests/policy.rs` or the release workflow (details in `UPDATE_SYSTEM.md`):
- official `tauri-plugin-updater`, signature verification always on, public key committed, private key never;
- one HTTPS GitHub feed, named in `tauri.conf.json`. ~~injected by CI so local builds never update~~ **revised
  2026-11-04** at the owner's request, so the exe they run locally can reach the feed. Signature checking, the
  newer-only rule and the click before each step are unchanged, and a local build still can't sign or publish;
- never `releases/latest/download`; feed must be newer than the published one (no downgrades);
- checking is the only automatic step and can be turned off; download and install each need a click;
- release notes are shown before installing and rendered as text.

**(c) Windows repair tools** (P2-8): a fixed-argv exception will be allowed. When P2-8 is built, exactly one module
may launch `sfc.exe` and `dism.exe` from `%SystemRoot%\System32`, with argument lists that are compile-time
constants (no user input, no paths), elevated through the broker, with output shown to the user. The guard test in
`tests/policy.rs` and the "No unrestricted command execution" row above change in that same change to allow exactly
those constants in that one file. Until then, the guard forbids all process launches except the broker relaunch and the fixed list in `links.rs`.

## Logging and privacy

- Logs and the change journal stay local (`%LOCALAPPDATA%\com.twovx.tweaks\`, per-user ACLs). No telemetry.
- The journal stores registry targets and values that 2VX changed, plus an audit log of apply/revert events. It
  stores no personal data or hardware identifiers.
- Game Mode reads the names of running programs through `get_running_programs`, only while its page is open. The
  command takes no arguments: the game list stays in the UI and is compared there, so no process name reaches a command.
  Paths stay in the backend, except a game's folder name. The list isn't stored or logged, and nothing is launched or
  stopped.
- Hardware details, the device list, service states and the Autoruns list are read on request and kept in memory for
  the page showing them. A shortcut is read for its stored target and never resolved, because resolving can search the
  disk and rewrite the file.
- The system report is written only to the file the user picks, capped at 2 MB, and its text is never logged. It leaves
  the machine only if the user sends that file themselves.
- The local account is one email address in `account.json` in the same folder, used only to label the account card.
  There's no server, password or sign-in, so it proves nothing and unlocks nothing. It's never transmitted or logged,
  and Log out deletes the file.
- Frontend log messages are stripped of control characters and capped at 2,000 chars, which prevents log injection.
