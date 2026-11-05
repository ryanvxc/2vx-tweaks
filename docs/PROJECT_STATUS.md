# Project status

Reviewed at commit `a961def` (2026-11-03); rows are updated for stage 3 and the uncommitted 0.2 redesign and updater work. Established by reading the code (Serena and the source tree), running
`npm run check:all`, and touring every page of the browser preview with Playwright. Nothing here is taken from earlier
plans. Priorities point to [FEATURE_BACKLOG.md](FEATURE_BACKLOG.md); stages to
[DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md).

**Verification (0.2, 2026-11-04):** `npm run check:all` passes: typecheck, lint, Prettier, 165 Vitest tests in 29 files
(including the updater state machine and the release scripts), `cargo fmt`, clippy with `-D warnings`, 99 Rust unit
tests (2 ignored real-system tests) and 8 policy guard tests. The GitHub workflows have run (see Updates and CI below). The
makeover was also toured on the desktop debug build with the real catalogue: every page, every Tweaks tab, a tweak review
and a gaming profile review, with no console errors. Nothing was applied, so applying through the new screens is
covered by unit tests only. The later layout pass (shorter window, title in the title bar, full-height pages) was
checked at 1280×720, 1024×600 and maximised. The window has since been fixed at 1280×820.

**Status legend**

| Status           | Meaning                                                       |
| ---------------- | ------------------------------------------------------------- |
| **Implemented**  | Works end to end against the real system, with tests          |
| **Backend only** | Implemented and tested in Rust, but nothing in the UI uses it |
| **Partial**      | Some of the intended behaviour works; the rest is missing     |
| **Sample**       | UI shows placeholder data, labelled with a `sample` badge     |
| **Placeholder**  | Page or card that only lists planned features                 |
| **Not started**  | No code                                                       |

## Summary

Version 0.3 adds the Library (`/library`): 1,221 more tweaks from the supplied tweak collection, 1,285 in all, with presets,
whole-library search and hardware checks. Every one passes the simulated apply, verify, re-plan and restore test, and every
state was read on a real PC, but none has been written to a real PC yet. See
[OPTIMISATION_IMPLEMENTATION_STATUS.md](OPTIMISATION_IMPLEMENTATION_STATUS.md).

Version 0.2: the app was restructured and restyled (`INFORMATION_ARCHITECTURE.md`, `REDESIGN_ROADMAP.md`,
`DESIGN_SYSTEM.md`). 64 documented tweaks ship with per-tweak undo, placed on Tweaks, Hardware, Network and Advanced, with Game Mode and its
presets built from those same tweaks. Monitoring, Hardware (GPU, CPU, RAM, Peripherals, Storage), Network (Tweaks, Network
Priority, Adapter Tuner, Bufferbloat), Advanced (Security, MSI Mode, Devices, Timer Resolution), PC Cleaner (System
Cleaner, Services, Uninstall, Autoruns), the Restore Centre and Tools are real pages,
and a signed Tauri updater with an Update Centre is built (`UPDATE_SYSTEM.md`). The repository is public at `ryanvxc/2vx-tweaks`,
the signing secrets are set, and the pipeline published prereleases v0.2.0 and v0.2.1 with the update feed. The
update from v0.2.2 to v0.2.3 installed through Check for updates, the first CI-to-CI update. The broker's real UAC path
(P0-7) still needs a VM. Not built: Adapter Tuner's per-adapter settings (read-only for now), and changing MSI mode or
disabling devices, which need new engine primitives and a VM to test on.

## Status by area

