# 2VX Tweaks

Windows 10/11 PC optimisation suite. Tauri 2 (Rust backend) + React 19 / TypeScript / Vite / Tailwind 4 / shadcn/ui.
Safety and honesty come before tweak count. Read `docs/security.md` before touching anything that modifies the system.

## Commands

```bash
npm run app:dev        # desktop app with hot reload (Vite on :1420 + cargo run)
npm run dev            # browser-only UI preview; backend calls return simulated "Preview" data
npm run check:all      # everything below, in order: must pass before a stage is done
npm run typecheck
npm run lint
npm run format:check   # npm run format to fix
npm run test           # Vitest (jsdom)
npm run rust:fmt       # cargo fmt --check
npm run rust:check     # cargo clippy --all-targets -D warnings
npm run rust:test      # cargo test (some tests read the real system: run on Windows)
cargo run --manifest-path src-tauri/Cargo.toml --example snapshot   # dump the real SystemSnapshot JSON
```

Rust lives in `%USERPROFILE%\.cargo\bin`; add it to PATH in new shells if `cargo` is not found.

## Layout

```
src/                         React frontend
  app/                       App root, routes, navigation config
  components/ui/             shadcn/ui primitives (generated, lightly adapted; keep shadcn style)
  components/common/         design-system components (SectionCard, DataSourceBadge, states, ConfirmDialog…)
  components/layout/         shell: Sidebar, AccountMenu, TitleBar, WindowControls, PageHero
  pages/                     one folder/file per section
  hooks/                     data hooks (useSystemSnapshot, useLiveMetrics…)
  services/backend.ts        THE ONLY place that calls invoke(); typed wrappers per command
  services/preview.ts        simulated backend for browser preview/tests
  services/logger.ts         frontend → backend log forwarding
  stores/                    zustand, UI preferences only (keep `2vx-ui` fields compatible)
  types/                     TS mirrors of Rust serde types
  lib/                       pure helpers (format, health, profiles, tweak-presentation) with unit tests
src-tauri/src/
  commands/                  Tauri command handlers: thin, validated
  system/                    read-only system info (os, hardware, hardware_details, services, ping, metrics sampler,
                             privilege, file_info, firmware, wmi)
  engine/                    declarative tweaks, policy, journal, plan/apply/revert, elevation broker,
                             the PC Cleaner's fixed actions (cleanup, startup, apps) and maintenance (restore_points,
                             drives, repair)
  links.rs                   fixed web links and the fixed list of Windows tools
  report.rs                  system report export (save dialog, 2 MB cap)
  account.rs                 the local account: one email in a file, no server or sign-in
  logging.rs, error.rs
scripts/                     release helpers for CI: next-version, release-notes, check-update-feed (+ tests)
.github/workflows/            ci.yml (checks), release.yml (signed main-channel release + update feed)
docs/                        status, backlog, roadmap, architecture, modules, Windows APIs, security, testing,
                             release; research/ holds the EXM competitor analysis (git-ignored, local only)
```

## Conventions

- **Frontend never runs system operations.** Add a Rust command, list it in `src-tauri/build.rs` (app manifest)
  AND `src-tauri/capabilities/default.json`, then wrap it in `services/backend.ts`. Commands not in a capability
  are denied.
- Commands take typed, enumerable arguments. Never accept free-form paths, registry keys, shell strings or command names.
- Rust types use `#[serde(rename_all = "camelCase")]`; keep `src/types/*.ts` in sync by hand.
- Errors: return `AppResult<T>`; the frontend receives `{ code, message }` (`BackendError`).
- Every data card shows a `DataSourceBadge`: `live` (measured, solid dot), `derived` (computed from live, hollow dot),
  `sample` (placeholder) and `planned` (not built) as words. Never present sample data without the badge. In browser
  preview, live/derived show "Preview".
- Layout: pages fill the window (`flex flex-1 flex-col`; see `docs/DESIGN_SYSTEM.md`), surfaces use the `panel` utility,
  anything you can switch or act on is an EXM card (`OptimisationCard` in a `CardGrid`), never a list or a folding section,
  read-only data is single-line table rows, state is `StatusLabel` (dot and word), no pill boxes.
- Styling: Tailwind tokens from `src/styles/globals.css` only (no raw hex except Windows caption red). Status colours
  (`success`/`warning`/`destructive`/`info`) mean state, never decoration. Dark-only for now.
- File names kebab-case; components PascalCase; one page per folder under `pages/` when it has sub-components.
- Comments explain *why*. Match the surrounding density.

## Safety rules (non-negotiable)

