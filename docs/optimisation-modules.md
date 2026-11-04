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
A service start type is an ordinary write to `Services\<name>\Start`, so backup, verify and revert apply unchanged.
Such a tweak names the service's `ImagePath` in `requires`: without it, writing `Start` for a service that isn't
installed would create a broken service key. Power schemes will be added as a new `Setting` primitive with their
first tweak.

## Registration gate (enforced in `TweakRegistry::register`)

- ID is `category.kebab-name`, unique, never reused (the journal references it).
- `name`, `description` (≤ 160 chars) and `details` are non-empty.
- At least one reference, all `https://`.
- High risk is never `recommended`, and always has a `warning`.
- `warning`, when set, is 1 to 160 characters.
- At least one setting; no target twice; every target passes `policy::check_target` (Defender, Firewall, Windows
  Update, UAC, SmartScreen, VBS/HVCI, LSA, Secure Boot, SAM/SECURITY/BCD and protected services are refused).
- `permission` matches the hives: any HKLM target ⇒ `Administrator`, HKCU only ⇒ `User`.
- No two tweaks want different values for the same target.

`tests/policy.rs` runs the built-in registry through this gate on every `cargo test`.

## Checklist before a tweak is merged

1. **Purpose**: what it changes and the measurable effect it is *expected* to have. No folklore.
2. **References**: Microsoft Learn or vendor docs for each setting. If it's undocumented, it doesn't ship.
3. **Risk**: Low / Moderate / High, using the definitions in `RiskLevel`. Prefer not shipping High.
   **Warning**: set `warning` only when a named feature stops working, and say which. Cards show it as the red
   triangle, so a warning on a merely cautious tweak would devalue the ones that matter.
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
- Policy CSP pages list the editions a policy applies to. A policy not documented for Home uses
  `EditionScope::ProOrHigher`, which reports `unsupportedEdition` on Home instead of writing a value nothing reads.

## Shipped tweaks

