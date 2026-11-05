# Visuals section and library audit (0.5)

The 0.4 library held 1,300 tweaks, most of them taken from public optimisation scripts. Every one was read against the brief in this project: what it changes, whether anything measurable or noticeable follows, whether ordinary users or gamers care, whether it repeats another tweak, and whether it does anything on a supported Windows build.

**Result: 241 kept, 59 moved to Visuals, 1000 retired, 20 new Visuals settings. The app now shows 320 tweaks (79 in Visuals).**

| Category before | Kept | Moved to Visuals | Retired |
|---|---:|---:|---:|
| apps | 37 | 0 | 103 |
| cpu | 6 | 0 | 33 |
| gaming | 16 | 1 | 41 |
| gpu | 6 | 2 | 125 |
| memory | 1 | 0 | 7 |
| network | 33 | 0 | 81 |
| performance | 6 | 0 | 1 |
| power | 17 | 1 | 15 |
| privacy | 69 | 4 | 137 |
| services | 11 | 0 | 207 |
| storage | 4 | 0 | 17 |
| tasks | 7 | 0 | 139 |
| windows | 28 | 51 | 94 |
| **Total** | **241** | **59** | **1000** |

## How this was decided

- **Who did what.** Four Opus 5.5 agents read the catalogue in parallel (lead: Opus 5.5). Fable 5.1 then challenged the result and made 7 overrides. Sonnet 5.5 built the page and navigation. Fable 5.1 reviewed the built section afterwards.
- **Visuals rule.** A setting whose main purpose is how Windows looks or feels belongs in Visuals, even if turning it off saves a little. Animations are in Visuals.
- **Placement rule.** A switch whose purpose is to stop Microsoft promotion stays in Privacy. A switch about layout or look lives in Visuals.
- **Default-value rule.** A card that only ensures a Windows default is kept where the state commonly drifts or where it's a security check people look for, and its text says so (`storage.trim`, `services.remote-registry`, `gaming.game-mode-auto`).
- **Low impact isn't zero value.** Privacy, networking, security and quality-of-life switches stayed when they do something someone would want, however small.

## Nothing on your PC changes

Moving or retiring a tweak writes nothing to Windows. Whatever is applied stays applied.

- A retired tweak stays in the program in `engine/tweaks/library/retired.rs` (or its own file, listed under `RETIRED` in `engine/tweaks/mod.rs`, or `adapter::RETIRED`). It isn't listed and can't be planned, but undo and the broker still accept it.
- The Restore Centre lists any retired tweak this PC still has under **Removed from 2VX, still on this PC**, each with Undo. History keeps its names.
- IDs are unchanged, including tweaks moved to Visuals, because the journal references them. So a Visuals card still reads as `windows.show-ext`.
- Saved Game Mode choices that name a moved tweak are ignored rather than applied. Earlier Game Mode batches can still be undone.

## Value bugs found and fixed

These shipped wrong. Two kinds: a switch that wrote Windows' own default, so it did nothing, and a setting that was the wrong way round.

| Tweak | Was | Now |
|---|---|---|
| `windows.taskbar-left` | `TaskbarAl` = 1 (centre, the default) | 0 (left) |
| `windows.desktop-icons` | `HideIcons` = 0 (shown) plus a shell bag flag | `HideIcons` = 1 |
| `windows.enthusiast` | `EnthusiastMode` = 0 | 1 |
| `windows.accent-taskbar` | `ColorPrevalence` = 0 | 1 |
| `windows.nav-expand` | `NavPaneExpandToCurrentFolder` = 0 | 1 |
| `windows.hub-mode` | Also wrote a path missing `Microsoft` | One real path |
| `cpu.global-timer` | No restart flag | Needs a restart |

A PC that had one of the first five applied will now read **Not applied**, which is the truth. Its undo still works. The two dropped settings are held by `windows.legacy-targets`, a hidden retired entry, so an old journal row can still be restored.

Retired rather than fixed, because the tweak was wrong in itself: `windows.file-zone`, `windows.scan-attachment` and `windows.zone-launch` weakened security, five NVIDIA values were inverted, `gaming.mouse-keys` wrote the wrong thing, `network.metered-device` had a bad path, and `windows.hooks-timeout`, `windows.restart-apps` and `windows.storage-sense-off` used the wrong hive.

Restart flags were also corrected on 37 tweaks: 21 boot-time keys under `HKLM\SYSTEM` now say restart, and 16 Visuals settings that Explorer or the desktop only read when they start now say sign out.

## Engine changes this needed

- `TweakCategory::Visuals`; `register_retired` and `undoable` in the registry.
- Windows hears about Visuals changes: `WM_SETTINGCHANGE` (`ImmersiveColorSet`, `TraySettings`) and `SHChangeNotify` after apply and undo.
- `ALLOWED_VALUES` in `policy.rs`: `EnableFirstLogonAnimation` and `VerboseStatus`, named value by value, because the same key holds the UAC switches. A guard test keeps the rest refused.
- `PRESENCE_KEYS` in `store.rs`: the classic right-click menu works while an empty key exists, so undo removes that key and its parent, and only while both are empty.

## Presets

- Tweak presets (`lib/tweak-presets.ts`) are hand-written lists that leave out Visuals: a preset never changes how Windows looks, and a new tweak can't walk into one.
- Service presets lost the retired services. Basic: telemetry, location, sync host, compatibility assistant. Advanced adds Print Spooler, WebClient and Connected Devices Platform.
- Gaming profiles lost `performance.foreground-boost` (retired) and `windows.window-animations-off` (now Visuals). Responsive gained `gaming.system-responsiveness`.

## Moved into Visuals

59 existing tweaks. Each keeps its ID.

| Tab | Section | Tweak | Name | Was in |
|---|---|---|---|---|
| Appearance | Theme and colour | `windows.accent-taskbar` | Accent colour on taskbar | windows |
| Appearance | Theme and colour | `windows.dark-apps` | Dark mode for apps | windows |
| Appearance | Theme and colour | `windows.dark-system` | Dark mode for Windows | windows |
| Appearance | Theme and colour | `windows.transparency` | Turn off transparency | windows |
| Appearance | Effects and animation | `gaming.mouse-hover` | Faster hover tips | gaming |
| Appearance | Effects and animation | `gpu.dwm-mode-animation` | No fade on mode change | gpu |
| Appearance | Effects and animation | `gpu.dwm-peek` | Turn off Peek | gpu |
| Appearance | Effects and animation | `windows.drag-full` | Drag windows as an outline | windows |
| Appearance | Effects and animation | `windows.list-alpha` | Plain selection box | windows |
| Appearance | Effects and animation | `windows.list-shadow` | No icon label shadows | windows |
| Appearance | Effects and animation | `windows.menu-delay` | Faster menus | windows |
| Appearance | Effects and animation | `windows.min-animate` | No minimise animation | windows |
| Appearance | Effects and animation | `windows.task-animations` | No taskbar animations | windows |
| Appearance | Effects and animation | `windows.window-animations-off` | Turn off window animations | windows |
| Appearance | Windows and desktop | `windows.alt-tab` | Hide Edge tabs in Alt+Tab | windows |
| Appearance | Windows and desktop | `windows.desktop-icons` | Hide desktop icons | windows |
| Appearance | Windows and desktop | `windows.snap-bar` | Hide the snap bar | windows |
| Appearance | Windows and desktop | `windows.snap-flyout` | Hide the snap layouts flyout | windows |
| Appearance | Windows and desktop | `windows.snap-suggest` | No snap suggestions | windows |
| Appearance | Windows and desktop | `windows.task-groups` | No snap groups | windows |
| Taskbar and Start | Taskbar | `privacy.touch-button` | Hide touch keyboard button | privacy |
| Taskbar and Start | Taskbar | `windows.copilot-button` | Hide the Copilot button | windows |
| Taskbar and Start | Taskbar | `windows.end-task` | End task in taskbar | windows |
| Taskbar and Start | Taskbar | `windows.small-icons` | Small taskbar icons | windows |
| Taskbar and Start | Taskbar | `windows.task-view` | Hide the Task View button | windows |
| Taskbar and Start | Taskbar | `windows.taskbar-flash` | No taskbar flashing | windows |
| Taskbar and Start | Taskbar | `windows.taskbar-glom` | Never combine taskbar buttons | windows |
| Taskbar and Start | Taskbar | `windows.taskbar-left` | Align taskbar left | windows |
| Taskbar and Start | Taskbar | `windows.tray-all` | All tray icons | windows |
| Taskbar and Start | Start menu | `windows.start-recommend` | No Start recommendations | windows |
| Taskbar and Start | Search and widgets | `privacy.news-interests` | Turn off the Widgets board | privacy |
| Taskbar and Start | Search and widgets | `privacy.search-highlights` | No search highlights | privacy |
| Taskbar and Start | Search and widgets | `windows.search-box` | Hide the search box | windows |
| Taskbar and Start | Search and widgets | `windows.widgets` | Hide the Widgets button | windows |
| File Explorer | View | `windows.compact-mode` | Compact File Explorer | windows |
| File Explorer | View | `windows.enthusiast` | Detailed copy dialog | windows |
| File Explorer | View | `windows.folder-type` | Generic folder views | windows |
| File Explorer | View | `windows.full-path` | Full path in title | windows |
| File Explorer | View | `windows.icons-only` | Icons instead of thumbnails | windows |
| File Explorer | View | `windows.info-tip` | No hover tips in Explorer | windows |
| File Explorer | View | `windows.preview-handlers` | Turn off file previews | windows |
| File Explorer | View | `windows.show-cloud-recent` | Hide cloud files in Home | windows |
| File Explorer | View | `windows.show-ext` | Show file extensions | windows |
| File Explorer | View | `windows.show-frequent` | Hide frequent folders | windows |
| File Explorer | View | `windows.show-hidden` | Show hidden files | windows |
| File Explorer | View | `windows.show-recent` | Hide recent files | windows |
| File Explorer | View | `windows.show-recommended` | Recommendations in Explorer | windows |
| File Explorer | View | `windows.show-system` | Show system files | windows |
| File Explorer | Navigation pane | `windows.home-namespace` | Hide Gallery | windows |
| File Explorer | Navigation pane | `windows.hub-mode` | Hide Home in File Explorer | windows |
| File Explorer | Navigation pane | `windows.nav-expand` | Expand to open folder | windows |
| File Explorer | Context menu | `windows.send-to-drives` | No drives in Send to | windows |
| File Explorer | Open and save | `windows.launch-pc` | Open to This PC | windows |
| File Explorer | Open and save | `windows.open-with-store` | No Store in Open with | windows |
| Sign-in | Lock and sign-in | `privacy.cdm-lockscreen` | Turn off lock screen Spotlight | privacy |
| Sign-in | Lock and sign-in | `windows.account-details` | Hide email at sign-in | windows |
| Sign-in | Lock and sign-in | `windows.lock-notifications` | No lock screen notifications | windows |
| Sign-in | Lock and sign-in | `windows.logon-animation` | Sign-in screen blur off | windows |
| Sign-in | Startup and shutdown | `power.startup-sound` | Turn off the startup sound | power |

## New in Visuals

20 settings researched against Windows 10 and 11. Value names were confirmed in Windows shell files or Microsoft documentation. `Community` ones are marked as such in the app and stay out of presets.

| Section | Tweak | Name | Evidence |
|---|---|---|---|
| Theme and colour | `visuals.accent-from-wallpaper` | Accent from wallpaper | observed |
| Theme and colour | `visuals.accent-title-bars` | Accent colour on title bars | observed |
| Windows and desktop | `visuals.desktop-this-pc` | This PC on the desktop | observed |
| Windows and desktop | `visuals.recycle-bin-off` | Hide the Recycle Bin icon | observed |
| Windows and desktop | `visuals.shake-off` | Title bar shake off | observed |
| Taskbar | `visuals.clock-seconds` | Seconds on the clock | observed |
| Taskbar | `visuals.taskbar-all-displays` | Taskbar on all displays | observed |
| Taskbar | `visuals.taskbar-badges-off` | No badges on taskbar apps | observed |
| Start menu | `visuals.start-account-alerts` | No account alerts in Start | observed |
| View | `visuals.item-checkboxes` | Item check boxes | observed |
| View | `visuals.status-bar-off` | Hide the status bar | observed |
| Navigation pane | `visuals.nav-all-folders` | Show all folders | observed |
| Navigation pane | `visuals.onedrive-nav-off` | Hide OneDrive | community |
| Context menu | `visuals.classic-context-menu` | Full right-click menu | community |
| Context menu | `visuals.menu-cast-off` | Remove Cast to Device | community |
| Context menu | `visuals.menu-give-access-off` | Remove Give access to | community |
| Lock and sign-in | `visuals.plain-sign-in` | Plain sign-in background | observed |
| Lock and sign-in | `visuals.restart-apps` | Reopen apps after sign-in | observed |
| Startup and shutdown | `visuals.first-logon-animation-off` | No first sign-in animation | documented |
| Startup and shutdown | `visuals.verbose-status` | Detailed startup messages | documented |

## Retained

241 tweaks stayed where they belong. Descriptions were rewritten for 173 of them, and for the Visuals tweaks that moved.

### apps (37)

| Tweak | Name | Section |
|---|---|---|
| `apps.brave-background` | No Brave background mode | Brave, Opera and Vivaldi |
| `apps.chrome-3p-cookies` | No Chrome third-party cookies | Chrome |
| `apps.chrome-autofill-card` | No Chrome card autofill | Chrome |
| `apps.chrome-background` | No Chrome background mode | Chrome |
| `apps.chrome-cookies-session` | Clear Chrome cookies on close | Chrome |
| `apps.chrome-geolocation` | Block Chrome location requests | Chrome |
| `apps.chrome-metrics` | No Chrome metrics | Chrome |
| `apps.chrome-notifications` | Block Chrome notifications | Chrome |
| `apps.chrome-password-manager` | No Chrome password manager | Chrome |
| `apps.chrome-prediction` | No Chrome page preloading | Chrome |
| `apps.chrome-promos` | No Chrome promotional tabs | Chrome |
| `apps.chrome-sb-reporting` | Fewer Safe Browsing reports | Chrome |
| `apps.chrome-sign-in` | No Chrome sign-in and sync | Chrome |
| `apps.chrome-spellcheck` | No Chrome spellcheck service | Chrome |
| `apps.chrome-suggest` | No Chrome search suggestions | Chrome |
| `apps.chrome-url-data` | No Chrome URL sharing | Chrome |
| `apps.edge-3p-cookies` | Block Edge third-party cookies | Edge |
| `apps.edge-autofill-card` | No Edge card autofill | Edge |
| `apps.edge-experiments` | No Edge experiments | Edge |
| `apps.edge-nav-errors` | No Edge similar-site suggestions | Edge |
| `apps.edge-personalization` | No Edge personalisation | Edge |
| `apps.edge-promotional` | No Edge promotional tabs | Edge |
| `apps.edge-rewards` | No Edge rewards | Edge |
| `apps.edge-searchbar` | No Edge search bar | Edge |
| `apps.edge-suggest` | No Edge search suggestions | Edge |
| `apps.edge-telemetry` | No Edge diagnostic data | Edge |
| `apps.edge-top-sites` | No Edge default top sites | Edge |
| `apps.edge-tracking` | Strict Edge tracking prevention | Edge |
| `apps.firefox-default-agent` | No Firefox default browser agent | Firefox |
| `apps.firefox-studies` | No Firefox studies | Firefox |
| `apps.firefox-telemetry` | No Firefox telemetry | Firefox |
| `apps.new-outlook-toggle` | Hide the New Outlook toggle | Office |
| `apps.office-telemetry` | Less Office telemetry | Office |
| `apps.onedrive-startup` | No OneDrive at startup | Other apps |
| `apps.onedrive-sync` | Turn off OneDrive | Other apps |
| `apps.spotify-startup` | No Spotify at startup | Other apps |
| `apps.steam-startup` | No Steam at startup | Other apps |

### cpu (5)

| Tweak | Name | Section |
|---|---|---|
| `cpu.boost-mode` | Aggressive boost mode | Power plan |
| `cpu.epp` | Energy preference: performance | Power plan |
| `cpu.global-timer` | Make timer requests global | Scheduler and timers |
| `cpu.min-state` | Raise minimum CPU speed | Power plan |
| `cpu.parking-min` | Keep all CPU cores awake | Power plan |

### gaming (17)

| Tweak | Name | Section |
|---|---|---|
| `gaming.audio-ducking` | No volume drop during calls | Audio |
| `gaming.cs2-priority` | CS2 high priority | Per-game priority |
| `gaming.filter-keys` | No Filter Keys prompt | Input |
| `gaming.fortnite-priority` | Fortnite high priority | Per-game priority |
| `gaming.fse-behavior` | Prefer exclusive fullscreen | Game Bar and capture |
| `gaming.game-bar-controller` | No Game Bar controller button | Game Bar and capture |
| `gaming.game-dvr-off` | Turn off Game DVR | - |
| `gaming.game-mode-auto` | Automatic Game Mode | Game Bar and capture |
| `gaming.language-hotkey` | No language switch hotkey | Input |
| `gaming.mmcss-games-priority` | Raise game priority in MMCSS | - |
| `gaming.mpo-off` | Disable multiplane overlay | - |
| `gaming.num-lock` | Num Lock at sign-in | Input |
| `gaming.print-screen` | Print Screen skips Snipping Tool | Input |
| `gaming.roblox-priority` | Roblox high priority | Per-game priority |
| `gaming.system-responsiveness` | Smaller MMCSS reserve | - |
| `gaming.toggle-keys` | No Toggle Keys shortcut | Input |
| `windows.narrator-key` | No Narrator shortcut | Input |

### gpu (9)

| Tweak | Name | Section |
|---|---|---|
| `gpu.hags` | Hardware GPU scheduling | Windows graphics |
| `gpu.nv-cpl-optin` | No NVIDIA Control Panel data | NVIDIA |
| `gpu.nv-pstate` | Hold the GPU at full clocks | NVIDIA |
| `gpu.nv-tray` | No NVIDIA tray | NVIDIA |
| `gpu.tdr-ddi` | Longer GPU driver call timeout | Windows graphics |
| `gpu.tdr-delay` | Longer GPU timeout | Windows graphics |
| `tasks.amd-start-cn` | No AMD Software at startup | AMD software |
| `tasks.amd-start-dvr` | No AMD recording at startup | AMD software |
| `tasks.amd-uep` | No AMD User Experience task | AMD software |

### memory (2)

| Tweak | Name | Section |
|---|---|---|
| `memory.svchost-split` | Group background services | Paging and cache |
| `performance.kernel-in-ram` | Keep kernel code in RAM | Paging and cache |

### network (33)

| Tweak | Name | Section |
|---|---|---|
| `network.adapter-eth-energy-efficient` | Energy-Efficient Ethernet off | Ethernet adapter |
| `network.adapter-eth-flow-control` | Flow control off | Ethernet adapter |
| `network.adapter-eth-interrupt-moderation` | Interrupt moderation off | Ethernet adapter |
| `network.adapter-eth-lso-ipv4` | No Large Send Offload (IPv4) | Ethernet adapter |
| `network.adapter-eth-lso-ipv6` | No Large Send Offload (IPv6) | Ethernet adapter |
| `network.adapter-eth-selective-suspend` | Selective suspend off | Ethernet adapter |
| `network.adapter-eth-wake-magic` | Wake on Magic Packet off | Ethernet adapter |
| `network.adapter-eth-wake-pattern` | Wake on pattern match off | Ethernet adapter |
| `network.adapter-wifi-wake-magic` | Wake on Magic Packet off | Wi-Fi adapter |
| `network.adapter-wifi-wake-pattern` | Wake on pattern match off | Wi-Fi adapter |
| `network.delivery-optimization-http` | Updates from Microsoft only | - |
| `network.dns-neg-ttl` | Forget failed DNS lookups sooner | DNS cache |
| `network.dns-netbios` | No NetBIOS name lookups | DNS cache |
| `network.doh` | Allow encrypted DNS (DoH) | Winsock and resolution |
| `network.icmp-redirect-off` | Ignore ICMP redirects | - |
| `network.llmnr-off` | Disable LLMNR | - |
| `network.name-release` | Ignore NetBIOS name release | TCP |
| `network.prefer-ipv4` | Prefer IPv4 over IPv6 | - |
| `network.qos-apex` | Apex Legends | - |
| `network.qos-cs2` | Counter-Strike 2 | - |
| `network.qos-fortnite` | Fortnite | - |
| `network.qos-league` | League of Legends | - |
| `network.qos-overwatch` | Overwatch 2 | - |
| `network.qos-rainbow-six` | Rainbow Six Siege | - |
| `network.qos-rocket-league` | Rocket League | - |
| `network.qos-valorant` | VALORANT | - |
| `network.smb-admin-shares-server` | No hidden admin shares | SMB |
| `network.smb-signing-require` | SMB signing required | SMB |
| `network.smb-throttling-off` | Disable SMB throttling | - |
| `network.smb1` | Turn off SMB 1 | SMB |
| `network.source-routing` | Drop source-routed packets | TCP |
| `network.throttling-index` | No network throttling | TCP |
| `network.winhttp-wpad` | Turn off WPAD | Winsock and resolution |

### performance (5)

| Tweak | Name | Section |
|---|---|---|
| `performance.edge-background-mode-off` | Stop Edge running in background | - |
| `performance.edge-startup-boost-off` | Turn off Edge Startup boost | - |
| `performance.fault-tolerant-heap-off` | Disable Fault Tolerant Heap | - |
| `performance.last-access-off` | Stop updating last-access times | - |
| `performance.short-names-off` | Stop creating 8.3 file names | - |

### power (17)

| Tweak | Name | Section |
|---|---|---|
| `power.adaptive-brightness` | No adaptive brightness | Display and devices |
| `power.ahci-link` | SATA link power saving off | Disk |
| `power.button` | Power button does nothing | Sleep and buttons |
| `power.disk-idle` | Keep hard drives spinning | Disk |
| `power.display` | Display never turns off | Display and devices |
| `power.fast-startup` | Turn off fast startup | System |
| `power.hibernate` | Never hibernate | Sleep and buttons |
| `power.intel-graphics` | Full Intel graphics performance | Display and devices |
| `power.lid` | Keep running with the lid shut | Sleep and buttons |
| `power.nvme-primary` | NVMe light idle off | Disk |
| `power.nvme-secondary` | NVMe deep idle off | Disk |
| `power.pcie-aspm` | PCIe link power saving off | USB and PCIe |
| `power.power-throttling-off` | Disable power throttling | - |
| `power.sleep` | Never sleep | Sleep and buttons |
| `power.usb-selective-suspend-off` | Disable USB selective suspend | - |
| `power.usb3-link` | USB 3 link power saving off | USB and PCIe |
| `power.wifi-power` | Wi-Fi power saving off | Display and devices |

### privacy (75)

