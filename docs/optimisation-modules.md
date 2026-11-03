# Writing an optimisation module

The contract lives in `src-tauri/src/engine/mod.rs`. The catalogue is in `src-tauri/src/engine/tweaks/`; the
first audited set (backlog P0-6) is listed at the end of this document.
This document is the gate each tweak must pass.

## The contract: tweaks are declarative

A tweak states **what** it wants. It contains no code that runs against the system.

```rust
pub trait Tweak: Send + Sync {
    fn meta(&self) -> &TweakMeta;      // static description
    fn settings(&self) -> &[Setting];  // desired end state, fixed (no context, no I/O)
}

// The usual case needs no impl: metadata plus a const list of settings.
pub const GAME_MODE: SettingsTweak = SettingsTweak {
    meta: TweakMeta { id: "gaming.game-mode", /* … */ },
    settings: &[Setting::dword(Hive::CurrentUser, r"Software\Microsoft\GameBar", "AutoGameModeEnabled", 1)],
};
```

`Setting { target, value }` with `value: None` means "this value must not exist". The engine
(`engine/service.rs`) does everything else, the same way for every tweak:

| Step   | What the engine does                                                                                    |
| ------ | ------------------------------------------------------------------------------------------------------- |
| detect | reads each target and compares it with the desired value → `applied` / `partial` / `notApplied` / `unknown` |
| plan   | lists only the targets that differ, with `from → to`. Writes nothing.                                     |
| apply  | re-checks the plan, journals the exact before-value, writes, reads back, rolls the batch back on failure |
| revert | restores before-values; skips (reports) any value changed by something else since                        |

So backup, atomicity, verification and revert are not per-tweak work; the checklist below is about choosing the
right settings and describing them honestly. Supported value types: `REG_DWORD`, `REG_QWORD`, `REG_SZ`,
`REG_EXPAND_SZ`. A target that currently holds another type is refused, because it couldn't be restored exactly.
Power schemes and service start types will be added as new `Setting` primitives with their first tweak.

## Registration gate (enforced in `TweakRegistry::register`)

- ID is `category.kebab-name`, unique, never reused (the journal references it).
- `name`, `description` (≤ 160 chars) and `details` are non-empty.
- At least one reference, all `https://`.
- High risk is never `recommended`.
- At least one setting; no target twice; every target passes `policy::check_target` (Defender, Firewall, Windows
  Update, UAC, SmartScreen, VBS/HVCI, LSA, Secure Boot, SAM/SECURITY/BCD and protected services are refused).
- `permission` matches the hives: any HKLM target ⇒ `Administrator`, HKCU only ⇒ `User`.
- No two tweaks want different values for the same target.

`tests/policy.rs` runs the built-in registry through this gate on every `cargo test`.

## Checklist before a tweak is merged

1. **Purpose**: what it changes and the measurable effect it is *expected* to have. No folklore.
2. **References**: Microsoft Learn or vendor docs for each setting. If it's undocumented, it doesn't ship.
3. **Risk**: Low / Moderate / High, using the definitions in `RiskLevel`. Prefer not shipping High.
4. **Scope**: HKCU where possible. HKLM means one UAC prompt per batch (the elevated broker).
5. **Builds and device**: tested min/max build (`supported_builds`) and `device` (`DesktopOnly` for anything that
   costs battery life).
6. **Desired state is complete**: list every value the setting needs, including values that must be absent.
7. **Policy awareness**: if Group Policy or MDM overrides the setting, the read-back check fails and the batch rolls
   back. Say so in `details` when it's likely.
8. **Tests**: a unit test that the tweak registers and that its plan against a `MemoryStore` is what you expect.
   Manual apply/revert on Windows 10 22H2 and Windows 11 VMs.

## Layout

```
src-tauri/src/engine/
  mod.rs                    contract, registry, metadata gate
  tweaks/
    mod.rs                  register_all(&mut TweakRegistry)
    <category>/<tweak>.rs   one tweak per file, `pub const TWEAK: SettingsTweak`
```

## Hive and permission pitfalls

- `HKCU\Software\Policies` is writable only by Administrators and SYSTEM, even though it is in the user's hive. A
  tweak targeting it would be registered as `User` (the permission follows the hive) and then fail with
  `elevation_required`; elevating would write the elevated account's hive, not necessarily the signed-in user's.
  `engine::tweaks::tests` refuses such targets. Use the HKLM policy, or the documented non-policy value.
- Machine policies (`HKLM\SOFTWARE\Policies\…`) make Settings or the affected app say "managed by your organization".
  Say so in `details`.
- Policy CSP pages list the editions a policy applies to. A policy not documented for Home is not shipped until
  `TweakContext` knows the edition (e.g. `AllowGameDVR`, Pro and above).

## Shipped tweaks

| ID | Target | Value | Permission | Reference |
|---|---|---|---|---|
| `performance.edge-startup-boost-off` | `HKLM\SOFTWARE\Policies\Microsoft\Edge\StartupBoostEnabled` | `0` | Administrator | [StartupBoostEnabled](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/startupboostenabled) |
| `performance.edge-background-mode-off` | `HKLM\SOFTWARE\Policies\Microsoft\Edge\BackgroundModeEnabled` | `0` | Administrator | [BackgroundModeEnabled](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/backgroundmodeenabled) |
| `privacy.app-launch-tracking-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\Start_TrackProgs` | `0` | User | [Manage connections, § 18.1](https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#181-general) |
| `privacy.language-list-opt-out` | `HKCU\Control Panel\International\User Profile\HttpAcceptLanguageOptOut` | `1` | User | same page, § 18.1 |

All are Low risk and not `recommended`: no measured benefit has been shown on a user's PC yet. Considered and not
shipped: Game DVR policy (`AllowGameDVR`, not documented for Home), Game Mode and Game Bar values (no Microsoft
documentation of the registry values), visual-effects values (undocumented).

The policy denylist also protects the Edge policies that must not be weakened (`SmartScreenEnabled`,
`SmartScreenPuaEnabled`, `PreventSmartScreenPromptOverride*`, `SmartScreenForTrustedDownloadsEnabled`,
`TyposquattingCheckerEnabled`, `SitePerProcess`), because the first tweaks share that key.
