# Testing

| Layer | Tool | Location | Run |
|---|---|---|---|
| Pure TS logic | Vitest | `src/lib/*.test.ts` | `npm test` |
| Components / shell | Vitest + Testing Library (jsdom) | `src/**/*.test.tsx` | `npm test` |
| Rust units | `cargo test` | `#[cfg(test)]` modules | `npm run rust:test` |
| Rust against the real system | `cargo test` | `system::*::tests`; `engine::store` (`#[ignore]`, writes `HKCU\Software\2VX-Test`) | Windows only; `cargo test -- --ignored` for the registry test |
| Policy guards | `cargo test` | `src-tauri/tests/policy.rs` | `npm run rust:test` |
| Backend wrappers in preview | Vitest | `src/services/backend.test.ts` | `npm test` |
| End-to-end | Playwright (used manually; suite not started) | `e2e/` | Backlog P1-9 |
| Engine UI flows (plan, apply, every outcome, revert) | Vitest with a mocked `services/backend` | `src/pages/tweaks/tweak-list.test.tsx`, `src/pages/history/history-page.test.tsx`, `src/lib/engine-text.test.ts` | `npm test` |
| Built-in tweak catalogue | `cargo test` | `engine::tweaks::tests` (registration, documented values, apply → revert against `MemoryStore`) | `npm run rust:test` |
| Elevation broker, real UAC | Manual, disposable VM | [checklist below](#vm-checklist-elevation-broker-and-first-tweaks) | Backlog P0-7 |

## Notes

- Frontend tests run outside Tauri, so `services/backend.ts` serves `services/preview.ts` data. Tests assert that
  preview and sample data are labelled.
- Some Rust tests read the real machine (registry, sysinfo, the sampler thread). They assert invariants (build ≥ 10240,
  at least one disk), not specific values.
- Tweak modules: unit-test registration and the plan against `MemoryStore`. Real apply/revert is checked by hand
  on a disposable VM with the checklist below.
- UI flow tests mock `services/backend` with `isDesktop: true` and the fixtures in `src/test/engine-fixtures.ts`.
  The browser preview serves two tweaks named "Preview sample: …" so the list can be laid out; applying there is
  refused, never simulated.
- E2E: Tauri's WebDriver (`tauri-driver`) supports Windows; Playwright can drive the Vite preview for UI flows.

## Definition of done for a stage

`npm run check:all` passes, the app is launched and inspected visually, and docs are updated. Features are reported
as implemented, partial, sample or planned.

## VM checklist: elevation broker and first tweaks

Backlog P0-7. Run on a **disposable** Windows 11 VM (and a Windows 10 22H2 VM when available) after taking a
checkpoint, signed in as an administrator account with UAC at its default level, so the app runs unelevated and HKLM
batches go through the broker. Use a build from `npm run app:build`, or `npm run app:dev` started from an
**unelevated** terminal. Never run this on a daily-use PC. Values to watch in `regedit`:

- `HKLM\SOFTWARE\Policies\Microsoft\Edge`: `StartupBoostEnabled`, `BackgroundModeEnabled`
- `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced`: `Start_TrackProgs`
- `HKCU\Control Panel\International\User Profile`: `HttpAcceptLanguageOptOut`

| # | Step | Expected |
|---|---|---|
| 1 | Open Tweaks | Four tweaks with a state read from the VM; the Edge tweaks say "Asks for administrator permission". No banner. Title bar shows "Standard user". |
| 2 | Select "Don't share your language list", Review | The dialog lists `HKCU\Control Panel\International\User Profile\HttpAcceptLanguageOptOut`, current → `1 (REG_DWORD)`, and no UAC notice. |
| 3 | Apply | **No UAC prompt.** "Applied and verified"; the row shows Applied; regedit shows `1`; the Restore Centre lists the batch. |
| 4 | Restore Centre → Undo → Undo | No UAC prompt; toast "Reverted"; the value is back to its earlier state (deleted if it was not set). |
| 5 | Select both Edge tweaks, Review | UAC notice in the dialog; two HKLM rows. |
| 6 | Apply, then **decline** UAC | "Administrator permission was not granted. Nothing was changed."; regedit unchanged. |
| 7 | Review again, Apply, **accept** UAC | Exactly one UAC prompt for 2VX Tweaks; "Applied and verified"; both values are `0`; `edge://policy` lists both. |
| 8 | Task Manager during step 7 | A second, elevated `2vx-tweaks.exe` exists only for the batch and exits afterwards. |
| 9 | Restore Centre → Undo the Edge batch, accept UAC | Both values removed; `edge://policy` no longer lists them (the empty `Edge` key may remain). |
| 10 | Apply the Edge batch, kill the app in Task Manager while the UAC prompt is open, restart the app | Either no new batch, or a banner "1 batch of changes didn't finish"; undoing it from the Restore Centre leaves neither value set. |
| 11 | Apply the Edge batch, set `StartupBoostEnabled` to `1` by hand, then Undo | That value is "Left as is" with an explanation; the other value is removed; toast "Partly reverted". |
| 12 | Settings → About → log folder | Apply, revert and broker start and stop are logged; no other registry values were written. |
| 13 | Restore the VM checkpoint | – |

Results:

| Date | Windows build | Tester | Steps passed | Notes |
|---|---|---|---|---|
| – | – | – | – | Not run yet |