| Tweak | Name | Section |
|---|---|---|
| `privacy.activity-history-off` | Turn off activity history | - |
| `privacy.ad-id-policy` | Block the advertising ID | Advertising and suggestions |
| `privacy.advertising-id-off` | Turn off the advertising ID | - |
| `privacy.ait` | No application telemetry | Telemetry |
| `privacy.app-account-info-off` | Block app access to account info | - |
| `privacy.app-diagnostics-off` | Block app access to diagnostics | - |
| `privacy.app-launch-tracking-off` | Stop tracking app launches | - |
| `privacy.app-location-off` | Block app access to location | - |
| `privacy.app-notification-access-off` | Block app access to notifications | - |
| `privacy.autologger-diagtrack` | No telemetry boot trace | Telemetry |
| `privacy.background-apps-off` | Block background apps | - |
| `privacy.cdm-lock-fun` | No lock screen fun facts | Advertising and suggestions |
| `privacy.cdm-settings` | No suggestions in Settings | Advertising and suggestions |
| `privacy.cdm-silent` | No silent app installs | Advertising and suggestions |
| `privacy.cdm-start` | No suggestions in Start | Advertising and suggestions |
| `privacy.cdm-tips` | No tips while you use Windows | Advertising and suggestions |
| `privacy.cdm-welcome` | No post-update welcome page | Advertising and suggestions |
| `privacy.cdp` | No shared experiences | Sync and cross-device |
| `privacy.ceip` | No Experience Program | Telemetry |
| `privacy.click-to-do` | Turn off Click to Do | Copilot and AI |
| `privacy.clipboard-cloud` | No cloud clipboard | Sync and cross-device |
| `privacy.clipboard-history` | No clipboard history | Sync and cross-device |
| `privacy.cloud-search` | No cloud search | Search |
| `privacy.copilot-policy` | Turn off Windows Copilot | Copilot and AI |
| `privacy.diagnostic-data-minimum` | Send the least diagnostic data | - |
| `privacy.dotnet-telemetry` | No .NET CLI telemetry | Telemetry |
| `privacy.driver-search` | No automatic driver downloads | System reports |
| `privacy.edge-shopping-off` | Turn off Edge Shopping | - |
| `privacy.error-report-policy` | No error reporting | Telemetry |
| `privacy.exclude-drivers` | No Windows Update drivers | System reports |
| `privacy.feedback-requests-off` | Stop Windows feedback requests | - |
| `privacy.find-my-device` | Turn off Find My Device | Sync and cross-device |
| `privacy.inking-typing-collection-off` | Stop typing and ink learning | - |
| `privacy.inventory` | No application inventory | Telemetry |
| `privacy.language-list-opt-out` | Hide language list from sites | - |
| `privacy.location-consent` | Turn off location | App permissions |
| `privacy.lock-camera` | No lock screen camera | App permissions |
| `privacy.mobility` | No Phone Link suggestions | Sync and cross-device |
| `privacy.mrt-report` | No removal tool reports | Telemetry |
| `privacy.nearby` | Turn off Nearby sharing | Sync and cross-device |
| `privacy.onesettings` | No OneSettings downloads | Telemetry |
| `privacy.online-speech-off` | Turn off online speech | - |
| `privacy.online-tips` | No online tips | Advertising and suggestions |
| `privacy.paint-creator` | No Paint Image Creator | Copilot and AI |
| `privacy.paint-fill` | No Paint generative fill | Copilot and AI |
| `privacy.perm-appointments` | Block calendar access | App permissions |
| `privacy.perm-camera` | Block camera access | App permissions |
| `privacy.perm-capture` | Block screen capture access | App permissions |
| `privacy.perm-contacts` | Block contacts access | App permissions |
| `privacy.perm-documents` | Block Documents access | App permissions |
| `privacy.perm-downloads` | Block Downloads access | App permissions |
| `privacy.perm-email` | Block email access | App permissions |
| `privacy.perm-files` | Block full file system access | App permissions |
| `privacy.perm-microphone` | Block microphone access | App permissions |
| `privacy.perm-pictures` | Block Pictures access | App permissions |
| `privacy.perm-videos` | Block Videos access | App permissions |
| `privacy.powershell-telemetry` | No PowerShell telemetry | Telemetry |
| `privacy.push-to-install` | No remote app installs | System reports |
| `privacy.recall` | Turn off Recall | Copilot and AI |
| `privacy.search-history` | No search history | Search |
| `privacy.search-location` | No location in Search | Search |
| `privacy.settings-sync` | No settings sync | Sync and cross-device |
| `privacy.spotlight-welcome` | No finish setup prompts | Advertising and suggestions |
| `privacy.sync-credentials` | No password sync | Sync and cross-device |
| `privacy.tailored-policy` | No tailored experiences | Advertising and suggestions |
| `privacy.tipc` | No inking and typing data | Speech and typing |
| `privacy.voice-activation` | No voice activation | Speech and typing |
| `privacy.vs-sqm` | No Visual Studio feedback | System reports |
| `privacy.web-search` | No web results in Search | Search |
| `tasks.compat-appraiser` | No Compatibility Appraiser | Telemetry |
| `tasks.compat-appraiser-exp` | No Compatibility Appraiser Exp | Telemetry |
| `tasks.device-info` | No Device Information task | Telemetry |
| `tasks.program-data-updater` | No Program Data Updater | Telemetry |
| `windows.start-docs` | No recent items in Start | Windows data |
| `windows.sync-ads` | No OneDrive ads in Explorer | Advertising and suggestions |

### services (11)

| Tweak | Name | Section |
|---|---|---|
| `services.cdp` | Disable Connected Devices | Sync and account |
| `services.compat-assistant-off` | Disable Compatibility Assistant | - |
| `services.diagtrack-off` | Disable Telemetry Service | - |
| `services.geolocation-off` | Disable Geolocation Service | - |
| `services.lanman-server` | Disable File and Printer Sharing | Remote access |
| `services.nahimic` | Disable Nahimic | Third-party |
| `services.print-spooler-off` | Disable Print Spooler | - |
| `services.remote-registry` | Disable Remote Registry | Remote access |
| `services.sync-host-off` | Disable Sync Host | - |
| `services.webclient` | Disable WebClient (WebDAV) | Remote access |
| `services.wsearch` | Disable Search indexing | Search and shell |

### storage (4)

| Tweak | Name | Section |
|---|---|---|
| `storage.long-paths` | Long file paths | File system |
| `storage.registry-backup` | Periodic registry backup | File system |
| `storage.reserved` | No reserved storage | File system |
| `storage.trim` | SSD TRIM | File system |

### windows (26)

| Tweak | Name | Section |
|---|---|---|
| `cpu.wpbt` | Block firmware-installed apps | Startup and shutdown |
| `windows.auto-end` | Close frozen apps at shutdown | Startup and shutdown |
| `windows.auto-restart` | Keep the blue screen on crash | Startup and shutdown |
| `windows.autoplay-nonvolume` | No AutoPlay for phones | Explorer |
| `windows.autoplay-off` | Turn off AutoPlay | - |
| `windows.biometric-antispoof` | Face sign-in anti-spoofing | System |
| `windows.cert-padding` | Strict file signature check | System |
| `windows.device-coinstallers` | Block device vendor installers | System |
| `windows.device-metadata-off` | Stop downloading device apps | - |
| `windows.edge-recommendations-off` | Stop Edge feature suggestions | - |
| `windows.edge-sidebar-off` | Hide the Edge sidebar | - |
| `windows.keyboard-repeat-fast` | Fastest key repeat | - |
| `windows.kill-service` | Shorter service shutdown wait | Startup and shutdown |
| `windows.kill-timeout` | Shorter app shutdown wait | Startup and shutdown |
| `windows.low-disk` | No low disk warnings | Explorer |
| `windows.mouse-acceleration-off` | Disable mouse acceleration | - |
| `windows.notif-all` | Turn off all notifications | Notifications |
| `windows.notif-sound` | Mute notification sounds | Notifications |
| `windows.notif-startup` | Startup app notifications | Notifications |
| `windows.rdp-off` | Turn off Remote Desktop | System |
| `windows.remote-assistance` | Turn off Remote Assistance | System |
| `windows.separate-process` | Folders in separate process | Explorer |
| `windows.startup-delay` | No startup app delay | Startup and shutdown |
| `windows.sticky-keys-shortcut-off` | Turn off Sticky Keys shortcut | - |
| `windows.storage-sense-policy` | Turn off Storage Sense | System |
| `windows.wsh` | Turn off Windows Script Host | System |

## Retired

1000 tweaks. None can be planned any more, and all can still be undone. Reasons are written per tweak, so the same sentence recurs across a family.

### apps (103)

| Tweak | Name | Why |
|---|---|---|
| `apps.adobe-optout` | Adobe telemetry | An undocumented Adobe value with no known behaviour, so nobody can say what it changes. |
| `apps.adobe-rum` | Adobe update manager | An undocumented Adobe value found only in scripts, with no known effect. |
| `apps.app-installer-exp` | App Installer experiments | Allows winget experimental features, a developer option with no benefit to a normal PC. |
| `apps.app-installer-source` | App Installer sources | Forces winget and App Installer to settings that are already the default, so nothing changes. |
| `apps.brave-hw` | Brave hardware acceleration | Forces hardware acceleration on, which is already Brave's default. |
| `apps.brave-metrics` | Brave metrics | A copy of the Chrome setting; Brave doesn't send Google usage reports and has its own analytics switches. |
| `apps.brave-reports` | Brave Safe Browsing reports | A copy of the Chrome setting; Brave doesn't use Google's extended Safe Browsing reports. |
| `apps.brave-spellcheck` | Brave spellcheck | A copy of the Chrome setting; Brave doesn't offer Google's online spelling service. |
| `apps.ccleaner-health` | CCleaner Health Check | Extremely niche: an unverified community value for one third-party app, with the option already in CCleaner's own settings. |
| `apps.ccleaner-monitoring` | CCleaner monitoring | Extremely niche: an unverified community value for one third-party app, not confirmed on current CCleaner versions. |
| `apps.ccleaner-offers` | CCleaner offers | Extremely niche: an unverified community value for one third-party app, not confirmed on current CCleaner versions. |
| `apps.ccleaner-telemetry` | CCleaner data | Extremely niche: an unverified community value for one third-party app, with the option already in CCleaner's own settings. |
| `apps.ccleaner-updates` | CCleaner update checks | Turns off update checks for third-party software, which leaves it without security fixes. |
| `apps.chrome-ad-block` | Chrome ad blocking | Could not be matched to a published Chrome policy, and Chrome already filters ads on abusive sites by default. |
| `apps.chrome-ai-search` | Chrome AI search | Could not be matched to a published Chrome policy, so it can't be shown to do anything. |
| `apps.chrome-autofill-address` | Chrome address autofill | An everyday convenience switch already in Chrome's settings, with no meaningful privacy result. |
| `apps.chrome-bluetooth` | Chrome Web Bluetooth | Sites must already ask before using Bluetooth, and almost no home user ever meets this feature. |
| `apps.chrome-cleanup` | Chrome Cleanup | Chrome Cleanup was removed from Chrome in 2023, so this has no function. |
| `apps.chrome-cloud-print` | Chrome cloud print | Google Cloud Print shut down at the end of 2020. |
| `apps.chrome-cloud-reporting` | Chrome cloud reporting | Enterprise only: cloud reporting needs a managed browser enrolment that a home PC never has. |
| `apps.chrome-components` | Chrome component updates | Stops Chrome updating safety lists, Widevine and other parts, which weakens a security protection. |
| `apps.chrome-dns` | Chrome built-in DNS | Changes an internal DNS path with no outcome a person could notice or state. |
| `apps.chrome-doh` | Chrome DoH | Turns off encrypted DNS lookups, which makes browsing less private rather than more. |
| `apps.chrome-error-pages` | Chrome error pages | The web service behind this policy appears to have been withdrawn, so it has no clear effect on current Chrome. |
| `apps.chrome-feedback` | Chrome feedback | Only hides the feedback option; nothing is sent unless you choose to send it. |
| `apps.chrome-lens` | Chrome Lens | Lens sends nothing unless you use it, and Chrome is replacing these policies, so this no longer covers the feature. |
| `apps.chrome-media` | Chrome camera and mic | Blocks every call in Chrome, with no benefit over the Windows camera and microphone switches. |
| `apps.chrome-privacy-sandbox` | Chrome Privacy Sandbox | Google is retiring the Privacy Sandbox ad technologies, including Topics, so these switches control features leaving Chrome. |
| `apps.chrome-sensors` | Chrome sensors | Desktop PCs have no motion sensors for sites to read, so this has no practical result. |
| `apps.chrome-translate` | Chrome translate | Nothing is sent until you choose to translate a page, so this only removes a useful feature. |
| `apps.chrome-updates` | Chrome update checks | Stops Chrome receiving security fixes, which weakens a security protection. |
| `apps.chrome-usb` | Chrome Web USB | Sites must already ask before using a USB device, so this has no practical result for home users. |
| `apps.edge-acrobat` | Edge Acrobat button | Hides one small button in Edge's PDF viewer; obscure with no practical benefit. |
| `apps.edge-address-bar` | Edge Microsoft Search | Only affects Microsoft Search for work accounts, which a home PC doesn't use. |
| `apps.edge-autofill-address` | Edge address autofill | An everyday convenience switch already in Edge's settings, with no meaningful privacy result. |
| `apps.edge-bing-ads` | Edge Bing ads | Microsoft limits this policy to education tenants, so it has no effect on a home PC. |
| `apps.edge-bing-chat` | Edge new tab Copilot | Applies to the Enterprise new tab page, so a home PC sees no change. |
| `apps.edge-collections` | Edge Collections | A feature switch with no privacy or performance result; if you don't use Collections it does nothing. |
| `apps.edge-copilot-page` | Edge Copilot page access | These policies only apply to Entra ID work profiles, and at least one is obsolete, so a home PC sees no change. |
| `apps.edge-discover` | Edge Discover | Controls an old sidebar feature; its status could not be confirmed, and the sidebar switch already covers that surface. |
| `apps.edge-dnt` | Edge Do Not Track | Do Not Track is ignored by nearly all sites and adds one more detail for fingerprinting. |
| `apps.edge-do-not-upgrade` | Chromium Edge upgrade | Legacy Edge left Windows in 2021, so there is nothing left to block. |
| `apps.edge-enhance-images` | Edge image enhancement | Microsoft retired this feature in Edge 122 and the policy is obsolete. |
| `apps.edge-family` | Edge family safety | Hides parental controls with no privacy or performance benefit. |
| `apps.edge-feedback` | Edge feedback | Only hides the feedback option; nothing is sent unless you choose to send it. |
| `apps.edge-follow` | Edge Follow | The policy is obsolete and stopped working after Edge 126. |
| `apps.edge-games` | Edge games menu | Hides one menu entry with no practical benefit. |
| `apps.edge-in-app-support` | Edge in-app support | Removes Edge's help option; nothing is sent unless you use it. |
| `apps.edge-insider` | Edge insider promotion | Hides an obscure promotion most people never see. |
| `apps.edge-install-default` | Edge install block | Blocks installs through Edge's updater, which can also stop the WebView2 runtime that many apps need. |
| `apps.edge-ntp-bg` | Edge new tab background | A cosmetic choice already on Edge's new tab page, with no wider result. |
| `apps.edge-payment-query` | Edge payment query | An obscure site check with no noticeable result for ordinary users. |
| `apps.edge-prelaunch` | Legacy Edge prelaunch | Applies to legacy Edge, which was removed from Windows in 2021. |
| `apps.edge-quick-links` | Edge quick links | Removes all quick links, including your own, when the top sites tweak already takes away the ones Microsoft chooses. |
| `apps.edge-related-matches` | Edge related matches | An extremely niche Find on Page feature with no clear privacy result. |
| `apps.edge-sidebar` | Edge standalone sidebar | Effectively covered by the main sidebar tweak, which turns off Edge's sidebar. |
| `apps.edge-signin-ntp` | Edge sign-in prompt | Hides one small prompt; obscure with no practical benefit. |
| `apps.edge-site-info` | Edge site info | An older Edge setting replaced by the diagnostic data policy, which the Edge diagnostic data tweak already sets. |
| `apps.edge-site-safety` | Edge site safety | An obscure lookup tied to Edge's site safety information, with no clear privacy gain and a possible loss of protection. |
| `apps.edge-spellcheck` | Edge spellcheck | Removes spell checking entirely for no privacy or performance benefit. |
| `apps.edge-spotlight` | Edge Spotlight | Repeats the Edge feature suggestions tweak with a vaguer outcome. |
| `apps.edge-tab-preload` | Legacy Edge tab preload | Applies to legacy Edge, which was removed from Windows in 2021. |
| `apps.edge-update-period` | Edge update checks | Stops Edge receiving security fixes, which weakens a security protection. |
| `apps.edge-update-window` | Edge update hours | Blocks Edge updates around the clock, which stops security fixes. |
| `apps.edge-widget` | Edge desktop widget | The older name for the Edge search bar, already covered by the search bar tweak. |
| `apps.firefox-accounts` | Firefox accounts | Firefox sync is opt-in and end-to-end encrypted, so blocking it has no real privacy result. |
| `apps.firefox-addons-update` | Firefox system add-on updates | Stops Firefox receiving fixes delivered through system add-ons, which weakens protection. |
| `apps.firefox-captive` | Captive portal check | A harmless connectivity check; turning it off only stops hotel and cafe Wi-Fi sign-in pages appearing. |
| `apps.firefox-crash` | Firefox crash reporter | No published Firefox policy of this name could be found, and crash reports are only sent with your consent. |
| `apps.firefox-doh` | Firefox DoH | Turns off encrypted DNS lookups, which makes browsing less private rather than more. |
| `apps.firefox-feedback` | Firefox feedback | Only hides menu items; nothing is sent unless you choose to send it. |
| `apps.firefox-home` | Firefox home | The policy needs a set of sub-values, so this single value does nothing. |
| `apps.firefox-import` | Firefox profile import | Blocks moving your data from another browser, with no benefit. |
| `apps.firefox-messaging` | Firefox messaging | The policy needs a set of sub-values, so this single value does nothing. |
| `apps.firefox-password-reveal` | Firefox password reveal | Hides one button, with no practical benefit on a personal PC. |
| `apps.firefox-pocket` | Pocket | Mozilla shut down Pocket in 2025, so there is nothing to turn off. |
| `apps.firefox-prediction` | Firefox network prediction | Written in the wrong form: Firefox expects a simple value called NetworkPrediction, so this does nothing. |
| `apps.firefox-private-shortcut` | Private browsing shortcut | An obscure shortcut change with no privacy or performance result. |
| `apps.firefox-recommendations` | Extension recommendations | Firefox reads this setting inside UserMessaging, so a value in this place does nothing. |
| `apps.firefox-refresh` | Firefox profile refresh | Removes a useful repair tool with no benefit. |
| `apps.firefox-updates` | Firefox updates | Stops Firefox receiving security fixes, which weakens a security protection. |
| `apps.new-outlook-migrate` | New Outlook migration | Only used by the admin-controlled migration that organisations set up, so it has no role on a home PC. |
| `apps.new-outlook-use` | Classic Outlook | A state flag behind the switch inside Outlook, which is a single click there anyway. |
| `apps.office-feedback` | Office feedback | Only hides the feedback option; nothing is sent unless you choose to send it. |
| `apps.office-outlook-calendar` | Outlook calendar logging | Debug logging that's off unless turned on for troubleshooting, so this changes nothing. |
| `apps.office-outlook-mail` | Outlook mail logging | Debug logging that's off unless turned on for troubleshooting, so this changes nothing. |
| `apps.office-pdfmaker` | Acrobat Office add-ins | Extremely niche: only matters with Acrobat installed, and add-ins can be turned off inside Office. |
| `apps.office-tfc` | Team Foundation add-ins | Extremely niche: only matters with Team Foundation tools installed. |
| `apps.office-verbose` | Office verbose logging | A debugging flag that's off by default, so this changes nothing. |
| `apps.office-vs-adaptors` | Visual Studio Office add-ins | Extremely niche: only matters with Visual Studio Office tools installed. |
| `apps.office-word-logging` | Word logging | Debug logging that's off unless turned on for troubleshooting, so this changes nothing. |
| `apps.onedrive-metered` | OneDrive on metered | A Windows 8.1 era policy; the current OneDrive app already pauses on metered networks. |
| `apps.onedrive-personal` | OneDrive personal sync | Written outside OneDrive's policy location, so it isn't expected to have any effect. |
| `apps.onedrive-save` | Save to OneDrive default | A Windows 8.1 era policy that the current OneDrive app doesn't use. |
| `apps.opera-background` | Opera GX background mode | Opera doesn't appear to implement Chromium's group policies, so this value isn't expected to be read. |
| `apps.opera-hw` | Opera GX hardware acceleration | Opera doesn't appear to implement Chromium's group policies, and hardware acceleration is already the default. |
| `apps.opera-metrics` | Opera GX metrics | Opera doesn't appear to implement Chromium's group policies, so this value isn't expected to be read. |
| `apps.opera-search` | Opera GX search provider | Enables a policy without choosing anything, so a person gets no stated result. |
| `apps.store-auto-update` | Store app auto updates | Stops Store apps receiving fixes, including media codecs and gaming components, which weakens protection. |
| `apps.store-remove` | Microsoft Store | Microsoft limits this policy to Enterprise and Education, so it has no effect on a home PC. |
| `apps.vivaldi-metrics` | Vivaldi metrics | A per-browser copy for a small audience; Vivaldi doesn't send Google usage reports, and its policy support could not be confirmed. |
| `apps.vivaldi-prediction` | Vivaldi prediction | A per-browser copy for a small audience, and Vivaldi's policy support could not be confirmed. |
| `apps.vivaldi-spellcheck` | Vivaldi spellcheck | A per-browser copy for a small audience, and Vivaldi's policy support could not be confirmed. |

### cpu (33)

| Tweak | Name | Why |
|---|---|---|
| `cpu.adjust-dpc` | DPC threshold | Undocumented DPC tuning value with no published behaviour on current Windows and no measured benefit. |
| `cpu.boost-policy` | Boost policy | Overlaps the boost mode setting, and CPUs that manage their own clocks largely decide this themselves. |
| `cpu.coalescing` | Timer coalescing | A scatter of undocumented values copied from scripts; timer coalescing is something apps opt into, and nothing shows these values turn it off. |
| `cpu.cooling-active` | Active cooling policy | Active cooling is already the default on mains power, and desktop fans are normally run by the motherboard rather than this setting. |
| `cpu.debug-poll` | Debug poll interval | Only concerns kernel debugger polling, which a normal PC never uses. |
| `cpu.deep-io-coalescing` | Deep I/O coalescing | Undocumented value with no published behaviour; nobody can state or measure what it changes. |
| `cpu.dpc-timeout` | DPC timeout | Undocumented kernel value aimed at removing a safety limit on driver work, with no credible benefit and a real stability risk. |
| `cpu.dpc-watchdog` | DPC watchdog period | Aims to remove the watchdog that catches stuck drivers, which weakens a stability protection for no credible gain. |
| `cpu.dpc-watchdog-offset` | DPC watchdog offset | Same aim as the DPC watchdog entry, an undocumented attempt to loosen a stability protection for no credible gain. |
| `cpu.dynamic-tick-duration` | Dynamic tick duration | Undocumented kernel value with no published behaviour and no measured benefit. |
| `cpu.energy-estimation` | Energy estimation | Undocumented switch that at best removes per-app power figures; there is no credible performance gain behind its medium impact rating. |
| `cpu.event-processor` | Power event processor | Undocumented power logging value; nobody can state a result from changing it. |
| `cpu.exit-latency` | Exit latency | Undocumented idle value with no published behaviour and no measured benefit. |
| `cpu.ideal-dpc-rate` | Ideal DPC rate | DPC queue tuning with no published behaviour on current Windows and no measured benefit. |
| `cpu.idle-demote` | Idle demote threshold | Hidden idle tuning too subtle for anyone to notice, with no practical benefit on a home or gaming PC. |
| `cpu.idle-disable` | Processor idle disable | Keeps every core out of idle, which adds heat and power use and can cost turbo headroom, so most PCs end up worse off. |
| `cpu.interrupt-steering` | Interrupt steering mode | Hidden setting for routing interrupts, with no practical benefit and no way for a person to tell what changed. |
| `cpu.low-qos-timer` | Low QoS timer resolution | Undocumented value; nobody has published what it does on current builds. |
| `cpu.max-state` | Maximum processor state | 100% on mains is already the default, and the 90% battery limit works against the stated purpose. |
| `cpu.minimum-dpc-rate` | Minimum DPC rate | DPC queue tuning with no published behaviour on current Windows and no measured benefit. |
| `cpu.parking-max` | Core parking max cores | 100% is already the default, and the minimum cores setting is the one that decides parking. |
| `cpu.passive-int` | Passive interrupt priority | Undocumented I/O worker priority with no published behaviour or measured benefit. |
| `cpu.per-cpu-clock` | Per-CPU clock ticks | Undocumented clock tick scheduling value; even its description can't say what it changes. |
| `cpu.priority-dpc` | Ideal DPC rate (priority key) | The same undocumented idea as the ideal DPC rate value, under a different key. |
| `cpu.ready-queue` | Shared ready queue size | Undocumented scheduler value with no published behaviour and no measured benefit. |
| `cpu.scheduler-assist` | Scheduler assist override | Undocumented scheduler flag; nobody can say in plain words what it does. |
| `cpu.serialize-timer` | Serialize timer expiration | Undocumented; pushing timer work onto one core has no published benefit and no credible basis as an optimisation. |
| `cpu.split-large-caches` | Split large caches | Undocumented scheduler value with no published behaviour; the claim about cache groups can't be checked. |
| `cpu.tagged-energy` | Tagged energy logging | Internal energy logging values with no visible result and no credible performance gain. |
| `cpu.threaded-dpc` | Threaded DPCs | Turns off a default kernel mechanism with no published benefit for a home PC, and the claimed latency gain has no credible basis. |
| `cpu.tick-power` | Exit latency check | Undocumented power value with no published behaviour and no measured benefit. |
| `cpu.timer-check` | Timer check flags | Undocumented internal flags with no published behaviour; nobody can state a result. |
| `cpu.unpark-count` | Initial unpark count | Undocumented power value with no published behaviour; the core parking plan setting is the real control. |

