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
  components/layout/         shell: Sidebar, AccountMenu, TitleBar, WindowControls, PageHeader
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
  system/                    read-only system info (os, hardware, metrics sampler, privilege)
  engine/                    declarative tweaks, policy, journal, plan/apply/revert, elevation broker
  account.rs                 the local account: one email in a file, no server or sign-in
  logging.rs, error.rs
scripts/                     release helpers for CI: next-version, release-notes, check-update-feed (+ tests)
.github/workflows/            ci.yml (checks), release.yml (signed main-channel release + update feed)
docs/                        status, backlog, roadmap, architecture, modules, Windows APIs, security, testing,
                             release; research/ holds the EXM competitor analysis
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
  lists are single-line table rows, state is `StatusLabel` (dot and word), no pill boxes.
- Styling: Tailwind tokens from `src/styles/globals.css` only (no raw hex except Windows caption red). Status colours
  (`success`/`warning`/`destructive`/`info`) mean state, never decoration. Dark-only for now.
- File names kebab-case; components PascalCase; one page per folder under `pages/` when it has sub-components.
- Comments explain *why*. Match the surrounding density.

## Safety rules (non-negotiable)

Full rationale in `docs/security.md`. In short:
- Never disable Defender, Firewall, Windows Update, UAC, SmartScreen or other security features as an "optimisation".
- No undocumented registry edits. Every tweak cites references (enforced by `TweakRegistry::register`).
- Every change: backup first → apply → verify → roll back on failure. Dry-run (`plan`) must be side-effect free.
- No bulk service disabling, no deleting outside an allowlist of locations, no boot-config edits without explicit,
  informed confirmation (`ConfirmDialog` with `risk="high"`).
- Request elevation only for operations that need it. Never claim FPS gains without measurement.

## Status

Stages 1 (foundation) and 2 (safety engine) are done; **stage 3 (first real optimisations) is in progress**. The single source
of truth for state is `docs/PROJECT_STATUS.md`; work items are in `docs/FEATURE_BACKLOG.md` (P0–P3) and stages in
`docs/DEVELOPMENT_ROADMAP.md`. Use those stage and task IDs, not the older "Phase N" labels.

In short (0.2): the app was restructured (`docs/INFORMATION_ARCHITECTURE.md`, `docs/NAVIGATION_STRUCTURE.md`,
`docs/DESIGN_SYSTEM.md`). Every feature has one primary location; pages and tabs live in `src/app/navigation.ts`, and
`/system` and `/history` redirect. 35 documented tweaks ship (`engine/tweaks/`, table in `docs/optimisation-modules.md`)
with per-tweak undo (`revert_tweak`), an impact rating, and a `warning` only when a named feature can break. A new tweak
needs an icon and a section in `lib/tweak-presentation.ts`; a test reads the Rust catalogue to enforce it. Gaming
profiles (`lib/profiles.ts`) are named lists of existing tweaks: "active" is derived from detected state, never stored,
and they make no FPS claims. Monitoring, Hardware, Startup (read-only), PC Cleaner (read-only scan), the Restore Centre
and Tools are real pages.
A signed Tauri updater with an Update Centre is built (`docs/UPDATE_SYSTEM.md`); the feed URL
is injected only by `.github/workflows/release.yml`, so local builds never update. P0-8b (no updater) is superseded;
P0-8a/c stand. Still open: the broker's real UAC path on a VM (P0-7), GitHub remote + signing secrets
(`docs/UPDATE_SETUP_GUIDE.md`), a real update test on a VM. `HKCU\Software\Policies` is admin-only on Windows: never
target it from a `User` tweak. The updater private key lives in `%USERPROFILE%\.tauri\`, never in the repo.
Known limitations: GPU usage is sysinfo's sum over all engines (clamped 100); used VRAM not available system-wide yet.
Build: cargo can run out of memory at default parallelism here; set `CARGO_BUILD_JOBS=2` (or 1).
Research on EXM Tweaks (a competitor, studied statically) is in `docs/research/`; never copy its code or assets.
