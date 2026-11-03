# Architecture

## Layers

```
┌───────────────────────── React (WebView2) ──────────────────────────┐
│ pages/ ── components/ ── hooks/ ──► services/backend.ts ──invoke()──┼──┐
└──────────────────────────────────────────────────────────────────────┘  │ IPC (capability-gated)
┌───────────────────────────── Rust ───────────────────────────────────┐  │
│ commands/  ◄─────────────────────────────────────────────────────────┼──┘
│   │ validate args, delegate, map errors to AppError                  │
│   ├── system/   read-only: os.rs (registry), hardware.rs (sysinfo),  │
│   │             metrics.rs (sampler thread + ring buffer),           │
│   │             privilege.rs (token elevation)                       │
│   ├── engine/   mod.rs   Tweak trait (declarative settings), registry, │
│   │                      metadata gate, compatibility                │
│   │             setting.rs  Hive / RegTarget / RegValue / Setting    │
│   │             policy.rs   protected-target denylist                │
│   │             store.rs    SettingStore: the only registry writer   │
│   │             plan.rs     dry-run plans (single use, 5 min TTL)    │
│   │             service.rs  Engine: plan → apply → verify → rollback │
│   │             journal.rs  SQLite change journal (write-ahead)      │
│   │             broker.rs   elevated helper over a private pipe      │
│   └── logging.rs (tracing → rolling file), error.rs                  │
└──────────────────────────────────────────────────────────────────────┘
         ▲ named pipe (typed writes only)
┌──── 2VX Tweaks.exe --2vx-broker (elevated, short-lived) ────┐
│ policy check + "declared by a built-in tweak" check → write │
└──────────────────────────────────────────────────────────────┘
```

The nine areas from the project brief map like this:

| Area | Where |
|---|---|
| UI and presentation | `src/pages`, `src/components` |
| App state and settings | `src/stores` (UI prefs). Durable engine state in the SQLite journal |
| System information | `src-tauri/src/system` |
| Optimisation engine | `src-tauri/src/engine/` (`service.rs` is the only path that changes the system) |
| Optimisation modules | `src-tauri/src/engine/tweaks/<category>/<tweak>.rs` (first tweaks: backlog P0-6) |
| Windows API integrations | `system/*`, `engine/store.rs`, `engine/broker.rs`. See `windows-integration.md` |
| Backup and restore | `engine/journal.rs`: `%LOCALAPPDATA%\com.twovx.tweaks\journal.db` |
| Logging and errors | `logging.rs`, `error.rs`, `src/services/logger.ts` |
| Diagnostics and benchmarking | Not started (backlog P2-3, P2-9) |

## Data flow

- **Snapshot**: `get_system_snapshot` runs `collect_snapshot` on a blocking thread. It is collected once per session
  (the frontend caches it in `use-backend-query`).
- **Live metrics**: `MetricsSampler` starts in `setup`, samples CPU, RAM, GPU and network every 1 s, and keeps 300 samples.
  The UI polls `get_metrics(sinceMs)` once a second and receives only newer samples. History survives page switches
  because it lives in Rust.
- **Logging**: Rust `tracing` → `%LOCALAPPDATA%\com.twovx.tweaks\logs\2vx.YYYY-MM-DD.log` (14 days kept). The frontend
  forwards logs, uncaught errors and React render errors via `log_frontend_event` (level enum, message sanitised and
  capped at 2,000 chars).

## Key decisions

- **sysinfo over WMI** for CPU, RAM, disks, GPU and motherboard: no COM threading concerns, and it covers
  everything Phase 1 needs. WMI (`wmi` crate) stays an option for data sysinfo lacks (e.g. driver versions).
- **windows-registry** (Microsoft's crate) for read-only OS version details. `ProductName` says "Windows 10" on
  Windows 11, so the generation comes from the build number (≥ 22000).
- **Frameless window + custom title bar** for a native-feeling premium shell; caption buttons mimic Windows 11.
- **HashRouter**: production assets are served from a custom protocol with no server-side rewrites.
- **`freezePrototype` is off**: Recharts' `decimal.js-light` writes to `Object.prototype`-derived objects and throws
  when prototypes are frozen. Revisit if the charting library changes.
- **Declarative tweaks**: a tweak lists the settings it wants; it has no code that runs against the system. Detection,
  backup, apply, verification, rollback and revert are implemented once in `engine/service.rs`, so no tweak can skip
  a backup or forget a verify.
- **SQLite journal (rusqlite, bundled)** with write-ahead discipline: each change is committed with its exact
  before-value (`intent`, `synchronous=FULL`) before the registry is touched, then marked `verified` after a
  read-back. On start-up, unfinished snapshots become `interrupted` and stay revertible. If the journal can't be
  opened or fails its integrity check, applying and reverting are disabled; detection still works.
- **Plans are bound to observed values**: `apply_plan` re-reads every target and refuses with `conflict` if anything
  moved since the user saw the diff. Plans are single-use and expire after 5 minutes.
- **Revert never clobbers**: a value is restored only if it still holds what 2VX wrote. Anything changed since is
  left alone and reported as a `conflict`.
- **Elevation broker** instead of an always-admin app. See `security.md` § Privilege.

### Engine flow

```
UI: tweak ids ──plan_tweaks──► Plan { changes: target, from → to; risk; restart; needsElevation } (no writes)
UI: confirm  ──apply_plan(planId)──► re-read & compare ─► [UAC once if HKLM] ─► per change:
        journal intent (before, after) ─► write ─► read back ─► journal verified
        on any failure: restore earlier changes in reverse ─► snapshot rolled_back (or failed)
UI: revert_snapshot(id) ──► per change: current == after ? restore before : (== before ? skip : conflict)
```

## Stages

Stages, their scope and status live in [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md); the current state of each
area is in [PROJECT_STATUS.md](PROJECT_STATUS.md).