### gaming (41)

| Tweak | Name | Why |
|---|---|---|
| `gaming.audio-capture` | Game audio capture | A clip-recording preference already in Settings; it has no performance effect and does nothing once Game DVR is off. |
| `gaming.audiodg-priority` | Audio engine priority | Audio engine threads are already scheduled for audio work, so raising the process priority class has no credible benefit. |
| `gaming.cod-vram` | Modern Warfare video memory | Modern Warfare keeps its settings in config files, and nothing shows this registry value is read. |
| `gaming.cod-vsync` | Modern Warfare V-Sync off | Modern Warfare keeps its settings in config files, and nothing shows this registry value is read; the in-game menu is the real switch. |
| `gaming.csrss-priority` | Client server priority | Pushes a critical system process to real-time priority, with no credible benefit and a risk to stability. |
| `gaming.cursor-capture` | Game cursor capture | A clip-recording preference already in Settings; it has no performance effect and does nothing once Game DVR is off. |
| `gaming.dwm-priority` | Desktop compositor priority | Pushes the desktop compositor to real-time priority, which has no credible benefit and can starve the rest of the system. |
| `gaming.fortnite-be-priority` | Fortnite BattlEye priority | This launcher stub only starts the real game process, which the main Fortnite entry already covers. |
| `gaming.fortnite-dx` | Fortnite prefers D3D11 | Fortnite takes its rendering mode from its own settings, and nothing shows this identifiers key is read. |
| `gaming.fortnite-eac-priority` | Fortnite EAC priority | This launcher stub only starts the real game process, which the main Fortnite entry already covers. |
| `gaming.fortnite-low` | Fortnite low quality | Fortnite stores quality settings in its own config file, and nothing shows this registry key is read. |
| `gaming.game-bar-link` | Game Bar links | Writes only two handler values and no open command, so it is unlikely to stop the pop-up, and it only matters after removing Game Bar. |
| `gaming.game-dvr-user` | Background recording | Reaches the same result as the Game DVR policy tweak that the gaming profiles already use. |
| `gaming.keyboard-queue` | Keyboard data queue | The buffer size does not change how fast key presses arrive, so shrinking it has no credible benefit and can drop keys. |
| `gaming.keyboard-response` | Keyboard response | These timings belong to Filter Keys, and the real repeat setting is already covered by the key repeat tweak. |
| `gaming.mic-capture` | Game microphone capture | Microphone capture is a clip preference in Settings that starts off by default, and it does nothing once Game DVR is off. |
| `gaming.mmcss-audio` | MMCSS audio priority | Only reaches apps that register with this scheduler task, and raising it has no measured benefit for a game or for normal audio. |
| `gaming.mmcss-display` | MMCSS display | Only touches a display post-processing task that has no known use in games or on a normal desktop. |
| `gaming.mmcss-games-extra` | MMCSS games extras | Affinity and Background Only are already the defaults, and Latency Sensitive has no published behaviour. |
| `gaming.mmcss-lazy` | MMCSS lazy mode | Undocumented scheduler timeout with no published behaviour and no measured benefit. |
| `gaming.mmcss-pro-audio` | MMCSS pro audio | Only concerns pro audio apps that register with this task, and the values largely match the defaults. |
| `gaming.mmcss-sfio` | Games I/O priority | Only reaches games that register with the scheduler, which the main MMCSS tweak already covers, and has no measured effect. |
| `gaming.mouse-keys` | Mouse Keys shortcut | The value it writes leaves the Mouse Keys shortcut active and only drops the confirmation prompt, so it does not do what it says. |
| `gaming.mouse-queue` | Mouse data queue | The buffer size does not change how fast mouse input arrives, so shrinking it has no credible benefit and can drop input on high polling rates. |
| `gaming.mouse-sensitivity` | Mouse sensitivity | Sets Windows' default pointer speed, a personal preference already in Settings, and can silently change someone's aim. |
| `gaming.mouse-trails` | Mouse trails | Pointer trails are off by default, so this changes nothing unless someone chose to turn them on. |
| `gaming.nahimic-none` | Realtek power management | Undocumented value for the old Realtek audio manager with no published behaviour, and its id does not match what it sets. |
| `gaming.obs-priority` | OBS above normal | OBS has its own process priority setting under Advanced, which does this without a registry entry. |
| `gaming.obs32-priority` | OBS 32-bit above normal | OBS no longer ships a 32-bit Windows build, and the idea is already covered by OBS's own priority setting. |
| `gaming.riot-updates` | Riot auto update | Nothing shows the Riot Client reads this value, and blocking updates would keep online games from launching. |
| `gaming.steam-dpi` | Steam DPI scaling | A cosmetic Steam preference with its own switch in Steam, not confirmed on the current client and with no effect on games. |
| `gaming.steam-dwrite` | Steam DirectWrite | A font option from the old Steam interface, not confirmed on the current client and with no effect on games. |
| `gaming.steam-gpu-web` | Steam web GPU acceleration | Steam's Interface settings hold this switch, the value name comes from old scripts and is not confirmed on the current client. |
| `gaming.steam-h264` | Steam H.264 acceleration | Only affects video inside Steam, which has its own switch, and software decoding uses more CPU rather than less. |
| `gaming.steam-overlay` | Steam overlay | Nothing shows Steam reads this value for its overlay; the switch in Steam's In Game settings is the real control. |
| `gaming.steam-silent` | Steam silent start | Nothing shows Steam reads this value, and starting Steam minimised is set in Steam or with the startup tweak. |
| `gaming.steam-smooth` | Steam smooth scrolling | A trivial scrolling preference with its own switch in Steam and no effect on games. |
| `gaming.steam-startup` | Steam startup mode | Only picks which window Steam opens to, which Steam's own settings already cover. |
| `gaming.steam-tcp` | Steam TCP settings | These are Windows TCP value names copied into Steam's key, and nothing shows Steam reads them. |
| `gaming.steam-updates` | Steam auto update | Nothing shows Steam reads this value, and the Steam client is not meant to run without updates. |
| `gaming.touch-pointer` | Absolute pointer | Only affects absolute pointing devices such as some tablets, not a normal gaming mouse; extremely niche. |

### gpu (125)

| Tweak | Name | Why |
|---|---|---|
| `gpu.amd-aa-flags` | Anti-aliasing flags | Undocumented values with no published behaviour; anti-aliasing is set in each game and in AMD Software. |
| `gpu.amd-apu-segment` | APU shared segment | Undocumented memory values for AMD APUs with no published behaviour and a risk of instability. |
| `gpu.amd-auto-update` | AMD auto update | No published behaviour, and AMD Software has its own update check setting. |
| `gpu.amd-bgm-ltr` | Background latency tolerance | Undocumented PCIe latency values copied from scripts, with no published behaviour or measured benefit. |
| `gpu.amd-block-write` | Block write | Undocumented driver value; turning off a graphics memory feature has no published benefit. |
| `gpu.amd-boost-cn` | Core Performance Boost | Undocumented flag under AMD Software's key, and nobody can say what it changes. |
| `gpu.amd-chill` | Radeon Chill | Radeon Chill is an opt-in switch in AMD Software, so this undocumented value adds nothing. |
| `gpu.amd-color-depth` | Auto colour depth | Undocumented value with no published behaviour; colour depth is set in AMD Software and Windows display settings. |
| `gpu.amd-compute-latency` | Compute latency | Undocumented kernel driver value with no published behaviour or measured benefit. |
| `gpu.amd-cross-gpu` | Cross-GPU display | Undocumented value with no published behaviour and no benefit on a single-GPU PC. |
| `gpu.amd-dal-latency` | Display latency | Undocumented display engine values with no published behaviour; a wrong value risks display faults. |
| `gpu.amd-dmacopy` | DMA copy | Undocumented driver value; turning off DMA copy has no published benefit and could slow transfers. |
| `gpu.amd-drmdma-gating` | DRM DMA gating | Undocumented power gating value with no published behaviour; at best it adds idle power. |
| `gpu.amd-dvr` | AMD recording | AMD's recorder has its own switch in AMD Software, and this undocumented value adds nothing. |
| `gpu.amd-dxr` | Ray tracing support | Breaks games that need ray tracing and gains nothing in games that do not use it. |
| `gpu.amd-dynamic-res` | Dynamic resolution | Undocumented value with no published behaviour; dynamic resolution is a per-game feature. |
| `gpu.amd-frtc` | Frame rate target | Undocumented value with no published behaviour; the frame limiter is opt-in through AMD Software. |
| `gpu.amd-fsr` | FidelityFX flag | Undocumented flag with no published behaviour; FidelityFX upscaling is chosen in each game. |
| `gpu.amd-gpu-down` | GPU power down | Undocumented power value with no published behaviour; at best it adds idle power. |
| `gpu.amd-ltr` | Latency tolerance | Undocumented PCIe latency values copied from scripts, with no published behaviour or measured benefit. |
| `gpu.amd-memclock-latency` | Memory clock switch | Undocumented memory clock value with no published behaviour; a wrong value risks screen flicker. |
| `gpu.amd-power-gating` | Power gating | Undocumented power gating value with no published behaviour; at best it adds idle power. |
| `gpu.amd-power-scheme` | AMD power scheme | Undocumented flag; nobody can say what this power scheme value selects. |
| `gpu.amd-pp-latency` | PowerPlay latency | Undocumented PowerPlay values copied from scripts, with no published behaviour or measured benefit. |
| `gpu.amd-px-video` | Video on integrated | Undocumented value for hybrid laptops only, with no published behaviour; Windows graphics settings choose the GPU per app. |
| `gpu.amd-rs-overlay` | Radeon overlay | The Radeon overlay is controlled in AMD Software, and this undocumented value is rewritten on driver installs. |
| `gpu.amd-rsr` | Radeon Super Resolution | Radeon Super Resolution is opt-in through AMD Software, so these undocumented values add nothing. |
| `gpu.amd-sclk-sleep` | Engine clock deep sleep | Undocumented power value with no published behaviour; at best it adds idle power. |
| `gpu.amd-skins` | Radeon skins | Undocumented cosmetic flag for AMD Software with no published behaviour. |
| `gpu.amd-snapshot` | Radeon snapshots | Undocumented flag for AMD Software with no published behaviour and no result anyone would notice. |
| `gpu.amd-stutter` | Stutter mode | Undocumented display memory power value despite its name; it has nothing to do with game stutter and at best adds idle power. |
| `gpu.amd-subscription` | Radeon subscription | Undocumented flag for AMD Software with no published behaviour and no result anyone would notice. |
| `gpu.amd-thermal-cn` | CN thermal limit | Undocumented thermal value with no published behaviour; nobody should guess at GPU temperature limits. |
| `gpu.amd-thermal-throttle` | Thermal auto throttling | Aims to turn off the GPU's thermal protection, which is unsafe and has no place in this app. |
| `gpu.amd-ulps` | Ultra low power state | Only matters on old CrossFire rigs and some hybrid laptops, gains nothing on a single-GPU desktop and has no AMD documentation. |
| `gpu.amd-usage-tracking` | AMD usage tracking | Undocumented values; the AMD User Experience task and AMD Software's own option are the real controls. |
| `gpu.amd-uvd-gating` | UVD power gating | Undocumented power gating value for the video decoder; at best it adds idle power. |
| `gpu.amd-vce-gating` | VCE power gating | Undocumented power gating value for the video encoder; at best it adds idle power. |
| `gpu.amd-vrs` | Variable rate shading flag | Undocumented flag with no published behaviour; variable rate shading is used by each game. |
| `gpu.amd-vsr` | Virtual Super Resolution | Virtual Super Resolution is opt-in through AMD Software, so these undocumented values add nothing. |
| `gpu.colour-calibration` | Calibration loader | Undocumented value with no published behaviour; colour profiles are managed in Windows display settings. |
| `gpu.d3d-flip-no-vsync` | Direct3D 9 flip | A legacy Direct3D value with no evidence it works on current Windows; frame pacing is set by each game and driver. |
| `gpu.ddraw-nonlocal` | Non-local video memory | A DirectDraw-era value that does nothing for modern games; extremely niche. |
| `gpu.dwm-advanced-flip` | Advanced direct flip | Undocumented compositor debug value; removing a flip mode has no credible benefit and may hurt latency. |
| `gpu.dwm-cpu-clipping` | CPU clipping | Undocumented compositor value with no published behaviour; it says it keeps something on that nobody turned off. |
| `gpu.dwm-debug` | DWM debug fail fast | A compositor debug value with no result anyone can state. |
| `gpu.dwm-drawlist` | Draw list caching | Undocumented compositor value; turning off a cache has no published benefit. |
| `gpu.dwm-effect-cache` | Effect caching | Undocumented compositor value with no published behaviour; it says it keeps something on that nobody turned off. |
| `gpu.dwm-hibernate` | Thumbnail hibernation | Saving taskbar thumbnails is already off by default, so nobody would notice this setting. |
| `gpu.dwm-hologram` | Holographic compositor | Undocumented value for a holographic compositor that a home or gaming PC does not use. |
| `gpu.dwm-independent-flip` | Independent flip | Writes the default and keeps an undocumented value off, so it changes nothing. |
| `gpu.dwm-linear` | Linear resampling | Undocumented compositor resampling values with no published behaviour; nobody can state a result. |
| `gpu.dwm-locking` | DWM memory locking | Undocumented compositor value with no published behaviour and no measured benefit. |
| `gpu.dwm-megarects` | Mega rects | Undocumented compositor value with no published behaviour; it says it keeps something on that nobody turned off. |
| `gpu.dwm-parallel` | Parallel mode policy | Undocumented compositor policy with no published behaviour; nobody can state a result. |
| `gpu.dwm-prediction` | Interaction prediction | Undocumented compositor value with no published behaviour and no measured benefit. |
| `gpu.dwm-primitive` | Primitive reordering | Undocumented compositor value with no published behaviour; nobody can state a result. |
| `gpu.dwm-shadows` | DWM projected shadows | Undocumented compositor values with no confirmed result; window shadows are handled by the visual effects setting. |
| `gpu.dwm-shift` | Animations shift key | A compositor debug shortcut that is off in normal use, so nobody would notice. |
| `gpu.dwm-wet-ink` | Super wet ink | Undocumented pen ink value with no published behaviour, and it has no use on a PC without a pen. |
| `gpu.gamma-range` | GDI gamma range | Only matters to the few apps, such as f.lux, that ask for a wider gamma range; extremely niche. |
| `gpu.intel-msaa` | Intel MSAA | Undocumented values with no published behaviour; anti-aliasing is set in each game. |
| `gpu.intel-preemption` | Intel preemption | Undocumented driver value; turning off GPU preemption has no credible benefit and risks hangs. |
| `gpu.intel-raytracing` | Intel ray tracing | Undocumented value with no published behaviour; a game that doesn't use ray tracing gains nothing. |
| `gpu.intel-scaling` | Intel dynamic scaling | Undocumented value with no published behaviour; nobody can say what it changes. |
| `gpu.intel-sr` | Intel super resolution | Undocumented value with no published behaviour; upscaling is chosen in Intel's software or in each game. |
| `gpu.intel-xess` | Intel XeSS | Undocumented value with no published behaviour; XeSS is chosen in each game. |
| `gpu.miracast` | Miracast support | Has no benefit and only stops wireless displays from working. |
| `gpu.monitor-latency` | Monitor latency tolerance | Undocumented latency values copied from scripts, written to three keys with no published behaviour or measured benefit. |
| `gpu.monitor-refresh-latency` | Refresh latency tolerance | Undocumented latency values copied from scripts, written to three keys with no published behaviour or measured benefit. |
| `gpu.nv-ansel` | Ansel | Undocumented value for a capture tool that NVIDIA has moved on from; it runs only when used. |
| `gpu.nv-async-compute` | Async compute | Undocumented driver value with no published behaviour, and it writes 0 while claiming to turn the feature on. |
| `gpu.nv-async-queue` | Async command queue | Undocumented driver value with no published behaviour; async compute is decided by each game. |
| `gpu.nv-camera` | Nvfcamera | Undocumented value for a capture feature that runs only when used. |
| `gpu.nv-context-isolation` | Context isolation | Undocumented driver value with no published behaviour; isolation between GPU workloads is not something to turn off blindly. |
| `gpu.nv-contiguous` | Contiguous system memory | A driver string with no published behaviour; proof the name exists says nothing about what it does. |
| `gpu.nv-debug-heap` | GPU debug heap | Undocumented value; debug features are not active on normal drivers, so there is nothing to gain. |
| `gpu.nv-debug-layer` | Driver debug layer | Undocumented value; debug features are not active on normal drivers, so there is nothing to gain. |
| `gpu.nv-display-power` | Display power saving | Undocumented value with unclear behaviour and, at most, a laptop-only effect. |
| `gpu.nv-dma` | DMA acceleration | Undocumented driver value with no published behaviour or measured benefit. |
| `gpu.nv-dynamic-pstate` | Dynamic P-state switch | Undocumented value that repeats the idea of the P-state switch, with no known effect of its own. |
| `gpu.nv-event-driven` | Event-driven mode | Undocumented driver value; nobody can say what this mode does. |
| `gpu.nv-event-tracing` | Kernel event tracing | Undocumented value with no published behaviour and no measured benefit. |
| `gpu.nv-frame-reporting` | Frame time reporting | Undocumented value with no published behaviour and no measured benefit. |
| `gpu.nv-freestyle` | Freestyle | Game filters run only when used, and NVIDIA's own app controls them; nothing shows this value is read. |
| `gpu.nv-gpudebugger` | GPU debugger | Debugger hooks are for developers and idle on a gaming PC; no practical benefit. |
| `gpu.nv-gr535` | GR535 flag | An unnamed driver feature flag; nobody can say what it does, so turning it off is guesswork. |
| `gpu.nv-hda-d3cold` | HD Audio D3 cold | Undocumented value for the GPU's HDMI audio device with no published behaviour; a wrong state risks lost audio. |
| `gpu.nv-hdcp` | HDCP key | Has no published benefit and can stop protected video from playing. |
| `gpu.nv-instrumentation` | Driver instrumentation | Undocumented value with no published behaviour and no measured benefit. |
| `gpu.nv-l2-compression` | L2 compression | Undocumented driver value; turning off a memory compression feature has no published benefit. |
| `gpu.nv-link-state` | Max link state | Undocumented value with no published behaviour; PCIe link power is a Windows power plan setting. |
| `gpu.nv-logging` | Driver logging | Undocumented values across many keys; the driver writes little in normal use and losing error logs makes crashes harder to diagnose. |
| `gpu.nv-low-latency` | Low latency boost | Undocumented value that writes 0 while claiming to turn something on; low latency is set in the NVIDIA Control Panel and in games. |
| `gpu.nv-msi` | NVIDIA MSI | Message-signalled interrupts are not set here, so nothing shows this value does what it says. |
| `gpu.nv-p2p` | Peer-to-peer transfers | Undocumented driver value with no published behaviour and no gaming use. |
| `gpu.nv-param-flags` | Driver parameter flags | Four unnamed driver flags; nobody can say what they do, so turning them off is guesswork. |
| `gpu.nv-per-core-dpc` | Per-core DPC | A popular script value with no published behaviour or measured benefit, written to five keys in the hope one is read. |
| `gpu.nv-perf-level` | Performance level source | Undocumented value left from old drivers, with no published behaviour on current cards. |
| `gpu.nv-perf-sdk` | Performance SDK | Developer tooling that is idle on a gaming PC; no practical benefit. |
| `gpu.nv-polling` | Completion polling | Undocumented value that writes 0 while claiming to poll; nobody can state a result. |
| `gpu.nv-power-saving` | Driver power saving | Undocumented value with no published behaviour; the Control Panel's power mode is the real setting. |
| `gpu.nv-powermizer` | PowerMizer | Undocumented value left from old drivers, and it writes 1 while claiming to turn the feature off. |
| `gpu.nv-preemption` | Driver preemption | Undocumented driver value; turning off GPU preemption has no credible benefit and risks hangs. |
| `gpu.nv-preemption-compute` | Compute preemption | Undocumented driver value; changing compute preemption has no credible benefit and risks hangs. |
| `gpu.nv-preemption-cuda` | CUDA preemption | Undocumented driver value that only concerns CUDA workloads and writes the off state of a disable flag. |
| `gpu.nv-preemption-level` | Preemption level | Undocumented driver value; changing the preemption level has no credible benefit and risks hangs. |
| `gpu.nv-preemption-s3` | Preemption on sleep | Undocumented driver value concerning sleep only, with no result anyone would notice. |
| `gpu.nv-profile-telemetry` | Profile telemetry | Undocumented value; the telemetry service and tasks are the real controls. |
| `gpu.nv-rebar` | Resizable BAR flag | Resizable BAR is set in firmware and driver profiles, and nothing shows this value does anything. |
| `gpu.nv-reflex-driver` | Driver low latency | Undocumented value that writes 0 while claiming to turn something on; Reflex and low latency mode are set in games and the Control Panel. |
| `gpu.nv-rid-flags` | RID feature flags | Two unnamed driver feature flags; nobody can say what they do, so setting them is guesswork. |
| `gpu.nv-rm-cache` | RM registry caching | Undocumented driver value with no published behaviour; turning off a cache has no published benefit. |
| `gpu.nv-scaling-flags` | Scaling flags | Undocumented driver flags with no published behaviour; scaling is set in the NVIDIA Control Panel. |
| `gpu.nv-stereo` | Stereo 3D | NVIDIA dropped 3D Vision from its drivers years ago, so this has nothing to turn off. |
| `gpu.nv-tdr` | NVIDIA TDR values | Copies the documented Windows timeout into the driver's key, where it has no published effect. |
| `gpu.nv-telemetry-driver` | Driver telemetry | Undocumented values with no sign the driver reads them; the telemetry service and tasks are the real controls. |
| `gpu.nv-thread-policy` | Thread policy | Undocumented driver value; nobody can say what this policy selects. |
| `gpu.nv-vsr` | RTX Video Super Resolution | Undocumented values; RTX Video is opt-in through the NVIDIA Control Panel, so nothing needs turning off. |
| `gpu.overlays` | Hardware overlays | Undocumented value backed only by forum anecdotes, one of which says it breaks variable refresh rate. |
| `gpu.preemption` | GPU preemption | Turning off scheduler preemption has no published gaming benefit and lets a busy GPU freeze the desktop for longer. |
| `gpu.tdr-async` | TDR async reset | Undocumented recovery value with no published behaviour; nobody can state a result. |
| `gpu.tiff-vk-debug` | Vulkan loader debug | Silences a Vulkan logger that stays quiet unless someone turns it on, so it changes nothing. |
| `gpu.vsync-idle` | V-Sync idle timeout | Undocumented scheduler value with no published behaviour and no measured benefit. |
| `gpu.wpf-hw` | WPF hardware acceleration | A last-resort troubleshooting switch that forces apps to render on the CPU; it makes things slower and is extremely niche. |

