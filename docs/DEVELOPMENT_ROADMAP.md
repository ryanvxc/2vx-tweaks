# Development roadmap

The single stage scheme for 2VX Tweaks. It replaces the "Phase 1–5" labels and the "2a/2b" split used earlier; task
IDs refer to [FEATURE_BACKLOG.md](FEATURE_BACKLOG.md), current state to [PROJECT_STATUS.md](PROJECT_STATUS.md).

| Stage                        | Goal                                                                                                                                          | Backlog             | Status               |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | -------------------- |
| 1. Foundation                | Shell, design system, dashboard, system info, live metrics, logging                                                                           | –                   | **Done** (`bd0c5e2`) |
| 2. Safety engine             | Journal, typed registry primitive, policy denylist, plan → apply → verify → rollback → revert, elevation broker, metadata gate, policy guards | Research roadmap P0 | **Done** (`cf28ed4`) |
| 3. First real optimisations  | The engine used end to end: tweak list, change inspector, history and revert, first audited tweaks, broker verified                           | P0-1 … P0-8         | **In progress**      |
| 4. First installable release | Visual identity, detection, monitoring, power plans, startup, cleaner, restore points, onboarding, CI, signing                                | P1-1 … P1-11        | Planned              |
| 5. Gaming and depth          | Game library and session profiles, frame-time measurement, QoS, debloat, services, repair set, tools                                          | P2-1 … P2-10        | Planned              |
| 6. Advanced                  | NIC tuning, NVIDIA profiles, driver advice, input insight, scheduled re-apply, light theme                                                    | P3-1 … P3-6         | Later                |

## Stage 3: first real optimisations (in progress)

**Progress.** Done and tested: P0-1 (copy, quick actions), P0-2 (engine banner, Settings engine card), P0-3 (tweak
list with detected state), P0-4 (change inspector), P0-5 (History, undo, "Recently applied", labelled change records),
P0-6 (four documented tweaks, two HKLM). P0-8 decided and recorded in `docs/security.md`. Waiting on a person: P0-7 (VM
checklist written in `docs/testing.md`, not run yet).

**Exit-criteria note.** "Both an HKCU and an HKLM tweak end to end in the desktop app" is met by the catalogue in
code; it counts as passed only after the VM checklist. The HKCU tweaks are privacy settings, not performance ones:
no documented HKCU performance value was found that passes the documented-only rule.

**Why this stage next.** The safety engine is the most important code in the project, and nothing uses it yet. It
has never applied a change chosen by a user, and its UAC path has never run. Building more primitives (power plans,
services) or more pages first would pile features on an untested core, and the journal would need its target type
generalised without real usage to inform the design. A thin vertical slice does the opposite: it proves the engine,
delivers the first real value, and exposes model problems while they are cheap to fix. The visual refresh is
valuable but cosmetic and independent, so it doesn't need to go first.

**Scope**

1. P0-1 Stale copy and dead-end quick actions (also restores trust in what the UI says).
2. P0-2 Engine health banner.
3. P0-3 Tweak list with detected state.
4. P0-4 Change inspector and apply flow.
5. P0-5 History and revert, including the "Recently applied" card.
6. P0-6 First 3–5 tweaks, after a documentation pass; at least one HKLM.
7. P0-7 Broker verification on a VM (needs a person for UAC).
8. P0-8 Policy decisions: `StartupApproved`, updater, repair tools.

P0-1, P0-2, P0-6 and P0-8 are independent of each other. P0-3 → P0-4 → P0-5 are sequential. P0-7 needs P0-6.

**Exit criteria**

- A user can pick a tweak, see the exact diff, apply it, see it verified, find it in history and revert it, in the
  desktop app, for both an HKCU and an HKLM tweak.
- Every engine outcome (`completed`, `rolledBack`, `failed`, `conflict`, `elevation_required`, journal unavailable,
  interrupted) has a tested UI state.
- The VM checklist in `docs/testing.md` is recorded as passed.
- No UI copy contradicts the code or `docs/security.md`.
- `npm run check:all` passes; docs updated.

**Out of scope for stage 3:** new primitives (power, services, files), restore points, the theme refresh, onboarding.

## Stage 4: first installable release

Order by dependency: P1-1 (theme) and P1-2 (detection) and P1-3 (monitoring) can start any time; P1-4 (generalised
target + power plans) before P1-5 (startup); P1-7 (restore points) before P1-8 (onboarding); P1-11 (signing,
updater) last. Exit: signed installer, every page functional or removed from navigation, CI green.

## Principles that order the work

1. Real functionality before breadth: a page ships working or stays clearly marked as planned.
2. Safety before features: nothing bypasses the plan → confirm → journal → verify path.
3. Measure before claiming: no performance claims without numbers from the user's machine.
4. Documented settings only, unless a deliberate, recorded exception says otherwise.