| Area                                                                                                              | Current status                                                       | Problems                                                                                                                                                                            | Recommended action                                          | Priority    |
| ----------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- | ----------- |
| App shell (title bar, window controls, sidebar, routing) | **Implemented** (0.2 makeover) | Grouped sidebar with an account card and menu (local email, updates, Settings); search in the account menu and on Ctrl K; a fixed window with only minimise and close; `/system`, `/history` and old Settings links redirect. | Keep | – |
| Loading screen (`splash-screen.tsx`) | **Implemented** | Progress is the real startup calls settling. Interactive dot logo, skippable, off switch in Settings, static with Reduce motion. | Keep | – |
| Design system (tokens, shadcn primitives, `SectionCard`, `DataSourceBadge`, states, `ConfirmDialog`, `RiskBadge`) | **Implemented** (0.2 redesign, `DESIGN_SYSTEM.md`) | Charcoal/violet/magenta tokens, typography scale, motion tokens with one Reduce motion switch, card/indicator/filter/tab/settings-row components. Light theme not built. | Light theme | P3 |
| Dashboard: system summary and quick actions | **Implemented** (Home, 0.2) | Heading, action button, three cards, chart, Quickstart (`lib/quickstart.ts`) and Updates. `system-summary.tsx`, `hardware-card.tsx` and `recent-activity.tsx` are unused and kept until deletion is confirmed. | Keep | – |
| Dashboard: live stats and utilisation chart                                                                       | **Implemented**                                                      | GPU % is sysinfo's sum over all engines, clamped to 100. No used VRAM.                                                                                                              | Per-engine GPU counters                                     | P1-3        |
| Dashboard: system health                                                                                          | **Implemented** (read-only heuristics, unit-tested)                  | Detail lines truncate at minimum width                                                                                                                                              | Layout fix                                                  | P1-1        |
| Dashboard: Recommendations | **Implemented** (derived) | Recommended tweaks are derived from detected state and drive the action button and Quickstart; no sample data left on Home. | Keep | – |
| Dashboard: Recently applied                                                                                       | **Implemented** (last 4 journal batches)                             | –                                                                                                                                                                                   | Keep                                                        | –           |
| Monitoring page | **Implemented** (Hardware page, 0.2) | `/monitoring` redirects to `/hardware`. Per-thread CPU, GPU, memory, storage, network tabs from the existing sampler. No disk throughput or persisted history. | Extend sampler | P1-3 |
| Game Mode page | **Implemented** (detection, ring, Configure, Add a Game) | Turning on reviews and applies the chosen tweaks; turning off reverts that batch. Stays on until turned off: session-scoped automatic undo is P2-2 | Automatic undo on game exit | P2-2 |
| Tweaks page | **Implemented** (topic tabs, 33 tweaks, card grid, filters, switch-off undo, break warnings) | Desktop flow not yet run on a VM | VM checklist | P0-7 |
| PC Cleaner page | **Implemented, not exercised on the dev PC** | Four tabs. Checked read-only in the real app, with every dialog opened and cancelled; cleaning, uninstalling, reinstalling, switching entries and the UAC path have only unit tests | Run them on a VM | P0-7, P1-5, P1-6 |
| Resources page | **Partial** | Copyable system report and Windows tool commands to copy; nothing is launched (no process spawning). | SFC/DISM per P0-8c | P2-8 |
| Settings page | **Implemented** (row-based sections, Update Centre) | No journal export or log viewer yet | Journal export, log viewer | P1-10 |
| Hardware detection (`system/hardware.rs`, `os.rs`, `power.rs`, `privilege.rs`)                                    | **Implemented** (snapshot once per session)                          | No hybrid-core topology, chassis type, HVCI or Secure Boot state. `TweakContext` has build, elevation and battery only.                                                             | Extend detection                                            | P1-2        |
| Live metrics sampler (`system/metrics.rs`)                                                                        | **Implemented** (1 s, 300 samples in memory)                         | No per-core, per-engine GPU, disk throughput or persisted history                                                                                                                   | Extend                                                      | P1-3        |
| Logging (`logging.rs`, `services/logger.ts`)                                                                      | **Implemented**                                                      | No in-app log viewer                                                                                                                                                                | Log viewer in Settings                                      | P1-10       |
| Error model (`error.rs`, `BackendError`)                                                                          | **Implemented**                                                      | –                                                                                                                                                                                   | Keep                                                        | –           |
| Engine contract, registry, metadata gate (`engine/mod.rs`)                                                        | **Implemented**                                                      | –                                                                                                                                                                                   | Keep                                                        | –           |
| Tweak catalogue (`engine/tweaks/`) | **Implemented** (37 tweaks with impact, `added_in` and break warnings; not yet VM-tested) | 27 from the pack review; 10 are Pro and above, 11 are single services guarded by `requires` | More documented tweaks | P0-7, P2-10 |
| Registry primitive (`engine/setting.rs`, `store.rs`)                                                              | **Implemented**                                                      | Only DWORD, QWORD, SZ and EXPAND_SZ, by design                                                                                                                                      | Keep                                                        | –           |
| Other primitives (power, services, AppX, QoS, files)                                                              | **Not started**                                                      | The journal's `changes.target` column stores a JSON `RegTarget`, so a second primitive needs a schema migration and a generalised target type                                       | Generalise the target before adding power plans             | P1-4        |
| Policy denylist (`engine/policy.rs`)                                                                              | **Implemented**, checked at registration, planning and in the broker | –                                                                                                                                                                                   | Keep                                                        | –           |
| Change journal (`engine/journal.rs`, SQLite)                                                                      | **Implemented** (Restore Centre; labelled change records)              | –                                                                                                                                                                                   | Export                                                      | P1-10       |
| Plan → apply → verify → rollback → revert (`engine/service.rs`, `plan.rs`) | **Implemented** (change inspector, per-batch and per-tweak undo) | Per-tweak undo restores the newest record per value, non-clobbering | Keep | – |
| Elevation broker (`engine/broker.rs`)                                                                             | **Wired, not verified** (pipe protocol tested over real pipes)       | **The real UAC launch and an elevated write have never run.** No SID-restricted DACL or replay protection (documented residual risk).                                               | Run the VM checklist in `docs/testing.md`                   | P0-7        |
| Engine IPC commands and TS wrappers                                                                               | **Implemented**, all used by the UI                                  | –                                                                                                                                                                                   | Keep                                                        | –           |
| Browser preview backend (`services/preview.ts`) | **Implemented** | Six "Preview sample" tweaks, startup and cleaner samples, mock update feeds (`?update=`); plan/apply/revert are refused, never simulated | Keep | – |
| Restore points                                                                                                    | **Not started**                                                      | –                                                                                                                                                                                   | `SRSetRestorePointW` before Medium+ batches                 | P1-7        |
| Restart handling                                                                                                  | **Partial** (`RestartRequirement` in metadata and plans)             | No pending-restart UI                                                                                                                                                               | Tray and countdown                                          | P1-7        |
| Onboarding / first run                                                                                            | **Not started**                                                      | –                                                                                                                                                                                   | First-run flow                                              | P1-8        |
| Updates | **Implemented, tested end to end on the dev PC** (Tauri updater, Update Centre) | Superseded P0-8b. The `updater-main` feed is live. A local release build installed v0.2.2 over the old 0.1.0 Program Files copy through the in-app flow on 2026-11-04 | Keep | P1-11 |
| Code signing, installer                                                                                           | **Partial** (NSIS/MSI bundles configured; unsigned)                  | SmartScreen warnings; no revert-all on uninstall                                                                                                                                    | Release readiness                                           | P1-11       |
| Unit and component tests | **Implemented** | Counts in the verification line above | Grows with each feature | – |
| Policy guard tests (`src-tauri/tests/policy.rs`)                                                                  | **Implemented**                                                      | –                                                                                                                                                                                   | Keep                                                        | –           |
| End-to-end tests                                                                                                  | **Not started**                                                      | `docs/testing.md` lists Playwright as planned; it is used manually only                                                                                                             | `e2e/` suite against the preview                            | P1-9        |
| CI | **Implemented** | `release.yml` run 41884400627 passed its checks and published v0.2.0 | Keep | P1-9, P1-11 |
| Documentation                                                                                                     | **Partial**                                                          | Several stale statements (below)                                                                                                                                                    | Fixed in this update                                        | –           |