### memory (7)

| Tweak | Name | Why |
|---|---|---|
| `memory.clear-pagefile` | Keep page file at shutdown | Already the Windows default, so it changes nothing on a normal PC. |
| `memory.compat-ndu` | Network Data Usage driver | A workaround for an old non-paged pool leak that costs you the Data usage page, with no proven benefit on current Windows. |
| `memory.large-system-cache` | Large system cache | A server-era setting that takes memory from apps and games to favour the file cache, which helps no desktop workload. |
| `memory.page-combining` | Memory page combining | Trades a tiny amount of background CPU work for higher memory use, with no difference anyone can notice. |
| `memory.prefetcher` | Prefetcher | Turning it off slows app starts and has no credible benefit on any drive; Windows already tunes it for SSDs. |
| `memory.sf-tracing` | Superfetch tracing | An internal tracing value with no documented behaviour and no visible result. |
| `memory.superfetch` | Superfetch parameter | A leftover value: current Windows controls preloading through the SysMain service, so this has no dependable effect. |

### network (81)

| Tweak | Name | Why |
|---|---|---|
| `network.ack-frequency` | TCP ACK frequency | Microsoft documents this value per network interface, but this writes the global key, and game traffic is mostly UDP anyway. |
| `network.adapter-eth-arp-offload` | ARP offload off | Only changes how the adapter behaves while the PC sleeps, with no practical benefit. |
| `network.adapter-eth-ns-offload` | NS offload off | Only changes how the adapter behaves while the PC sleeps, with no practical benefit. |
| `network.adapter-wifi-arp-offload` | ARP offload off | Only changes how the adapter behaves while the PC sleeps, with no practical benefit. |
| `network.adapter-wifi-ns-offload` | NS offload off | Only changes how the adapter behaves while the PC sleeps, with no practical benefit. |
| `network.adapter-wifi-packet-coalescing` | Packet coalescing off | Only affects broadcast and multicast packets, not game traffic, and turning it off costs battery for no practical gain. |
| `network.afd-receive` | Default receive window | Undocumented Winsock default that only reaches apps not setting their own buffers; TCP auto-tuning already handles normal traffic. |
| `network.afd-send` | Default send window | Undocumented Winsock default that only reaches apps not setting their own buffers; TCP auto-tuning already handles normal traffic. |
| `network.arp-retry` | ARP retries | Tunes a low-level retry count with no result anyone would notice on a home network. |
| `network.arp-single` | ARP single route | A leftover from Token Ring networks, which no home or gaming PC has. |
| `network.arp-snap` | ARP SNAP | Selects an old frame format that a normal Ethernet network never needs. |
| `network.connect-retrans` | Connect retransmissions | Makes connections give up sooner on a bad link, with no benefit to speed or latency. |
| `network.connections` | HTTP connections | Applies to the old WinINet connection limit, which modern browsers do not use. |
| `network.connections-1` | HTTP/1.0 connections | Applies to the old WinINet limit for HTTP/1.0 servers, which modern browsers do not use. |
| `network.dca` | Direct cache access | Direct Cache Access is a server-era feature that needs matching hardware, with no benefit on a home or gaming PC. |
| `network.dead-gw` | Dead gateway detection | Only matters on PCs with more than one default gateway, which a home or gaming PC does not have. |
| `network.del-ack` | Delayed ACK ticks | Documented per network interface but written to the global key, and it has no credible benefit for games, which mostly use UDP. |
| `network.dns-bucket` | DNS cache bucket | Writes the old default for an internal DNS cache table, with no result anyone would notice. |
| `network.dns-max-size` | DNS cache size | No published behaviour for this value on current Windows, and a home PC's DNS cache is small. |
| `network.dns-max-ttl` | DNS maximum TTL | 86400 seconds is already the Windows default, so this changes nothing. |
| `network.dns-neg-time` | Negative cache time | An older name for the negative cache limit that the DNS negative TTL entry already covers. |
| `network.dns-net-failure` | Network failure cache | Writes what is already the usual value for an old setting, so nobody would notice. |
| `network.dns-soa-ttl` | DNS SOA TTL | Tunes an internal DNS record lifetime with no result anyone would notice. |
| `network.dns-table` | DNS cache table | Tunes the size of an internal hash table; a home PC's DNS cache never strains it. |
| `network.dns-timeout` | DNS cache timeout | Not a known DNS Client value; it looks borrowed from an old browser setting, and nothing shows it is read. |
| `network.do-machine` | Delivery Optimization mode | Same result as the Delivery Optimization policy tweak, without the policy that makes it stick. |
| `network.dup-acks` | Duplicate ACKs | Writes what was already the usual value for a setting from the old TCP stack, with no result anyone would notice. |
| `network.forward-buffers` | Forward buffer memory | Applies to PCs acting as routers, which a home or gaming PC is not. |
| `network.forward-packets` | Forward packets | Applies to PCs acting as routers, which a home or gaming PC is not. |
| `network.free-tcbs` | TCP control blocks | Tuning for servers holding huge numbers of connections, and a registry value from the older TCP stack; a home PC never reaches it. |
| `network.hash-table` | TCP hash table | Microsoft treats this as a legacy value from before the current TCP stack, and a home PC never strains the table. |
| `network.hotspot-login` | Hotspot auto login | An undocumented leftover from Wi-Fi Sense with no result anyone can state. |
| `network.igmp` | IGMP level | Writes the default multicast level, so it changes nothing. |
| `network.irp-stack` | Network IRP stack | Tuning for file servers with old filter drivers; there is no benefit to a home or gaming PC. |
| `network.lm-hosts-wsd` | WSD in TCP/IP | No published behaviour for this value on current Windows, and its description cannot be confirmed. |
| `network.lmhosts` | LMHOSTS lookup | The LMHOSTS file is unused on a modern home PC, so nobody would notice. |
| `network.max-data-retrans` | Retransmission limit | An older name for a limit that has no practical use. |
| `network.max-retrans` | Data retransmissions | Only makes connections drop sooner on a shaky link, with no benefit to speed or latency. |
| `network.max-user-port` | Dynamic port limit | Netsh is the documented control on current Windows, and a home PC never runs out of the default 16,384 dynamic ports. |
| `network.mcast-forward` | Multicast forwarding | Applies to PCs acting as multicast routers, which a home or gaming PC is not. |
| `network.media-sense-log` | Media sense log | Only stops a log entry when a cable is plugged in or removed, with no result anyone would notice. |
| `network.metered-device` | Costed network policy | Its key path has no SOFTWARE level, so it cannot match a real Windows key and does nothing useful. |
| `network.minimise-connections` | Minimise connections | Only clears a policy that a home PC does not have, so it changes nothing. |
| `network.ndiscap-note` | Native Wi-Fi filter | Writes the usual start type for a Wi-Fi driver, so nobody would notice, and getting it wrong can break Wi-Fi. |
| `network.ndisvirtual` | NDIS Virtual Bus | Turning off this virtual bus driver has no benefit and can break features that use virtual adapters. |
| `network.neg-cache-policy` | Negative DNS cache (policy) | The same limit as the DNS negative TTL entry, set again through a policy. |
| `network.netbios-driver` | NetBIOS driver | Turning off this driver has no benefit and can break older network features; the name lookup setting is the useful control. |
| `network.no-delay` | TCP no delay | Nagle's algorithm is controlled per connection by each app, and this global value has no documented effect on supported Windows. |
| `network.null-sessions` | Null session access | Restricting anonymous share access is already the Windows default, so this changes nothing. |
| `network.pmtu` | Path MTU discovery | Path MTU discovery is already on by default, so this changes nothing. |
| `network.pmtu-bh` | Black hole detection | Writes the default state for a rarely needed setting, so this changes nothing. |
| `network.provider-dns` | DNS priority | Keeps the same lookup order as the Windows default, only with smaller numbers, so nothing changes. |
| `network.provider-hosts` | Hosts priority | Keeps the same lookup order as the Windows default, only with smaller numbers, so nothing changes. |
| `network.provider-local` | Local priority | Keeps the same lookup order as the Windows default, only with smaller numbers, so nothing changes. |
| `network.provider-netbt` | NetBT priority | Keeps the same lookup order as the Windows default, only with smaller numbers, so nothing changes. |
| `network.psched-sends` | Packet scheduler sends | A packet scheduler limit that a home network never reaches. |
| `network.psched-timer` | Packet scheduler timer | A legacy packet scheduler setting with no result anyone would notice. |
| `network.qos-reserve` | QoS bandwidth reserve | Windows does not hold back bandwidth from normal traffic unless an app asks for a reservation, so this fixes a myth rather than a limit. |
| `network.qos-valorant-launcher` | VALORANT launcher QoS tag | Tags a launcher that carries no match traffic, and the game itself has its own QoS tweak. |
| `network.rss-base` | RSS base CPU | Receive side scaling is set per adapter on current Windows, and nothing shows this global value is read. |
| `network.rto` | Initial retransmission time | Windows sets this through netsh rather than this value, and nothing shows it is read. |
| `network.smb-autodisconnect` | SMB auto disconnect | Idle SMB clients reconnect by themselves, so keeping them attached has no benefit anyone would notice. |
| `network.smb-client-threads` | SMB client threads | Tuning for heavy file server workloads that a home PC never reaches. |
| `network.smb-dir-cache` | SMB directory cache | Tuning for heavy file server workloads that a home PC never reaches. |
| `network.smb-file-cache` | SMB file info cache | Tuning for heavy file server workloads that a home PC never reaches. |
| `network.smb-keepconn` | SMB keep connection | Tuning for heavy file server workloads, with no result anyone would notice. |
| `network.smb-large-mtu` | SMB large MTU | Large MTU is already on by default, so this changes nothing. |
| `network.smb-max-cmds` | SMB max commands | Tuning for heavy file server workloads, and Microsoft's own sources disagree on the default and on whether it affects SMB 2 or later. |
| `network.smb-oplocks` | SMB oplocks | Turning off opportunistic locks slows file sharing and is only a workaround for old database apps on shares. |
| `network.smb-sharing-delay` | Sharing violation delay | Undocumented file server value with no result anyone can state. |
| `network.smb-signing-enable` | SMB signing | Microsoft says this value is ignored for SMB 2 and later, so it does nothing while SMB 1 is off. |
| `network.smb-size` | Server size | A legacy file server size setting with no benefit to a home or gaming PC. |
| `network.task-offload` | Task offload | Task offload is already on by default, so this changes nothing. |
| `network.tcp1323` | TCP window scaling | Microsoft says window scaling is now auto-tuned, so this older value is not needed. |
| `network.teredo-mdns` | Local name providers | Turning off Winsock name providers has no benefit and can break name lookups. |
| `network.timed-wait` | TIME_WAIT delay | Only matters to servers opening thousands of short connections, not a home or gaming PC. |
| `network.ttl` | Default TTL | A lower starting TTL gives no speed or latency benefit, and 128 works everywhere. |
| `network.urgent` | RFC 1122 urgent pointer | Selects an old TCP urgent pointer style that games and modern apps do not use. |
| `network.vwifi` | Virtual Wi-Fi filter | Writes the usual start type for a Wi-Fi filter driver, so nobody would notice, and getting it wrong can break Wi-Fi. |
| `network.wcn` | Windows Connect Now | Windows Connect Now is a rarely used router setup feature, so nobody would notice this on a home PC. |
| `network.wifi-sense` | Wi-Fi Sense | Microsoft retired Wi-Fi Sense's hotspot sharing years ago, so these values have little or nothing left to control on supported Windows. |

### performance (1)

| Tweak | Name | Why |
|---|---|---|
| `performance.foreground-boost` | Favour the app you're using | The value it writes decodes to what client Windows already does by default, so it changes nothing. |

### power (15)

| Tweak | Name | Why |
|---|---|---|
| `power.away-mode` | Away mode | A Media Center-era mode that makes a PC look asleep while it keeps running, with no use on a home or gaming PC. |
| `power.brightness` | Full display brightness | The brightness slider already does this, and forcing full brightness only costs battery and eye comfort. |
| `power.cs-enabled` | Connected standby | Windows 10 version 2004 dropped this value, so it does nothing on supported versions. |
| `power.custom-setup` | Plan customisation at setup | Only read during Windows setup, so changing it on a running PC does nothing useful. |
| `power.device-idle` | Device idle policy | A hidden plan value with no measurable result; nobody can say what changes when it is set. |
| `power.dim-brightness` | No dimming | Only affects a few seconds of dimming on laptops before the screen turns off. |
| `power.gpu-preference` | GPU preference policy | A hidden plan value with no published meaning for its options; Windows Graphics settings is the working way to choose a GPU per app. |
| `power.nvme-noppme` | NVMe NOPPME | Its label was wrong: it allows background work in idle states rather than turning those states off, and it has no practical benefit on a home PC. |
| `power.personality` | Plan personality | Only relabels the plan as high performance without changing any real setting, and can confuse software that reads it. |
| `power.platform-aoac` | Modern Standby override | Only works on laptops that still have S3 sleep in their firmware; on newer ones it can crash the PC or lose sleep altogether. |
| `power.platform-role` | Platform role | Windows takes this from firmware; overriding it has no defined result and can confuse laptop power handling. |
| `power.standby-network` | Network in standby | Only applies to Modern Standby laptops, where it keeps the network awake during sleep and drains the battery, with no benefit to games or speed. |
| `power.timer-frequency` | JavaScript timer frequency | Only applied to Internet Explorer and old Edge; today's browsers ignore it. |
| `power.usb-ioc` | USB IOC on all TDs | Forces an interrupt on every USB transfer, adding CPU work with no proven benefit to input or speed. |
| `power.usb-selective` | USB selective suspend (plan) | Same goal as the machine-wide USB selective suspend switch, so a second card adds nothing. |

### privacy (137)

| Tweak | Name | Why |
|---|---|---|
| `privacy.account-consent` | Account info consent | Same outcome as the first-class account info block. |
| `privacy.autocorrect` | Touch keyboard autocorrect | A touch keyboard preference already in Settings, with no privacy or performance result. |
| `privacy.autologger-diaglog` | DiagLog trace | A local trace used by Windows diagnostics; nothing shows it sends data, and stopping it saves nothing measurable. |
| `privacy.autologger-sqm` | SQM trace | A trace for the old improvement program logger, with no known effect on current Windows. |
| `privacy.autologger-wifi` | Wi-Fi session trace | A local Wi-Fi diagnostic trace that sends nothing, so stopping it has no practical benefit. |
| `privacy.background-global` | Background apps (user) | The older per-user version of the background apps block, which covers every user. |
| `privacy.camera-effects` | Camera effects | Writes directly to the policy defaults store and only touches PCs with camera effects hardware, so the outcome is unclear. |
| `privacy.camera-policy` | Camera policy | Same outcome as the camera switch, but it also locks the setting in Windows. |
| `privacy.cdm-content` | Content delivery | An undocumented master switch with no confirmed behaviour; the individual suggestion settings do the real work. |
| `privacy.cdm-feature-mgmt` | Feature management | A community value with no known behaviour. |
| `privacy.cdm-mru` | Spotlight history | An unnamed suggestion channel with no known behaviour. |
| `privacy.cdm-preinstalled` | Preinstalled apps | Same aim as the silent installs tweak, and it only acts when a new account is set up. |
| `privacy.cdm-rest` | Other suggestion channels | An unnamed suggestion channel with no known behaviour. |
| `privacy.cdm-slideshow` | Content slideshow | No known behaviour of its own; the lock screen slideshow is handled by the Spotlight tweak. |
| `privacy.cloud-optimised` | Cloud-optimised content | A vague policy; nobody can state what a person would see change. |
| `privacy.commercial-pipeline` | Commercial data pipeline | Applies to organisations sending data to their own Microsoft tenant, never a home PC. |
| `privacy.compat-engine` | Application compatibility engine | Turning off the compatibility engine can break older games and programs for no measurable gain. |
| `privacy.compat-tab` | Compatibility tab | Hides a useful troubleshooting tab with no benefit. |
| `privacy.consumer-features` | Consumer features | Microsoft limits this policy to Enterprise and Education; the silent installs tweak does the job on home PCs. |
| `privacy.copilot-large` | Copilot on large screens | Applies to the old Copilot sidebar, which Windows has replaced with an app. |
| `privacy.cortana` | Cortana | Microsoft retired Cortana in Windows in 2023. |
| `privacy.cortana-lock` | Cortana on the lock screen | Microsoft retired Cortana in Windows in 2023. |
| `privacy.cpss-advertising` | Advertising setting store | A copy of the advertising ID setting; the policy version covers every account. |
| `privacy.cpss-ink` | Inking store | A stored copy of the inking and typing setting. |
| `privacy.cross-device` | Cross-device experiences | The value names could not be confirmed, and the real Windows switch appears to be a different value. |
| `privacy.cross-resume` | Cross-device resume | Appears to target the wrong value, so it isn't expected to work. |
| `privacy.desktop-analytics` | Desktop Analytics | Desktop Analytics was a retired service for organisations, never used on a home PC. |
| `privacy.device-metadata` | Device metadata | Same setting as the first-class device apps tweak. |
| `privacy.device-name` | Device name in telemetry | Windows already leaves the device name out of diagnostic data by default. |
| `privacy.device-registration` | Device registration | Only concerns joining a PC to an organisation's directory. |
| `privacy.edge-data-optin` | Edge data in telemetry | Applies to legacy Edge, which was removed from Windows in 2021. |
| `privacy.enhanced-audio` | Enhanced speech audio | A community value with no known behaviour. |
| `privacy.error-consent` | Error report consent | Pointless once error reporting is off, which is the clearer choice. |
| `privacy.error-extra` | Additional crash data | Pointless once error reporting is off, which is the clearer choice. |
| `privacy.error-logging` | Error report logging | Pointless once error reporting is off, which is the clearer choice. |
| `privacy.error-ui` | Crash report prompts | Pointless once error reporting is off, which is the clearer choice. |
| `privacy.experimentation` | Windows experiments | Writes directly to the MDM policy store, and Microsoft lists this policy for Pro and above only, so the outcome is uncertain. |
| `privacy.feedback-notifications` | Feedback notifications | Same outcome as the first-class feedback tweak, which the owner uses. |
| `privacy.handwriting-data` | Handwriting data sharing | An old Tablet PC option covered by the typing and inking tweak. |
| `privacy.handwriting-reports` | Handwriting error reports | An old Tablet PC reporting option for pen users, with no noticeable result. |
| `privacy.harvest-contacts` | Contact harvesting | Part of the same Windows setting as the typing and inking tweak. |
| `privacy.ie-geolocation` | IE geolocation | Internet Explorer was retired in 2022. |
| `privacy.ie-logging` | InPrivate logging | Internet Explorer was retired in 2022. |
| `privacy.ie-sqm` | Internet Explorer CEIP | Internet Explorer was retired in 2022. |
| `privacy.index-attachments` | Index email attachments | Extremely niche: only affects indexing of Outlook attachments, and the index stays on your PC. |
| `privacy.index-encrypted` | Index encrypted files | Windows already keeps encrypted items out of the index by default. |
| `privacy.index-language` | Language detection | An internal indexing detail with no outcome a person could notice. |
| `privacy.index-outlook` | Index Outlook | Makes Outlook search slower and more limited, and the index never leaves your PC. |
| `privacy.index-public` | Index public folders | Only for Exchange public folders in organisations. |
| `privacy.index-removable` | Index removable drives | Extremely niche, and the index never leaves your PC. |
| `privacy.indexer-backoff` | Indexer back-off | Lets indexing carry on at full pace while you work, which can slow the PC rather than help. |
| `privacy.ink-collection` | Ink collection | The policy copy of the typing and inking tweak. |
| `privacy.ink-workspace` | Windows Ink Workspace | Only matters to pen users, and the Ink Workspace is gone from Windows 11. |
| `privacy.input-personalization` | Online speech policy | The policy copy of the online speech tweak. |
| `privacy.input-preload` | Input app preload | An internal value with no known behaviour. |
| `privacy.kms-ticket` | KMS online validation | Only applies to PCs activated by an organisation's KMS server. |
| `privacy.limit-enhanced` | Enhanced diagnostic limit | Tied to Windows Analytics, a retired service for organisations, and a value of 0 doesn't turn the limit on. |
| `privacy.location-policy` | Location policy | Same outcome as the Windows location switch, but it also locks the setting. |
| `privacy.lock-creative` | Lock screen creative | Clears an internal Spotlight ID with no lasting result. |
| `privacy.map-traffic` | Maps traffic | The Windows Maps app has been retired, and its downloads only mattered if you had offline maps. |
| `privacy.mdm-enrol` | Automatic MDM enrolment | Only concerns enrolling a PC in an organisation's device management. |
| `privacy.media-player-codecs` | Media Player codec download | A legacy Windows Media Player option with no practical result today. |
| `privacy.media-player-usage` | Media Player usage tracking | A legacy Windows Media Player setting most people never touch. |
| `privacy.meet-now` | Meet Now | Windows 10 only, and the Skype service behind Meet Now closed in May 2025, so the button leads nowhere. |
| `privacy.messenger-ceip` | Messenger CEIP | Windows Messenger was discontinued years ago. |
| `privacy.mic-policy` | Microphone for apps | Covers Store apps only, while the Windows microphone switch covers every app. |
| `privacy.ms-account` | Microsoft accounts | Breaks Store, Xbox and OneDrive sign-in, and using a local account already avoids it. |
| `privacy.online-troubleshooters` | Online troubleshooters | Microsoft is retiring these legacy troubleshooters, and there is no useful result. |
| `privacy.paint-cocreator` | Paint Cocreator | Cocreator is the earlier name for Paint's picture generator, so this repeats the Image Creator tweak. |
| `privacy.pca` | Program Compatibility Assistant | Same outcome as turning off the compatibility assistant service, which is the guarded tweak. |
| `privacy.perftrack` | PerfTrack | A local responsiveness tracker that sends nothing, with no practical result. |
| `privacy.perm-activity` | Motion and activity | Needs motion sensors that home PCs don't have. |
| `privacy.perm-bg-spatial` | Background spatial | Only for mixed reality headsets. |
| `privacy.perm-bluetooth` | Bluetooth access | Has no Settings page and limits nothing sensitive, so the result is unclear. |
| `privacy.perm-bluetooth-sync` | Bluetooth sync | Has no clear result; it only concerns apps talking to unpaired devices. |
| `privacy.perm-capture-border` | Borderless capture | Extremely niche: it only decides whether an app may hide the capture border. |
| `privacy.perm-chat` | Messaging | Only matters on PCs with a mobile data connection. |
| `privacy.perm-diagnostics` | App diagnostics | Same outcome as the first-class app diagnostics block. |
| `privacy.perm-gaze` | Eye tracking | Only matters with an eye tracker. |
| `privacy.perm-hid` | Human interface devices | Extremely niche, with no Settings page and no clear result. |
| `privacy.perm-music` | Music | The same idea as the other library permissions, for a folder that rarely holds anything sensitive. |
| `privacy.perm-notifications` | Notification access | Same outcome as the first-class notification access block. |
| `privacy.perm-phone` | Phone calls | Only matters on PCs with a mobile data connection. |
| `privacy.perm-phone-history` | Call history | Only matters on PCs with a mobile data connection. |
| `privacy.perm-presence` | Presence sensing | Only on laptops with a presence sensor, and that data stays on the device. |
| `privacy.perm-radios` | Radios | Only stops apps switching radios on and off, with no privacy result. |
| `privacy.perm-serial` | Serial devices | Extremely niche: serial devices are rare on home PCs. |
| `privacy.perm-spatial` | Spatial perception | Only for mixed reality headsets. |
| `privacy.perm-tasks` | Tasks | The Tasks store is barely used now that Microsoft has retired its Mail and Calendar apps. |
| `privacy.perm-wifi` | Wi-Fi data | An old permission with no Settings page and no known current use. |
| `privacy.privacy-policy-input` | Inking privacy flag | Part of the same Windows setting as the typing and inking tweak. |
| `privacy.push-policy` | Cloud notifications policy | Written outside the documented policy location, and it would block notifications people want. |
| `privacy.remote-queries` | Remote search queries | Only concerns other PCs searching shared folders, which almost no home user does. |
| `privacy.safe-search` | SafeSearch | Turns off an adult content filter, which is neither a privacy nor a performance result. |
| `privacy.scheduled-diagnostics` | Scheduled diagnostics | Local maintenance troubleshooting with no privacy result. |
| `privacy.search-background` | Search background toggle | An internal toggle with no known behaviour on current builds. |
| `privacy.search-box-suggestions` | Search box suggestions | Written in the wrong place: this policy belongs under Explorer, not Windows Search, so it isn't expected to work. |
| `privacy.search-disable` | Search feature | Breaks Start and taskbar search with no benefit. |
| `privacy.search-privacy` | Search privacy level | A vague older setting with nothing a person can see; turning off web results is the clear version. |
| `privacy.sensor-permission` | Location sensor override | The older machine switch for location, covered by the Windows location switch. |
| `privacy.sensors-policy` | Sensors policy | Turns off auto-rotate and adaptive brightness with no privacy gain. |
| `privacy.snipping-tool` | Snipping Tool policy | Removes a useful tool with no privacy or performance result. |
| `privacy.soft-landing` | Tips and tricks | Microsoft limits this policy to Enterprise and Education; the tips tweak does the job on home PCs. |
| `privacy.speech-model` | Speech model updates | Stops small speech downloads; nothing personal is sent, so there is no real benefit. |
| `privacy.spellcheck-touch` | Touch keyboard spelling | A touch keyboard preference already in Settings, with no privacy or performance result. |
| `privacy.spotlight` | Windows Spotlight | A user-level policy limited to Enterprise and Education, so this machine-wide value has no effect on a home PC. |
| `privacy.spotlight-action` | Spotlight on Action Center | A user-level policy limited to Enterprise and Education, so this machine-wide value has no effect on a home PC. |
| `privacy.spotlight-lock` | Spotlight on lock screen | A user-level policy limited to Enterprise and Education, so this machine-wide value has no effect on a home PC. |
| `privacy.spotlight-settings` | Spotlight in Settings | A user-level policy limited to Enterprise and Education, so this machine-wide value has no effect on a home PC. |
| `privacy.steps-recorder` | Steps Recorder | Steps Recorder only records when you start it, and Microsoft is removing it from Windows. |
| `privacy.steps-recorder-log` | Steps Recorder log | An event log for a manual tool, with no practical result. |
| `privacy.sync-accessibility` | Sync accessibility | A per-group copy of the settings sync switch. |
| `privacy.sync-apps` | Sync apps | An old per-group sync switch with no current use. |
| `privacy.sync-browser` | Sync browser | Synced settings for legacy Edge and Internet Explorer, both gone. |
| `privacy.sync-language` | Sync language | A per-group copy of the settings sync switch. |
| `privacy.sync-packages` | Sync package state | An old per-group sync switch with no current use. |
| `privacy.sync-personal` | Sync personalisation | A per-group copy of the settings sync switch. |
| `privacy.sync-start` | Sync Start layout | A per-group copy of the settings sync switch. |
| `privacy.sync-theme` | Sync theme | A per-group copy of the settings sync switch. |
| `privacy.sync-windows` | Sync Windows settings | A per-group copy of the settings sync switch. |
| `privacy.telemetry-machine` | Telemetry level | A second location for the same diagnostic level setting. |
| `privacy.text-prediction` | Hardware keyboard prediction | A typing preference that's off by default, with no privacy or performance result. |
| `privacy.toast-diagtrack` | Diagnostic prompt | Marks a prompt as seen, with no lasting effect. |
| `privacy.touch-auto` | Touch keyboard auto-show | A touch keyboard preference already in Settings, with no privacy or performance result. |
| `privacy.touch-keyboard` | Touch keyboard policy | Turns off a Windows input feature with no privacy or performance gain. |
| `privacy.typing-insights` | Typing insights | Local statistics that stay on your PC and only cover the touch keyboard. |
| `privacy.update-compliance` | Update Compliance data | Update Compliance was a service for organisations, never used on a home PC. |
| `privacy.video-ai` | Video AI enhancement | An unverified community value with no clear result. |
| `privacy.voice-lock` | Voice on the lock screen | Covered by turning voice activation off altogether. |
| `privacy.voice-typing` | Voice typing | Voice typing only listens after you start it, and online speech is the setting that matters. |
| `privacy.vs-feedback` | Visual Studio feedback | Only hides feedback options; nothing is sent unless you choose to send it. |
| `privacy.vs-intellicode` | IntelliCode remote analysis | Extremely niche: only for developers using one optional Visual Studio feature. |
| `privacy.vs-telemetry` | Visual Studio telemetry | A community value for the same opt-out as the Visual Studio improvement program tweak. |
| `privacy.vscode-telemetry` | VS Code telemetry flags | VS Code doesn't read these registry values; its telemetry setting lives in its own settings file. |
| `privacy.web-search-metered` | Web search on metered | Pointless once web results are off. |
| `privacy.wmdrm` | WMDRM online | A legacy Windows Media DRM option that only touches old protected files. |
| `privacy.wufb-cloud` | WUfB cloud processing | Windows Update for Business reports are for organisations, never a home PC. |

