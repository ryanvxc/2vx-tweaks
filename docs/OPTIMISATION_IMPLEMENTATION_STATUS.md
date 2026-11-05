# Optimisation implementation status

1,300 tweaks are implemented: 64 that already shipped, 1,221 added in 0.3.0, and 15 adapter tweaks added in 0.4.0. Where each one
came from is in [OPTIMISATION_LIBRARY_AUDIT.md](OPTIMISATION_LIBRARY_AUDIT.md).

Implemented means the tweak is in the Rust engine with detection, backup, apply, verify, rollback and revert, not just a
card in the UI.

## Totals

| Risk | Tweaks |
| --- | --- |
| Low | 958 |
| Medium | 306 |
| High | 21 |

Critical is not used: nothing rated Critical ships.

| Impact | Tweaks |
| --- | --- |
| Low | 1,242 |
| Medium | 41 |
| High | 2 |

178 tweaks carry a warning because they can break a named feature. 1,010 library tweaks need administrator rights.
112 need a restart and 1 a sign-out.

## What was tested

| Check | How | Result |
| --- | --- | --- |
| Apply, verify, idempotent re-plan, restore of the original value | `library_tweaks_apply_once_and_restore_what_the_pc_had`, every usable library tweak, two passes (desktop with Intel, laptop with AMD) | Pass, simulated store |
| Duplicate prevention | `rejects_conflicting_targets_across_tweaks`, plus `TweakRegistry::register` at start-up for the whole catalogue | Pass |
| Hardware and build checks | `hardware_tweaks_stay_on_their_hardware` | Pass |
| Tasks are only ever turned off | `scheduled_tasks_are_only_turned_off` | Pass |
| Security denylist | `scheduled_tasks_stay_off_security_paths`, `core_services_and_mitigations_stay_protected`, `non_registry_values_keep_their_shape` | Pass |
| Text fits its row | `library_tweaks_read_well_in_a_row` | Pass |
| Real read on a Windows PC | Every tweak's state read on the reference PC, nothing written | Pass |
| Library page | Every tab, search, filters and details in the real app | Pass |

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
| `performance.edge-startup-boost-off` | Turn off Microsoft Edge Startup boost | Shipped / performance | Low | Medium | Documented | Registry | 1 | Admin | Windows 10/11 |
| `performance.edge-background-mode-off` | Stop Microsoft Edge running in the background | Shipped / performance | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-launch-tracking-off` | Stop tracking app launches | Shipped / privacy | Low | Low | Documented | Registry | 1 | Sign out | Windows 10/11 |
| `privacy.language-list-opt-out` | Don't share your language list with websites | Shipped / privacy | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `privacy.edge-shopping-off` | Turn off Shopping in Microsoft Edge | Shipped / privacy | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.advertising-id-off` | Turn off the advertising ID | Shipped / privacy | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.edge-sidebar-off` | Hide the Microsoft Edge sidebar | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.edge-recommendations-off` | Stop Microsoft Edge feature suggestions | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.online-speech-off` | Turn off online speech recognition | Shipped / privacy | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `privacy.inking-typing-collection-off` | Stop learning from typing and inking | Shipped / privacy | Low | Low | Documented | Registry | 2 | Sign out | Windows 10/11 |
| `privacy.feedback-requests-off` | Stop Windows feedback requests | Shipped / privacy | Low | Low | Documented | Registry | 2 | None | Windows 10/11 |
| `privacy.diagnostic-data-minimum` | Send the least diagnostic data | Shipped / privacy | Medium | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `privacy.activity-history-off` | Turn off activity history | Shipped / privacy | Medium | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.background-apps-off` | Block background apps | Shipped / privacy | Medium | Medium | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-location-off` | Block app access to location | Shipped / privacy | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-diagnostics-off` | Block app access to diagnostics | Shipped / privacy | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-account-info-off` | Block app access to account info | Shipped / privacy | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.app-notification-access-off` | Block app access to notifications | Shipped / privacy | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `gaming.mmcss-games-priority` | Raise game priority in MMCSS | Shipped / gaming | Medium | Medium | Documented | Registry | 2 | Admin, Restart | Windows 10/11 |
| `gaming.game-dvr-off` | Turn off Game DVR | Shipped / gaming | Medium | Medium | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.foreground-boost` | Favour the app you're using | Shipped / performance | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `performance.last-access-off` | Stop updating last-access times | Shipped / performance | Medium | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.window-animations-off` | Turn off window animations | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin, Sign out | Windows 10/11 |
| `windows.storage-sense-off` | Turn off Storage Sense | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `services.maps-broker-off` | Disable Downloaded Maps Manager | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.bluetooth-off` | Disable Bluetooth Support | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.wap-push-off` | Disable WAP push routing | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.compat-assistant-off` | Disable Program Compatibility Assistant | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.sensor-off` | Disable Sensor Service | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.sensor-monitoring-off` | Disable Sensor Monitoring | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.sync-host-off` | Disable Sync Host | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.print-spooler-off` | Disable Print Spooler | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.print-notify-off` | Disable printer extensions | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.xbox-auth-off` | Disable Xbox Live Auth Manager | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.xbox-save-off` | Disable Xbox Live Game Save | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.diagtrack-off` | Disable Telemetry Service | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `services.geolocation-off` | Disable Geolocation Service | Shipped / services | Medium | Low | Documented | Service | 1 | Admin, Restart | Windows 10/11 |
| `gaming.mpo-off` | Disable multiplane overlay | Shipped / gaming | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.system-responsiveness` | Less CPU for background tasks | Shipped / gaming | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.power-throttling-off` | Disable power throttling | Shipped / power | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.kernel-in-ram` | Keep kernel code in RAM | Shipped / performance | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.mouse-acceleration-off` | Disable mouse acceleration | Shipped / windows | Low | Low | Documented | Registry | 3 | Sign out | Windows 10/11 |
| `windows.keyboard-repeat-fast` | Fastest key repeat | Shipped / windows | Low | Low | Documented | Registry | 2 | Sign out | Windows 10/11 |
| `windows.sticky-keys-shortcut-off` | Turn off the Sticky Keys shortcut | Shipped / windows | Low | Low | Documented | Registry | 1 | Sign out | Windows 10/11 |
| `power.usb-selective-suspend-off` | Disable USB selective suspend | Shipped / power | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.short-names-off` | Stop creating 8.3 file names | Shipped / performance | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.delivery-optimization-http` | Updates from Microsoft only | Shipped / network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.prefer-ipv4` | Prefer IPv4 over IPv6 | Shipped / network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.smb-throttling-off` | Disable SMB throttling | Shipped / network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.llmnr-off` | Disable LLMNR | Shipped / network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.icmp-redirect-off` | Ignore ICMP redirects | Shipped / network | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `performance.fault-tolerant-heap-off` | Disable Fault Tolerant Heap | Shipped / performance | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.autoplay-off` | Turn off AutoPlay | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.device-metadata-off` | Stop downloading device apps | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.rdp-com-redirect-off` | Block COM port redirection | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `windows.rdp-pnp-redirect-off` | Block device redirection | Shipped / windows | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.qos-cs2` | Counter-Strike 2 | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-valorant` | VALORANT | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-fortnite` | Fortnite | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-apex` | Apex Legends | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-league` | League of Legends | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-overwatch` | Overwatch 2 | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-rocket-league` | Rocket League | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `network.qos-rainbow-six` | Rainbow Six Siege | Shipped / network | Low | Low | Documented | Registry | 11 | Admin, Restart | Windows 10/11 |
| `apps.edge-telemetry` | Edge diagnostic data | Library / Applications / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-personalization` | Edge personalisation | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-site-info` | Edge site info | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-feedback` | Edge feedback | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-experiments` | Edge experiments | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-copilot-page` | Edge Copilot page access | Library / Applications / Edge | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.edge-bing-chat` | Edge new tab Copilot | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-collections` | Edge Collections | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-discover` | Edge Discover | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-enhance-images` | Edge image enhancement | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-follow` | Edge Follow | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-games` | Edge games menu | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-rewards` | Edge rewards | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-family` | Edge family safety | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-insider` | Edge insider promotion | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-promotional` | Edge promotional tabs | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-spotlight` | Edge Spotlight | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-signin-ntp` | Edge sign-in prompt | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-searchbar` | Edge search bar | Library / Applications / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-widget` | Edge desktop widget | Library / Applications / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-sidebar` | Edge standalone sidebar | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-ntp-bg` | Edge new tab background | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-top-sites` | Edge top sites | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-quick-links` | Edge quick links | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-suggest` | Edge search suggestions | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-bing-ads` | Edge Bing ads | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-related-matches` | Edge related matches | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-nav-errors` | Edge error pages | Library / Applications / Edge | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-address-bar` | Edge Microsoft Search | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-autofill-address` | Edge address autofill | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-autofill-card` | Edge card autofill | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-payment-query` | Edge payment query | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-3p-cookies` | Edge third-party cookies | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-tracking` | Edge tracking prevention | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-dnt` | Edge Do Not Track | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-site-safety` | Edge site safety | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-spellcheck` | Edge spellcheck | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-in-app-support` | Edge in-app support | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-acrobat` | Edge Acrobat button | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-update-window` | Edge update hours | Library / Applications / Edge | High | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.edge-update-period` | Edge update checks | Library / Applications / Edge | High | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-install-default` | Edge install block | Library / Applications / Edge | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.edge-prelaunch` | Legacy Edge prelaunch | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-tab-preload` | Legacy Edge tab preload | Library / Applications / Edge | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.edge-do-not-upgrade` | Chromium Edge upgrade | Library / Applications / Edge | Low | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-background` | Chrome background mode | Library / Applications / Chrome | Low | Medium | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-metrics` | Chrome metrics | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-feedback` | Chrome feedback | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-url-data` | Chrome URL data | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-privacy-sandbox` | Chrome Privacy Sandbox | Library / Applications / Chrome | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `apps.chrome-lens` | Chrome Lens | Library / Applications / Chrome | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.chrome-ai-search` | Chrome AI search | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-3p-cookies` | Chrome third-party cookies | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-cookies-session` | Chrome session cookies | Library / Applications / Chrome | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-geolocation` | Chrome location | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-media` | Chrome camera and mic | Library / Applications / Chrome | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-notifications` | Chrome notifications | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-sensors` | Chrome sensors | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-bluetooth` | Chrome Web Bluetooth | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-usb` | Chrome Web USB | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-prediction` | Chrome network prediction | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-suggest` | Chrome search suggestions | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-translate` | Chrome translate | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-spellcheck` | Chrome spellcheck service | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-password-manager` | Chrome password manager | Library / Applications / Chrome | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-autofill-address` | Chrome address autofill | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-autofill-card` | Chrome card autofill | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-sign-in` | Chrome sign-in | Library / Applications / Chrome | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.chrome-dns` | Chrome built-in DNS | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-doh` | Chrome DoH | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-error-pages` | Chrome error pages | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-promos` | Chrome promotional tabs | Library / Applications / Chrome | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.chrome-ad-block` | Chrome ad blocking | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-cleanup` | Chrome Cleanup | Library / Applications / Chrome | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.chrome-cloud-reporting` | Chrome cloud reporting | Library / Applications / Chrome | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.chrome-cloud-print` | Chrome cloud print | Library / Applications / Chrome | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.chrome-sb-reporting` | Chrome Safe Browsing reports | Library / Applications / Chrome | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.chrome-updates` | Chrome update checks | Library / Applications / Chrome | High | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `apps.chrome-components` | Chrome component updates | Library / Applications / Chrome | High | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-telemetry` | Firefox telemetry | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-studies` | Firefox studies | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-crash` | Firefox crash reporter | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-default-agent` | Firefox default agent | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-feedback` | Firefox feedback | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-accounts` | Firefox accounts | Library / Applications / Firefox | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-pocket` | Pocket | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-private-shortcut` | Private browsing shortcut | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-import` | Firefox profile import | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-refresh` | Firefox profile refresh | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-password-reveal` | Firefox password reveal | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-recommendations` | Extension recommendations | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-home` | Firefox home | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-messaging` | Firefox messaging | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-captive` | Captive portal check | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-doh` | Firefox DoH | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-prediction` | Firefox network prediction | Library / Applications / Firefox | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.firefox-updates` | Firefox updates | Library / Applications / Firefox | High | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.firefox-addons-update` | Firefox system add-on updates | Library / Applications / Firefox | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.brave-background` | Brave background mode | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.brave-metrics` | Brave metrics | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.brave-hw` | Brave hardware acceleration | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.brave-reports` | Brave Safe Browsing reports | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.brave-spellcheck` | Brave spellcheck | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.opera-background` | Opera GX background mode | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.opera-metrics` | Opera GX metrics | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.opera-hw` | Opera GX hardware acceleration | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.opera-search` | Opera GX search provider | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.vivaldi-metrics` | Vivaldi metrics | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.vivaldi-prediction` | Vivaldi prediction | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.vivaldi-spellcheck` | Vivaldi spellcheck | Library / Applications / Brave, Opera and Vivaldi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.office-telemetry` | Office telemetry | Library / Applications / Office | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `apps.office-verbose` | Office verbose logging | Library / Applications / Office | Low | Low | Found in Windows | Registry | 3 | None | Windows 10/11 |
| `apps.office-feedback` | Office feedback | Library / Applications / Office | Low | Low | Documented | Registry | 2 | None | Windows 10/11 |
| `apps.office-outlook-calendar` | Outlook calendar logging | Library / Applications / Office | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `apps.office-outlook-mail` | Outlook mail logging | Library / Applications / Office | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `apps.office-word-logging` | Word logging | Library / Applications / Office | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `apps.new-outlook-migrate` | New Outlook migration | Library / Applications / Office | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `apps.new-outlook-toggle` | New Outlook toggle | Library / Applications / Office | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `apps.new-outlook-use` | Classic Outlook | Library / Applications / Office | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `apps.office-pdfmaker` | Acrobat Office add-ins | Library / Applications / Office | Low | Low | Community | Registry | 8 | Admin | Windows 10/11 |
| `apps.office-vs-adaptors` | Visual Studio Office add-ins | Library / Applications / Office | Low | Low | Community | Registry | 4 | Admin | Windows 10/11 |
| `apps.office-tfc` | Team Foundation add-ins | Library / Applications / Office | Low | Low | Community | Registry | 4 | Admin | Windows 10/11 |
| `apps.onedrive-sync` | OneDrive sync | Library / Applications / Other apps | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.onedrive-save` | Save to OneDrive default | Library / Applications / Other apps | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.onedrive-metered` | OneDrive on metered | Library / Applications / Other apps | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.onedrive-personal` | OneDrive personal sync | Library / Applications / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.onedrive-startup` | OneDrive at startup | Library / Applications / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.spotify-startup` | Spotify at startup | Library / Applications / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.steam-startup` | Steam at startup | Library / Applications / Other apps | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `apps.store-auto-update` | Store app auto updates | Library / Applications / Other apps | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `apps.store-remove` | Microsoft Store | Library / Applications / Other apps | High | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.adobe-optout` | Adobe telemetry | Library / Applications / Other apps | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `apps.adobe-rum` | Adobe update manager | Library / Applications / Other apps | Low | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `apps.ccleaner-monitoring` | CCleaner monitoring | Library / Applications / Other apps | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `apps.ccleaner-updates` | CCleaner update checks | Library / Applications / Other apps | Low | Low | Community | Registry | 3 | None | Windows 10/11 |
| `apps.ccleaner-telemetry` | CCleaner data | Library / Applications / Other apps | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `apps.ccleaner-offers` | CCleaner offers | Library / Applications / Other apps | Low | Low | Community | Registry | 4 | None | Windows 10/11 |
| `apps.ccleaner-health` | CCleaner Health Check | Library / Applications / Other apps | Low | Low | Community | Registry | 3 | None | Windows 10/11 |
| `apps.app-installer-exp` | App Installer experiments | Library / Applications / Other apps | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `apps.app-installer-source` | App Installer sources | Library / Applications / Other apps | Low | Low | Documented | Registry | 5 | Admin | Windows 10/11 |
| `cpu.boost-mode` | Aggressive boost mode | Library / CPU / Power plan | Low | Medium | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.boost-policy` | Boost policy | Library / CPU / Power plan | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.max-state` | Maximum processor state | Library / CPU / Power plan | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.min-state` | Minimum processor state | Library / CPU / Power plan | Medium | Medium | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `cpu.idle-disable` | Processor idle disable | Library / CPU / Power plan | High | Medium | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `cpu.cooling-active` | Active cooling policy | Library / CPU / Power plan | Low | Low | Documented | Power | 1 | Admin | Windows 10/11, desktopOnly |
| `cpu.epp` | Energy preference: performance | Library / CPU / Power plan | Low | Medium | Documented | Power | 1 | Admin | Windows 10/11, desktopOnly |
| `cpu.parking-max` | Core parking max cores | Library / CPU / Power plan | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.parking-min` | Core parking min cores | Library / CPU / Power plan | Low | Medium | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `cpu.idle-demote` | Idle demote threshold | Library / CPU / Power plan | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.interrupt-steering` | Interrupt steering mode | Library / CPU / Power plan | Medium | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `cpu.dpc-watchdog` | DPC watchdog period | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.dpc-timeout` | DPC timeout | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.dpc-watchdog-offset` | DPC watchdog offset | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.ideal-dpc-rate` | Ideal DPC rate | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.minimum-dpc-rate` | Minimum DPC rate | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.threaded-dpc` | Threaded DPCs | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.global-timer` | Global timer requests | Library / CPU / Scheduler and timers | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.serialize-timer` | Serialize timer expiration | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.low-qos-timer` | Low QoS timer resolution | Library / CPU / Scheduler and timers | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.ready-queue` | Shared ready queue size | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.dynamic-tick-duration` | Dynamic tick duration | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.per-cpu-clock` | Per-CPU clock ticks | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.timer-check` | Timer check flags | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.debug-poll` | Debug poll interval | Library / CPU / Scheduler and timers | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.coalescing` | Timer coalescing | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 7 | Admin | Windows 10/11 |
| `cpu.split-large-caches` | Split large caches | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.adjust-dpc` | DPC threshold | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.deep-io-coalescing` | Deep I/O coalescing | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.scheduler-assist` | Scheduler assist override | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.priority-dpc` | Ideal DPC rate (priority key) | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.passive-int` | Passive interrupt priority | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.wpbt` | Platform binary table | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.tick-power` | Exit latency check | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.exit-latency` | Exit latency | Library / CPU / Scheduler and timers | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.unpark-count` | Initial unpark count | Library / CPU / Scheduler and timers | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.energy-estimation` | Energy estimation | Library / CPU / Scheduler and timers | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.event-processor` | Power event processor | Library / CPU / Scheduler and timers | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `cpu.tagged-energy` | Tagged energy logging | Library / CPU / Scheduler and timers | Low | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `gaming.mouse-queue` | Mouse data queue | Library / Gaming / Input | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.keyboard-queue` | Keyboard data queue | Library / Gaming / Input | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gaming.mouse-hover` | Mouse hover time | Library / Gaming / Input | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.mouse-sensitivity` | Mouse sensitivity | Library / Gaming / Input | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.mouse-trails` | Mouse trails | Library / Gaming / Input | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.filter-keys` | Filter Keys | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.keyboard-response` | Keyboard response | Library / Gaming / Input | Medium | Low | Found in Windows | Registry | 8 | None | Windows 10/11 |
| `gaming.mouse-keys` | Mouse Keys shortcut | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.toggle-keys` | Toggle Keys shortcut | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.num-lock` | Num Lock at sign-in | Library / Gaming / Input | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.language-hotkey` | Language switch hotkey | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 3 | None | Windows 10/11 |
| `gaming.touch-pointer` | Absolute pointer | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.print-screen` | Print Screen opens Snipping | Library / Gaming / Input | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.fse-behavior` | Fullscreen optimisations | Library / Gaming / Game Bar and capture | Low | Medium | Found in Windows | Registry | 4 | None | Windows 10/11 |
| `gaming.game-mode-auto` | Automatic Game Mode | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.game-bar-controller` | Game Bar controller button | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.game-bar-link` | Game Bar links | Library / Gaming / Game Bar and capture | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `gaming.audio-capture` | Game audio capture | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.cursor-capture` | Game cursor capture | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.mic-capture` | Game microphone capture | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.audio-ducking` | Communications ducking | Library / Gaming / Audio | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.mmcss-audio` | MMCSS audio priority | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 5 | Admin | Windows 10/11 |
| `gaming.mmcss-pro-audio` | MMCSS pro audio | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.mmcss-display` | MMCSS display | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.mmcss-games-extra` | MMCSS games extras | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `gaming.mmcss-lazy` | MMCSS lazy mode | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.audiodg-priority` | Audio engine priority | Library / Gaming / Audio | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.nahimic-none` | Realtek power management | Library / Gaming / Audio | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.cs2-priority` | CS2 high priority | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.fortnite-priority` | Fortnite high priority | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gaming.fortnite-be-priority` | Fortnite BattlEye priority | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.fortnite-eac-priority` | Fortnite EAC priority | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.roblox-priority` | Roblox high priority | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.obs-priority` | OBS above normal | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.obs32-priority` | OBS 32-bit above normal | Library / Gaming / Per-game priority | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gaming.dwm-priority` | Desktop compositor priority | Library / Gaming / Per-game priority | Medium | Low | Found in Windows | Registry | 4 | Admin | Windows 10/11 |
| `gaming.csrss-priority` | Client server priority | Library / Gaming / Per-game priority | Medium | Low | Found in Windows | Registry | 4 | Admin | Windows 10/11 |
| `gaming.cod-vsync` | Modern Warfare V-Sync off | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.cod-vram` | Modern Warfare video memory | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.fortnite-dx` | Fortnite prefers D3D11 | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.fortnite-low` | Fortnite low quality | Library / Gaming / Game settings | Low | Low | Community | Registry | 6 | None | Windows 10/11 |
| `gaming.steam-overlay` | Steam overlay | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-gpu-web` | Steam web GPU acceleration | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-h264` | Steam H.264 acceleration | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-smooth` | Steam smooth scrolling | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-dwrite` | Steam DirectWrite | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-dpi` | Steam DPI scaling | Library / Gaming / Game settings | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-silent` | Steam silent start | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-updates` | Steam auto update | Library / Gaming / Game settings | Medium | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-startup` | Steam startup mode | Library / Gaming / Game settings | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gaming.steam-tcp` | Steam TCP settings | Library / Gaming / Game settings | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.riot-updates` | Riot auto update | Library / Gaming / Game settings | Medium | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gaming.game-dvr-user` | Background recording | Library / Gaming / Game Bar and capture | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `gaming.mmcss-sfio` | Games I/O priority | Library / Gaming / Game settings | Low | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `gpu.tdr-delay` | GPU timeout | Library / GPU / Windows graphics | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `gpu.tdr-ddi` | GPU DDI timeout | Library / GPU / Windows graphics | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `gpu.overlays` | Hardware overlays | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.miracast` | Miracast support | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.preemption` | GPU preemption | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.vsync-idle` | V-Sync idle timeout | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.monitor-latency` | Monitor latency tolerance | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 3 | Admin, Restart | Windows 10/11 |
| `gpu.monitor-refresh-latency` | Refresh latency tolerance | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 3 | Admin, Restart | Windows 10/11 |
| `gpu.tdr-async` | TDR async reset | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.dwm-mode-animation` | Mode change animation | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-independent-flip` | Independent flip | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-advanced-flip` | Advanced direct flip | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-prediction` | Interaction prediction | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-peek` | Aero Peek | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gpu.dwm-hibernate` | Thumbnail hibernation | Library / GPU / Windows graphics | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gpu.dwm-locking` | DWM memory locking | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-shadows` | DWM projected shadows | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gpu.dwm-hologram` | Holographic compositor | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-drawlist` | Draw list caching | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-effect-cache` | Effect caching | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-megarects` | Mega rects | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-cpu-clipping` | CPU clipping | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-parallel` | Parallel mode policy | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-linear` | Linear resampling | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `gpu.dwm-wet-ink` | Super wet ink | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-primitive` | Primitive reordering | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-debug` | DWM debug fail fast | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.dwm-shift` | Animations shift key | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.wpf-hw` | WPF hardware acceleration | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gpu.colour-calibration` | Calibration loader | Library / GPU / Windows graphics | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `gpu.gamma-range` | GDI gamma range | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `gpu.tiff-vk-debug` | Vulkan loader debug | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `gpu.nv-pstate` | Dynamic P-states | Library / GPU / NVIDIA | Medium | High | Found in Windows | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, nvidiaGpu |
| `gpu.nv-powermizer` | PowerMizer | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, desktopOnly, nvidiaGpu |
| `gpu.nv-perf-level` | Performance level source | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-power-saving` | Driver power saving | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-display-power` | Display power saving | Library / GPU / NVIDIA | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-link-state` | Max link state | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, desktopOnly, nvidiaGpu |
| `gpu.nv-dynamic-pstate` | Dynamic P-state switch | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, desktopOnly, nvidiaGpu |
| `gpu.nv-low-latency` | Low latency boost | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-msi` | NVIDIA MSI | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-dma` | DMA acceleration | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-async-queue` | Async command queue | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-p2p` | Peer-to-peer transfers | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-instrumentation` | Driver instrumentation | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-event-tracing` | Kernel event tracing | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-event-driven` | Event-driven mode | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-polling` | Completion polling | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-thread-policy` | Thread policy | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-telemetry-driver` | Driver telemetry | Library / GPU / NVIDIA | Low | Low | Community | Registry | 4 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-profile-telemetry` | Profile telemetry | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-cpl-optin` | Control Panel feedback | Library / GPU / NVIDIA | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-tray` | NVIDIA tray | Library / GPU / NVIDIA | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11, nvidiaGpu |
| `gpu.nv-ansel` | Ansel | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-freestyle` | Freestyle | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-rebar` | Resizable BAR flag | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-camera` | Nvfcamera | Library / GPU / NVIDIA | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-perf-sdk` | Performance SDK | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-stereo` | Stereo 3D | Library / GPU / NVIDIA | Low | Low | Community | Registry | 2 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-vsr` | RTX Video Super Resolution | Library / GPU / NVIDIA | Low | Low | Community | Registry | 4 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-gpudebugger` | GPU debugger | Library / GPU / NVIDIA | Low | Low | Community | Registry | 4 | Admin | Windows 10/11, nvidiaGpu |
| `gpu.nv-rm-cache` | RM registry caching | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-l2-compression` | L2 compression | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-per-core-dpc` | Per-core DPC | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 5 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-preemption-compute` | Compute preemption | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-preemption` | Driver preemption | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-preemption-cuda` | CUDA preemption | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-preemption-s3` | Preemption on sleep | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-preemption-level` | Preemption level | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-context-isolation` | Context isolation | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-reflex-driver` | Driver low latency | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-async-compute` | Async compute | Library / GPU / NVIDIA | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-frame-reporting` | Frame time reporting | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-debug-heap` | GPU debug heap | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-debug-layer` | Driver debug layer | Library / GPU / NVIDIA | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-logging` | Driver logging | Library / GPU / NVIDIA | Low | Low | Community | Registry | 11 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-hda-d3cold` | HD Audio D3 cold | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-contiguous` | Contiguous system memory | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-tdr` | NVIDIA TDR values | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-hdcp` | HDCP key | Library / GPU / NVIDIA | Medium | Low | Found in Windows | GPU | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.amd-ulps` | Ultra low power state | Library / GPU / AMD | Medium | Medium | Community | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, amdGpu |
| `gpu.amd-power-gating` | Power gating | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, amdGpu |
| `gpu.amd-dmacopy` | DMA copy | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-block-write` | Block write | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-uvd-gating` | UVD power gating | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-vce-gating` | VCE power gating | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-drmdma-gating` | DRM DMA gating | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-sclk-sleep` | Engine clock deep sleep | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, amdGpu |
| `gpu.amd-thermal-throttle` | Thermal auto throttling | Library / GPU / AMD | High | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, amdGpu |
| `gpu.amd-gpu-down` | GPU power down | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, desktopOnly, amdGpu |
| `gpu.amd-stutter` | Stutter mode | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-color-depth` | Auto colour depth | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-rs-overlay` | Radeon overlay | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-skins` | Radeon skins | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-snapshot` | Radeon snapshots | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-subscription` | Radeon subscription | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-cross-gpu` | Cross-GPU display | Library / GPU / AMD | Low | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-usage-tracking` | AMD usage tracking | Library / GPU / AMD | Low | Low | Community | Registry | 4 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-auto-update` | AMD auto update | Library / GPU / AMD | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-dvr` | AMD recording | Library / GPU / AMD | Low | Low | Community | Registry | 2 | None | Windows 10/11, amdGpu |
| `gpu.amd-chill` | Radeon Chill | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-rsr` | Radeon Super Resolution | Library / GPU / AMD | Low | Low | Community | Registry | 3 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-frtc` | Frame rate target | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-boost-cn` | Core Performance Boost | Library / GPU / AMD | Medium | Low | Community | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-power-scheme` | AMD power scheme | Library / GPU / AMD | Medium | Low | Community | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-fsr` | FidelityFX flag | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-vrs` | Variable rate shading flag | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.amd-apu-segment` | APU shared segment | Library / GPU / AMD | Medium | Low | Community | Registry | 2 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.intel-preemption` | Intel preemption | Library / GPU / Intel | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.intel-msaa` | Intel MSAA | Library / GPU / Intel | Low | Low | Community | Registry | 2 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.intel-raytracing` | Intel ray tracing | Library / GPU / Intel | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.intel-scaling` | Intel dynamic scaling | Library / GPU / Intel | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.intel-sr` | Intel super resolution | Library / GPU / Intel | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.intel-xess` | Intel XeSS | Library / GPU / Intel | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, intelGpu |
| `gpu.hags` | Hardware GPU scheduling | Library / GPU / Windows graphics | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `gpu.amd-ltr` | Latency tolerance | Library / GPU / AMD | Medium | Low | Community | GPU | 4 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-bgm-ltr` | Background latency tolerance | Library / GPU / AMD | Medium | Low | Community | GPU | 6 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-dal-latency` | Display latency | Library / GPU / AMD | Medium | Low | Community | GPU | 2 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-pp-latency` | PowerPlay latency | Library / GPU / AMD | Medium | Low | Community | GPU | 3 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-compute-latency` | Compute latency | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-memclock-latency` | Memory clock switch | Library / GPU / AMD | Medium | Low | Community | GPU | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-vsr` | Virtual Super Resolution | Library / GPU / AMD | Low | Low | Community | Registry | 2 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-dxr` | Ray tracing support | Library / GPU / AMD | Low | Low | Community | Registry | 2 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-aa-flags` | Anti-aliasing flags | Library / GPU / AMD | Low | Low | Community | Registry | 3 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-dynamic-res` | Dynamic resolution | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-px-video` | Video on integrated | Library / GPU / AMD | Low | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11, amdGpu |
| `gpu.amd-thermal-cn` | CN thermal limit | Library / GPU / AMD | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11, amdGpu |
| `gpu.nv-gr535` | GR535 flag | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-rid-flags` | RID feature flags | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-param-flags` | Driver parameter flags | Library / GPU / NVIDIA | Medium | Low | Found in Windows | Registry | 4 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.nv-scaling-flags` | Scaling flags | Library / GPU / NVIDIA | Low | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11, nvidiaGpu |
| `gpu.d3d-flip-no-vsync` | Direct3D 9 flip | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 4 | Admin | Windows 10/11 |
| `gpu.ddraw-nonlocal` | Non-local video memory | Library / GPU / Windows graphics | Low | Low | Found in Windows | Registry | 8 | Admin | Windows 10/11 |
| `memory.large-system-cache` | Large system cache | Library / Memory / Paging and cache | Medium | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `memory.page-combining` | Memory page combining | Library / Memory / Paging and cache | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.clear-pagefile` | Keep page file at shutdown | Library / Memory / Paging and cache | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.svchost-split` | Service grouping threshold | Library / Memory / Paging and cache | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.prefetcher` | Prefetcher | Library / Memory / Prefetch | Medium | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.superfetch` | Superfetch parameter | Library / Memory / Prefetch | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.sf-tracing` | Superfetch tracing | Library / Memory / Prefetch | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `memory.compat-ndu` | Network Data Usage driver | Library / Memory / Prefetch | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.ack-frequency` | TCP ACK frequency | Library / Network / TCP | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.del-ack` | Delayed ACK ticks | Library / Network / TCP | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.no-delay` | TCP no delay | Library / Network / TCP | Low | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.ttl` | Default TTL | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.timed-wait` | TIME_WAIT delay | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.max-user-port` | Dynamic port limit | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.tcp1323` | TCP window scaling | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.max-retrans` | Data retransmissions | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.max-data-retrans` | Retransmission limit | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.connect-retrans` | Connect retransmissions | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dup-acks` | Duplicate ACKs | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.rto` | Initial retransmission time | Library / Network / TCP | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.urgent` | RFC 1122 urgent pointer | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.pmtu` | Path MTU discovery | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.pmtu-bh` | Black hole detection | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dead-gw` | Dead gateway detection | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dca` | Direct cache access | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.task-offload` | Task offload | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.free-tcbs` | TCP control blocks | Library / Network / TCP | Medium | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `network.hash-table` | TCP hash table | Library / Network / TCP | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.irp-stack` | Network IRP stack | Library / Network / TCP | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `network.forward-buffers` | Forward buffer memory | Library / Network / TCP | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `network.forward-packets` | Forward packets | Library / Network / TCP | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `network.mcast-forward` | Multicast forwarding | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.igmp` | IGMP level | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.media-sense-log` | Media sense log | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.source-routing` | IP source routing | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.name-release` | NetBIOS name release | Library / Network / TCP | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.arp-retry` | ARP retries | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.arp-single` | ARP single route | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.arp-snap` | ARP SNAP | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.afd-receive` | Default receive window | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.afd-send` | Default send window | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.provider-dns` | DNS priority | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.provider-hosts` | Hosts priority | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.provider-local` | Local priority | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.provider-netbt` | NetBT priority | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.lmhosts` | LMHOSTS lookup | Library / Network / Winsock and resolution | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.rss-base` | RSS base CPU | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.winhttp-wpad` | WPAD | Library / Network / Winsock and resolution | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.connections-1` | HTTP/1.0 connections | Library / Network / Winsock and resolution | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `network.connections` | HTTP connections | Library / Network / Winsock and resolution | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `network.doh` | DNS over HTTPS | Library / Network / Winsock and resolution | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.neg-cache-policy` | Negative DNS cache (policy) | Library / Network / Winsock and resolution | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-table` | DNS cache table | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-bucket` | DNS cache bucket | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-max-ttl` | DNS maximum TTL | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-neg-ttl` | DNS negative TTL | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-soa-ttl` | DNS SOA TTL | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-neg-time` | Negative cache time | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-net-failure` | Network failure cache | Library / Network / DNS cache | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-timeout` | DNS cache timeout | Library / Network / DNS cache | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-max-size` | DNS cache size | Library / Network / DNS cache | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.dns-netbios` | NetBIOS in DNS Client | Library / Network / DNS cache | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-oplocks` | SMB oplocks | Library / Network / SMB | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-signing-enable` | SMB signing | Library / Network / SMB | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-signing-require` | SMB signing required | Library / Network / SMB | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-autodisconnect` | SMB auto disconnect | Library / Network / SMB | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-admin-shares-server` | Admin shares | Library / Network / SMB | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `network.smb1` | SMB 1 | Library / Network / SMB | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-size` | Server size | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-sharing-delay` | Sharing violation delay | Library / Network / SMB | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `network.null-sessions` | Null session access | Library / Network / SMB | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-dir-cache` | SMB directory cache | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-file-cache` | SMB file info cache | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-large-mtu` | SMB large MTU | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-keepconn` | SMB keep connection | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.smb-max-cmds` | SMB max commands | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.qos-valorant-launcher` | VALORANT launcher QoS tag | Library / Network / QoS | Low | Low | Found in Windows | Registry | 11 | Admin | Windows 10/11, desktopOnly |
| `network.psched-sends` | Packet scheduler sends | Library / Network / QoS | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.psched-timer` | Packet scheduler timer | Library / Network / QoS | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.wifi-sense` | Wi-Fi Sense | Library / Network / Discovery and Wi-Fi | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `network.hotspot-login` | Hotspot auto login | Library / Network / Discovery and Wi-Fi | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.wcn` | Windows Connect Now | Library / Network / Discovery and Wi-Fi | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `network.lm-hosts-wsd` | WSD in TCP/IP | Library / Network / Discovery and Wi-Fi | Low | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `network.teredo-mdns` | Local name providers | Library / Network / Discovery and Wi-Fi | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.metered-device` | Costed network policy | Library / Network / Discovery and Wi-Fi | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.minimise-connections` | Minimise connections | Library / Network / Discovery and Wi-Fi | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `network.ndiscap-note` | Native Wi-Fi filter | Library / Network / Discovery and Wi-Fi | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.vwifi` | Virtual Wi-Fi filter | Library / Network / Discovery and Wi-Fi | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.ndisvirtual` | NDIS Virtual Bus | Library / Network / Discovery and Wi-Fi | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.netbios-driver` | NetBIOS driver | Library / Network / Discovery and Wi-Fi | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `network.throttling-index` | Network throttling | Library / Network / TCP | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.smb-client-threads` | SMB client threads | Library / Network / SMB | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `network.qos-reserve` | QoS bandwidth reserve | Library / Network / QoS | Low | Low | Documented | Registry | 2 | Admin, Restart | Windows 10/11 |
| `network.do-machine` | Delivery Optimization mode | Library / Network / Discovery and Wi-Fi | Low | Low | Community | Registry | 2 | Admin | Windows 10/11 |
| `power.disk-idle` | Turn off hard disk after | Library / Power / Disk | Low | Medium | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.ahci-link` | AHCI link power | Library / Power / Disk | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.nvme-noppme` | NVMe NOPPME | Library / Power / Disk | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.nvme-primary` | NVMe primary idle timeout | Library / Power / Disk | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.nvme-secondary` | NVMe secondary idle timeout | Library / Power / Disk | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.usb-selective` | USB selective suspend (plan) | Library / Power / USB and PCIe | Low | Medium | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.usb3-link` | USB 3 link power | Library / Power / USB and PCIe | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.usb-ioc` | USB IOC on all TDs | Library / Power / USB and PCIe | Medium | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.pcie-aspm` | PCIe link power | Library / Power / USB and PCIe | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.sleep` | Never sleep | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.hibernate` | Never hibernate | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.away-mode` | Away mode | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.standby-network` | Network in standby | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.device-idle` | Device idle policy | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.personality` | Plan personality | Library / Power / Sleep and buttons | Medium | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.button` | Power button does nothing | Library / Power / Sleep and buttons | Medium | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.lid` | Lid close when plugged in | Library / Power / Sleep and buttons | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, laptopOnly |
| `power.display` | Display never turns off | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, desktopOnly |
| `power.brightness` | Full display brightness | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, laptopOnly |
| `power.dim-brightness` | No dimming | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, laptopOnly |
| `power.adaptive-brightness` | Adaptive brightness | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, laptopOnly |
| `power.gpu-preference` | GPU preference policy | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.intel-graphics` | Intel graphics performance | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11, intelGpu |
| `power.timer-frequency` | JavaScript timer frequency | Library / Power / Display and devices | Low | Low | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.wifi-power` | Wi-Fi power saving | Library / Power / Display and devices | Low | Medium | Documented | Power | 2 | Admin | Windows 10/11 |
| `power.platform-aoac` | Modern Standby override | Library / Power / System | Medium | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.cs-enabled` | Connected standby | Library / Power / System | Medium | Low | Community | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.fast-startup` | Fast startup | Library / Power / System | Low | Medium | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `power.startup-sound` | Startup sound | Library / Power / System | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `power.custom-setup` | Plan customisation at setup | Library / Power / System | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `power.platform-role` | Platform role | Library / Power / System | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11, desktopOnly |
| `privacy.commercial-pipeline` | Commercial data pipeline | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.desktop-analytics` | Desktop Analytics | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.device-name` | Device name in telemetry | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.update-compliance` | Update Compliance data | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.wufb-cloud` | WUfB cloud processing | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.onesettings` | OneSettings downloads | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.feedback-notifications` | Feedback notifications | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.limit-enhanced` | Enhanced diagnostic limit | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.edge-data-optin` | Edge data in telemetry | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.ceip` | Customer Experience Program | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.ait` | Application telemetry | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.inventory` | Application inventory | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.steps-recorder` | Steps Recorder | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.pca` | Program Compatibility Assistant | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.compat-engine` | Application compatibility engine | Library / Privacy / Telemetry | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.compat-tab` | Compatibility tab | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.mrt-report` | Malicious tool reports | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.error-report-policy` | Error reporting | Library / Privacy / Telemetry | Low | Medium | Documented | Registry | 5 | Admin | Windows 10/11 |
| `privacy.error-extra` | Additional crash data | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.error-ui` | Crash report prompts | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.error-logging` | Error report logging | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.error-consent` | Error report consent | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.handwriting-reports` | Handwriting error reports | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.handwriting-data` | Handwriting data sharing | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.ie-sqm` | Internet Explorer CEIP | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.ie-logging` | InPrivate logging | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.messenger-ceip` | Messenger CEIP | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.media-player-usage` | Media Player usage tracking | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `privacy.toast-diagtrack` | Diagnostic prompt | Library / Privacy / Telemetry | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.experimentation` | Windows experiments | Library / Privacy / Telemetry | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.autologger-diagtrack` | Diagtrack listener | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `privacy.autologger-diaglog` | DiagLog trace | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `privacy.autologger-sqm` | SQM trace | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `privacy.autologger-wifi` | Wi-Fi session trace | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `privacy.dotnet-telemetry` | .NET CLI telemetry | Library / Privacy / Telemetry | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.powershell-telemetry` | PowerShell telemetry | Library / Privacy / Telemetry | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.consumer-features` | Consumer features | Library / Privacy / Advertising and suggestions | Low | Medium | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.cloud-optimised` | Cloud-optimised content | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.soft-landing` | Tips and tricks | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.tailored-policy` | Tailored experiences | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.spotlight` | Windows Spotlight | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.spotlight-action` | Spotlight on Action Center | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.spotlight-lock` | Spotlight on lock screen | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.spotlight-settings` | Spotlight in Settings | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.spotlight-welcome` | Welcome experience | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.cdm-preinstalled` | Preinstalled apps | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 3 | None | Windows 10/11 |
| `privacy.cdm-silent` | Silent app installs | Library / Privacy / Advertising and suggestions | Low | Medium | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-lockscreen` | Lock screen tips | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 3 | None | Windows 10/11 |
| `privacy.cdm-settings` | Suggestions in Settings | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 3 | None | Windows 10/11 |
| `privacy.cdm-tips` | Tips while you use Windows | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-start` | Suggestions in Start | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.cdm-welcome` | Welcome suggestions | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-lock-fun` | Fun facts | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-mru` | Spotlight history | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-feature-mgmt` | Feature management | Library / Privacy / Advertising and suggestions | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-content` | Content delivery | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-slideshow` | Content slideshow | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.cdm-rest` | Other suggestion channels | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.lock-creative` | Lock screen creative | Library / Privacy / Advertising and suggestions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.online-tips` | Online tips | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.meet-now` | Meet Now | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.news-interests` | News and interests | Library / Privacy / Advertising and suggestions | Low | Medium | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.copilot-policy` | Windows Copilot | Library / Privacy / Copilot and AI | Low | Medium | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.copilot-large` | Copilot on large screens | Library / Privacy / Copilot and AI | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.recall` | Recall | Library / Privacy / Copilot and AI | Low | Medium | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.click-to-do` | Click to Do | Library / Privacy / Copilot and AI | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.paint-cocreator` | Paint Cocreator | Library / Privacy / Copilot and AI | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.paint-fill` | Paint generative fill | Library / Privacy / Copilot and AI | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.paint-creator` | Paint Image Creator | Library / Privacy / Copilot and AI | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.video-ai` | Video AI enhancement | Library / Privacy / Copilot and AI | Low | Low | Community | Registry | 4 | Admin | Windows 10/11 |
| `privacy.web-search` | Web results in Search | Library / Privacy / Search | Low | Medium | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.web-search-metered` | Web search on metered | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.search-privacy` | Search privacy level | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.search-location` | Search location | Library / Privacy / Search | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.cortana` | Cortana | Library / Privacy / Search | Low | Low | Documented | Registry | 8 | Admin | Windows 10/11 |
| `privacy.cortana-lock` | Cortana on the lock screen | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.cloud-search` | Cloud search | Library / Privacy / Search | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.search-history` | Search history | Library / Privacy / Search | Low | Low | Documented | Registry | 4 | Admin | Windows 10/11 |
| `privacy.search-highlights` | Search highlights | Library / Privacy / Search | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.search-box-suggestions` | Search box suggestions | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.safe-search` | SafeSearch | Library / Privacy / Search | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.search-background` | Search background toggle | Library / Privacy / Search | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.search-disable` | Search feature | Library / Privacy / Search | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.remote-queries` | Remote search queries | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.indexer-backoff` | Indexer back-off | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-encrypted` | Index encrypted files | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-language` | Language detection | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-removable` | Index removable drives | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-attachments` | Index email attachments | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-outlook` | Index Outlook | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.index-public` | Index public folders | Library / Privacy / Search | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.settings-sync` | Settings sync | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-accessibility` | Sync accessibility | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-apps` | Sync apps | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-browser` | Sync browser | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-credentials` | Sync passwords | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-theme` | Sync theme | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-language` | Sync language | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-packages` | Sync package state | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-personal` | Sync personalisation | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-start` | Sync Start layout | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.sync-windows` | Sync Windows settings | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.clipboard-history` | Clipboard history | Library / Privacy / Sync and cross-device | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.clipboard-cloud` | Cloud clipboard | Library / Privacy / Sync and cross-device | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.cross-device` | Cross-device experiences | Library / Privacy / Sync and cross-device | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.cross-resume` | Cross-device resume | Library / Privacy / Sync and cross-device | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.cdp` | Shared experiences | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.nearby` | Nearby sharing | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.mobility` | Phone Link suggestion | Library / Privacy / Sync and cross-device | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.find-my-device` | Find My Device | Library / Privacy / Sync and cross-device | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.ms-account` | Microsoft accounts | Library / Privacy / Sync and cross-device | High | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.device-registration` | Device registration | Library / Privacy / Sync and cross-device | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.mdm-enrol` | Automatic MDM enrolment | Library / Privacy / Sync and cross-device | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-activity` | Motion and activity | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-diagnostics` | App diagnostics | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-appointments` | Calendar | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-bg-spatial` | Background spatial | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-bluetooth` | Bluetooth access | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-bluetooth-sync` | Bluetooth sync | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-files` | Full file system access | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-chat` | Messaging | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-contacts` | Contacts | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-documents` | Documents | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-downloads` | Downloads | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-email` | Email | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-gaze` | Eye tracking | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-capture` | Screen capture | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `privacy.perm-capture-border` | Borderless capture | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-hid` | Human interface devices | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-presence` | Presence sensing | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-microphone` | Microphone | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 3 | Admin | Windows 10/11 |
| `privacy.perm-music` | Music | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-phone` | Phone calls | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-phone-history` | Call history | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-pictures` | Pictures | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-radios` | Radios | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-serial` | Serial devices | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-spatial` | Spatial perception | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.perm-tasks` | Tasks | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-notifications` | Notification access | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-videos` | Videos | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-camera` | Camera | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perm-wifi` | Wi-Fi data | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.camera-policy` | Camera policy | Library / Privacy / App permissions | High | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.camera-effects` | Camera effects | Library / Privacy / App permissions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.lock-camera` | Lock screen camera | Library / Privacy / App permissions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.location-policy` | Location policy | Library / Privacy / App permissions | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.sensors-policy` | Sensors policy | Library / Privacy / App permissions | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.ie-geolocation` | IE geolocation | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.sensor-permission` | Location sensor override | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.background-global` | Background apps (user) | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.push-policy` | Cloud notifications policy | Library / Privacy / App permissions | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.input-personalization` | Online speech policy | Library / Privacy / Speech and typing | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.ink-collection` | Ink collection | Library / Privacy / Speech and typing | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.harvest-contacts` | Contact harvesting | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.privacy-policy-input` | Inking privacy flag | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.speech-model` | Speech model updates | Library / Privacy / Speech and typing | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.voice-activation` | Voice activation | Library / Privacy / Speech and typing | Low | Low | Community | Registry | 3 | Admin | Windows 10/11 |
| `privacy.voice-lock` | Voice on the lock screen | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.enhanced-audio` | Enhanced speech audio | Library / Privacy / Speech and typing | Low | Low | Community | Registry | 2 | Admin | Windows 10/11 |
| `privacy.voice-typing` | Voice typing | Library / Privacy / Speech and typing | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.typing-insights` | Typing insights | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.text-prediction` | Hardware keyboard prediction | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.tipc` | Inking and typing data | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.autocorrect` | Touch keyboard autocorrect | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.spellcheck-touch` | Touch keyboard spelling | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.touch-auto` | Touch keyboard auto-show | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.touch-button` | Touch keyboard button | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.touch-keyboard` | Touch keyboard policy | Library / Privacy / Speech and typing | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.input-preload` | Input app preload | Library / Privacy / Speech and typing | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.ink-workspace` | Windows Ink Workspace | Library / Privacy / Speech and typing | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.snipping-tool` | Snipping Tool policy | Library / Privacy / Speech and typing | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.cpss-advertising` | Advertising setting store | Library / Privacy / System reports | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `privacy.cpss-ink` | Inking store | Library / Privacy / System reports | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `privacy.push-to-install` | PushToInstall policy | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.kms-ticket` | KMS online validation | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.map-traffic` | Maps traffic | Library / Privacy / System reports | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.media-player-codecs` | Media Player codec download | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.wmdrm` | WMDRM online | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.device-metadata` | Device metadata | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.driver-search` | Driver search | Library / Privacy / System reports | Medium | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.exclude-drivers` | Driver updates in Windows Update | Library / Privacy / System reports | Medium | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.scheduled-diagnostics` | Scheduled diagnostics | Library / Privacy / System reports | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.online-troubleshooters` | Online troubleshooters | Library / Privacy / System reports | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `privacy.perftrack` | PerfTrack | Library / Privacy / System reports | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.vs-feedback` | Visual Studio feedback | Library / Privacy / System reports | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.vs-intellicode` | IntelliCode remote analysis | Library / Privacy / System reports | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `privacy.vs-sqm` | Visual Studio CEIP | Library / Privacy / System reports | Low | Low | Documented | Registry | 9 | Admin | Windows 10/11 |
| `privacy.vs-telemetry` | Visual Studio telemetry | Library / Privacy / System reports | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `privacy.vscode-telemetry` | VS Code telemetry flags | Library / Privacy / System reports | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `privacy.ad-id-policy` | Advertising ID policy | Library / Privacy / Advertising and suggestions | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.mic-policy` | Microphone for apps | Library / Privacy / App permissions | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `privacy.location-consent` | Location consent | Library / Privacy / App permissions | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.account-consent` | Account info consent | Library / Privacy / App permissions | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `privacy.telemetry-machine` | Telemetry level | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `privacy.steps-recorder-log` | Steps Recorder log | Library / Privacy / Telemetry | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `services.diagnostics-hub` | Diagnostics Hub Collector | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.diagsvc` | Diagnostic Execution | Library / Services / Telemetry and diagnostics | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.troubleshooting` | Recommended Troubleshooting | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wersvc` | Windows Error Reporting | Library / Services / Telemetry and diagnostics | Low | Medium | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wercpl` | Problem Reports Control Panel | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.pla` | Performance Logs and Alerts | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.perfhost` | Performance Counter Host | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wmiapsrv` | WMI Performance Adapter | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wecsvc` | Windows Event Collector | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.graphicsperf` | Graphics Performance Monitor | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.gpu-energy` | GPU Energy Driver | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | GPU | 1 | Admin | Windows 10/11 |
| `services.dusmsvc` | Data Usage | Library / Services / Telemetry and diagnostics | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.sysmon` | System Monitor Driver | Library / Services / Telemetry and diagnostics | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ndiscap` | NDIS Capture | Library / Services / Telemetry and diagnostics | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.xbox-net` | Xbox Live Networking | Library / Services / Xbox | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.xbox-gip` | Xbox Accessory Management | Library / Services / Xbox | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.game-input` | GameInput | Library / Services / Xbox | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.bcast-dvr` | Game Recording User Service | Library / Services / Xbox | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.capture` | Screen Capture Service | Library / Services / Xbox | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.bluetooth-user` | Bluetooth User Support | Library / Services / Bluetooth | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.btag` | Bluetooth Audio Gateway | Library / Services / Bluetooth | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.avctp` | AVCTP | Library / Services / Bluetooth | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.device-assoc` | Device Association | Library / Services / Bluetooth | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.device-assoc-broker` | Device Association Broker | Library / Services / Bluetooth | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.device-picker` | Device Picker | Library / Services / Bluetooth | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.devices-flow` | Devices Flow | Library / Services / Bluetooth | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.devquery` | DevQuery Background Discovery | Library / Services / Bluetooth | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.hvhost` | Hyper-V Host | Library / Services / Hyper-V | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-guest` | Hyper-V Guest Interface | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-heartbeat` | Hyper-V Heartbeat | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-kvp` | Hyper-V Data Exchange | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-rdv` | Hyper-V Remote Desktop | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-shutdown` | Hyper-V Guest Shutdown | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-time` | Hyper-V Time Sync | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-session` | Hyper-V PowerShell Direct | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vmic-vss` | Hyper-V Shadow Copy | Library / Services / Hyper-V | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.print-workflow` | Print Workflow | Library / Services / Printing and scanning | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.stisvc` | Windows Image Acquisition | Library / Services / Printing and scanning | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wiarpc` | Still Image Events | Library / Services / Printing and scanning | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fax` | Fax | Library / Services / Printing and scanning | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.phonesvc` | Phone Service | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.tapisrv` | Telephony | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.smsrouter` | SMS Router | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.messaging` | Messaging | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wwansvc` | WWAN AutoConfig | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.wlpasvc` | Local Profile Assistant | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.autotime` | Cellular Time | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.semgr` | Payments and NFC | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wallet` | Wallet | Library / Services / Phone and cellular | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fdphost` | Function Discovery Host | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fdrespub` | Function Discovery Publisher | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ssdp` | SSDP Discovery | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.upnphost` | UPnP Device Host | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lltdsvc` | Link-Layer Topology Discovery | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lmhosts` | TCP/IP NetBIOS Helper | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ncdautosetup` | Network Devices Auto-Setup | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wcncsvc` | Windows Connect Now | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wfds` | Wi-Fi Direct Connection Manager | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.qwave` | Quality Windows Audio Video | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.icssvc` | Mobile Hotspot | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.sharedaccess-note` | SNMP Trap | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.dot3svc` | Wired AutoConfig | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.eaphost` | Extensible Authentication | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ncb` | Network Connection Broker | Library / Services / Network discovery | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ncasvc` | Network Connectivity Assistant | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.termservice` | Remote Desktop Services | Library / Services / Remote access | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.session-env` | Remote Desktop Configuration | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.umrdp` | Remote Desktop Port Redirector | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.remote-registry` | Remote Registry | Library / Services / Remote access | Low | Medium | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.winrm` | Windows Remote Management | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.remote-access` | Routing and Remote Access | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.rpclocator` | RPC Locator | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.webclient` | WebClient | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lanman-server` | File and Printer Sharing | Library / Services / Remote access | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lanman-workstation` | SMB Client | Library / Services / Remote access | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.netlogon` | Netlogon | Library / Services / Remote access | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.seclogon` | Secondary Logon | Library / Services / Remote access | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ssh-agent` | OpenSSH Agent | Library / Services / Remote access | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.rasman` | Remote Access Connection Manager | Library / Services / VPN | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.rasauto` | Remote Access Auto Connection | Library / Services / VPN | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.sstp` | Secure Socket Tunneling | Library / Services / VPN | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ikeext` | IKE and AuthIP Keying | Library / Services / VPN | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.policy-agent` | IPsec Policy Agent | Library / Services / VPN | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.iphlpsvc` | IP Helper | Library / Services / VPN | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ipxlat` | IP Translation Config | Library / Services / VPN | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.sensor-data` | Sensor Data | Library / Services / Location and sensors | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.tzautoupdate` | Auto Time Zone Updater | Library / Services / Location and sensors | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.w32time` | Windows Time | Library / Services / Location and sensors | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.natural-auth` | Natural Authentication | Library / Services / Location and sensors | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.install-service` | Store Install Service | Library / Services / Store and apps | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.push-to-install` | PushToInstall | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.license-manager` | Windows License Manager | Library / Services / Store and apps | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.clipsvc` | Client License Service | Library / Services / Store and apps | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.app-readiness` | App Readiness | Library / Services / Store and apps | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.dsmsvc` | Device Setup Manager | Library / Services / Store and apps | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.app-mgmt` | Application Management | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.axinst` | ActiveX Installer | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.appv` | App-V Client | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ent-app` | Enterprise App Management | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.retail-demo` | Retail Demo | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wisvc` | Windows Insider | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lxp` | Language Experience | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.embedded-mode` | Embedded Mode | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.assigned-access` | Assigned Access | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.dialog-blocking` | Dialog Blocking | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.warp-jit` | WarpJIT | Library / Services / Store and apps | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.storsvc` | Storage Service | Library / Services / Store and apps | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.openxr` | Mixed Reality OpenXR | Library / Services / Mixed reality | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.perception-sim` | Perception Simulation | Library / Services / Mixed reality | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.spectrum` | Windows Perception | Library / Services / Mixed reality | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.shared-reality` | Spatial Data | Library / Services / Mixed reality | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vacsvc` | Volumetric Audio | Library / Services / Mixed reality | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.scardsvr` | Smart Card | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.scdevice` | Smart Card Enumeration | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.scpolicy` | Smart Card Removal Policy | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.certprop` | Certificate Propagation | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wbio` | Windows Biometric | Library / Services / Smart card and sign-in | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ngc-ctnr` | Passport Container | Library / Services / Smart card and sign-in | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ngc` | Microsoft Passport | Library / Services / Smart card and sign-in | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wlidsvc` | Microsoft Account Sign-in | Library / Services / Smart card and sign-in | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.cred-enroll` | Credential Enrollment | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.consent-ux` | ConsentUX | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.cloud-id` | Cloud Identity | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.dmenroll` | Device Management Enrollment | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.lpasvc-note` | Policy Platform Agent | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wepsvc` | Encryption Provider Host | Library / Services / Smart card and sign-in | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.sdrsvc` | Windows Backup | Library / Services / Backup and storage | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wbengine` | Block Level Backup | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fhsvc` | File History | Library / Services / Backup and storage | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vss` | Volume Shadow Copy | Library / Services / Backup and storage | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.swprv` | Shadow Copy Provider | Library / Services / Backup and storage | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.vds` | Virtual Disk | Library / Services / Backup and storage | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.defragsvc` | Optimize Drives | Library / Services / Backup and storage | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.smphost` | Storage Spaces SMP | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.tiering` | Storage Tiers | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.svsvc` | Spot Verifier | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.msiscsi` | iSCSI Initiator | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.csc` | Offline Files | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.workfolders` | Work Folders | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.file-sync-helper` | File Sync Helper | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.onedrive-updater` | OneDrive Updater | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.storqos` | Storage QoS Filter | Library / Services / Backup and storage | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.bindflt` | Bind Filter | Library / Services / Backup and storage | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.msdtc` | Distributed Transactions | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ktmrm` | Transaction Manager Bridge | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.comsysapp` | COM+ System Application | Library / Services / Enterprise | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.branchcache` | BranchCache | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.shared-pc` | Shared PC Accounts | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.uev` | User Experience Virtualization | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wman` | Windows Management Service | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.netsetup` | Network Setup | Library / Services / Enterprise | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.keyboard-filter` | Keyboard Filter | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.net-tcp-sharing` | Net.Tcp Port Sharing | Library / Services / Enterprise | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.p2pimsvc` | Peer Identity Manager | Library / Services / Peer networking | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.p2psvc` | Peer Grouping | Library / Services / Peer networking | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.pnrpauto` | PNRP Name Publication | Library / Services / Peer networking | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.pnrpsvc` | Peer Name Resolution | Library / Services / Peer networking | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wsearch` | Windows Search | Library / Services / Search and shell | Medium | High | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.shell-hw` | Shell Hardware Detection | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.themes` | Themes | Library / Services / Search and shell | High | Medium | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.text-input` | Text Input Management | Library / Services / Search and shell | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.tablet-input` | Touch Keyboard and Handwriting | Library / Services / Search and shell | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.hidserv` | Human Interface Devices | Library / Services / Search and shell | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fontcache` | Font Cache | Library / Services / Search and shell | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.fontcache3` | WPF Font Cache | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.disp-broker` | Display Policy | Library / Services / Search and shell | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.display-enhance` | Display Enhancement | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, desktopOnly |
| `services.sens` | System Event Notification | Library / Services / Search and shell | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.event-system` | COM+ Event System | Library / Services / Search and shell | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.biometric-mcp` | MCP Management | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.beep` | Beep Driver | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.cbdhsvc` | Clipboard User Service | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.cdp` | Connected Devices Platform | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.cdp-user` | Connected Devices User | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.dssvc` | Data Sharing | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.unistore` | User Data Storage | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.userdata` | User Data Access | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.pim-index` | Contact Indexing | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.aarsvc` | Agent Activation Runtime | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wpn` | Push Notifications | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wpn-user` | Push Notifications User | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wpc-mon` | Parental Controls | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.frameserver` | Camera Frame Server | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.frameserver-monitor` | Camera Frame Server Monitor | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wmp-network` | Media Player Sharing | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.wpd-bus` | Portable Devices | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.alg` | Application Layer Gateway | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.npsm` | Now Playing | Library / Services / Sync and account | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.udk` | Shell Experience Host User | Library / Services / Sync and account | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.amd-crash` | AMD Crash Defender | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, amdGpu |
| `services.amd-external` | AMD External Events | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, amdGpu |
| `services.amd-ppm` | AMD Processor Driver | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, amdCpu |
| `services.nv-profile-updater` | NVIDIA Profile Updater | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, nvidiaGpu |
| `services.nv-telemetry` | NVIDIA Telemetry | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, nvidiaGpu |
| `services.nv-container` | NVIDIA Display Container | Library / Services / Third-party | High | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, nvidiaGpu |
| `services.edge-update` | Edge Update | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.edge-updatem` | Edge Update Manual | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.edge-elevation` | Edge Elevation | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.nahimic` | Nahimic | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.rtk-audio` | Realtek Audio | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.rtk-universal` | Realtek Audio Universal | Library / Services / Third-party | Medium | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.waves` | Waves Audio | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.ajrouter` | AllJoyn Router | Library / Services / Network discovery | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.search-in-bing` | Microsoft Search in Bing | Library / Services / Search and shell | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11 |
| `services.amd-ryzen-master` | Ryzen Master driver | Library / Services / Third-party | Low | Low | Found in Windows | Service | 1 | Admin | Windows 10/11, amdCpu |
| `storage.long-paths` | Long file paths | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `storage.memoryusage` | NTFS memory usage | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.mft-zone` | MFT zone | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.no-efs` | NTFS encryption | Library / Storage / File system | Medium | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.no-paging-encryption` | Page file encryption | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.trim` | SSD TRIM | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `storage.boot-optimize` | Boot file optimisation | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `storage.autochk-timeout` | Disk check countdown | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.registry-backup` | Periodic registry backup | Library / Storage / File system | Low | Low | Documented | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.reserved` | Reserved storage | Library / Storage / File system | Medium | Low | Documented | Registry | 5 | Admin, Restart | Windows 10/11 |
| `storage.ahci-dipm` | AHCI device power saving | Library / Storage / Drives | Low | Medium | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.ahci-hipm` | AHCI host power saving | Library / Storage / Drives | Low | Medium | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.nvme-logging` | NVMe driver logging | Library / Storage / Drives | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `storage.nvme-queue` | NVMe queue depth | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 4 | Admin, Restart | Windows 10/11 |
| `storage.nvme-requests` | NVMe request limit | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-arbitration` | NVMe arbitration burst | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-queues` | NVMe submission queues | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-immediate` | NVMe immediate data | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-hmb` | NVMe host memory buffer | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-flush` | NVMe flush | Library / Storage / Drives | Medium | Low | Found in Windows | Registry | 2 | Admin, Restart | Windows 10/11 |
| `storage.nvme-timeout` | NVMe I/O timeout | Library / Storage / Drives | Low | Low | Found in Windows | Registry | 1 | Admin, Restart | Windows 10/11 |
| `tasks.compat-appraiser` | Compatibility Appraiser | Library / Tasks / Telemetry | Low | Medium | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.compat-appraiser-exp` | Compatibility Appraiser Exp | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ait-agent` | Application Impact Telemetry | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.mare-backup` | MareBackup | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.program-data-updater` | Program Data Updater | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.pca-patch` | PCA Patch Database | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.sdbinst-merge` | Compatibility Database Merge | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.startup-app` | Startup App Report | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-consolidator` | CEIP Consolidator | Library / Tasks / Telemetry | Low | Medium | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-usb` | USB CEIP | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-kernel` | Kernel CEIP | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-bth` | Bluetooth CEIP | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-uploader` | CEIP Uploader | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-server-assistant` | Server CEIP Assistant | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-server-role` | Server Role Collector | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ceip-server-usage` | Server Role Usage Collector | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.device-info` | Device Information | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.device-info-user` | Device Information User | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.siuf-dmclient` | Feedback Client | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.siuf-scenario` | Feedback Scenario Download | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.sqm` | Sqm Tasks | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.sustainability-telemetry` | Sustainability Telemetry | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.grid-forecast` | Power Grid Forecast | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.net-trace` | Gather Network Info | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.request-trace` | Performance Trace Request | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.queue-reporting` | Error Report Queue | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.error-details` | Error Details Update | Library / Tasks / Telemetry | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.flight-reconcile` | Reconcile Features | Library / Tasks / Flighting and experiments | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.flight-flush` | Feature Usage Flushing | Library / Tasks / Flighting and experiments | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.flight-receiver` | Feature Usage Receiver | Library / Tasks / Flighting and experiments | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.flight-reporting` | Feature Usage Reporting | Library / Tasks / Flighting and experiments | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.onesettings` | OneSettings Refresh | Library / Tasks / Flighting and experiments | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.troubleshooting-scanner` | Troubleshooting Scanner | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.scheduled-diagnosis` | Scheduled Diagnosis | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.disk-diag-data` | Disk Diagnostic Collector | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.disk-diag-resolver` | Disk Diagnostic Resolver | Library / Tasks / Diagnostics | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.disk-footprint` | Disk Footprint Diagnostics | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.power-efficiency` | Power Efficiency Diagnostics | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.memory-events` | Memory Diagnostic Events | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.memory-full` | Full Memory Diagnostic | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.resolution-host` | Diagnostic Resolution Host | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.autochk-proxy` | Autochk Proxy | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.winsat` | WinSAT | Library / Tasks / Diagnostics | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.maps-toast` | Maps Toast | Library / Tasks / Maps and location | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.maps-update` | Maps Update | Library / Tasks / Maps and location | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.location-notify` | Location Notifications | Library / Tasks / Maps and location | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.location-dialog` | Location Action Dialog | Library / Tasks / Maps and location | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.family-monitor` | Family Safety Monitor | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.family-refresh` | Family Safety Refresh | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.family-refresh-legacy` | Family Safety Refresh (legacy) | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.family-upload` | Family Safety Upload | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.user-picture` | Update User Picture | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.themes-sync` | Synced Theme Images | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.cloud-host` | Cloud Experience Host | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.hive-upload` | Profile Hive Upload | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.workplace-join` | Automatic Device Join | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.mdm-maintenance` | MDM Maintenance | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.provisioning-cellular` | Cellular Provisioning | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.provisioning-logon` | Logon Provisioning | Library / Tasks / Family and account | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.temp-license` | Temporary Licence Exchange | Library / Tasks / Family and account | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-restore` | Input Settings Restore | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-user` | Input Local User Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-mouse` | Mouse Settings Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-pen` | Pen Settings Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-touchpad` | Touchpad Settings Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.input-syncpen` | Pen Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.lang-sync` | Language Settings Sync | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.lang-install` | Language Components Install | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.lang-reconcile` | Language Resources Reconcile | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.lang-uninstall` | Language Components Removal | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.lp-remove` | Language Pack Clean-up | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.speech-model` | Speech Model Download | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.settings-sync` | Settings Sync Network Task | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.cross-device-resume` | Cross Device Resume | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.cross-device-trigger` | Cross Device Resume Trigger | Library / Tasks / Input sync | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.scheduled-defrag` | Scheduled Defrag | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.silent-cleanup` | Silent Disk Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.storage-sense` | Storage Sense | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.reg-idle-backup` | Registry Idle Backup | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.component-cleanup` | Component Clean-up | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.setup-cleanup` | Setup Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.snapshot-cleanup` | Setup Snapshot Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.verify-winre` | Verify Recovery Environment | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.state-repo` | State Repository Maintenance | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.indexer-maint` | Indexer Maintenance | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ngen` | .NET NGEN | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ngen-64` | .NET NGEN 64-bit | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ngen-crit` | .NET NGEN Critical | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ngen-64-crit` | .NET NGEN 64-bit Critical | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wof-hash` | WIM Hash Management | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wof-validate` | WIM Hash Validation | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.res-pri` | Sysmain Resource Sync | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ws-swap` | Sysmain Swap Assessment | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.space-agent` | Storage Spaces Agent | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.space-manager` | Storage Spaces Manager | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.tiers-init` | Storage Tiers Init | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.file-history` | File History Maintenance | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.dusm` | Data Usage Task | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.dssvc-cleanup` | Data Sharing Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.appuri` | App Link Verifier | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.print-edu` | Education Printer Provisioning | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.print-job-cleanup` | Print Job Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.printer-cleanup` | Printer Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.push-register` | PushToInstall Registration | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.scan-updates` | Store Update Scan | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.scan-updates-user` | Store Update Scan as User | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.retail-cleanup` | Retail Demo Clean-up | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.remote-assist` | Remote Assistance | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.upnp-config` | UPnP Host Config | Library / Tasks / Maintenance | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.xbl-save` | Xbox Game Save | Library / Tasks / Maintenance | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.time-sync` | Scheduled Time Sync | Library / Tasks / Network | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.time-force` | Forced Time Sync | Library / Tasks / Network | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.time-zone` | Time Zone Sync | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ncsi-proxy` | NCSI Proxy Detection | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.nla-wifi` | NLA Wi-Fi Task | Library / Tasks / Network | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wcm-wifi` | WCM Wi-Fi Task | Library / Tasks / Network | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.ras-mobility` | VPN Mobility Manager | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wlan-cds` | Wi-Fi CDS Sync | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wlan-profile` | Wi-Fi Profile Management | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wwan-notify` | Mobile Broadband Notifications | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.wwan-oobe` | Mobile Broadband Discovery | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.work-folders-logon` | Work Folders Sign-in Sync | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.work-folders` | Work Folders Maintenance | Library / Tasks / Network | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.edge-core` | Edge Update Core | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.edge-ua` | Edge Update Hourly | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.google-core` | Google Update Core | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.google-ua` | Google Update Hourly | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.dropbox-core` | Dropbox Update Core | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.dropbox-ua` | Dropbox Update Hourly | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.adobe-update` | Adobe Acrobat Update | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.office-heartbeat` | Office Subscription Heartbeat | Library / Tasks / Third-party | Medium | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.office-fallback` | Office Telemetry Fallback | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.office-fallback16` | Office 2016 Telemetry Fallback | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.office-logon` | Office Telemetry at Sign-in | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.office-logon16` | Office 2016 Telemetry at Sign-in | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.onedrive-machine` | OneDrive Machine Update | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11 |
| `tasks.nv-mon` | NVIDIA Telemetry Monitor | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, nvidiaGpu |
| `tasks.nv-rep-logon` | NVIDIA Telemetry at Sign-in | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, nvidiaGpu |
| `tasks.nv-rep` | NVIDIA Telemetry Report | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, nvidiaGpu |
| `tasks.amd-launcher` | AMD Install Launcher | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `tasks.amd-uep` | AMD User Experience | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `tasks.amd-link` | AMD Link Update | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `tasks.amd-ryzen-master` | Ryzen Master Task | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdCpu |
| `tasks.amd-modify-link` | AMD Modify Link Update | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `tasks.amd-start-cn` | AMD Software at Startup | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `tasks.amd-start-dvr` | AMD Recording at Startup | Library / Tasks / Third-party | Low | Low | Found in Windows | Task | 1 | Admin | Windows 10/11, amdGpu |
| `windows.startup-delay` | Startup app delay | Library / Windows / Startup and shutdown | Low | Medium | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.idle-wait` | Startup idle wait | Library / Windows / Startup and shutdown | Low | Low | Community | Registry | 1 | None | Windows 11 |
| `windows.auto-end` | End hung tasks | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `windows.hung-timeout` | Hung app timeout | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `windows.kill-timeout` | App kill timeout | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | None | Windows 10/11 |
| `windows.kill-service` | Service kill timeout | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.hooks-timeout` | Low-level hooks timeout | Library / Windows / Startup and shutdown | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.restart-apps` | Restart apps after sign-in | Library / Windows / Startup and shutdown | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.auto-restart` | Auto restart on crash | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.logon-animation` | Lock screen blur | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.account-details` | Sign-in account details | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.lock-notifications` | Lock screen notifications | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.fast-user-switch` | Switch user on lock screen | Library / Windows / Startup and shutdown | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.sign-in-mode` | Boot to desktop | Library / Windows / Startup and shutdown | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.smart-card-remove` | Smart card removal | Library / Windows / Startup and shutdown | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.cached-logons` | Cached sign-ins | Library / Windows / Startup and shutdown | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.visual-fx` | Visual effects: best performance | Library / Windows / Visual effects | Low | Medium | Community | Registry | 1 | None | Windows 10/11 |
| `windows.menu-delay` | Menu show delay | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.min-animate` | Minimise animation | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.drag-full` | Show window contents while dragging | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.smooth-scroll` | Smooth scrolling | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.font-smoothing` | Font smoothing | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.font-gamma` | Font gamma | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.foreground-lock` | Foreground lock | Library / Windows / Visual effects | Medium | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.foreground-flash` | Foreground flash | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.window-arrange` | Window arrangement | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.lock-auto` | Lock screen auto-lock | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.dpi-96` | Custom DPI | Library / Windows / Visual effects | Medium | Low | Found in Windows | Registry | 2 | Sign out | Windows 10/11 |
| `windows.list-alpha` | Translucent selection | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.list-shadow` | Icon label shadows | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.list-watermark` | Folder watermarks | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.task-animations` | Taskbar animations | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.icons-only` | Icons instead of thumbnails | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.preview-desktop` | Peek at desktop | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.transparency` | Transparency | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.dark-apps` | Dark mode for apps | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.dark-system` | Dark mode for Windows | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.accent-taskbar` | Accent colour on taskbar | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.sound-scheme` | No sounds | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.beep` | Error beep | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.color-filter` | Colour filter | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.narrator-key` | Narrator shortcut | Library / Windows / Visual effects | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.live-captions` | Live captions at sign-in | Library / Windows / Visual effects | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.voice-access` | Voice access at sign-in | Library / Windows / Visual effects | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.launch-pc` | Open to This PC | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-ext` | Show file extensions | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-hidden` | Show hidden files | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-system` | Show system files | Library / Windows / Explorer | Medium | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.separate-process` | Folders in separate process | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.desktop-process` | Desktop in separate process | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.full-path` | Full path in title | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.nav-expand` | Expand to open folder | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.map-drive-btn` | Map drive button | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.sharing-wizard` | Sharing wizard | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.compressed-color` | Colour for compressed files | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.info-tip` | File tips | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.preview-handlers` | Preview handlers | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.sync-ads` | OneDrive ads in Explorer | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-frequent` | Frequent folders | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.show-recent` | Recent files | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.show-cloud-recent` | Cloud files in Quick access | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-recommended` | Recommendations in Explorer | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.show-quick-access` | Quick access | Library / Windows / Explorer | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.preview-pane` | Preview pane | Library / Windows / Explorer | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.thumb-cache` | Thumbnail cache | Library / Windows / Explorer | Medium | Low | Community | Registry | 2 | Admin | Windows 10/11 |
| `windows.confirm-all` | Do this for all | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.enthusiast` | Detailed copy dialog | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.folder-type` | Generic folder views | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.autocomplete` | Autocomplete append | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.cached-icons` | Icon cache size | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.hub-mode` | Compact Explorer hub | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `windows.previous-versions` | Previous Versions tab | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.desktop-icons` | Hide desktop icons | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 2 | None | Windows 10/11 |
| `windows.aic-anywhere` | Install apps from anywhere | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.no-autoplay` | AutoPlay for this user | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.autoplay-nonvolume` | AutoPlay for non-volumes | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.no-autorun` | AutoRun policy | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.rename-wizards` | Web tasks | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.prints-wizard` | Online prints wizard | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.open-with-web` | Web search in Open With | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.open-with-store` | Store in Open With | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.low-disk` | Low disk warnings | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.recent-docs` | Recent documents | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.recent-nethood` | Recent shares | Library / Windows / Explorer | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.send-to-drives` | Send To drives | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.net-connect` | Connect menu | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.active-desktop` | Active Desktop | Library / Windows / Explorer | Low | Low | Documented | Registry | 5 | Admin | Windows 10/11 |
| `windows.shell-security` | Shell extension security | Library / Windows / Explorer | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.home-namespace` | Linux in This PC | Library / Windows / Explorer | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.non-enum` | Hide Documents folder | Library / Windows / Explorer | Low | Low | Community | Registry | 1 | Admin | Windows 10/11 |
| `windows.file-zone` | Downloaded file zones | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.scan-attachment` | Scan attachments | Library / Windows / Explorer | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.taskbar-left` | Align taskbar left | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.task-view` | Task View button | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.widgets` | Widgets button | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.chat` | Chat button | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.copilot-button` | Copilot button | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.cortana-button` | Cortana button | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.search-box` | Search box | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.people` | People band | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.end-task` | End task in taskbar | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.tray-all` | All tray icons | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.small-icons` | Small taskbar icons | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.taskbar-glom` | Never combine | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.taskbar-flash` | Taskbar flashing | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.taskbar-size` | Lock taskbar size | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.task-groups` | Snap groups | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.compact-mode` | Compact Explorer | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.snap-flyout` | Snap layouts flyout | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.snap-bar` | Snap bar | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.snap-suggest` | Snap suggestions | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.alt-tab` | Alt+Tab Edge tabs | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.balloon-tips` | Balloon tips | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `windows.start-recommend` | Start recommendations | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.start-docs` | Recent documents in Start | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.start-pinned` | Pinned section | Library / Windows / Taskbar and Start | Medium | Low | Found in Windows | Registry | 1 | None | Windows 11 |
| `windows.pin-taskbar` | Taskbar pinning | Library / Windows / Taskbar and Start | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.notification-center` | Notification centre | Library / Windows / Taskbar and Start | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.tablet-mode` | Tablet mode | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.notif-sound` | Notification sounds | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.notif-critical` | Critical toasts on lock | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.notif-all` | Toast notifications | Library / Windows / Taskbar and Start | Medium | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.notif-security` | Security notifications | Library / Windows / Taskbar and Start | Medium | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.notif-startup` | Startup app notifications | Library / Windows / Taskbar and Start | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.notif-edge` | Edge notification ranking | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.notif-outlook` | Outlook notification ranking | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.app-assoc-toast` | Default app nags | Library / Windows / Taskbar and Start | Low | Low | Community | Registry | 6 | None | Windows 10/11 |
| `windows.console-vt` | Virtual terminal | Library / Windows / System | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.developer-sideload` | Sideload apps | Library / Windows / System | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.developer-mode` | Developer mode | Library / Windows / System | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.maintenance` | Automatic maintenance | Library / Windows / System | Medium | Medium | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.storage-sense-policy` | Storage Sense | Library / Windows / System | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `windows.file-history` | File History policy | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.remote-assistance` | Remote Assistance | Library / Windows / System | Low | Low | Documented | Registry | 5 | Admin | Windows 10/11 |
| `windows.rdp-off` | Remote Desktop | Library / Windows / System | Medium | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.ts-enabled` | Terminal Services flag | Library / Windows / System | Medium | Low | Community | Registry | 4 | Admin | Windows 10/11 |
| `windows.dcom` | DCOM | Library / Windows / System | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.jit-debug` | Script JIT debugger | Library / Windows / System | Low | Low | Found in Windows | Registry | 1 | None | Windows 10/11 |
| `windows.wsh` | Windows Script Host | Library / Windows / System | High | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.aedebug` | Automatic debugger | Library / Windows / System | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.cert-padding` | Certificate padding check | Library / Windows / System | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.smart-screen-note` | Smart card provider | Library / Windows / System | Low | Low | Documented | Registry | 3 | Admin | Windows 10/11 |
| `windows.device-coinstallers` | Device co-installers | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.tcg-disabled` | TCG security activation | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.shutdown-tracker` | Shutdown Event Tracker | Library / Windows / System | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.always-elevated` | Always install elevated | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.dep-explorer` | DEP for Explorer | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.heap-termination` | Help pane DEP | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.credentials-delegation` | Protected credentials | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.biometric-antispoof` | Biometric anti-spoofing | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.winrm-client` | WinRM basic auth | Library / Windows / System | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.winrs` | Remote shell | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.ie-ssl3` | SSL 3 fallback | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.cert-errors` | Certificate error override | Library / Windows / System | Low | Low | Documented | Registry | 1 | Admin | Windows 10/11 |
| `windows.zone-launch` | Launch unsafe files prompt | Library / Windows / System | Low | Low | Documented | Registry | 2 | Admin | Windows 10/11 |
| `windows.mi-layout` | Optimal layout | Library / Windows / System | Low | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
| `windows.hdr-gaming` | Auto HDR | Library / Windows / System | Low | Low | Community | Registry | 2 | None | Windows 10/11 |
| `windows.hdr-playback` | HDR video playback | Library / Windows / System | Low | Low | Community | Registry | 5 | Admin | Windows 10/11 |
| `windows.colour-correction` | Automatic colour correction | Library / Windows / System | Low | Low | Community | Registry | 2 | Admin | Windows 10/11 |
| `windows.media-frameserver` | Media Foundation frame server | Library / Windows / System | Medium | Low | Found in Windows | Registry | 2 | Admin | Windows 10/11 |
| `windows.account-picture` | Account pictures | Library / Windows / System | Low | Low | Community | Registry | 1 | None | Windows 10/11 |
| `windows.settings-account` | Your account page | Library / Windows / System | Medium | Low | Found in Windows | Registry | 1 | Admin | Windows 10/11 |
