# Testing

| Layer | Tool | Location | Run |
|---|---|---|---|
| Pure TS logic | Vitest | `src/lib/*.test.ts` | `npm test` |
| Components / shell | Vitest + Testing Library (jsdom) | `src/**/*.test.tsx` | `npm test` |
| Rust units | `cargo test` | `#[cfg(test)]` modules | `npm run rust:test` |
| Rust against the real system | `cargo test` | `system::*::tests` | Windows only |
| End-to-end | Playwright (planned) | `e2e/` | Phase 2 |

## Notes

- Frontend tests run outside Tauri, so `services/backend.ts` serves `services/preview.ts` data. Tests assert that
  preview and sample data are labelled.
- Some Rust tests read the real machine (registry, sysinfo, the sampler thread). They assert invariants (build ≥ 10240,
  at least one disk), not specific values.
- Tweak modules (Phase 2+): unit-test plan/backup logic against a fake registry trait. Real apply/revert tests are
  `#[ignore]` and run on a disposable VM: `cargo test -- --ignored`.
- E2E: Tauri's WebDriver (`tauri-driver`) supports Windows; Playwright can drive the Vite preview for UI flows.

## Definition of done for a stage

`npm run check:all` passes, the app is launched and inspected visually, and docs are updated. Features are reported
as implemented, partial, sample or planned.