### services (207)

| Tweak | Name | Why |
|---|---|---|
| `services.aarsvc` | Agent Activation Runtime | Per-user service that only starts for voice assistant wake-word activation, so it is idle on nearly every PC and disabling it saves nothing. |
| `services.ajrouter` | AllJoyn Router | AllJoyn is an abandoned smart-device protocol, and this trigger-start service sits idle, so disabling it achieves nothing. |
| `services.alg` | Application Layer Gateway | Manual service used only by Internet Connection Sharing plug-ins; it is idle otherwise, so disabling it saves nothing. |
| `services.amd-crash` | AMD Crash Defender | A small vendor process with no measured benefit from stopping it, and driver crashes become harder to diagnose without it. |
| `services.amd-external` | AMD External Events | Disabling it can break FreeSync and Radeon hotkeys with no measured benefit, which is a bad trade on a gaming PC. |
| `services.amd-ppm` | AMD Processor Driver | This sets the driver to Manual, which is already its normal setting, so the tweak changes nothing. |
| `services.amd-ryzen-master` | Ryzen Master driver | Tied to one driver version (V19) and already loaded on demand by Ryzen Master, so it has no practical effect. |
| `services.app-mgmt` | Application Management | Enterprise-only: it installs software deployed through domain Group Policy and is idle on home PCs. |
| `services.app-readiness` | App Readiness | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.appv` | App-V Client | App-V is an enterprise feature and its client is already disabled by default, so the tweak changes nothing. |
| `services.assigned-access` | Assigned Access | Kiosk-mode service that stays idle unless kiosk mode is set up, which a home PC never does. |
| `services.autotime` | Cellular Time | Only sets the clock from mobile networks on PCs with a cellular modem and is idle everywhere else. |
| `services.avctp` | AVCTP | Idle without Bluetooth audio and breaks audio controls with it; services.bluetooth-off covers anyone who wants Bluetooth gone. |
| `services.axinst` | ActiveX Installer | Installs ActiveX controls, a technology that went with Internet Explorer; it is manual and idle. |
| `services.bcast-dvr` | Game Recording User Service | Per-user service that only runs while recording; gaming.game-dvr-off is the proper off switch for game recording. |
| `services.beep` | Beep Driver | A loaded but idle driver with no meaningful cost; windows.beep is the simpler way to silence error beeps. |
| `services.bindflt` | Bind Filter | File-redirection driver used by containers, Windows Sandbox and some Store games; disabling it risks breaking them and saves nothing. |
| `services.biometric-mcp` | MCP Management | Manual service with no documented function, so nobody can say what disabling it does. |
| `services.bluetooth-off` | Disable Bluetooth Support | Trigger-start and idle unless a Bluetooth device is used, so disabling it saves nothing, and it breaks Bluetooth for anyone who pairs a device later. |
| `services.bluetooth-user` | Bluetooth User Support | Per-user part of Bluetooth that does nothing unless Bluetooth is in use; services.bluetooth-off covers turning it off. |
| `services.branchcache` | BranchCache | Enterprise branch-office caching; it is manual, idle and unused on home networks. |
| `services.btag` | Bluetooth Audio Gateway | Idle without a Bluetooth headset and breaks headset calls with one; services.bluetooth-off covers turning Bluetooth off. |
| `services.capture` | Screen Capture Service | Only runs during screen capture, so disabling it saves nothing and breaks recording and screen-sharing apps. |
| `services.cbdhsvc` | Clipboard User Service | privacy.clipboard-history turns off clipboard history properly; disabling the service as well adds risk to copy and screen clips for no extra benefit. |
| `services.cdp-user` | Connected Devices User | Per-user half of Connected Devices; services.cdp already stops the same features. |
| `services.certprop` | Certificate Propagation | Only copies certificates from smart cards for enterprise sign-in; it is manual and idle at home. |
| `services.clipsvc` | Client License Service | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.cloud-id` | Cloud Identity | Enterprise cloud identity service that is idle on home PCs. |
| `services.comsysapp` | COM+ System Application | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.consent-ux` | ConsentUX | Per-user service that only shows device connection prompts; it is idle until needed, so disabling it saves nothing. |
| `services.cred-enroll` | Credential Enrollment | Per-user service used only for credential enrolment prompts; it is idle, and disabling it risks breaking sign-in set-up. |
| `services.csc` | Offline Files | Offline Files is a business feature that does nothing unless set up, so the service stays idle. |
| `services.defragsvc` | Optimize Drives | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.device-assoc` | Device Association | Only runs while pairing, so there is no saving, and disabling it stops new wireless devices pairing. |
| `services.device-assoc-broker` | Device Association Broker | Per-user broker that only runs when an app pairs a device; it is idle otherwise. |
| `services.device-picker` | Device Picker | Per-user service that only runs while the device picker is on screen; it is idle otherwise. |
| `services.devices-flow` | Devices Flow | Per-user service that only runs when connecting a wireless display or audio device; disabling it can break Cast and saves nothing. |
| `services.devquery` | DevQuery Background Discovery | Trigger-start helper that only runs when an app looks for devices; it is idle otherwise. |
| `services.diagnostics-hub` | Diagnostics Hub Collector | Only used by Visual Studio's diagnostic tools; it is manual and idle on every other PC. |
| `services.diagsvc` | Diagnostic Execution | Only runs when a troubleshooter is used, so disabling it saves nothing and breaks troubleshooters. |
| `services.dialog-blocking` | Dialog Blocking | Kiosk feature that is already disabled by default, so the tweak changes nothing. |
| `services.disp-broker` | Display Policy | Windows starts this display service when it needs it, so moving it to Manual saves nothing and risks display layout changes failing. |
| `services.display-enhance` | Display Enhancement | Trigger-start helper for adaptive brightness that is idle on desktops; power.adaptive-brightness is the proper control. |
| `services.dmenroll` | Device Management Enrollment | Enterprise device management enrolment that is idle on home PCs. |
| `services.dot3svc` | Wired AutoConfig | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.dsmsvc` | Device Setup Manager | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.dssvc` | Data Sharing | Trigger-start broker that only runs when apps share data; it is idle otherwise, and disabling it can break sharing between apps. |
| `services.dusmsvc` | Data Usage | It runs all the time but costs very little and keeps its counts on this PC, so disabling it only loses the Data usage page and data limits. |
| `services.eaphost` | Extensible Authentication | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.edge-elevation` | Edge Elevation | Manual is already the default for this service, so the tweak changes nothing. |
| `services.edge-update` | Edge Update | Holding back a browser updater weakens a security protection, and its scheduled tasks still start it, so nothing is saved. |
| `services.edge-updatem` | Edge Update Manual | Manual is already the default for this service, so the tweak changes nothing. |
| `services.embedded-mode` | Embedded Mode | Background-app support for Windows IoT devices that is idle on normal PCs. |
| `services.ent-app` | Enterprise App Management | Installs work apps on managed enterprise PCs and is idle on home PCs. |
| `services.event-system` | COM+ Event System | Sign-in and many apps rely on this core service, so moving it to Manual saves nothing and carries a real risk. |
| `services.fax` | Fax | Manual service that never runs unless you send a fax from Windows, so disabling it saves nothing. |
| `services.fdphost` | Function Discovery Host | Only active with network discovery, and setting the network to Public does the job without disabling a service. |
| `services.fdrespub` | Function Discovery Publisher | Windows already stops announcing the PC when the network is set to Public, so disabling the service adds nothing. |
| `services.fhsvc` | File History | Only runs if File History has been turned on; it is idle otherwise, and disabling it risks backups stopping unnoticed. |
| `services.file-sync-helper` | File Sync Helper | A vague helper for sync apps with no job a person can name and no visible result from disabling it. |
| `services.fontcache` | Font Cache | Windows starts it as soon as fonts are needed, so moving it to Manual saves nothing and can slow app starts. |
| `services.fontcache3` | WPF Font Cache | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.frameserver` | Camera Frame Server | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.frameserver-monitor` | Camera Frame Server Monitor | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.game-input` | GameInput | Idle until a game needs it, so disabling it saves nothing and can cost controller input in games. |
| `services.gpu-energy` | GPU Energy Driver | Driver that estimates GPU power use; there is no credible performance basis for disabling it and no known visible result. |
| `services.graphicsperf` | Graphics Performance Monitor | Manual service that is idle unless something asks for graphics performance data, with no known result from disabling it. |
| `services.hidserv` | Human Interface Devices | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.hvhost` | Hyper-V Host | Only runs when Hyper-V features are in use, so it is idle otherwise, and disabling it can slow WSL 2 and Windows Sandbox. |
| `services.icssvc` | Mobile Hotspot | Only runs when Mobile Hotspot is on, so disabling it saves nothing. |
| `services.ikeext` | IKE and AuthIP Keying | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.install-service` | Store Install Service | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.iphlpsvc` | IP Helper | It runs all the time but costs very little, and disabling it removes IPv6 tunnels that Xbox multiplayer and some VPNs use, with no measured benefit. |
| `services.ipxlat` | IP Translation Config | Trigger-start helper for IPv6 translation on mobile networks that is idle on home PCs. |
| `services.keyboard-filter` | Keyboard Filter | Kiosk feature that is already disabled by default, so the tweak changes nothing. |
| `services.ktmrm` | Transaction Manager Bridge | Manual bridge used with enterprise database transactions that is idle on home PCs. |
| `services.lanman-workstation` | SMB Client | Disabling the SMB client breaks network drives and anything built on them, and its cost is negligible, so the trade is poor. |
| `services.license-manager` | Windows License Manager | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.lltdsvc` | Link-Layer Topology Discovery | Only draws the old network map; it is manual and idle. |
| `services.lmhosts` | TCP/IP NetBIOS Helper | Trigger-start helper for old NetBIOS lookups that is idle on modern home networks, and disabling it can hide older shares. |
| `services.lpasvc-note` | Policy Platform Agent | Enterprise policy component that a normal home or gaming PC does not have. |
| `services.lxp` | Language Experience | Trigger-start service that only runs when a language pack is installed, and disabling it can break adding a language. |
| `services.maps-broker-off` | Disable Downloaded Maps Manager | Stops itself on most PCs, so disabling it saves nothing. |
| `services.messaging` | Messaging | Per-user text messaging support that is idle on nearly all PCs. |
| `services.msdtc` | Distributed Transactions | Manual coordinator for database transactions that is idle on home PCs. |
| `services.msiscsi` | iSCSI Initiator | Manual service for iSCSI storage, which home PCs don't use, so it stays idle. |
| `services.natural-auth` | Natural Authentication | Manual service that only runs for Dynamic Lock and similar presence signals; it is idle otherwise. |
| `services.ncasvc` | Network Connectivity Assistant | Reports the status of DirectAccess, an enterprise VPN feature, and is idle on home PCs. |
| `services.ncb` | Network Connection Broker | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.ncdautosetup` | Network Devices Auto-Setup | Trigger-start service tied to one option in the Windows network sharing settings, which is the proper control. |
| `services.ndiscap` | NDIS Capture | Idle driver only used during packet capture, so disabling it saves nothing and breaks network diagnostics. |
| `services.net-tcp-sharing` | Net.Tcp Port Sharing | Already disabled by default, so the tweak changes nothing. |
| `services.netlogon` | Netlogon | Only used on domain-joined PCs; it is manual and idle at home. |
| `services.netsetup` | Network Setup | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.ngc` | Microsoft Passport | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.ngc-ctnr` | Passport Container | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.npsm` | Now Playing | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.nv-container` | NVIDIA Display Container | High risk with no measured benefit, and mislabelled: the display container that runs NVIDIA Control Panel is a different service. |
| `services.nv-profile-updater` | NVIDIA Profile Updater | Could not confirm a service by this name on current NVIDIA drivers, and blocking game profile updates has no benefit anyway. |
| `services.nv-telemetry` | NVIDIA Telemetry | The name dates from old drivers, and NVIDIA has since dropped its separate telemetry service, so this has nothing to act on. |
| `services.onedrive-updater` | OneDrive Updater | Blocks OneDrive security updates for no saving, since it only runs briefly when checking for them. |
| `services.openxr` | Mixed Reality OpenXR | Windows Mixed Reality is deprecated and this service is manual and idle. |
| `services.p2pimsvc` | Peer Identity Manager | Legacy peer networking service that nothing on a home PC uses; it is manual and idle. |
| `services.p2psvc` | Peer Grouping | Legacy peer networking service that nothing on a home PC uses; it is manual and idle. |
| `services.perception-sim` | Perception Simulation | Mixed reality developer tool that is manual and idle. |
| `services.perfhost` | Performance Counter Host | Manual helper for 32-bit monitoring tools that is idle otherwise. |
| `services.phonesvc` | Phone Service | Manual telephony service that is idle unless a phone app needs it. |
| `services.pim-index` | Contact Indexing | Per-user contact indexer that only serves the older Mail and People apps; services.sync-host-off covers that area. |
| `services.pla` | Performance Logs and Alerts | Manual service that only runs for Performance Monitor collectors, so disabling it saves nothing. |
| `services.pnrpauto` | PNRP Name Publication | Legacy peer name feature that is manual and idle. |
| `services.pnrpsvc` | Peer Name Resolution | Legacy peer name feature that is manual and idle. |
| `services.policy-agent` | IPsec Policy Agent | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.print-notify-off` | Disable printer extensions | Trigger-start and idle without a printer, so disabling it saves nothing. The print spooler card is the real switch. |
| `services.print-workflow` | Print Workflow | Per-user service that only runs for print support apps; services.print-spooler-off covers PCs that never print. |
| `services.push-to-install` | PushToInstall | Manual service that only runs when an app is pushed from another device; it is idle otherwise. |
| `services.qwave` | Quality Windows Audio Video | Manual service for a home-network media feature that almost nothing uses, with no measured benefit from disabling it. |
| `services.rasauto` | Remote Access Auto Connection | Manual dial-up era helper that is idle. |
| `services.rasman` | Remote Access Connection Manager | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.remote-access` | Routing and Remote Access | Already disabled by default on Windows 10 and 11, so the tweak changes nothing. |
| `services.retail-demo` | Retail Demo | Only used on shop display PCs and idle everywhere else. |
| `services.rpclocator` | RPC Locator | Legacy RPC name service that is manual and idle. |
| `services.rtk-audio` | Realtek Audio | A small vendor process with no measured benefit from stopping it, and Realtek audio settings can stop working. |
| `services.rtk-universal` | Realtek Audio Universal | A small vendor process with no measured benefit from stopping it, and Realtek audio settings can stop working. |
| `services.scardsvr` | Smart Card | Trigger-start service that is idle without a smart card reader, which home PCs don't have. |
| `services.scdevice` | Smart Card Enumeration | Trigger-start service that is idle without a smart card reader, which home PCs don't have. |
| `services.scpolicy` | Smart Card Removal Policy | Manual enterprise smart card service that is idle on home PCs. |
| `services.sdrsvc` | Windows Backup | Manual service that only runs for Windows Backup; it is idle otherwise, and disabling it risks backups failing unnoticed. |
| `services.search-in-bing` | Microsoft Search in Bing | Not a standard Windows service: it comes from certain Microsoft 365 deployments, so home PCs don't have it. |
| `services.seclogon` | Secondary Logon | Manual service that only runs for Run as different user; it is idle otherwise. |
| `services.semgr` | Payments and NFC | Payments and NFC manager that is idle on PCs without NFC, which is nearly all of them. |
| `services.sens` | System Event Notification | Apps use this core service for sign-in and network events, so moving it to Manual saves nothing and carries a risk. |
| `services.sensor-data` | Sensor Data | Trigger-start service that never runs without sensors; services.sensor-off covers sensor features. |
| `services.sensor-monitoring-off` | Disable Sensor Monitoring | Trigger-start and idle on a PC without sensors, so disabling it saves nothing. |
| `services.sensor-off` | Disable Sensor Service | Trigger-start and idle on a PC without sensors, so disabling it saves nothing. |
| `services.session-env` | Remote Desktop Configuration | Only supports incoming Remote Desktop sessions, which windows.rdp-off already blocks; it is manual and idle. |
| `services.shared-pc` | Shared PC Accounts | Shared PC mode is for schools and businesses, and the service is already disabled by default. |
| `services.shared-reality` | Spatial Data | Mixed reality service that is manual and idle. |
| `services.sharedaccess-note` | SNMP Trap | Receives SNMP messages for network administrators; it is manual and idle on home PCs. |
| `services.shell-hw` | Shell Hardware Detection | windows.autoplay-off turns off AutoPlay properly, and disabling the service adds risk for no extra benefit. |
| `services.smphost` | Storage Spaces SMP | Manual service that only runs for Storage Spaces management; it is idle otherwise. |
| `services.smsrouter` | SMS Router | Routes text messages on cellular PCs and is idle everywhere else. |
| `services.spectrum` | Windows Perception | Windows Mixed Reality is deprecated and this service is manual and idle. |
| `services.ssdp` | SSDP Discovery | Only active with network discovery, so disabling it saves nothing and stops media devices being found. |
| `services.ssh-agent` | OpenSSH Agent | Already disabled by default, and a developer who turned it on did so on purpose. |
| `services.sstp` | Secure Socket Tunneling | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.stisvc` | Windows Image Acquisition | Only does anything when a scanner or camera is in use, so disabling it stops scanners for no saving. |
| `services.storqos` | Storage QoS Filter | Storage filter for virtual machines with no known result on a home PC. |
| `services.storsvc` | Storage Service | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.svsvc` | Spot Verifier | Manual service that only runs when NTFS suspects an error, so disabling it removes a disk safeguard for no saving. |
| `services.swprv` | Shadow Copy Provider | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.sysmon` | System Monitor Driver | Sysmon is a security logging tool someone must install on purpose, so disabling it can only weaken that protection. |
| `services.tablet-input` | Touch Keyboard and Handwriting | Trigger-start service, so disabling it saves nothing and breaks the touch keyboard and emoji panel. |
| `services.tapisrv` | Telephony | Manual telephony API service that is idle on home PCs. |
| `services.termservice` | Remote Desktop Services | Only accepts incoming Remote Desktop, which windows.rdp-off blocks properly; it is manual and idle otherwise, and Home editions can't host sessions at all. |
| `services.text-input` | Text Input Management | Typing in Start and modern apps depends on this service, so moving it to Manual saves nothing and carries a real risk. |
| `services.themes` | Themes | This is breakage, not a customisation: apps lose their themed look with no supported result and no real saving, so it does not belong in Visuals either. |
| `services.tiering` | Storage Tiers | Manual service for Storage Spaces tiers, which home PCs don't use. |
| `services.troubleshooting` | Recommended Troubleshooting | Trigger-start service with its own control in Settings for recommended troubleshooting; disabling it saves nothing. |
| `services.tzautoupdate` | Auto Time Zone Updater | Already disabled by default until automatic time zone is turned on, and turning that off in Settings is the proper control. |
| `services.udk` | Shell Experience Host User | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.uev` | User Experience Virtualization | Enterprise feature that is already disabled by default, so the tweak changes nothing. |
| `services.umrdp` | Remote Desktop Port Redirector | Only redirects devices into incoming Remote Desktop sessions, which windows.rdp-off blocks; it is manual and idle. |
| `services.unistore` | User Data Storage | Per-user storage for the older Mail and Calendar apps; services.sync-host-off covers that area. |
| `services.upnphost` | UPnP Device Host | Hosts UPnP devices on this PC, which almost nothing does, so it stays manual and idle. |
| `services.userdata` | User Data Access | Per-user data access for the older Mail and Calendar apps; services.sync-host-off covers that area. |
| `services.vacsvc` | Volumetric Audio | Mixed reality spatial audio service that is manual and idle. |
| `services.vds` | Virtual Disk | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.vmic-guest` | Hyper-V Guest Interface | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-heartbeat` | Hyper-V Heartbeat | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-kvp` | Hyper-V Data Exchange | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-rdv` | Hyper-V Remote Desktop | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-session` | Hyper-V PowerShell Direct | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-shutdown` | Hyper-V Guest Shutdown | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-time` | Hyper-V Time Sync | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vmic-vss` | Hyper-V Shadow Copy | Only runs inside a Hyper-V virtual machine, never on a real PC. |
| `services.vss` | Volume Shadow Copy | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.w32time` | Windows Time | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.wallet` | Wallet | Manual service for a payments feature home PCs don't use; it is idle. |
| `services.wap-push-off` | Disable WAP push routing | Trigger-start with no measurable cost on a home PC, so disabling it saves nothing. |
| `services.warp-jit` | WarpJIT | Only runs for software rendering, so it is idle on a PC with a working GPU. |
| `services.waves` | Waves Audio | Vendor audio tuning on some laptops with no measured benefit from stopping it, and the speakers lose their tuning. |
| `services.wbengine` | Block Level Backup | Manual engine for the old Windows backup tool that is idle otherwise. |
| `services.wbio` | Windows Biometric | Only runs for fingerprint and face sign-in, so disabling it saves nothing and breaks Windows Hello. |
| `services.wcncsvc` | Windows Connect Now | Manual WPS helper that is idle on nearly all PCs. |
| `services.wecsvc` | Windows Event Collector | Manual enterprise service for collecting events from other PCs; it is idle at home. |
| `services.wepsvc` | Encryption Provider Host | Trigger-start host for third-party encryption that is idle otherwise. |
| `services.wercpl` | Problem Reports Control Panel | Manual service that only runs when the Problem Reports page is open. |
| `services.wersvc` | Windows Error Reporting | privacy.error-report-policy is the documented off switch, and this trigger-start service is idle between crashes. |
| `services.wfds` | Wi-Fi Direct Connection Manager | Trigger-start service that is idle until a Wi-Fi Direct device connects, and disabling it can break wireless displays. |
| `services.wiarpc` | Still Image Events | Manual service that only reacts to scanner buttons; it is idle otherwise. |
| `services.winrm` | Windows Remote Management | Manual, and it listens for nothing until someone sets it up on purpose, so disabling it achieves nothing on a home PC. |
| `services.wisvc` | Windows Insider | Manual service that is idle unless the PC is in the Insider Program. |
| `services.wlidsvc` | Microsoft Account Sign-in | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.wlpasvc` | Local Profile Assistant | Manages eSIM profiles and is idle on PCs without one. |
| `services.wman` | Windows Management Service | Enterprise device management service that is idle on home PCs. |
| `services.wmiapsrv` | WMI Performance Adapter | Manual WMI helper that is idle unless a monitoring tool calls it. |
| `services.wmp-network` | Media Player Sharing | Legacy Windows Media Player sharing that is manual and idle. |
| `services.workfolders` | Work Folders | Business file sync that is manual and idle on home PCs. |
| `services.wpc-mon` | Parental Controls | Idle without family rules, and on a family-managed PC disabling it only undermines a parent's controls. |
| `services.wpd-bus` | Portable Devices | Manual is already the Windows default for this service, so the tweak changes nothing. |
| `services.wpn` | Push Notifications | Notifications depend on this service, so moving it to Manual saves nothing and risks losing them. |
| `services.wpn-user` | Push Notifications User | Notifications depend on this service, so moving it to Manual saves nothing and risks losing them. |
| `services.wwansvc` | WWAN AutoConfig | Trigger-start service that is idle without a mobile broadband modem. |
| `services.xbox-auth-off` | Disable Xbox Live Auth Manager | Manual and idle, so it saves nothing, and it breaks Xbox sign-in for Game Pass players. |
| `services.xbox-gip` | Xbox Accessory Management | Trigger-start service, so disabling it saves nothing and can stop Xbox controllers working. |
| `services.xbox-net` | Xbox Live Networking | Manual service, so disabling it saves nothing and can break Xbox multiplayer. |
| `services.xbox-save-off` | Disable Xbox Live Game Save | Manual and idle, so it saves nothing, and it breaks cloud saves for Game Pass players. |

