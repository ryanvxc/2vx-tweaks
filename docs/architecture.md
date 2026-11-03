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
│   ├── engine/   Tweak trait, TweakMeta, TweakRegistry, compatibility │
│   │             (Phase 2+: executor, backup store, tweak modules)    │
│   └── logging.rs (tracing → rolling file), error.rs                  │
└──────────────────────────────────────────────────────────────────────┘
```

The nine areas from the project brief map like this:

| Area | Where |
|---|---|
| UI and presentation | `src/pages`, `src/components` |
| App state and settings | `src/stores` (UI prefs). Phase 2: SQLite in Rust for durable settings |
| System information | `src-tauri/src/system` |
| Optimisation engine | `src-tauri/src/engine/mod.rs` |
| Optimisation modules | `src-tauri/src/engine/tweaks/<category>/<tweak>.rs` (Phase 2+) |
| Windows API integrations | `system/*`, and future `src-tauri/src/win/` wrappers. See `windows-integration.md` |
| Backup and restore | Phase 2: `src-tauri/src/backup/` + SQLite |
| Logging and errors | `logging.rs`, `error.rs`, `src/services/logger.ts` |
| Diagnostics and benchmarking | Phase 5 |

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
- **No SQLite yet**: Phase 1 has nothing durable to store. It arrives with the backup engine, where it's needed
  for transactional backups.

## Phases

| Phase | Scope | Status |
|---|---|---|
| 1 | Shell, design system, navigation, dashboard, system info, live metrics, logging, engine contracts | **Done** |
| 2 | SQLite, backup/restore engine, executor (backup → apply → verify → rollback), restore points, first low-risk tweaks (power plan, visual effects), Monitoring page, per-engine GPU counters | Next |
| 3 | Gaming: Game Mode, HAGS, Game Bar/DVR, game detection, per-game profiles, frame-time measurement | |
| 4 | Startup manager, PC Cleaner (allowlisted locations, scan-then-clean) | |
| 5 | Tools: network diagnostics, DNS, reports; installer, code signing, updater | |