## Inconsistencies found

Fixed in docs with this update:

- `docs/windows-integration.md` said elevation "is decided in Phase 2" (the broker exists) and recommended
  `StartupApproved` (undocumented).
- `docs/testing.md` didn't mention the policy guard tests or the backend wrapper tests.
- `docs/build-release.md` planned `tauri-plugin-updater` without noting the guard that forbids it.
- Three phase-numbering schemes were in use (Phase 2–5, P0–P3 in the research roadmap, Phase "2a/2b" in
  `architecture.md`). [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md) now defines one scheme.

Fixed in the UI in stage 3 (P0-1); kept for the record:

- `pages/placeholder-pages.tsx`: "Phase 2/3/4/5" labels; "documented `powercfg` schemes"; "the same mechanism Task
  Manager uses".
- `pages/dashboard/recommendations.tsx`: "once the backup engine lands"; "Phase 2+" comment.
- `pages/settings-page.tsx`: "Phase 2" planned list including restore history, which the backend now provides.
- `stores/ui-store.ts`: comment refers to the SQLite store as "Phase 2".
- Gaming and Tweaks empty state: "The engine is connected…" is also shown in browser preview, where no engine
  is connected.

## Risks

1. **Unexercised engine.** The most safety-critical code has only been driven by tests. The first real use may
   expose UX or model problems (for example, per-snapshot revert only).
2. **UAC path untested.** The broker launch, the PID handshake under real elevation and an elevated write need a
   manual run on a VM.
3. **Primitive generalisation.** Power plans, services and QoS all need the journal to store more than registry
   targets. Doing this after the first UI slice avoids designing it blind.
4. **Evidence, not only documentation.** Many popular tweaks have no Microsoft documentation. The Library keeps them with
   a stated evidence tier and leaves them out of presets. Policy CSP / ADMX settings remain the most reliable source,
   at the cost of Windows showing "managed by your organization" on affected Settings pages.
5. **Library writes need a VM.** Scheduled task, power plan and GPU writes have only run on the simulated store, and no
   library tweak has been applied to a real PC.