Full rationale in `docs/security.md`. In short:
- Never disable Defender, Firewall, Windows Update, UAC, SmartScreen or other security features as an "optimisation".
- Every tweak cites references (enforced by `TweakRegistry::register`) and states its evidence: `Documented`, `Observed`
  (the value name is in a Windows or driver file) or `Community`. Community tweaks stay out of presets.
- Every change: backup first → apply → verify → roll back on failure. Dry-run (`plan`) must be side-effect free.
- No bulk service disabling, no deleting outside an allowlist of locations, no boot-config edits without explicit,
  informed confirmation (`ConfirmDialog` with `risk="high"`).
- Request elevation only for operations that need it. Never claim FPS gains without measurement.

## Status

Stages 1 (foundation) and 2 (safety engine) are done; **stage 3 (first real optimisations) is in progress**. The single source
of truth for state is `docs/PROJECT_STATUS.md`; work items are in `docs/FEATURE_BACKLOG.md` (P0–P3) and stages in
`docs/DEVELOPMENT_ROADMAP.md`. Use those stage and task IDs, not the older "Phase N" labels.

In short (0.6.2): the motion pass (`docs/DESIGN_SYSTEM.md`, Motion). `.spot` and `useSpotlight` light what you can act on, so never add them to read-only panels. `burst` in `lib/dot-burst.ts` is the
signature: how many dots fly says how much happened, and its speed is time-based for high refresh rates. `useStaggerIn` is a wave and slides from the tab's side (`noteTabStep`). `scroll-in` is a CSS scroll-driven animation on `[data-stagger]`.
Each section's tint is `AURORA` in `app-shell.tsx`, never a status colour. All of it follows Settings › Reduce motion, and no new motion library was added.

In short (0.6.1): a tab is one grid of cards, as on EXM, which has only two headings on Core (Core, Power) and none on most tabs. `TAB_HEADINGS` in `lib/tweak-presentation.ts` is the short list of
tabs with headings; don't add one to sort a tab into topics. `section` stays on a tweak as its topic, for its icon and Visuals' tab and filter. The Power tab joined Core, and Power profiles are on Presets. The default order is
`mostUsefulFirst` in `lib/tweak-order.ts`: `KNOWN_FIRST` is written by hand per tab, best known first, so put a tweak there if people will come looking for it. The rest follow with anything risky last, like EXM's Advanced scripts.

In short (0.6): a full read of EXM 2.2.1's files (`docs/research/EXM_2.2.1_DELTA.md`) gave the Restore Centre tabs (Changes, Restore Points, Fixes),
Undo everything, Optimize Drives (Hardware > Storage), Restart now (`announceRestart`), Timer hold, a read-only BIOS page (`lib/bios-guide.ts`, `/bios`), Downloads on Resources (`lib/downloads.ts`,
links only from `links.rs`), Reinstall all, `lib/known-games.ts` for Game Mode, Power profiles and the first-run dialog. The BIOS page never writes firmware (`firmware_is_never_written`): no kernel driver, ever.
`engine/repair.rs` is the one module that starts a program for its output, two constant commands, P0-8c. Restoring and deleting restore points stay in Microsoft's tools. `Wmi::open` is named that because the network
guard bans `connect(`. `__PATH` needs `SELECT *`. Long-running work lives in `stores/task-store.ts` so it outlives its page. None of the writes in 0.6 have run on a real PC yet: they need a VM.

In short (0.5): 320 tweaks, 79 of them on the new Visuals page (tabs and sections in `VISUALS_SECTION_TAB`, `pages/visuals/`, one icon each in
`lib/visual-icons.ts`). Tweaks is system optimisation; anything mainly about how Windows looks goes in Visuals, even if it saves a little, and a
switch that stops Microsoft promotion stays in Privacy. 1000 tweaks left the catalogue into `library/retired.rs` (plus `RETIRED` in `tweaks/mod.rs`
and `adapter::RETIRED`): they can't be listed or planned but `undoable` still finds them, the broker accepts their targets, and the Restore Centre shows
any still on the PC. Never delete one, and never reuse its ID. IDs don't change when a tweak moves. `docs/VISUALS_SECTION_AUDIT.md` lists every decision.
Two exceptions were added and have guard tests: `ALLOWED_VALUES` (two values under the UAC key) and `PRESENCE_KEYS` (the one key undo may remove).
A card's name must be true when its switch is on ("Turn off X", "Block X", "No X"). A setting Windows ignores on some builds gets `.builds(n)` or `.until(n)`. Tweak presets are
hand-written lists (`PRESET_TWEAKS`), not category rules. There's no Tasks tab: scheduled tasks sit with their topic (Privacy or GPU).

