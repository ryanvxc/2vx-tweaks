# Writing an optimisation module

The contract lives in `src-tauri/src/engine/mod.rs`. Phase 1 ships **no** tweaks. This document is the gate each
future tweak must pass.

## The contract

```rust
pub trait Tweak: Send + Sync {
    fn meta(&self) -> &TweakMeta;                                        // static description
    fn detect(&self, ctx: &TweakContext) -> AppResult<Detection>;        // current vs target, no side effects
    fn plan(&self, ctx: &TweakContext) -> AppResult<Vec<PlannedChange>>; // dry run, no side effects
    fn apply(&self, ctx: &TweakContext) -> AppResult<Backup>;            // backup FIRST, then change
    fn revert(&self, ctx: &TweakContext, backup: &Backup) -> AppResult<()>;
    fn verify(&self, ctx: &TweakContext) -> AppResult<bool>;             // re-read and confirm
}
```

`TweakMeta` holds the ID, name, description, `details` (consequences shown before applying), category, risk,
permission, supported build range, the `recommended` flag, restart requirement and `references`. The registry
rejects invalid IDs, duplicate IDs and empty `references`.

## Checklist before a tweak is merged

1. **Purpose**: what it changes and the measurable effect it is *expected* to have. No folklore.
2. **References**: Microsoft Learn or vendor docs for each setting. If it's undocumented, it doesn't ship.
3. **Risk**: Low / Moderate / High, using the definitions in `RiskLevel`. Prefer not shipping High.
4. **Scope**: HKCU or per-user where possible (`Permission::User`). HKLM, services or power need `Administrator`.
5. **Builds**: tested min/max build. Windows 10 and 11 differ, so test both or restrict `supported_builds`.
6. **Backup**: capture every value `apply` touches, including "value did not exist" (revert must delete it).
7. **Atomicity**: if step N fails, undo steps 1..N-1 before returning the error.
8. **Verify**: re-read from the system, not from what was written.
9. **Policy awareness**: if a Group Policy or MDM value overrides the setting, `detect` returns `Unknown` and the UI
   explains why.
10. **Tests**: unit tests for plan/backup logic with a fake registry; integration test behind `#[ignore]` that applies
    and reverts on a real machine/VM.

## Layout

```
src-tauri/src/engine/
  mod.rs                    contract + registry
  tweaks/
    mod.rs                  register_all(&mut TweakRegistry)
    gaming/game_mode.rs     one tweak per file
```

ID format: `category.kebab-name`, e.g. `gaming.game-mode`. IDs are permanent, because backups reference them.