### storage (17)

| Tweak | Name | Why |
|---|---|---|
| `storage.ahci-dipm` | AHCI device power saving | An undocumented driver value for what the documented SATA link power setting already controls. |
| `storage.ahci-hipm` | AHCI host power saving | An undocumented driver value for what the documented SATA link power setting already controls. |
| `storage.autochk-timeout` | Disk check countdown | Saves a few seconds on the rare boot when a disk check is due, and leaves less time to cancel it. |
| `storage.boot-optimize` | Boot file optimisation | Already on by default, and it only ever matters on hard drives. |
| `storage.memoryusage` | NTFS memory usage | Meant for busy file servers; it gives NTFS more memory with no measurable gain on a desktop. |
| `storage.mft-zone` | MFT zone | Reserves disk space for a problem Windows already manages, and means nothing on an SSD. |
| `storage.no-efs` | NTFS encryption | Turns off a security feature to save nothing; EFS costs nothing when it is not used. |
| `storage.no-paging-encryption` | Page file encryption | Already the Windows default, so it does nothing, and it could only ever weaken protection. |
| `storage.nvme-arbitration` | NVMe arbitration burst | An undocumented NVMe driver value with no known behaviour or measured benefit. |
| `storage.nvme-flush` | NVMe flush | An undocumented NVMe driver value; changing how flushes work risks data safety for no measured benefit. |
| `storage.nvme-hmb` | NVMe host memory buffer | An undocumented driver value that interferes with the host memory DRAM-less SSDs rely on, with no benefit. |
| `storage.nvme-immediate` | NVMe immediate data | An undocumented NVMe driver value with no known behaviour or measured benefit. |
| `storage.nvme-logging` | NVMe driver logging | An undocumented driver logging value with no measurable cost or benefit. |
| `storage.nvme-queue` | NVMe queue depth | Home workloads rarely fill the default NVMe queue, so raising an undocumented limit gains nothing. |
| `storage.nvme-queues` | NVMe submission queues | Forces a fixed queue count on a driver that already sizes this itself; undocumented, and it can only hurt. |
| `storage.nvme-requests` | NVMe request limit | An undocumented limit that home workloads never reach. |
| `storage.nvme-timeout` | NVMe I/O timeout | Only changes how long Windows waits on a failing drive, which makes a hang last longer. |

### tasks (139)

| Tweak | Name | Why |
|---|---|---|
| `tasks.adobe-update` | Adobe Acrobat Update | Stops Acrobat security updates, which weakens a protection on a common attack target for no benefit. |
| `tasks.ait-agent` | Application Impact Telemetry | This task no longer ships on current Windows, and privacy.ait is the policy for the same data. |
| `tasks.amd-launcher` | AMD Install Launcher | Only supports AMD's installer at sign-in, and nobody can name a result from blocking it. |
| `tasks.amd-link` | AMD Link Update | Trivial updater for one AMD companion feature with no visible result from blocking it. |
| `tasks.amd-modify-link` | AMD Modify Link Update | Trivial updater for one AMD companion feature with no visible result from blocking it. |
| `tasks.amd-ryzen-master` | Ryzen Master Task | Only exists with Ryzen Master installed, and blocking it can only break the tool it came with. |
| `tasks.appuri` | App Link Verifier | Trivial daily check of app web links with no visible result. |
| `tasks.autochk-proxy` | Autochk Proxy | Only reports disk check data for the improvement programme, which privacy.ceip leaves. |
| `tasks.ceip-bth` | Bluetooth CEIP | Dates from older Windows, and privacy.ceip leaves the improvement programme it served. |
| `tasks.ceip-consolidator` | CEIP Consolidator | Does nothing once you leave the improvement programme, and privacy.ceip is the documented way to do that. |
| `tasks.ceip-kernel` | Kernel CEIP | Dates from older Windows, and privacy.ceip leaves the improvement programme it served. |
| `tasks.ceip-server-assistant` | Server CEIP Assistant | Windows Server task that doesn't exist on Windows 10 or 11. |
| `tasks.ceip-server-role` | Server Role Collector | Windows Server task that doesn't exist on Windows 10 or 11. |
| `tasks.ceip-server-usage` | Server Role Usage Collector | Windows Server task that doesn't exist on Windows 10 or 11. |
| `tasks.ceip-uploader` | CEIP Uploader | Dates from older Windows, and privacy.ceip leaves the improvement programme it served. |
| `tasks.ceip-usb` | USB CEIP | Collects USB data only for the improvement programme, which privacy.ceip leaves. |
| `tasks.cloud-host` | Cloud Experience Host | A one-time account set-up step with nothing to gain from blocking it. |
| `tasks.component-cleanup` | Component Clean-up | Useful idle-time maintenance: disabling it only lets the Windows component store grow, with no benefit. |
| `tasks.cross-device-resume` | Cross Device Resume | Could not confirm this task controls the Resume feature, and reports say its process keeps running regardless, so nobody can state the result. |
| `tasks.cross-device-trigger` | Cross Device Resume Trigger | Same unconfirmed feature as the Cross Device Resume task, with no result anyone can state. |
| `tasks.device-info-user` | Device Information User | Per-user copy of the same census, so one card is enough. |
| `tasks.disk-diag-data` | Disk Diagnostic Collector | Only sends disk data for the improvement programme, which privacy.ceip leaves. |
| `tasks.disk-diag-resolver` | Disk Diagnostic Resolver | Its only job is warning you about a failing disk, so turning it off has a cost and no benefit. |
| `tasks.disk-footprint` | Disk Footprint Diagnostics | Trivial disk use measurement with no visible result. |
| `tasks.dropbox-core` | Dropbox Update Core | Stops updates to an internet-connected app, a security cost for no benefit. |
| `tasks.dropbox-ua` | Dropbox Update Hourly | Stops updates to an internet-connected app, a security cost for no benefit. |
| `tasks.dssvc-cleanup` | Data Sharing Clean-up | Trivial clean-up of temporary shared data with no visible result. |
| `tasks.dusm` | Data Usage Task | Trivial background step for the Data usage page with no visible result. |
| `tasks.edge-core` | Edge Update Core | Stops browser security updates, which weakens a protection and should never be an optimisation. |
| `tasks.edge-ua` | Edge Update Hourly | Stops browser security updates, which weakens a protection and should never be an optimisation. |
| `tasks.error-details` | Error Details Update | Trivial download of error descriptions with no visible result. |
| `tasks.family-monitor` | Family Safety Monitor | Trivial on a PC without family rules, and on one with them disabling it only undermines a parent's controls. |
| `tasks.family-refresh` | Family Safety Refresh | Trivial on a PC without family rules, and on one with them disabling it only undermines a parent's controls. |
| `tasks.family-refresh-legacy` | Family Safety Refresh (legacy) | Old name of a task that current Windows no longer uses. |
| `tasks.family-upload` | Family Safety Upload | Trivial on a PC without family rules, and on one with them disabling it only undermines a parent's controls. |
| `tasks.file-history` | File History Maintenance | Does nothing unless you turned File History on, and then disabling it stops your backups. |
| `tasks.flight-flush` | Feature Usage Flushing | Undocumented internal step with no result a person can state. |
| `tasks.flight-receiver` | Feature Usage Receiver | Undocumented internal step with no result a person can state. |
| `tasks.flight-reconcile` | Reconcile Features | Undocumented, and interfering with how Windows applies feature rollouts has no result a person can state. |
| `tasks.flight-reporting` | Feature Usage Reporting | Undocumented internal step with no result a person can state, and overall diagnostic data has its own controls. |
| `tasks.google-core` | Google Update Core | Stops browser security updates, which weakens a protection and should never be an optimisation. |
| `tasks.google-ua` | Google Update Hourly | Stops browser security updates, which weakens a protection and should never be an optimisation. |
| `tasks.grid-forecast` | Power Grid Forecast | Trivial download of carbon forecasts with no visible result. |
| `tasks.hive-upload` | Profile Hive Upload | Only for roaming profiles on business networks; it never does anything on a home PC. |
| `tasks.indexer-maint` | Indexer Maintenance | Trivial maintenance that keeps the search index healthy; services.wsearch is the real control. |
| `tasks.input-mouse` | Mouse Settings Sync | Trivial settings sync with no visible result. |
| `tasks.input-pen` | Pen Settings Sync | Trivial settings sync with no visible result. |
| `tasks.input-restore` | Input Settings Restore | Trivial settings restore with no visible result. |
| `tasks.input-syncpen` | Pen Sync | Trivial settings sync with no visible result. |
| `tasks.input-touchpad` | Touchpad Settings Sync | Trivial settings sync with no visible result. |
| `tasks.input-user` | Input Local User Sync | Trivial settings sync with no visible result. |
| `tasks.lang-install` | Language Components Install | Trivial language maintenance that can only leave a language feature half-installed. |
| `tasks.lang-reconcile` | Language Resources Reconcile | Trivial language maintenance that can only leave a language feature half-installed. |
| `tasks.lang-sync` | Language Settings Sync | Trivial settings sync with no visible result. |
| `tasks.lang-uninstall` | Language Components Removal | Trivial language maintenance with no visible result. |
| `tasks.location-dialog` | Location Action Dialog | Only shows location prompts; services.geolocation-off is the real control. |
| `tasks.location-notify` | Location Notifications | Only shows location prompts; services.geolocation-off is the real control. |
| `tasks.lp-remove` | Language Pack Clean-up | Trivial language pack clean-up with no visible result. |
| `tasks.maps-toast` | Maps Toast | Only shows offline map notices; services.maps-broker-off covers offline maps. |
| `tasks.maps-update` | Maps Update | Only updates offline maps; services.maps-broker-off covers offline maps. |
| `tasks.mare-backup` | MareBackup | Trivial backup of compatibility data with no visible result. |
| `tasks.mdm-maintenance` | MDM Maintenance | Enterprise device management task that does nothing on an unmanaged PC. |
| `tasks.memory-events` | Memory Diagnostic Events | Only reports memory test results, so it is idle unless you run the test. |
| `tasks.memory-full` | Full Memory Diagnostic | Only supports the Windows memory test, so it is idle unless a test is needed. |
| `tasks.ncsi-proxy` | NCSI Proxy Detection | Trivial step in the connection check, and interfering with it risks a false no-internet state. |
| `tasks.net-trace` | Gather Network Info | Trivial network diagnostic with no visible result. |
| `tasks.ngen` | .NET NGEN | Useful idle-time work: disabling it makes .NET apps start slower for no benefit. |
| `tasks.ngen-64` | .NET NGEN 64-bit | Useful idle-time work: disabling it makes .NET apps start slower for no benefit. |
| `tasks.ngen-64-crit` | .NET NGEN 64-bit Critical | Useful idle-time work: disabling it makes .NET apps start slower for no benefit. |
| `tasks.ngen-crit` | .NET NGEN Critical | Useful idle-time work: disabling it makes .NET apps start slower for no benefit. |
| `tasks.nla-wifi` | NLA Wi-Fi Task | Internal Wi-Fi status step with no result a person can state, and a risk to network detection. |
| `tasks.nv-mon` | NVIDIA Telemetry Monitor | This task name dates from old NVIDIA drivers and could not be confirmed on current ones, so it likely has nothing to act on. |
| `tasks.nv-rep` | NVIDIA Telemetry Report | This task name dates from old NVIDIA drivers and could not be confirmed on current ones, so it likely has nothing to act on. |
| `tasks.nv-rep-logon` | NVIDIA Telemetry at Sign-in | This task name dates from old NVIDIA drivers and could not be confirmed on current ones, so it likely has nothing to act on. |
| `tasks.office-fallback` | Office Telemetry Fallback | Old Office telemetry agent task; apps.office-telemetry is the proper control. |
| `tasks.office-fallback16` | Office 2016 Telemetry Fallback | Old Office telemetry agent task; apps.office-telemetry is the proper control. |
| `tasks.office-heartbeat` | Office Subscription Heartbeat | Office 2013 licence check that applies to almost nobody now, and blocking it can only cause licence trouble. |
| `tasks.office-logon` | Office Telemetry at Sign-in | Old Office telemetry agent task; apps.office-telemetry is the proper control. |
| `tasks.office-logon16` | Office 2016 Telemetry at Sign-in | Old Office telemetry agent task; apps.office-telemetry is the proper control. |
| `tasks.onedrive-machine` | OneDrive Machine Update | Blocks OneDrive updates, a security cost for no benefit. |
| `tasks.onesettings` | OneSettings Refresh | privacy.onesettings is the documented control for remote configuration downloads. |
| `tasks.pca-patch` | PCA Patch Database | Trivial compatibility database update that can only help old programs run. |
| `tasks.power-efficiency` | Power Efficiency Diagnostics | Trivial local power report with no visible result. |
| `tasks.print-edu` | Education Printer Provisioning | Only provisions school printers, so it does nothing on a home PC. |
| `tasks.print-job-cleanup` | Print Job Clean-up | Trivial print maintenance with no visible result. |
| `tasks.printer-cleanup` | Printer Clean-up | Trivial print maintenance with no visible result. |
| `tasks.provisioning-cellular` | Cellular Provisioning | Only provisions mobile networks, so it does nothing on PCs without a modem. |
| `tasks.provisioning-logon` | Logon Provisioning | Applies provisioning packages, mainly an enterprise and PC-maker tool, with no visible result on a home PC. |
| `tasks.push-register` | PushToInstall Registration | Trivial registration task for pushed app installs with no visible result. |
| `tasks.queue-reporting` | Error Report Queue | Only sends queued crash reports; privacy.error-report-policy is the documented off switch. |
| `tasks.ras-mobility` | VPN Mobility Manager | Internal VPN helper that does nothing without a Windows VPN and can only hurt one. |
| `tasks.reg-idle-backup` | Registry Idle Backup | Windows has not kept these registry backups by default since version 1803, so the task does nothing useful to stop. |
| `tasks.remote-assist` | Remote Assistance | Only checks Remote Assistance policy; windows.remote-assistance is the real control. |
| `tasks.request-trace` | Performance Trace Request | Only records traces on request, so it is idle and has no visible result. |
| `tasks.res-pri` | Sysmain Resource Sync | Internal SysMain sync with no documented job and no result a person can state. |
| `tasks.resolution-host` | Diagnostic Resolution Host | Only runs when diagnostics find a problem, so it is idle and has no visible result. |
| `tasks.retail-cleanup` | Retail Demo Clean-up | Only clears shop demo content, so it does nothing on a normal PC. |
| `tasks.scan-updates` | Store Update Scan | apps.store-auto-update is the documented control, and blocking this task also holds back Store app security fixes. |
| `tasks.scan-updates-user` | Store Update Scan as User | apps.store-auto-update is the documented control, and blocking this task also holds back Store app security fixes. |
| `tasks.scheduled-defrag` | Scheduled Defrag | Harmful: it stops scheduled SSD TRIM and hard drive defragmentation, runs only when the PC is idle, and has no benefit to disabling. |
| `tasks.scheduled-diagnosis` | Scheduled Diagnosis | privacy.scheduled-diagnostics is the documented control for scheduled troubleshooting. |
| `tasks.sdbinst-merge` | Compatibility Database Merge | Trivial compatibility database merge with no visible result. |
| `tasks.settings-sync` | Settings Sync Network Task | Trivial settings sync step with no visible result, and Windows Backup settings control what syncs. |
| `tasks.setup-cleanup` | Setup Clean-up | Clears leftovers after upgrades, so disabling it can only waste disk space. |
| `tasks.silent-cleanup` | Silent Disk Clean-up | Frees space when a drive runs low, so disabling it can only make that worse. |
| `tasks.siuf-dmclient` | Feedback Client | Part of the feedback system with no clear separate result; privacy.feedback-requests-off stops the requests people notice. |
| `tasks.siuf-scenario` | Feedback Scenario Download | Part of the feedback system with no clear separate result; privacy.feedback-requests-off stops the requests people notice. |
| `tasks.snapshot-cleanup` | Setup Snapshot Clean-up | Clears setup leftovers, so disabling it can only waste disk space. |
| `tasks.space-agent` | Storage Spaces Agent | Storage Spaces maintenance that does nothing on PCs without Storage Spaces and can only hurt those with it. |
| `tasks.space-manager` | Storage Spaces Manager | Storage Spaces maintenance that does nothing on PCs without Storage Spaces and can only hurt those with it. |
| `tasks.speech-model` | Speech Model Download | privacy.speech-model is the documented control for speech model downloads. |
| `tasks.sqm` | Sqm Tasks | Old quality-measurement task with no visible result; privacy.ceip leaves the programme. |
| `tasks.startup-app` | Startup App Report | Trivial report on slow startup apps with no benefit from blocking it. |
| `tasks.state-repo` | State Repository Maintenance | Maintains the database Store apps rely on, so disabling it can only cause trouble later. |
| `tasks.storage-sense` | Storage Sense | Only acts if you turned Storage Sense on, and the switch in Settings is the proper control. |
| `tasks.sustainability-telemetry` | Sustainability Telemetry | Trivial energy reporting with no visible result, and overall diagnostic data has its own controls. |
| `tasks.temp-license` | Temporary Licence Exchange | Renews Store licences, so disabling it can only risk Store apps failing to open. |
| `tasks.themes-sync` | Synced Theme Images | Trivial download of synced theme images that nobody would notice. |
| `tasks.tiers-init` | Storage Tiers Init | Storage tier set-up that does nothing on PCs without tiered storage. |
| `tasks.time-force` | Forced Time Sync | Harmful: a correct clock matters for sign-in and secure websites, and the task costs nothing. |
| `tasks.time-sync` | Scheduled Time Sync | Harmful: a correct clock matters for sign-in and secure websites, and the task costs nothing. |
| `tasks.time-zone` | Time Zone Sync | Trivial time zone data update, and blocking it can leave the clock wrong. |
| `tasks.troubleshooting-scanner` | Troubleshooting Scanner | Has its own control in Settings for recommended troubleshooting, with no visible result from blocking the task. |
| `tasks.upnp-config` | UPnP Host Config | Trivial UPnP set-up step with no visible result. |
| `tasks.user-picture` | Update User Picture | Trivial account picture sync that nobody would want stopped. |
| `tasks.verify-winre` | Verify Recovery Environment | Checks that Windows recovery still works, so disabling it can only hide a broken recovery environment. |
| `tasks.wcm-wifi` | WCM Wi-Fi Task | Internal Wi-Fi management step with no result a person can state, and a risk to Wi-Fi connections. |
| `tasks.winsat` | WinSAT | Rarely runs, nothing a person uses shows its score now, and there is no visible result from blocking it. |
| `tasks.wlan-cds` | Wi-Fi CDS Sync | Internal Wi-Fi sync with no result a person can state. |
| `tasks.wlan-profile` | Wi-Fi Profile Management | Only manages mobile operator Wi-Fi profiles, so it does nothing on home Wi-Fi. |
| `tasks.wof-hash` | WIM Hash Management | Maintains compressed system files, so disabling it can only risk them. |
| `tasks.wof-validate` | WIM Hash Validation | Maintains compressed system files, so disabling it can only risk them. |
| `tasks.work-folders` | Work Folders Maintenance | Business file sync that does nothing on home PCs. |
| `tasks.work-folders-logon` | Work Folders Sign-in Sync | Business file sync that does nothing on home PCs. |
| `tasks.workplace-join` | Automatic Device Join | Joins work directories, so it does nothing on a home PC. |
| `tasks.ws-swap` | Sysmain Swap Assessment | Internal SysMain measurement with no documented job and no result a person can state. |
| `tasks.wwan-notify` | Mobile Broadband Notifications | Only shows mobile network alerts, so it does nothing without a modem. |
| `tasks.wwan-oobe` | Mobile Broadband Discovery | Only finds mobile networks at set-up, so it does nothing without a modem. |
| `tasks.xbl-save` | Xbox Game Save | Task half of Xbox cloud saves; services.xbox-save-off covers the same thing. |

