# Changelog

Main-channel builds are released automatically from `main` (see `docs/GITHUB_RELEASE_PIPELINE.md`); their
notes come from commit subjects. This file records the larger milestones.

## 0.2.0 (unreleased)

### Redesign
- New visual identity: charcoal surfaces, violet icons and focus, magenta primary actions and toggles;
  Inter typography scale; reduced-motion support. See `docs/DESIGN_SYSTEM.md`.
- Grouped sidebar with privilege and version footer; compact mode.
- Home: greeting, stat tiles, live utilisation chart with a series filter, Quickstart, recommended
  tweaks derived from detected state (the sample recommendations are gone), health and recent changes.
- System Tweaks and Gaming: card grid with category tabs, search, filters, sort, impact estimate,
  indicators, "Stage recommended", a review bar and a details dialog with values and references.
- Settings: row-based sections (General, Appearance, Notifications, Updates, Behaviour, Security, About).

### Features
- Switch a tweak off to undo exactly what 2VX changed (`revert_tweak`), after a confirmation.
- Four new documented tweaks: Edge shopping assistant, advertising ID, Edge sidebar, Edge
  recommendations.
- Hardware page: per-thread CPU load, GPU, memory, storage and network tabs (replaces Monitoring).
- Startup page: read-only inventory of Run/RunOnce values and Startup-folder shortcuts, with Task
  Manager's disabled state.
- PC Cleaner: read-only scan of allowlisted temp, cache and error-report folders and the Recycle Bin.
- Tools: copyable system report and Windows tool commands.
- Signed in-app updates (Tauri updater) with an Update Centre, release notes before install, an optional
  automatic check and a Home notice. Nothing downloads or installs without a click.

### Build and release
- GitHub Actions: CI on pull requests and branches; main-channel release with monotonic versions,
  signed NSIS prerelease and a validated update feed. Publishing needs the signing secrets
  (`docs/UPDATE_SETUP_GUIDE.md`).
- App icons regenerated from the new logo.

## 0.1.0

- Stage 1 foundation (Tauri 2 + React shell, dashboard, live metrics) and stage 2 safety engine
  (journal, plan/apply/verify/rollback/revert, elevation broker, policy denylist).
- Stage 3 first optimisations: four documented tweaks, change inspector, History with undo.
- Per-machine NSIS installer into Program Files.