In short (0.4): the app was redesigned to EXM's layout, and there is no Library or Monitoring page. A tweak's page and tab
come from `category` and `section` (`CATEGORY_TAB`, `tweakPlacement` in `lib/tweak-presentation.ts`), with a short hand-placed
set in `TWEAK_SECTIONS`, and every tweak is a card. Every tweak page is `TweakGrid` with a `TweakView`: one search across the
page's tabs, the same filters and sorts, Unavailable last. Every page title is `PageHero`; the title bar is only the window
controls. The top-right action is only ever `RecommendedAction`. Presets, service presets and GPU profiles are all `PresetCard`, and
their facts are counts of real tweaks, never ratings. Services have one home, PC Cleaner. `docs/MODEL_CONTRIBUTIONS.md` records
which model built what.

In short (0.3): 1,221 more tweaks (0.5 cut these down; see above), generated into `engine/tweaks/library/<tab>.rs`
with handwritten helpers in `library/mod.rs`. Tabs and presets are in `src/app/navigation.ts` and `lib/tweak-presets.ts`. Besides
the registry, a `Setting` can target `ScheduledTask` (only ever turned off), `PowerPlan` (the active plan), `Gpu` (`Nvidia`,
`Amd` or `Intel`, resolved to that vendor's display adapter key) and `Adapter` (`Ethernet` or `Wifi`, resolved to the primary
adapter's driver key, limited to the documented keywords in `policy.rs`); all are handled in `engine/system_store.rs` and
`engine/store.rs`, with no process spawned. One target has one owner, even for the same value. `HardwareScope` keeps vendor tweaks
on their hardware. What was found, and why anything was left out, is in `docs/OPTIMISATION_LIBRARY_AUDIT.md` (+ `.csv`);
`docs/OPTIMISATION_IMPLEMENTATION_STATUS.md` lists every current tweak. Library and adapter writes have not run on a real PC yet: they need a VM.

In short (0.2): the app was restructured (`docs/INFORMATION_ARCHITECTURE.md`, `docs/NAVIGATION_STRUCTURE.md`,
`docs/DESIGN_SYSTEM.md`). Every feature has one primary location; pages and tabs live in `src/app/navigation.ts`, and
`/system` and `/history` redirect. 64 documented tweaks ship (`engine/tweaks/`, table in `docs/optimisation-modules.md`), placed by `lib/tweak-presentation.ts` on Tweaks, Hardware, Network and Advanced,
with per-tweak undo (`revert_tweak`), an impact rating, and a `warning` only when a named feature can break. A new tweak
needs an icon and a section in `lib/tweak-presentation.ts`; a test reads the Rust catalogue to enforce it. Gaming
profiles (`lib/profiles.ts`) are named lists of existing tweaks: "active" is derived from detected state, never stored,
and they make no FPS claims. Game Mode (`pages/gaming/`) applies a chosen list through the normal review and undoes that
batch with `revert_snapshot`; the game list stays in the UI because commands never take process names.
The window is a fixed 1280×820 with only minimise and close; Quickstart's ticks are derived where the PC can say (`lib/quickstart.ts`) and Patch Notes reads `CHANGELOG.md`, so write its
newest section for users. The Restore Centre and Resources are real pages. Hardware, Network and Advanced are `TweakGrid` with a `TweakView` (`pages/tweaks/tweak-views.ts`); device facts come from `get_device_report` (`system/devices.rs`, read-only) and links only from the fixed list in `links.rs`. Advanced's Security tab is a read-out: it must never gain a switch for a security feature. PC Cleaner has four tabs (`pages/cleaner/`):
System Cleaner, Services (two presets, a read-out of real service states, and the service tweaks), Uninstall (fixed app
table) and Autoruns (which replaced Startup; `/startup` redirects).
A signed Tauri updater with an Update Centre is built (`docs/UPDATE_SYSTEM.md`); the feed URL
is in `tauri.conf.json`, so every release build can update, and only `.github/workflows/release.yml` signs and publishes.
Every release is public with an installer for people to download (`2VX-Tweaks-Setup.exe`): the owner wants this for every update, and a guard test enforces it.
P0-8b (no updater) is superseded; P0-8a/c stand. The repo is public at `ryanvxc/2vx-tweaks`. Still open: the broker's real UAC path on a VM
(P0-7), a real update test on a VM. `HKCU\Software\Policies` is admin-only on Windows: never
target it from a `User` tweak. The updater private key lives in `%USERPROFILE%\.tauri\`, never in the repo.
Known limitations: GPU usage is sysinfo's sum over all engines (clamped 100); used VRAM not available system-wide yet.
Build: cargo can run out of memory at default parallelism here; set `CARGO_BUILD_JOBS=2` (or 1).
Research on EXM Tweaks (a competitor, studied statically) is in `docs/research/` (git-ignored, never publish it); never copy its code or assets.