### windows (94)

| Tweak | Name | Why |
|---|---|---|
| `windows.account-picture` | Account pictures | A community value with no known behaviour. |
| `windows.active-desktop` | Active Desktop | Active Desktop was removed from Windows years ago, so these policies do nothing. |
| `windows.aedebug` | Automatic debugger | Already the default, and it only matters to developers with a debugger installed. |
| `windows.aic-anywhere` | Install apps from anywhere | This is already the usual default, and where it isn't, it would loosen a safety setting the user chose. |
| `windows.always-elevated` | Always install elevated | Off by default and only turns on if an admin or malware already has control, so it changes nothing on a normal PC. |
| `windows.app-assoc-toast` | Default app nags | Community values with no confirmed result; they don't reliably stop Edge's default-browser prompts. |
| `windows.autocomplete` | Autocomplete append | Off by default, and nobody notices inline completion in the Run box. |
| `windows.balloon-tips` | Balloon tips | Balloon tips were replaced by toast notifications, so this no longer does anything on current Windows. |
| `windows.beep` | Error beep | Only controls the old motherboard speaker beep, which modern PCs don't use. |
| `windows.cached-icons` | Icon cache size | An old value from before today's icon cache, with no confirmed effect on current Windows. |
| `windows.cached-logons` | Cached sign-ins | Only applies to domain accounts, which home PCs don't use. |
| `windows.cert-errors` | Certificate error override | An Internet Explorer policy; IE is gone and current browsers ignore it. |
| `windows.chat` | Chat button | The Chat button left Windows 11 in version 23H2, so this has nothing to hide on current versions. |
| `windows.color-filter` | Colour filter | Colour filters and their shortcut are off by default, so this changes nothing on almost every PC. |
| `windows.colour-correction` | Automatic colour correction | A community value with no known effect; Windows keeps its real colour management setting elsewhere. |
| `windows.compressed-color` | Colour for compressed files | Off by default, and only visible on NTFS-compressed files, which few people have. |
| `windows.confirm-all` | Do this for all | Only changes the starting state of one checkbox in a rarely seen dialog. |
| `windows.console-vt` | Virtual terminal | A developer setting for the old console; Windows Terminal already handles this. |
| `windows.cortana-button` | Cortana button | Cortana was retired in 2023, so the button has nothing left to open. |
| `windows.credentials-delegation` | Protected credentials | A remote-admin credential policy for managed networks, irrelevant to a home PC. |
| `windows.dcom` | DCOM | Breaks management tools and some Windows components, with no benefit on a home PC behind a firewall. |
| `windows.dep-explorer` | DEP for Explorer | Already the Windows default, so it changes nothing. |
| `windows.desktop-process` | Desktop in separate process | An obscure value with no visible result on current Windows. |
| `windows.developer-mode` | Developer mode | Belongs in Settings for the developers who need it, and it loosens app install rules for everyone else. |
| `windows.developer-sideload` | Sideload apps | Allowed by default since Windows 10 version 2004, so this changes nothing. |
| `windows.dpi-96` | Custom DPI | Forces one legacy scale on every monitor by switching Windows back to Windows 8 style scaling, which the Display settings page already handles per monitor and without a sign-out; on a 4K or small laptop screen the result is unreadable text. |
| `windows.fast-user-switch` | Switch user on lock screen | An unreliable value that Windows rewrites itself; the supported control is a different policy, and home PCs rarely need it. |
| `windows.file-history` | File History policy | Sets an active-hours value, not an off switch, so it doesn't do what it claimed. |
| `windows.file-zone` | Downloaded file zones | As written it did the opposite of its label: the value stops Windows marking downloads, which weakens SmartScreen and Office protections, and Windows marks them by default. |
| `windows.font-gamma` | Font gamma | A community value with no visible result. |
| `windows.font-smoothing` | Font smoothing | ClearType is already the default, and the values here don't select it correctly. |
| `windows.foreground-flash` | Foreground flash | As written it did the opposite of its label: a count of 0 makes a taskbar button flash until clicked. |
| `windows.foreground-lock` | Foreground lock | Lets apps steal focus at once, which pulls you out of games, and it has no benefit. |
| `windows.hdr-gaming` | Auto HDR | Community values; Auto HDR is stored elsewhere, so this has no dependable effect. |
| `windows.hdr-playback` | HDR video playback | Community values with no clear result, and no reason to turn HDR video off. |
| `windows.heap-termination` | Help pane DEP | Already the Windows default, so it changes nothing. |
| `windows.hooks-timeout` | Low-level hooks timeout | Written to the wrong place: Windows reads this value per user, so it has no effect here. |
| `windows.hung-timeout` | Hung app timeout | A shorter limit only flags busy apps as hung sooner, which risks closing them with no real gain. |
| `windows.idle-wait` | Startup idle wait | A community value with no confirmed behaviour; the startup app delay tweak covers the same goal. |
| `windows.ie-ssl3` | SSL 3 fallback | An Internet Explorer policy for a protocol that is already disabled everywhere. |
| `windows.jit-debug` | Script JIT debugger | Already off by default, and it only matters to script developers. |
| `windows.list-watermark` | Folder watermarks | Folder background watermarks disappeared with older Windows versions, so nothing changes. |
| `windows.live-captions` | Live captions at sign-in | Only acts if you chose to start Live captions at sign-in, which the feature's own setting already controls. |
| `windows.lock-auto` | Lock screen auto-lock | A community value with no known behaviour. |
| `windows.maintenance` | Automatic maintenance | Also stops TRIM, drive optimisation and update upkeep, which costs far more than the idle work it saves. |
| `windows.map-drive-btn` | Map drive button | The toolbar this button belonged to is gone, so there is nothing to hide. |
| `windows.media-frameserver` | Media Foundation frame server | A workaround for a 2016 webcam bug that is long fixed, and it can break webcam apps today. |
| `windows.mi-layout` | Optimal layout | A hard drive file layout helper with no effect on SSDs and nothing noticeable on hard drives. |
| `windows.net-connect` | Connect menu | The Connect To menu it hides left the Start menu long ago. |
| `windows.no-autoplay` | AutoPlay for this user | Same result as the AutoPlay tweak, which covers every drive type. |
| `windows.no-autorun` | AutoRun policy | Windows already ignores autorun files on USB drives, and the AutoPlay tweak covers the rest. |
| `windows.non-enum` | Hide Documents folder | A community value that hides one shell folder, with no known use. |
| `windows.notif-critical` | Critical toasts on lock | A community value; the lock screen notifications tweak covers the same need. |
| `windows.notif-edge` | Edge notification ranking | A per-app notification choice that Settings already handles; there's no reason to copy it for one app. |
| `windows.notif-outlook` | Outlook notification ranking | A per-app notification choice that Settings already handles; there's no reason to copy it for one app. |
| `windows.notif-security` | Security notifications | Hides Windows' own security warnings, which is a protection worth keeping. |
| `windows.notification-center` | Notification centre | A per-user policy written machine-wide; where it applies it removes the panel outright, which the notifications switch handles better. |
| `windows.open-with-web` | Web search in Open With | Targets an old web lookup that current Windows replaced with the Store prompt, which the Store tweak covers. |
| `windows.people` | People band | The People bar was removed from Windows, so there is nothing to hide. |
| `windows.pin-taskbar` | Taskbar pinning | A kiosk-style lockdown that stops you pinning apps, with no use on a personal PC. |
| `windows.preview-desktop` | Peek at desktop | Same Peek feature as gpu.dwm-peek: DisablePreviewDesktop only governs the Windows 10 taskbar-corner hover, while EnableAeroPeek is the Performance Options switch that turns Peek off everywhere on both versions. |
| `windows.preview-pane` | Preview pane | A community value that isn't the real switch, and the pane is hidden by default anyway. |
| `windows.previous-versions` | Previous Versions tab | Hides a rarely opened tab, with no practical benefit. |
| `windows.prints-wizard` | Online prints wizard | The online print ordering wizard left Windows years ago. |
| `windows.rdp-com-redirect-off` | Block COM port redirection | Only matters for incoming Remote Desktop sessions, which are off by default, absent on Home and blocked outright by the Remote Desktop tweak. |
| `windows.rdp-pnp-redirect-off` | Block device redirection | Only matters for incoming Remote Desktop sessions, which are off by default, absent on Home and blocked outright by the Remote Desktop tweak. |
| `windows.recent-docs` | Recent documents | Same goal as the recent items switch, but as a policy that also blocks recent lists in other apps. |
| `windows.recent-nethood` | Recent shares | Applied to the old Network Places feature; current Windows has no such list. |
| `windows.rename-wizards` | Web tasks | XP-era web publishing wizards that no longer exist. |
| `windows.restart-apps` | Restart apps after sign-in | Written to the wrong place: Windows reads this per user, so the machine-wide value does nothing. |
| `windows.scan-attachment` | Scan attachments | As written it did the opposite of its label: the value tells Windows not to notify your antivirus, and real-time protection already checks files anyway. |
| `windows.settings-account` | Your account page | Hiding a Settings page has no benefit on a personal PC. |
| `windows.sharing-wizard` | Sharing wizard | Only swaps one file-sharing dialog for an older one, which few people ever see. |
| `windows.shell-security` | Shell extension security | An enterprise policy that stops many useful right-click tools loading, with little gain at home. |
| `windows.show-quick-access` | Quick access | A community value that isn't the real switch; the Hide Home tweak covers this. |
| `windows.shutdown-tracker` | Shutdown Event Tracker | Only ever shown on Windows Server, so it does nothing on a home PC. |
| `windows.sign-in-mode` | Boot to desktop | A Windows 8 era setting; current Windows goes to the desktop anyway. |
| `windows.smart-card-remove` | Smart card removal | Smart cards aren't used on home PCs, and the value here logs off rather than locks as it claimed. |
| `windows.smart-screen-note` | Smart card provider | Smart card sign-in rules for managed networks, and nothing to do with SmartScreen. |
| `windows.smooth-scroll` | Smooth scrolling | Only ever affected old list boxes; apps handle their own scrolling now, so nobody would notice. |
| `windows.sound-scheme` | No sounds | Only renames the active scheme: the sounds themselves are stored per event and keep playing. |
| `windows.start-pinned` | Pinned section | The value name wasn't found in the Start menu's files on build 26200, whose real values have other names, so the switch may do nothing. |
| `windows.storage-sense-off` | Turn off Storage Sense | Writes a machine-wide path Windows doesn't read; Storage Sense is stored per user, which the Storage Sense policy tweak sets correctly. |
| `windows.tablet-mode` | Tablet mode | A Windows 10 value that is already off on desktops, and Windows 11 has no tablet mode. |
| `windows.taskbar-size` | Lock taskbar size | Locked by default on Windows 10, and Windows 11 can't be unlocked, so this changes nothing. |
| `windows.tcg-disabled` | TCG security activation | A policy for self-encrypting drives on managed PCs, irrelevant at home. |
| `windows.thumb-cache` | Thumbnail cache | A community value; stopping the cache only makes folders slower, with no gain. |
| `windows.ts-enabled` | Terminal Services flag | Community values with no known behaviour, touching Remote Desktop keys. |
| `windows.visual-fx` | Visual effects: best performance | Writes only the radio-button choice the Performance Options dialog stores; the effects themselves are applied through UserPreferencesMask and the individual values, so the value alone changes nothing until someone opens the dialog, and the individual effects are already separate Visuals cards. |
| `windows.voice-access` | Voice access at sign-in | Only acts if you chose to start Voice access at sign-in, which its own setting controls. |
| `windows.window-arrange` | Window arrangement | Already on by default, so it changes nothing. |
| `windows.winrm-client` | WinRM basic auth | A remote-management setting for managed networks, and one of its two values is in the wrong place. |
| `windows.winrs` | Remote shell | Remote shell only runs when WinRM is set up, which home PCs never do. |
| `windows.zone-launch` | Launch unsafe files prompt | As written it did the opposite of its label: the value lets programs from the internet run without the warning prompt. |

## Duplicates

164 tweaks repeated another. Where the tweak a duplicate pointed to was itself removed (35 cases), the whole family went for its own reason, given in the table above.

| Tweak | Repeats |
|---|---|
| `apps.chrome-media` | `privacy.perm-camera` |
| `apps.edge-discover` | `windows.edge-sidebar-off` |
| `apps.edge-quick-links` | `apps.edge-top-sites` |
| `apps.edge-sidebar` | `windows.edge-sidebar-off` |
| `apps.edge-site-info` | `apps.edge-telemetry` |
| `apps.edge-spotlight` | `windows.edge-recommendations-off` |
| `apps.edge-widget` | `apps.edge-searchbar` |
| `apps.new-outlook-use` | `apps.new-outlook-toggle` |
| `cpu.boost-policy` | `cpu.boost-mode` |
| `cpu.dpc-watchdog-offset` | `cpu.dpc-watchdog` |
| `cpu.minimum-dpc-rate` | `cpu.ideal-dpc-rate` |
| `cpu.parking-max` | `cpu.parking-min` |
| `cpu.priority-dpc` | `cpu.ideal-dpc-rate` |
| `cpu.tagged-energy` | `cpu.energy-estimation` |
| `cpu.unpark-count` | `cpu.parking-min` |
| `gaming.audio-capture` | `gaming.game-dvr-off` |
| `gaming.cursor-capture` | `gaming.game-dvr-off` |
| `gaming.fortnite-be-priority` | `gaming.fortnite-priority` |
| `gaming.fortnite-eac-priority` | `gaming.fortnite-priority` |
| `gaming.game-dvr-user` | `gaming.game-dvr-off` |
| `gaming.keyboard-response` | `windows.keyboard-repeat-fast` |
| `gaming.mic-capture` | `gaming.game-dvr-off` |
| `gaming.mmcss-sfio` | `gaming.mmcss-games-priority` |
| `gaming.obs32-priority` | `gaming.obs-priority` |
| `gaming.steam-silent` | `apps.steam-startup` |
| `gpu.amd-ltr` | `gpu.amd-bgm-ltr` |
| `gpu.amd-usage-tracking` | `tasks.amd-uep` |
| `gpu.dwm-shadows` | `windows.visual-fx` |
| `gpu.intel-preemption` | `gpu.preemption` |
| `gpu.monitor-refresh-latency` | `gpu.monitor-latency` |
| `gpu.nv-dynamic-pstate` | `gpu.nv-pstate` |
| `gpu.nv-preemption` | `gpu.preemption` |
| `gpu.nv-preemption-compute` | `gpu.preemption` |
| `gpu.nv-preemption-cuda` | `gpu.preemption` |
| `gpu.nv-preemption-level` | `gpu.preemption` |
| `gpu.nv-preemption-s3` | `gpu.preemption` |
| `gpu.nv-profile-telemetry` | `services.nv-telemetry` |
| `gpu.nv-tdr` | `gpu.tdr-delay` |
| `gpu.nv-telemetry-driver` | `services.nv-telemetry` |
| `gpu.overlays` | `gaming.mpo-off` |
| `network.connections-1` | `network.connections` |
| `network.del-ack` | `network.ack-frequency` |
| `network.dns-neg-time` | `network.dns-neg-ttl` |
| `network.do-machine` | `network.delivery-optimization-http` |
| `network.forward-packets` | `network.forward-buffers` |
| `network.max-data-retrans` | `network.max-retrans` |
| `network.neg-cache-policy` | `network.dns-neg-ttl` |
| `network.netbios-driver` | `network.dns-netbios` |
| `network.provider-hosts` | `network.provider-dns` |
| `network.provider-local` | `network.provider-dns` |
| `network.provider-netbt` | `network.provider-dns` |
| `network.qos-valorant-launcher` | `network.qos-valorant` |
| `network.smb-signing-enable` | `network.smb-signing-require` |
| `power.usb-selective` | `power.usb-selective-suspend-off` |
| `privacy.account-consent` | `privacy.app-account-info-off` |
| `privacy.autologger-sqm` | `privacy.ceip` |
| `privacy.background-global` | `privacy.background-apps-off` |
| `privacy.camera-policy` | `privacy.perm-camera` |
| `privacy.cdm-content` | `privacy.cdm-silent` |
| `privacy.cdm-preinstalled` | `privacy.cdm-silent` |
| `privacy.cdm-slideshow` | `privacy.cdm-lockscreen` |
| `privacy.consumer-features` | `privacy.cdm-silent` |
| `privacy.cpss-advertising` | `privacy.ad-id-policy` |
| `privacy.cpss-ink` | `privacy.tipc` |
| `privacy.cross-device` | `privacy.cdp` |
| `privacy.device-metadata` | `windows.device-metadata-off` |
| `privacy.error-consent` | `privacy.error-report-policy` |
| `privacy.error-extra` | `privacy.error-report-policy` |
| `privacy.error-logging` | `privacy.error-report-policy` |
| `privacy.error-ui` | `privacy.error-report-policy` |
| `privacy.feedback-notifications` | `privacy.feedback-requests-off` |
| `privacy.handwriting-data` | `privacy.inking-typing-collection-off` |
| `privacy.harvest-contacts` | `privacy.inking-typing-collection-off` |
| `privacy.ink-collection` | `privacy.inking-typing-collection-off` |
| `privacy.input-personalization` | `privacy.online-speech-off` |
| `privacy.location-policy` | `privacy.location-consent` |
| `privacy.mic-policy` | `privacy.perm-microphone` |
| `privacy.paint-cocreator` | `privacy.paint-creator` |
| `privacy.pca` | `services.compat-assistant-off` |
| `privacy.perm-diagnostics` | `privacy.app-diagnostics-off` |
| `privacy.perm-notifications` | `privacy.app-notification-access-off` |
| `privacy.privacy-policy-input` | `privacy.inking-typing-collection-off` |
| `privacy.search-privacy` | `privacy.web-search` |
| `privacy.sensor-permission` | `privacy.location-consent` |
| `privacy.soft-landing` | `privacy.cdm-tips` |
| `privacy.spotlight` | `privacy.cdm-lockscreen` |
| `privacy.spotlight-lock` | `privacy.cdm-lockscreen` |
| `privacy.spotlight-settings` | `privacy.cdm-settings` |
| `privacy.sync-accessibility` | `privacy.settings-sync` |
| `privacy.sync-apps` | `privacy.settings-sync` |
| `privacy.sync-browser` | `privacy.settings-sync` |
| `privacy.sync-language` | `privacy.settings-sync` |
| `privacy.sync-packages` | `privacy.settings-sync` |
| `privacy.sync-personal` | `privacy.settings-sync` |
| `privacy.sync-start` | `privacy.settings-sync` |
| `privacy.sync-theme` | `privacy.settings-sync` |
| `privacy.sync-windows` | `privacy.settings-sync` |
| `privacy.telemetry-machine` | `privacy.diagnostic-data-minimum` |
| `privacy.voice-lock` | `privacy.voice-activation` |
| `privacy.voice-typing` | `privacy.online-speech-off` |
| `privacy.vs-telemetry` | `privacy.vs-sqm` |
| `privacy.web-search-metered` | `privacy.web-search` |
| `services.avctp` | `services.bluetooth-off` |
| `services.bcast-dvr` | `gaming.game-dvr-off` |
| `services.beep` | `windows.beep` |
| `services.bluetooth-user` | `services.bluetooth-off` |
| `services.btag` | `services.bluetooth-off` |
| `services.cbdhsvc` | `privacy.clipboard-history` |
| `services.cdp-user` | `services.cdp` |
| `services.display-enhance` | `power.adaptive-brightness` |
| `services.pim-index` | `services.sync-host-off` |
| `services.print-workflow` | `services.print-spooler-off` |
| `services.sensor-data` | `services.sensor-off` |
| `services.session-env` | `windows.rdp-off` |
| `services.shell-hw` | `windows.autoplay-off` |
| `services.termservice` | `windows.rdp-off` |
| `services.umrdp` | `windows.rdp-off` |
| `services.unistore` | `services.sync-host-off` |
| `services.userdata` | `services.sync-host-off` |
| `services.wersvc` | `privacy.error-report-policy` |
| `storage.ahci-dipm` | `power.ahci-link` |
| `storage.ahci-hipm` | `power.ahci-link` |
| `tasks.ait-agent` | `privacy.ait` |
| `tasks.autochk-proxy` | `privacy.ceip` |
| `tasks.ceip-bth` | `privacy.ceip` |
| `tasks.ceip-consolidator` | `privacy.ceip` |
| `tasks.ceip-kernel` | `privacy.ceip` |
| `tasks.ceip-uploader` | `privacy.ceip` |
| `tasks.ceip-usb` | `privacy.ceip` |
| `tasks.device-info-user` | `tasks.device-info` |
| `tasks.disk-diag-data` | `privacy.ceip` |
| `tasks.indexer-maint` | `services.wsearch` |
| `tasks.location-dialog` | `services.geolocation-off` |
| `tasks.location-notify` | `services.geolocation-off` |
| `tasks.maps-toast` | `services.maps-broker-off` |
| `tasks.maps-update` | `services.maps-broker-off` |
| `tasks.office-fallback` | `apps.office-telemetry` |
| `tasks.office-fallback16` | `apps.office-telemetry` |
| `tasks.office-logon` | `apps.office-telemetry` |
| `tasks.office-logon16` | `apps.office-telemetry` |
| `tasks.onesettings` | `privacy.onesettings` |
| `tasks.queue-reporting` | `privacy.error-report-policy` |
| `tasks.remote-assist` | `windows.remote-assistance` |
| `tasks.scan-updates` | `apps.store-auto-update` |
| `tasks.scan-updates-user` | `apps.store-auto-update` |
| `tasks.scheduled-diagnosis` | `privacy.scheduled-diagnostics` |
| `tasks.siuf-dmclient` | `privacy.feedback-requests-off` |
| `tasks.siuf-scenario` | `privacy.feedback-requests-off` |
| `tasks.speech-model` | `privacy.speech-model` |
| `tasks.sqm` | `privacy.ceip` |
| `tasks.xbl-save` | `services.xbox-save-off` |
| `windows.foreground-flash` | `windows.taskbar-flash` |
| `windows.idle-wait` | `windows.startup-delay` |
| `windows.no-autoplay` | `windows.autoplay-off` |
| `windows.no-autorun` | `windows.autoplay-off` |
| `windows.notif-critical` | `windows.lock-notifications` |
| `windows.notification-center` | `windows.notif-all` |
| `windows.open-with-web` | `windows.open-with-store` |
| `windows.preview-desktop` | `gpu.dwm-peek` |
| `windows.rdp-com-redirect-off` | `windows.rdp-off` |
| `windows.rdp-pnp-redirect-off` | `windows.rdp-off` |
| `windows.recent-docs` | `windows.start-docs` |
| `windows.show-quick-access` | `windows.hub-mode` |
| `windows.storage-sense-off` | `windows.storage-sense-policy` |