| ID | Target | Value | Permission | Reference |
|---|---|---|---|---|
| `performance.edge-startup-boost-off` | `HKLM\SOFTWARE\Policies\Microsoft\Edge\StartupBoostEnabled` | `0` | Administrator | [StartupBoostEnabled](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/startupboostenabled) |
| `performance.edge-background-mode-off` | `HKLM\SOFTWARE\Policies\Microsoft\Edge\BackgroundModeEnabled` | `0` | Administrator | [BackgroundModeEnabled](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/backgroundmodeenabled) |
| `privacy.app-launch-tracking-off` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\Start_TrackProgs` | `0` | User | [Manage connections, § 18.1](https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#181-general) |
| `privacy.language-list-opt-out` | `HKCU\Control Panel\International\User Profile\HttpAcceptLanguageOptOut` | `1` | User | same page, § 18.1 |

All are Low risk and not `recommended`: no measured benefit has been shown on a user's PC yet. Considered and not
shipped: Game Mode and Game Bar values (no Microsoft documentation of the registry values), visual-effects values
(undocumented).

The policy denylist also protects the Edge policies that must not be weakened (`SmartScreenEnabled`,
`SmartScreenPuaEnabled`, `PreventSmartScreenPromptOverride*`, `SmartScreenForTrustedDownloadsEnabled`,
`TyposquattingCheckerEnabled`, `SitePerProcess`), because the first tweaks share that key.

## Added from the third-party pack review (0.2)

A 375+ line batch file (titled "EXM TWEAKING PACK V2") was read, not run, and every line triaged. Only settings Microsoft
documents were taken; names, descriptions and details are written here, and a setting is implemented from Microsoft's
page, not the script, where the two differ (`PeriodInNanoSeconds` is set to 0, not deleted; the supported
`AppPrivacy` policies replace the pack's `ConsentStore` writes).

| ID | Target = value | Risk | Impact | Edition | Warning shown |
|---|---|---|---|---|---|
| `gaming.game-dvr-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR\AllowGameDVR` = `0` | Moderate | Medium | Pro+ | Game Bar recording, screenshots and broadcasting stop working. |
| `gaming.mmcss-games-priority` | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games\Priority` = `6`<br>`HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games\Scheduling Category` = `High` | Moderate | Medium | All | – |
| `performance.foreground-boost` | `HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl\Win32PrioritySeparation` = `38` | Low | Low | All | – |
| `performance.last-access-off` | `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\NtfsDisableLastAccessUpdate` = `0x8000_0001` | Moderate | Low | All | Backup, archive and cleanup tools that sort by last access can misjudge files. |
| `privacy.activity-history-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\EnableActivityFeed` = `0`<br>`HKLM\SOFTWARE\Policies\Microsoft\Windows\System\PublishUserActivities` = `0`<br>`HKLM\SOFTWARE\Policies\Microsoft\Windows\System\UploadUserActivities` = `0` | Moderate | Low | Pro+ | – |
| `privacy.app-account-info-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\AppPrivacy\LetAppsAccessAccountInfo` = `2` | Moderate | Low | Pro+ | Store apps that show your account name or picture can't read them. |
| `privacy.app-diagnostics-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\AppPrivacy\LetAppsGetDiagnosticInfo` = `2` | Moderate | Low | Pro+ | – |
| `privacy.app-location-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\AppPrivacy\LetAppsAccessLocation` = `2` | Moderate | Low | Pro+ | Maps, weather and other location features stop working in Store apps. |
| `privacy.app-notification-access-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\AppPrivacy\LetAppsAccessNotifications` = `2` | Moderate | Low | Pro+ | Apps that mirror or read your notifications stop working. |
| `privacy.background-apps-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\AppPrivacy\LetAppsRunInBackground` = `2` | Moderate | Medium | Pro+ | Message, email and voicemail notifications and background mail sync can stop working. |
| `privacy.diagnostic-data-minimum` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry` = `0` | Moderate | Low | Pro+ | Windows Insider builds stop being offered: they need optional diagnostic data. |
| `privacy.feedback-requests-off` | `HKCU\Software\Microsoft\Siuf\Rules\NumberOfSIUFInPeriod` = `0`<br>`HKCU\Software\Microsoft\Siuf\Rules\PeriodInNanoSeconds` = `0` | Low | Low | All | – |
| `privacy.inking-typing-collection-off` | `HKCU\Software\Microsoft\InputPersonalization\RestrictImplicitTextCollection` = `1`<br>`HKCU\Software\Microsoft\InputPersonalization\RestrictImplicitInkCollection` = `1` | Low | Low | All | – |
| `privacy.online-speech-off` | `HKCU\Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy\HasAccepted` = `0` | Low | Low | All | Voice typing and other cloud speech features stop working. |
| `services.bluetooth-off` | `HKLM\SYSTEM\CurrentControlSet\Services\bthserv\Start` = `4` | Moderate | Low | All | Bluetooth headsets, mice, keyboards and controllers stop working. |
| `services.compat-assistant-off` | `HKLM\SYSTEM\CurrentControlSet\Services\PcaSvc\Start` = `4` | Moderate | Low | All | Windows stops offering compatibility fixes for older programs. |
| `services.diagtrack-off` | `HKLM\SYSTEM\CurrentControlSet\Services\DiagTrack\Start` = `4` | Moderate | Low | All | Windows Insider builds and problem reports can stop working. |
| `services.geolocation-off` | `HKLM\SYSTEM\CurrentControlSet\Services\lfsvc\Start` = `4` | Moderate | Low | All | Location, Find my device and automatic time zone stop working. |
| `services.maps-broker-off` | `HKLM\SYSTEM\CurrentControlSet\Services\MapsBroker\Start` = `4` | Moderate | Low | All | Offline maps stop updating in apps that use them. |
| `services.print-notify-off` | `HKLM\SYSTEM\CurrentControlSet\Services\PrintNotify\Start` = `4` | Moderate | Low | All | Printer notifications and extensions stop working. |
| `services.print-spooler-off` | `HKLM\SYSTEM\CurrentControlSet\Services\Spooler\Start` = `4` | Moderate | Low | All | All printing stops, including printing to PDF. |
| `services.sensor-off` | `HKLM\SYSTEM\CurrentControlSet\Services\SensorService\Start` = `4` | Moderate | Low | All | Auto-rotate and other sensor features stop working. |
| `services.sensor-monitoring-off` | `HKLM\SYSTEM\CurrentControlSet\Services\SensrSvc\Start` = `4` | Moderate | Low | All | Adaptive brightness and other sensor-driven changes stop working. |
| `services.sync-host-off` | `HKLM\SYSTEM\CurrentControlSet\Services\OneSyncSvc\Start` = `4` | Moderate | Low | All | Mail, Calendar and People apps stop syncing. |
| `services.wap-push-off` | `HKLM\SYSTEM\CurrentControlSet\Services\dmwappushservice\Start` = `4` | Moderate | Low | All | Work or school device management can fail on this PC. |
| `services.xbox-auth-off` | `HKLM\SYSTEM\CurrentControlSet\Services\XblAuthManager\Start` = `4` | Moderate | Low | All | Xbox sign-in fails, which breaks the Xbox app and Game Pass games. |
| `services.xbox-save-off` | `HKLM\SYSTEM\CurrentControlSet\Services\XblGameSave\Start` = `4` | Moderate | Low | All | Xbox cloud saves stop syncing. |
| `windows.storage-sense-off` | `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy\01` = `0` | Low | Low | Pro+ | Windows stops freeing disk space automatically when it runs low. |
| `windows.window-animations-off` | `HKLM\SOFTWARE\Policies\Microsoft\Windows\DWM\DisallowAnimations` = `1` | Low | Low | Pro+ | – |

Service guidance is Microsoft's page for Windows Server 2016, the closest published; each tweak says so in `details`.
None of the 2VX impact ratings is a measurement.

### Reviewed and not shipped

- **Refused: security features** (`docs/security.md`). UAC (`EnableLUA`), Spectre/Meltdown overrides, `DisableTsx = 0`,
  `Set-ProcessMitigation`, `nx optout`, `hypervisorlaunchtype off`, `tpmbootentropy`, all Windows Update lines
  (services, `NoAutoUpdate`, `SetDisableUXWUAccess`, deleting `SoftwareDistribution`), `WerSvc`/`Wecsvc` (Microsoft's
  guidance says not to disable them).
- **Refused: unsafe mechanics.** Downloading a `.pow` from a Discord CDN, deleting the built-in power plans, `nsudo`
  and the Atlas scripts, killing processes, `regedit` exports.
- **Not possible in the engine.** No process launches: `bcdedit` (timers, boot UI), `powercfg`, `fsutil memoryusage`,
  `Disable-MMAgent`, `lodctr`. No `REG_BINARY`: mouse curves, `GlobalPowerPolicy`. No per-device loops: USB `Enum\…`
  power values.
- **Undocumented or wrong.** Memory pool sizes, `Start` under Memory Management, `VxD\BIOS`, `Processor` C-states,
  `PowerSettings` keys filled with MMCSS names, `ThreadPriority` values, `TaggedEnergy`, `nvlddmkm` preemption,
  `GameConfigStore`, `HwSchMode`, `SvcHostSplitThresholdInKB`, input queue sizes, `VisualEffects`, `SettingSync`,
  IFEO priorities (`csrss.exe`, Fortnite), `DisableTaskOffload` on the wrong key, mouse and keyboard timing values
  (no official registry documentation was found for them).
- **Not writable.** `HKCU\…\Policies\Explorer` and `HKCU\Software\Policies` are admin-only ACLs, so a `User` tweak
  can't set them.
- **Bulk or no guidance.** `DiagTrack`, `DPS`, `Wdi*`, `FontCache`, `WbioSrvc` (Windows Hello), `CertPropSvc`, `fhsvc`,
  `GraphicsPerfSvc`, `ssh-agent`, `XboxGipSvc`, `XboxNetApiSvc`, `BTAGService`, `WSearch`, `RemoteAccess`,
  `AppVClient`, `tzautoupdate`: no client guidance to cite.
- **No-ops.** Most other MMCSS tasks, `FontSmoothing`, `EnablePrefetcher = 3`, `ClearPageFileAtShutdown = 0`,
  `LargeSystemCache = 0` and similar repeat Windows defaults; the pack's Audio task lowers priority from 6 to 2.
