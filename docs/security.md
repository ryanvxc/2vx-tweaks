# Security and safety

## Threat model

2VX modifies operating-system configuration. The two real risks are (1) the app damaging the user's system and
(2) the app's privileges being abused through its UI (e.g. injected script calling commands). Both are designed
against.

## Boundary: frontend ↔ backend

- The WebView has **no** shell, fs or opener plugins. It can only call app commands. `open_link` is one of those commands:
  it takes an entry from the fixed list in `links.rs` (https pages and Windows Settings pages), never an address.
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
- **Policy denylist** (`engine/policy.rs`), checked at registration, planning and in the broker: Defender, Firewall,
  Windows Update, UAC/system policies, VBS/HVCI/Device Guard, LSA, Secure Boot, SmartScreen, Windows Security,
  SAM/SECURITY/BCD hives, and the configuration of protected services (WinDefend, mpssvc, BFE, wuauserv,
  WaaSMedicSvc, UsoSvc, BITS, TrustedInstaller, EventLog and others).

## Commitments (enforced by `src-tauri/tests/policy.rs`)

2VX does not and will not include:

| Commitment | Enforcement |
| --- | --- |
| No telemetry, analytics or crash upload | No direct network, analytics or crash-reporting crates in `Cargo.toml` (`tauri-plugin-updater` is the one reviewed exception); CSP allows only the IPC bridge |
| No hardware fingerprinting or device IDs | No machine-ID crates; nothing leaves the machine |
| No remote kill switch or remote content | Tweaks are compiled in, never downloaded. The only network client is the signed updater (decision P0-8b, superseded): it fetches the release feed and, on request, a signed installer |
| No background persistence | No service or scheduled task is created; nothing runs when the app is closed |
| No kernel drivers | None bundled; not in scope |
| No unrestricted command execution | The backend source may not spawn processes or shells (`Command::new`, PowerShell, cmd, reg, sc, schtasks, bcdedit, wmic…). The only process launch is the broker's self-relaunch. `links.rs` may call `ShellExecuteW` on its fixed list of pages, so Hardware, Network and Advanced can point to driver downloads and Windows Settings for what 2VX won't do itself; it never takes an address from the frontend. |
| No unnecessary privilege | `asInvoker`, per-batch broker, never SYSTEM or TrustedInstaller |
| No protected changes, and evidence on everything else | Metadata gate (https references, stated evidence tier) and the policy denylist |
| Every IPC command is deliberate | `build.rs` manifest, capability grants and `generate_handler!` must list the same commands; plugin permissions limited to window chrome, `updater:default` and `process:allow-restart` |

Changing any of these requires updating this document and the guard test in the same change, with a reason.

Revised 2026-11-05: `backend_never_spawns_shells_or_processes` scans `engine/tweaks/library/<tab>.rs` with string literals
removed, because real setting names contain the banned words (`POWERSHELL_TELEMETRY_OPTOUT`, and "wmic" inside
`ShowMicrosoftRewards`). The code in those files, and every other file in full, is scanned as before.

## Rules for system changes

1. Never disable Defender, Firewall, Windows Update, UAC, SmartScreen, VBS/HVCI, Secure Boot or similar protections.
2. Never bulk-disable services. Service changes are individual, explained and reversible. A preset is a short, fixed
   list of reviewed tweaks (`lib/service-presets.ts`): it shows every change first, and leaves out anything protected.
   Revised 2026-11-04 for the PC Cleaner's Services tab, with no guard change, because each entry is still its own
   tweak with its own warning, journal entry and undo.
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
    adapter key of the named vendor. All three use the same journal, broker and declared targets as the registry.

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
- The local account is one email address in `account.json` in the same folder, used only to label the account card.
  There's no server, password or sign-in, so it proves nothing and unlocks nothing. It's never transmitted or logged,
  and Log out deletes the file.
- Frontend log messages are stripped of control characters and capped at 2,000 chars, which prevents log injection.