## Fable's challenge

**Accepted.** Retire 8 service tweaks and `performance.foreground-boost`, and fix the presets. Move `cpu.wpbt` to Windows, `windows.narrator-key` to Gaming, `performance.kernel-in-ram` to Memory. Flip `windows.accent-taskbar` and restore `windows.nav-expand` with the right value. Rename `windows.home-namespace` to Hide Gallery, since that's what its CLSID is.

**Rejected, with reasons.**

| Proposal | Why not |
|---|---|
| Fold Memory into CPU | The RAM tab carries RAM facts and matches the layout |
| Fold `gpu.tdr-ddi`, `tasks.compat-appraiser-exp` and the LSO cards into one card each | One target has one owner; retired entries would need surgery for no real gain |
| Merge Tasks into Services | Different mechanisms and different undo |
| Per-interface `TcpAckFrequency` | A new engine target that can't be tested here |
| `Start_Layout` | Gone from newer builds |
| `LaunchTo=3`, `SearchboxTaskbarMode=1`, `TaskbarGlomLevel=1` | One target has one owner; a second card for another value breaks that |
| Num Lock at sign-in for every account | `HKU` isn't a supported target |
| Include in library | The CLSID couldn't be confirmed |
| `DynamicScrollbars` | Not found in the files checked |
| Rebuild `privacy.search-box-suggestions` | The HKLM value isn't verified and HKCU policies are admin-only |
| Merge the advertising ID cards | The owner's PC has one applied |

## Fable's second review

Fable 5.1 reviewed the built page, the screenshots and the remaining library, and was asked to challenge rather than approve.

**Taken.**

| Finding | What changed |
|---|---|
| On half the Visuals cards, on meant the named feature was off, and the same ran through Tweaks | 173 cards renamed so the name is true when the switch is on ("Turn off transparency", "Block camera access"). IDs didn't change |
| Version-specific cards showed as working switches everywhere | 15 cards now have a build range and show as Unavailable where Windows ignores them. The builder is `.until(n)` |
| `windows.min-animate` was retired as covered by a policy Home can't use | Restored to Effects and animation, so every edition has a window animation switch |
| The Tasks tab was all Unavailable on the owner's PC | `Get-ScheduledTask` confirmed those four tasks aren't on this PC, so detection was right. They ship on other PCs, so they stay. The tab is gone: Microsoft's four are in Privacy > Telemetry, AMD's three in Hardware > GPU > AMD software |
| Presets were category sweeps, so they muted notifications and closed apps at shutdown | Every preset is now a hand-written list (`PRESET_TWEAKS`), with `safe()` as a backstop. Eight tweaks gained the warning they lacked |
| Placement | `windows.start-docs` to Privacy > Windows data, `windows.sync-ads` to Privacy > Advertising and suggestions, `performance.last-access-off` to Hardware > Storage, and the Core tab's "Core" heading is gone |
| Gaming Essentials was padded with CS2 priority | Per-game cards never fill a row, and a section with no hand-placed card is never filled |
| Sign-in had a row with a hole | Lock screen and Sign-in merged into Lock and sign-in, and `visuals.restart-apps` rebuilt on the per-user value (`RestartApps` is in `SettingsHandlers_User.dll`) |
| Text | 15 clipped lines replaced, `windows.folder-type` corrected, `gaming.system-responsiveness` no longer overstates, every-account cards say so, `cpu.wpbt` is `documented` |
| Icons | Ten replaced, such as a sound-off icon for the startup sound card |
| Filters | Impact, risk and evidence hide when every card on the page has the same value |
| QoS cards hedged the router but not Windows | Line and details now say Windows may skip the policy outside a domain |

**Went further than the review.** `windows.start-pinned` is retired: `ShowPinnedSection` isn't in the Start menu's files on build 26200, whose real values have other names. `privacy.meet-now` is retired: Windows 10 only, and Skype closed in May 2025.

**Rejected, with reasons.**

| Proposal | Why not |
|---|---|
| A Turn off button on Visuals cards that are Already set | Needs a proven Windows default for every setting, and that needs a VM. Writing guesses to a real PC isn't acceptable. The tooltip now names the Windows page instead |
| `DontDisplayLastUserName` | Everyone would then have to type their account name to sign in, and it would widen the exception under the UAC key for a cosmetic card |
| Retire the policy card now `windows.min-animate` is back | It stops more than minimise and maximise, and covers every account |
| Retire `windows.small-icons` and `windows.tray-all` | The app still supports Windows 10. They're Unavailable on 11 and can go in a later release |
| Warnings on `windows.notif-sound`, `windows.auto-restart`, `windows.kill-service`, `power.intel-graphics`, `apps.chrome-notifications` | Their line already says what they do, and warnings are for a named feature that breaks. They're out of the presets instead |
| Split a Home page group out of View | A heading for four cards adds nothing while View fills its rows |

## Needs further research

- **Classic right-click menu undo.** Needs a VM to prove removing the key brings the Windows 11 menu straight back.
- **QoS rows.** Whether they apply without Do not use NLA on home PCs (all 8 `network.qos-*`).
- **Fable's VM list.** `visuals.clock-seconds` and `visuals.taskbar-all-displays` without an Explorer restart, `visuals.accent-from-wallpaper` from a bare write, `windows.widgets` under UCPD, `gpu.dwm-mode-animation` on 24H2, whether `proOrHigher` registry policies are honoured on Home, and the four Tasks paths on a stock 26200.
- **Binary Visual settings.** Window shadows, fading menus, combo-box slide, smooth scrolling and the cursor shadow live in the `UserPreferencesMask` bitfield. The engine has no binary value type, so there's no card for them yet.
- **Lock screen policy.** `visuals.no-lock-screen` was dropped: Microsoft lists it for Enterprise and Education, unverified on Home and Pro.

| Tweak | Decision | Open question |
|---|---|---|
| `apps.chrome-ad-block` | remove | Google's published control is AdsSettingForIntrusiveAdsSites. |
| `apps.chrome-ai-search` | remove | Google's published AI controls are AIModeSettings, GenAiDefaultSettings and SearchContentSharingSettings. Rebuild on those if an AI switch is wanted. |
| `apps.chrome-error-pages` | remove | Evidence of removal is indirect (Chromium test data and a deprecated CIS item). Check the live Chrome policy list if this matters. |
| `apps.chrome-lens` | remove | LensRegionSearchEnabled is due for deprecation in Chrome 147 in favour of SearchContentSharingSettings. Rebuild on that if wanted. |
| `apps.chrome-privacy-sandbox` | remove | Reports say removal by about Chrome 150. Confirm the policies' status on current Chrome before publishing the reason. |
| `apps.chrome-sign-in` | keep | SigninAllowed was replaced by BrowserSignin. SyncDisabled should still work; check whether sign-in itself is still blocked on current Chrome. |
| `apps.edge-bing-chat` | remove | One report says it didn't remove the icon even on Edge 126. The newer CopilotNewTabPageEnabled is Entra-only too. |
| `apps.edge-install-default` | remove | InstallDefault applies to everything managed by Edge Update, not just Edge. The app itself runs on WebView2, so this matters here. |
| `apps.firefox-home` | remove | FirefoxHome is an object (Search, TopSites, SponsoredTopSites and so on). Rebuild with sub-values if wanted. |
| `apps.firefox-messaging` | remove | UserMessaging is an object (ExtensionRecommendations, FeatureRecommendations, SkipOnboarding and so on). Rebuild with sub-values if wanted. |
| `apps.firefox-prediction` | remove | The correct form is a single DWORD NetworkPrediction under the Firefox policy key, not a subkey with Enabled. |
| `apps.new-outlook-migrate` | remove | The consumer-side control appears to be NewOutlookMigrationUserSetting under the HKCU Policies key, which is admin-only here. |
| `apps.new-outlook-toggle` | keep | Sources disagree on whether 1 or 0 hides the toggle. This entry uses 1. Check Microsoft's control install page before shipping. |
| `apps.office-telemetry` | keep | Microsoft's current privacy control is the SendTelemetry policy. Check how much DisableTelemetry still does on Microsoft 365 builds. |
| `apps.opera-background` | remove | Based on an Opera forum post, not official documentation. Could be tested at opera://policy. |
| `cpu.global-timer` | keep | Undocumented by Microsoft. Sources conflict on which builds honour it and it has not been confirmed on Windows 10. Community guides say a restart is needed, but the tweak's restart field is none. |
| `cpu.wpbt` | keep | Filed under Scheduler and timers, which is the wrong home: it is a startup and security setting. Community sources only; check it on a PC with a WPBT table. |
| `gaming.cs2-priority` | keep | Mechanism is real but the benefit is unmeasured. Not tested alongside VAC. |
| `gaming.fortnite-priority` | keep | Mechanism is real but the benefit is unmeasured. Not tested alongside Easy Anti-Cheat. |
| `gaming.fse-behavior` | keep | Undocumented system-wide version of the per-game compatibility tick. Confirm it still applies on Windows 11 24H2 and later. |
| `gaming.game-bar-link` | remove | The usual fix also sets an open command for ms-gamebar. If this is rebuilt, test it on a PC with Game Bar removed. |
| `gaming.mouse-keys` | remove | Flags 38 keeps the hotkey-active bit. Value 58 clears it, matching how the Toggle Keys entry is set, if this is rebuilt. |
| `gaming.mpo-off` | keep | Reports conflict on 24H2 and 25H2. DisableOverlays (gpu.overlays) is the anecdotal replacement and one report says it breaks VRR. Needs a test on real GPUs. |
| `gaming.roblox-priority` | keep | Mechanism is real but the benefit is unmeasured. Not tested alongside Roblox's anti-tamper. |
| `gpu.amd-ulps` | remove | The one AMD value here with well-known behaviour, through MSI Afterburner's option. Reconsider only if hybrid Radeon laptops show wake faults. |
| `gpu.dwm-mode-animation` | visuals | Guides date it to build 22631.3810. An unanswered forum question asks whether it works on 24H2, so test it there. |
| `gpu.dwm-peek` | visuals | Overlaps windows.preview-desktop, which sets the separate taskbar switch for the desktop corner. Consider showing one card in Visuals. |
| `gpu.nv-cpl-optin` | keep | Widely listed in privacy guides, but NVIDIA does not document it. Check it on a current driver. |
| `gpu.nv-pstate` | keep | Not documented by NVIDIA, but the effect is easy to see in clock readings. Check it on a current driver and consider whether it belongs in a preset. |
| `gpu.overlays` | remove | If OverlayTestMode proves dead on 24H2 and later, this is the candidate replacement. Test it for VRR side effects first. |
| `memory.svchost-split` | keep | The value is 32 GB in KB, so PCs with 32 GB or more see no change. Scripts usually match it to installed RAM. |
| `network.ack-frequency` | remove | Popular tweak. Any rebuilt version would have to target the interface key, with a measured reason to exist. |
| `network.dns-netbios` | keep | Added to the DNS client with Windows 11 22H2. Sources differ on whether the local key or the policy key is honoured, and there is a report of the policy misbehaving. Confirm on a VM, and on Windows 10. |
| `network.doh` | keep | Value 2 is Allow, not Require, so the old description overclaimed. A script's comments and the policy's options agree, but check the number against the ADMX. |
| `network.metered-device` | remove | Bad path in the catalogue: HKLM\Microsoft\Windows. Check how the app handles a root key like this. |
| `network.qos-apex` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-cs2` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-fortnite` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-league` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-overwatch` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-rainbow-six` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-rocket-league` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.qos-valorant` | keep | QoS policies may apply only on the domain network profile unless 'Do not use NLA' is set under Tcpip\QoS, which is not among these values. Confirm with a packet capture on a home PC. |
| `network.throttling-index` | keep | Microsoft's original note dates from Windows Vista and was only seen through secondary copies. Measure it on current Windows. |
| `performance.foreground-boost` | remove | Win32PrioritySeparation 38 (0x26) decodes to short, variable, 3:1, which is what the client default of 2 already means. Effectively a no-op on client Windows. Kept only because profiles use it. |
| `power.gpu-preference` | remove | Only the GUID name is documented. Could not confirm what value 2 does, or that apps follow it on hybrid laptops. |
| `power.platform-aoac` | remove | Reported to work on some 2018-era Intel laptops and to cause blue screens on some Ryzen 4000 laptops. Community reports only. Desktops without Modern Standby see no change. |
| `privacy.ad-id-policy` | keep | Overlaps privacy.advertising-id-off, which is guarded. This is the documented control, so it stays; the two could be merged. |
| `privacy.advertising-id-off` | keep | Writes the HKLM value, while the switch in Settings is per-user under HKCU, so it may not change that switch. Kept as guarded. Consider merging into privacy.ad-id-policy. |
| `privacy.autologger-diagtrack` | keep | Overlaps services.diagtrack-off. If that service is off this adds little. No measured data. |
| `privacy.copilot-policy` | keep | TurnOffWindowsCopilot is marked deprecated. RemoveMicrosoftCopilotApp is the newer policy; check which editions and builds honour it. |
| `privacy.cross-device` | remove | The Settings switch for mobile devices appears to be CrossDeviceEnabled under the HKCU Mobility key. Rebuild on that if wanted. |
| `privacy.cross-resume` | remove | The Resume switch is reported as IsResumeAllowed under CrossDeviceResume\Configuration. Rebuild on that if wanted. |
| `privacy.dotnet-telemetry` | keep | Microsoft publishes this variable, so the evidence could be raised from community to documented. |
| `privacy.exclude-drivers` | keep | Writes the MDM policy store rather than the WindowsUpdate policy key. Check it's honoured on Home; Windows Update policies often aren't. |
| `privacy.inventory` | keep | This Group Policy setting may be deprecated on recent builds. Check it still acts on 24H2 and later. |
| `privacy.news-interests` | visuals | This is the Windows 11 Widgets policy. Windows 10's News and interests uses a different policy, so this doesn't touch it. Partly overlaps windows.widgets, which only hides the button. |
| `privacy.paint-cocreator` | remove | These may be separate switches on Copilot+ PCs. If Cocreator stays visible after the Image Creator tweak, restore this one. |
| `privacy.powershell-telemetry` | keep | Microsoft publishes this variable, so the evidence could be raised from community to documented. |
| `privacy.search-box-suggestions` | remove | The working value is DisableSearchBoxSuggestions under Policies\Microsoft\Windows\Explorer. It's the known fix for web results in Start on Windows 11. HKCU Policies is admin-only here, so check it through the broker. |
| `privacy.settings-sync` | keep | SyncPolicy is undocumented. The Group Policy control is DisableSettingSync, and Windows 11 has moved this to Windows Backup. Check it still acts. |
| `privacy.spotlight-welcome` | keep | The policy value here is user-level and Enterprise or Education only. The per-user Scoobe value does the work, so the policy half could be dropped. |
| `privacy.web-search` | keep | The Group Policy half may be limited to Enterprise and Education. Check on 24H2 whether BingSearchEnabled alone still works; DisableSearchBoxSuggestions under Explorer is the known fix. |
| `services.biometric-mcp` | remove | The id and app description look wrong. Forum reports describe McpManagementService as Universal Print Management Service from about 2022, before Windows MCP support was announced, so 'agent connector' is unconfirmed. |
| `services.bluetooth-off` | remove | Trigger-start and idle without Bluetooth, so low value. Kept because service-presets uses it. |
| `services.cdp` | keep | Stronger than privacy.cdp and privacy.nearby, which only switch sharing off. Check on a VM that the Bluetooth and Cast pages in Settings still behave with this off. |
| `services.compat-assistant-off` | keep | Overlaps privacy.pca, the policy version. Kept because service-presets uses it. |
| `services.maps-broker-off` | remove | Low value: it stops itself on most PCs. Kept because service-presets uses it. tasks.maps-toast and tasks.maps-update were removed in its favour. |
| `services.nv-container` | remove | NvContainerLocalSystem is the LocalSystem container used by the NVIDIA app and GeForce Experience. NVDisplay.ContainerLocalSystem is the display container. Worth confirming on a current driver. |
| `services.nv-telemetry` | remove | Reports say NvTelemetryContainer is gone and telemetry moved into NVDisplay.Container. Sources are old, so check a current driver. gpu.nv-telemetry-driver is the other NVIDIA telemetry tweak. |
| `services.print-notify-off` | remove | Trigger-start and idle without a printer. Kept because service-presets uses it. |
| `services.sensor-monitoring-off` | remove | Trigger-start and idle on desktops. Kept because service-presets uses it. |
| `services.sensor-off` | remove | Trigger-start and idle on desktops. Kept because service-presets uses it. |
| `services.wap-push-off` | remove | Trigger-start with little value on a home PC. Kept because service-presets uses it. |
| `services.xbox-auth-off` | remove | Harmful for Game Pass players and no saving while idle. Kept because service-presets uses it. Worth keeping out of any gaming-facing preset. |
| `services.xbox-save-off` | remove | Manual and idle, and harmful for Game Pass players. Kept because service-presets uses it. |
| `storage.reserved` | keep | DISM /Set-ReservedStorageState is the supported method. Confirm on 24H2 or later that these registry writes alone release the space. |
| `tasks.amd-uep` | keep | The task name suggests it installs the programme rather than sends data itself. Confirm on an AMD PC that it still exists and what it runs. |
| `tasks.compat-appraiser-exp` | keep | No Microsoft source for the Exp suffix. Kept only because tasks.compat-appraiser leaves a gap without it. Could be folded into that tweak as a second setting. |
| `tasks.cross-device-resume` | remove | No source for the WS task path. Users report CrossDeviceResume.exe keeps running even with the Settings switch and policy off, and no official off switch was found. |
| `tasks.device-info` | keep | tasks.device-info-user runs the same census per user and was removed as a copy. Could be folded into this tweak as a second setting. |
| `tasks.nv-mon` | remove | Search found only 2016 to 2017 sources for NvTmMon, NvTmRep and NvTmRepOnLogon. Check Task Scheduler on a current driver. If they exist, one card could return. |
| `tasks.program-data-updater` | keep | Present on Windows 10 22H2. Could not confirm it survives on Windows 11 24H2 and later; check on a current build. |
| `windows.copilot-button` | visuals | No effect on Windows 11 24H2 or later, where Copilot is an app. Worth dropping once Windows 10 goes. |
| `windows.desktop-icons` | visuals | VALUE BUG, fix before shipping: HideIcons must be 1 to hide icons; the catalogue sets 0, which shows them. The FFlags value looks like auto-arrange and align to grid, not hide, so drop it. This text describes the fixed tweak. |
| `windows.enthusiast` | visuals | VALUE BUG, fix before shipping: EnthusiastMode must be 1 for more details; the catalogue sets 0, which is fewer details and the default. This text describes the fixed tweak. |
| `windows.hub-mode` | visuals | HubMode is unofficial, and one of the two paths is missing Microsoft in the key. Check it still works on 24H2 or later. |
| `windows.restart-apps` | remove | The real value is under HKCU, the Settings switch for restartable apps. Worth re-adding there if the owner wants it. |
| `windows.sound-scheme` | remove | Not tested. Removing sounds also needs each event's current sound cleared, so a working version would be a different tweak. |
| `windows.start-pinned` | remove | `ShowAllPinsList` and `ShowRecentList` are in HKCU on 26200. Find out what each does before offering a Start layout card. |
| `windows.storage-sense-off` | remove | First-class tweak. The Settings switch for Storage Sense lives under HKCU, so this HKLM value looks like a no-op. The policy tweak also writes the HKCU value. |
| `windows.taskbar-left` | visuals | VALUE BUG, fix before shipping: TaskbarAl 0 is left and 1 is centre; the catalogue sets 1, which is the default. This text describes the fixed tweak. |
| `windows.tray-all` | visuals | Community reports say EnableAutoTray has no effect on Windows 11 24H2. Not officially confirmed. |
| `windows.visual-fx` | remove | This value may only select the preset in the dialog. Sources point to the user preferences mask as what really applies the effects. Test on a VM after sign-out. |
| `windows.widgets` | visuals | Recent Windows 11 builds protect this value with the UCPD driver, so a direct write can fail with access denied. Test on a VM. privacy.news-interests is the policy route. |

## What couldn't be tested on this PC

No tweak was applied to the development PC. Apply, read-back and undo were proved for every library tweak against an in-memory store (`library_tweaks_apply_once_and_restore_what_the_pc_had`), and policy checks ran on every target. Still unproved until a VM is available: the visible result of each Visuals setting on real Windows, the `WM_SETTINGCHANGE` broadcast, the classic menu undo, and which Explorer settings apply without signing out.
