# Redesign roadmap

The makeover in controlled stages. All nine ran as one uncommitted change, and `npm run check:all` is run on the result.

## Stages

| Stage | Work | State |
| --- | --- | --- |
| 1. Audit and plan | `UX_AUDIT`, `INFORMATION_ARCHITECTURE`, `NAVIGATION_STRUCTURE`, `FEATURE_CATEGORISATION`, this file | Done |
| 2. Foundation | Design tokens, heading font, `navigation.ts` with tabs, URL-backed tabs, redirects, sidebar, title bar | Done |
| 3. Tweaks | Topic tabs and sections, simpler cards, an icon per tweak, break warnings in review | Done |
| 4. Gaming | Profiles built from existing tweaks | Done |
| 5. Split Hardware | Monitoring for live data, Hardware for specifications | Done |
| 6. Home | One screen: alerts, numbers, chart, health, summary | Done |
| 7. Other pages | Restore Centre, Cleaner list, Startup rows, Settings tabs, planned lists collapsed | Done |
| 8. Search | Ctrl K for pages, tabs and tweaks | Done |
| 9. Quality control | Playwright at 1320×840 and 1080×700, comparison with the reference pack, `check:all`, real-app check | Done |

Screenshots of the browser preview are in `docs/screenshots/redesign/`.

## Needs backend work (not part of the makeover)

The redesign leaves room for these without new navigation guesses. Each needs its own design, tests and VM check.

| Feature | Home in the new structure | Needs |
| --- | --- | --- |
| Cleaning files | PC Cleaner: selection and summary above the ranked list | Allowlisted delete command, in-use handling, confirmation (`docs/security.md`) |
| Disable startup entries | Startup row switch | Journaled primitive for `StartupApproved` |
| Power plans | Tweaks › Power tab | Power scheme primitive; no process launches |
| Network diagnostics, DNS | Monitoring › Network or a Network tab | Fixed-target commands |
| Game detection | Gaming | Process enumeration; never promise FPS |
| Benchmarks | Monitoring | A measurement that's repeatable |

## Risks kept in view

- **Old links and stored data.** Redirects cover old routes; `2vx-ui` keys are unchanged; tweak IDs don't change.
- **Drift between Rust and presentation.** A test reads the Rust catalogue and checks placement, icons and profiles.
- **Claims.** Profiles describe what changes, never how much faster a game gets.
