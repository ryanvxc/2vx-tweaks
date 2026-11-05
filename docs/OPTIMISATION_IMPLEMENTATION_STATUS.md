# Optimisation implementation status

320 tweaks are offered after the 0.5 audit, 79 of them in Visuals. [VISUALS_SECTION_AUDIT.md](VISUALS_SECTION_AUDIT.md) says what moved, what was retired and why. Retired tweaks are no longer offered, but stay in the engine so anything they changed can still be undone.

Implemented means the tweak is in the Rust engine with detection, backup, apply, verify, rollback and revert, not just a
card in the UI.

## Totals

| Risk | Tweaks |
| --- | --- |
| Low | 266 |
| Moderate | 53 |
| High | 1 |

Critical is not used: nothing rated Critical ships.

| Impact | Tweaks |
| --- | --- |
| Low | 315 |
| Medium | 5 |

60 tweaks carry a warning because they can break a named feature. 210 need administrator rights.
70 need a restart and 27 a sign-out.

## What was tested

| Check | How | Result |
| --- | --- | --- |
| Apply, verify, idempotent re-plan, restore of the original value | `library_tweaks_apply_once_and_restore_what_the_pc_had`, every usable library tweak, two passes (desktop with Intel, laptop with AMD) | Pass, simulated store |
| Duplicate prevention | `rejects_conflicting_targets_across_tweaks`, plus `TweakRegistry::register` at start-up for the whole catalogue | Pass |
| Hardware and build checks | `hardware_tweaks_stay_on_their_hardware` | Pass |
| Tasks are only ever turned off | `scheduled_tasks_are_only_turned_off` | Pass |
| Security denylist | `scheduled_tasks_stay_off_security_paths`, `core_services_and_mitigations_stay_protected`, `non_registry_values_keep_their_shape` | Pass |
| Text fits its card | `library_tweaks_read_well_in_a_row` | Pass |
| Real read on a Windows PC | Every tweak's state read on the reference PC, nothing written | Pass |
| Retired tweaks stay undoable and can't be planned | `a_retired_tweak_can_be_neither_listed_nor_planned_but_keeps_its_targets`, `a_change_outlives_the_tweak_that_made_it` | Pass |
| Pages | Every tab, search, filters and details in the real app | Pass |

## Not tested

- No library tweak has been applied to a real PC. That needs a virtual machine, and it was deliberately not done on the
  development PC.
- Scheduled task, power plan and GPU writes have only run on the simulated store.
- The elevated broker's real UAC path is still open (P0-7).

## Limits worth knowing

- Power tweaks act on the active plan. Undo writes to whichever plan is active at that time, and the details say so.
- A library tweak undoes only what it changed. It doesn't know about changes made outside 2VX after it was applied.

## All tweaks

| ID | Name | Location | Risk | Impact | Evidence | Writes | Targets | Needs | Compatibility |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `apps.brave-background` | No Brave background mode | Apps / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-3p-cookies` | No Chrome third-party cookies | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-autofill-card` | No Chrome card autofill | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-background` | No Chrome background mode | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-cookies-session` | Clear Chrome cookies on close | Apps / Chrome | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-geolocation` | Block Chrome location requests | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-metrics` | No Chrome metrics | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-notifications` | Block Chrome notifications | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-password-manager` | No Chrome password manager | Apps / Chrome | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-prediction` | No Chrome page preloading | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-promos` | No Chrome promotional tabs | Apps / Chrome | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.chrome-sb-reporting` | Fewer Safe Browsing reports | Apps / Chrome | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.chrome-sign-in` | No Chrome sign-in and sync | Apps / Chrome | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.chrome-spellcheck` | No Chrome spellcheck service | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-suggest` | No Chrome search suggestions | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-url-data` | No Chrome URL sharing | Apps / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-3p-cookies` | Block Edge third-party cookies | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-autofill-card` | No Edge card autofill | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-experiments` | No Edge experiments | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-nav-errors` | No Edge similar-site suggestions | Apps / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-personalization` | No Edge personalisation | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-promotional` | No Edge promotional tabs | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-rewards` | No Edge rewards | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-searchbar` | No Edge search bar | Apps / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-suggest` | No Edge search suggestions | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-telemetry` | No Edge diagnostic data | Apps / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-top-sites` | No Edge default top sites | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-tracking` | Strict Edge tracking prevention | Apps / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-default-agent` | No Firefox default browser agent | Apps / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-studies` | No Firefox studies | Apps / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-telemetry` | No Firefox telemetry | Apps / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.new-outlook-toggle` | Hide the New Outlook toggle | Apps / Office | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `apps.office-telemetry` | Less Office telemetry | Apps / Office | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `apps.onedrive-startup` | No OneDrive at startup | Apps / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.onedrive-sync` | Turn off OneDrive | Apps / Other apps | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.spotify-startup` | No Spotify at startup | Apps / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.steam-startup` | No Steam at startup | Apps / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `cpu.boost-mode` | Aggressive boost mode | Cpu / Power plan | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `cpu.epp` | Energy preference: performance | Cpu / Power plan | Low | Medium | Documented | Registry | 1 | Admin | Windows 10/11 |
| `cpu.min-state` | Raise minimum CPU speed | Cpu / Power plan | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `cpu.parking-min` | Keep all CPU cores awake | Cpu / Power plan | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `cpu.global-timer` | Make timer requests global | Cpu / Scheduler and timers | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.game-dvr-off` | Turn off Game DVR | Gaming | Moderate | Medium | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.mmcss-games-priority` | Raise game priority in MMCSS | Gaming | Moderate | Low | Documented | Registry | 2 | Admin, Restart | Windows 10/11 |
| `gaming.mpo-off` | Disable multiplane overlay | Gaming | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.system-responsiveness` | Smaller MMCSS reserve | Gaming | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.audio-ducking` | No volume drop during calls | Gaming / Audio | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.fse-behavior` | Prefer exclusive fullscreen | Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 4 | None | Windows 10/11 |
| `gaming.game-bar-controller` | No Game Bar controller button | Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.game-mode-auto` | Automatic Game Mode | Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.filter-keys` | No Filter Keys prompt | Gaming / Input | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.language-hotkey` | No language switch hotkey | Gaming / Input | Low | Low | Found in Windows | Registry | 3 | None | Windows 10/11 |
| `gaming.num-lock` | Num Lock at sign-in | Gaming / Input | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.print-screen` | Print Screen skips Snipping Tool | Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.toggle-keys` | No Toggle Keys shortcut | Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.narrator-key` | No Narrator shortcut | Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.cs2-priority` | CS2 high priority | Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.fortnite-priority` | Fortnite high priority | Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.roblox-priority` | Roblox high priority | Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.amd-start-cn` | No AMD Software at startup | Gpu / AMD software | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.amd-start-dvr` | No AMD recording at startup | Gpu / AMD software | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.amd-uep` | No AMD User Experience task | Gpu / AMD software | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.nv-cpl-optin` | No NVIDIA Control Panel data | Gpu / NVIDIA | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.nv-pstate` | Hold the GPU at full clocks | Gpu / NVIDIA | Moderate | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.nv-tray` | No NVIDIA tray | Gpu / NVIDIA | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gpu.hags` | Hardware GPU scheduling | Gpu / Windows graphics | Moderate | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.tdr-ddi` | Longer GPU driver call timeout | Gpu / Windows graphics | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.tdr-delay` | Longer GPU timeout | Gpu / Windows graphics | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `memory.svchost-split` | Group background services | Memory / Paging and cache | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.kernel-in-ram` | Keep kernel code in RAM | Memory / Paging and cache | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.delivery-optimization-http` | Updates from Microsoft only | Network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.icmp-redirect-off` | Ignore ICMP redirects | Network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.llmnr-off` | Disable LLMNR | Network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.prefer-ipv4` | Prefer IPv4 over IPv6 | Network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.qos-apex` | Apex Legends | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-cs2` | Counter-Strike 2 | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-fortnite` | Fortnite | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-league` | League of Legends | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-overwatch` | Overwatch 2 | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-rainbow-six` | Rainbow Six Siege | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-rocket-league` | Rocket League | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-valorant` | VALORANT | Network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.smb-throttling-off` | Disable SMB throttling | Network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.dns-neg-ttl` | Forget failed DNS lookups sooner | Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.dns-netbios` | No NetBIOS name lookups | Network / DNS cache | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-energy-efficient` | Energy-Efficient Ethernet off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-flow-control` | Flow control off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-interrupt-moderation` | Interrupt moderation off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-lso-ipv4` | No Large Send Offload (IPv4) | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-lso-ipv6` | No Large Send Offload (IPv6) | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-selective-suspend` | Selective suspend off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-wake-magic` | Wake on Magic Packet off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-eth-wake-pattern` | Wake on pattern match off | Network / Ethernet adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.smb-admin-shares-server` | No hidden admin shares | Network / SMB | Moderate | Low | Documented | Registry | 2 | Admin, Restart | Windows 10/11 |
| `network.smb-signing-require` | SMB signing required | Network / SMB | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.smb1` | Turn off SMB 1 | Network / SMB | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.name-release` | Ignore NetBIOS name release | Network / TCP | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.source-routing` | Drop source-routed packets | Network / TCP | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.throttling-index` | No network throttling | Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-wifi-wake-magic` | Wake on Magic Packet off | Network / Wi-Fi adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.adapter-wifi-wake-pattern` | Wake on pattern match off | Network / Wi-Fi adapter | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.doh` | Allow encrypted DNS (DoH) | Network / Winsock and resolution | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.winhttp-wpad` | Turn off WPAD | Network / Winsock and resolution | Moderate | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `performance.edge-background-mode-off` | Stop Edge running in background | Performance | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `performance.edge-startup-boost-off` | Turn off Edge Startup boost | Performance | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `performance.fault-tolerant-heap-off` | Disable Fault Tolerant Heap | Performance | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.last-access-off` | Stop updating last-access times | Performance | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.short-names-off` | Stop creating 8.3 file names | Performance | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.power-throttling-off` | Disable power throttling | Power | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.usb-selective-suspend-off` | Disable USB selective suspend | Power | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.ahci-link` | SATA link power saving off | Power / Disk | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.disk-idle` | Keep hard drives spinning | Power / Disk | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.nvme-primary` | NVMe light idle off | Power / Disk | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.nvme-secondary` | NVMe deep idle off | Power / Disk | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.adaptive-brightness` | No adaptive brightness | Power / Display and devices | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.display` | Display never turns off | Power / Display and devices | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.intel-graphics` | Full Intel graphics performance | Power / Display and devices | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.wifi-power` | Wi-Fi power saving off | Power / Display and devices | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.button` | Power button does nothing | Power / Sleep and buttons | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.hibernate` | Never hibernate | Power / Sleep and buttons | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.lid` | Keep running with the lid shut | Power / Sleep and buttons | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.sleep` | Never sleep | Power / Sleep and buttons | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.fast-startup` | Turn off fast startup | Power / System | Low | Medium | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.pcie-aspm` | PCIe link power saving off | Power / USB and PCIe | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `power.usb3-link` | USB 3 link power saving off | Power / USB and PCIe | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.activity-history-off` | Turn off activity history | Privacy | Moderate | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.advertising-id-off` | Turn off the advertising ID | Privacy | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-account-info-off` | Block app access to account info | Privacy | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-diagnostics-off` | Block app access to diagnostics | Privacy | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-launch-tracking-off` | Stop tracking app launches | Privacy | Low | Low | Documented | Registry | 1 | Sign out | Windows 10/11 |
| `privacy.app-location-off` | Block app access to location | Privacy | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-notification-access-off` | Block app access to notifications | Privacy | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.background-apps-off` | Block background apps | Privacy | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.diagnostic-data-minimum` | Send the least diagnostic data | Privacy | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `privacy.edge-shopping-off` | Turn off Edge Shopping | Privacy | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.feedback-requests-off` | Stop Windows feedback requests | Privacy | Low | Low | Documented | Registry | 2 | None | Windows 10/11 |
| `privacy.inking-typing-collection-off` | Stop typing and ink learning | Privacy | Low | Low | Documented | Registry | 2 | Sign out | Windows 10/11 |
| `privacy.language-list-opt-out` | Hide language list from sites | Privacy | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `privacy.online-speech-off` | Turn off online speech | Privacy | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `privacy.ad-id-policy` | Block the advertising ID | Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.cdm-lock-fun` | No lock screen fun facts | Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-settings` | No suggestions in Settings | Privacy / Advertising and suggestions | Low | Low | Community | Registry | 3 | None | Windows 10/11 |
| `privacy.cdm-silent` | No silent app installs | Privacy / Advertising and suggestions | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-start` | No suggestions in Start | Privacy / Advertising and suggestions | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-tips` | No tips while you use Windows | Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-welcome` | No post-update welcome page | Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.online-tips` | No online tips | Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.spotlight-welcome` | No finish setup prompts | Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.tailored-policy` | No tailored experiences | Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.sync-ads` | No OneDrive ads in Explorer | Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.location-consent` | Turn off location | Privacy / App permissions | Moderate | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.lock-camera` | No lock screen camera | Privacy / App permissions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-appointments` | Block calendar access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-camera` | Block camera access | Privacy / App permissions | Moderate | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-capture` | Block screen capture access | Privacy / App permissions | Moderate | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `privacy.perm-contacts` | Block contacts access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-documents` | Block Documents access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-downloads` | Block Downloads access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-email` | Block email access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-files` | Block full file system access | Privacy / App permissions | Moderate | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-microphone` | Block microphone access | Privacy / App permissions | Moderate | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `privacy.perm-pictures` | Block Pictures access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-videos` | Block Videos access | Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.click-to-do` | Turn off Click to Do | Privacy / Copilot and AI | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.copilot-policy` | Turn off Windows Copilot | Privacy / Copilot and AI | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.paint-creator` | No Paint Image Creator | Privacy / Copilot and AI | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.paint-fill` | No Paint generative fill | Privacy / Copilot and AI | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.recall` | Turn off Recall | Privacy / Copilot and AI | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.cloud-search` | No cloud search | Privacy / Search | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.search-history` | No search history | Privacy / Search | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.search-location` | No location in Search | Privacy / Search | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.web-search` | No web results in Search | Privacy / Search | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.tipc` | No inking and typing data | Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.voice-activation` | No voice activation | Privacy / Speech and typing | Low | Low | Community | Registry | 3 | Admin | Windows 10/11 |
| `privacy.cdp` | No shared experiences | Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.clipboard-cloud` | No cloud clipboard | Privacy / Sync and cross-device | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.clipboard-history` | No clipboard history | Privacy / Sync and cross-device | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.find-my-device` | Turn off Find My Device | Privacy / Sync and cross-device | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.mobility` | No Phone Link suggestions | Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.nearby` | Turn off Nearby sharing | Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.settings-sync` | No settings sync | Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-credentials` | No password sync | Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.driver-search` | No automatic driver downloads | Privacy / System reports | Moderate | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.exclude-drivers` | No Windows Update drivers | Privacy / System reports | Moderate | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.push-to-install` | No remote app installs | Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.vs-sqm` | No Visual Studio feedback | Privacy / System reports | Low | Low | Documented | Registry | 9 | Admin | Windows 10/11 |
| `privacy.ait` | No application telemetry | Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.autologger-diagtrack` | No telemetry boot trace | Privacy / Telemetry | Low | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `privacy.ceip` | No Experience Program | Privacy / Telemetry | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.dotnet-telemetry` | No .NET CLI telemetry | Privacy / Telemetry | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.error-report-policy` | No error reporting | Privacy / Telemetry | Low | Low | Documented | Registry | 5 | Admin | Windows 10/11 |
| `privacy.inventory` | No application inventory | Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.mrt-report` | No removal tool reports | Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.onesettings` | No OneSettings downloads | Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.powershell-telemetry` | No PowerShell telemetry | Privacy / Telemetry | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `tasks.compat-appraiser` | No Compatibility Appraiser | Privacy / Telemetry | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.compat-appraiser-exp` | No Compatibility Appraiser Exp | Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.device-info` | No Device Information task | Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `tasks.program-data-updater` | No Program Data Updater | Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.start-docs` | No recent items in Start | Privacy / Windows data | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `services.compat-assistant-off` | Disable Compatibility Assistant | Services | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.diagtrack-off` | Disable Telemetry Service | Services | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.geolocation-off` | Disable Geolocation Service | Services | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.print-spooler-off` | Disable Print Spooler | Services | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.sync-host-off` | Disable Sync Host | Services | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.lanman-server` | Disable File and Printer Sharing | Services / Remote access | Moderate | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.remote-registry` | Disable Remote Registry | Services / Remote access | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.webclient` | Disable WebClient (WebDAV) | Services / Remote access | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.wsearch` | Disable Search indexing | Services / Search and shell | Moderate | Medium | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.cdp` | Disable Connected Devices | Services / Sync and account | Moderate | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `services.nahimic` | Disable Nahimic | Services / Third-party | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.long-paths` | Long file paths | Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.registry-backup` | Periodic registry backup | Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.reserved` | No reserved storage | Storage / File system | Moderate | Low | Documented | Registry | 5 | Admin, Restart | Windows 10/11 |
| `storage.trim` | SSD TRIM | Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `visuals.classic-context-menu` | Full right-click menu | Visuals / Context menu | Low | Low | Community | Registry | 1 | Sign out | Windows 11 |
| `visuals.menu-cast-off` | Remove Cast to Device | Visuals / Context menu | Low | Low | Community | Registry | 1 | Sign out | Windows 10/11 |
| `visuals.menu-give-access-off` | Remove Give access to | Visuals / Context menu | Low | Low | Community | Registry | 1 | Sign out | Windows 10/11 |
| `windows.send-to-drives` | No drives in Send to | Visuals / Context menu | Low | Low | Documented | Registry | 2 | Admin, Sign out | Windows 10/11 |
| `gaming.mouse-hover` | Faster hover tips | Visuals / Effects and animation | Low | Low | Community | Registry | 1 | Sign out | Windows 10/11 |
| `gpu.dwm-mode-animation` | No fade on mode change | Visuals / Effects and animation | Low | Low | Found in Windows | Registry | 1 | Admin, Sign out | Windows 11 (22631+) |
| `gpu.dwm-peek` | Turn off Peek | Visuals / Effects and animation | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `windows.drag-full` | Drag windows as an outline | Visuals / Effects and animation | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `windows.list-alpha` | Plain selection box | Visuals / Effects and animation | Low | Low | Community | Registry | 1 | Sign out | Windows 10/11 |
| `windows.list-shadow` | No icon label shadows | Visuals / Effects and animation | Low | Low | Community | Registry | 1 | Sign out | Windows 10/11 |
| `windows.menu-delay` | Faster menus | Visuals / Effects and animation | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `windows.min-animate` | No minimise animation | Visuals / Effects and animation | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `windows.task-animations` | No taskbar animations | Visuals / Effects and animation | Low | Low | Found in Windows | Scheduled task | 1 | Sign out | Windows 10/11 |
| `windows.window-animations-off` | Turn off window animations | Visuals / Effects and animation | Low | Low | Documented | Registry | 1 | Admin, Sign out | Windows 10/11 |
| `privacy.cdm-lockscreen` | Turn off lock screen Spotlight | Visuals / Lock and sign-in | Low | Low | Found in Windows | Registry | 3 | None | Windows 10/11 |
| `visuals.plain-sign-in` | Plain sign-in background | Visuals / Lock and sign-in | Low | Low | Found in Windows | Registry | 1 | Admin, Sign out | Windows 10/11 |
| `visuals.restart-apps` | Reopen apps after sign-in | Visuals / Lock and sign-in | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.account-details` | Hide email at sign-in | Visuals / Lock and sign-in | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.lock-notifications` | No lock screen notifications | Visuals / Lock and sign-in | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.logon-animation` | Sign-in screen blur off | Visuals / Lock and sign-in | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `visuals.nav-all-folders` | Show all folders | Visuals / Navigation pane | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.onedrive-nav-off` | Hide OneDrive | Visuals / Navigation pane | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.home-namespace` | Hide Gallery | Visuals / Navigation pane | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 (22631+) |
| `windows.hub-mode` | Hide Home in File Explorer | Visuals / Navigation pane | Low | Low | Found in Windows | Registry | 1 | Admin, Sign out | Windows 10/11 |
| `windows.nav-expand` | Expand to open folder | Visuals / Navigation pane | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.launch-pc` | Open to This PC | Visuals / Open and save | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.open-with-store` | No Store in Open with | Visuals / Open and save | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.news-interests` | Turn off the Widgets board | Visuals / Search and widgets | Low | Low | Documented | Registry | 2 | Admin, Sign out | Windows 10/11 |
| `privacy.search-highlights` | No search highlights | Visuals / Search and widgets | Low | Low | Documented | Registry | 2 | Admin, Sign out | Windows 10/11 |
| `windows.search-box` | Hide the search box | Visuals / Search and widgets | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 10/11 |
| `windows.widgets` | Hide the Widgets button | Visuals / Search and widgets | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 |
| `visuals.start-account-alerts` | No account alerts in Start | Visuals / Start menu | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 (22621+) |
| `windows.start-recommend` | No Start recommendations | Visuals / Start menu | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 (22621+) |
| `power.startup-sound` | Turn off the startup sound | Visuals / Startup and shutdown | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 11 |
| `visuals.first-logon-animation-off` | No first sign-in animation | Visuals / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `visuals.verbose-status` | Detailed startup messages | Visuals / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.touch-button` | Hide touch keyboard button | Visuals / Taskbar | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `visuals.clock-seconds` | Seconds on the clock | Visuals / Taskbar | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.taskbar-all-displays` | Taskbar on all displays | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 10/11 |
| `visuals.taskbar-badges-off` | No badges on taskbar apps | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 |
| `windows.copilot-button` | Hide the Copilot button | Visuals / Taskbar | Low | Low | Found in Windows | Registry | 1 | None | 19045-22631 |
| `windows.end-task` | End task in taskbar | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 (22621+) |
| `windows.small-icons` | Small taskbar icons | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | 10240-21999 |
| `windows.task-view` | Hide the Task View button | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 10/11 |
| `windows.taskbar-flash` | No taskbar flashing | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 (22621+) |
| `windows.taskbar-glom` | Never combine taskbar buttons | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 10/11 |
| `windows.taskbar-left` | Align taskbar left | Visuals / Taskbar | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 |
| `windows.tray-all` | All tray icons | Visuals / Taskbar | Low | Low | Found in Windows | Registry | 1 | Sign out | 10240-21999 |
| `visuals.accent-from-wallpaper` | Accent from wallpaper | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `visuals.accent-title-bars` | Accent colour on title bars | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.accent-taskbar` | Accent colour on taskbar | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.dark-apps` | Dark mode for apps | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.dark-system` | Dark mode for Windows | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.transparency` | Turn off transparency | Visuals / Theme and colour | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.item-checkboxes` | Item check boxes | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.status-bar-off` | Hide the status bar | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.compact-mode` | Compact File Explorer | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.enthusiast` | Detailed copy dialog | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.folder-type` | Generic folder views | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.full-path` | Full path in title | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.icons-only` | Icons instead of thumbnails | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.info-tip` | No hover tips in Explorer | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.preview-handlers` | Turn off file previews | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-cloud-recent` | Hide cloud files in Home | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.show-ext` | Show file extensions | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-frequent` | Hide frequent folders | Visuals / View | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.show-hidden` | Show hidden files | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-recent` | Hide recent files | Visuals / View | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.show-recommended` | Recommendations in Explorer | Visuals / View | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 (22621+) |
| `windows.show-system` | Show system files | Visuals / View | Moderate | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.desktop-this-pc` | This PC on the desktop | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.recycle-bin-off` | Hide the Recycle Bin icon | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `visuals.shake-off` | Title bar shake off | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.alt-tab` | Hide Edge tabs in Alt+Tab | Visuals / Windows and desktop | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 10/11 |
| `windows.desktop-icons` | Hide desktop icons | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | Sign out | Windows 10/11 |
| `windows.snap-bar` | Hide the snap bar | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.snap-flyout` | Hide the snap layouts flyout | Visuals / Windows and desktop | Low | Low | Community | Registry | 2 | None | Windows 11 |
| `windows.snap-suggest` | No snap suggestions | Visuals / Windows and desktop | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.task-groups` | No snap groups | Visuals / Windows and desktop | Low | Low | Found in Windows | Scheduled task | 1 | None | Windows 11 |
| `windows.autoplay-off` | Turn off AutoPlay | Windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.device-metadata-off` | Stop downloading device apps | Windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.edge-recommendations-off` | Stop Edge feature suggestions | Windows | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.edge-sidebar-off` | Hide the Edge sidebar | Windows | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.keyboard-repeat-fast` | Fastest key repeat | Windows | Low | Low | Documented | Registry | 2 | Sign out | Windows 10/11 |
| `windows.mouse-acceleration-off` | Disable mouse acceleration | Windows | Low | Low | Documented | Registry | 3 | Sign out | Windows 10/11 |
| `windows.sticky-keys-shortcut-off` | Turn off Sticky Keys shortcut | Windows | Low | Low | Documented | Registry | 1 | Sign out | Windows 10/11 |
| `windows.autoplay-nonvolume` | No AutoPlay for phones | Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.low-disk` | No low disk warnings | Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.separate-process` | Folders in separate process | Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.notif-all` | Turn off all notifications | Windows / Notifications | Moderate | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.notif-sound` | Mute notification sounds | Windows / Notifications | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.notif-startup` | Startup app notifications | Windows / Notifications | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `cpu.wpbt` | Block firmware-installed apps | Windows / Startup and shutdown | Moderate | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.auto-end` | Close frozen apps at shutdown | Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `windows.auto-restart` | Keep the blue screen on crash | Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.kill-service` | Shorter service shutdown wait | Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.kill-timeout` | Shorter app shutdown wait | Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `windows.startup-delay` | No startup app delay | Windows / Startup and shutdown | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.biometric-antispoof` | Face sign-in anti-spoofing | Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.cert-padding` | Strict file signature check | Windows / System | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.device-coinstallers` | Block device vendor installers | Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.rdp-off` | Turn off Remote Desktop | Windows / System | Moderate | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.remote-assistance` | Turn off Remote Assistance | Windows / System | Low | Low | Documented | Registry | 5 | Admin | Windows 10/11 |
| `windows.storage-sense-policy` | Turn off Storage Sense | Windows / System | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `windows.wsh` | Turn off Windows Script Host | Windows / System | High | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
