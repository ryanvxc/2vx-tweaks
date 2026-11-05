# Feature categorisation

Where every shipped feature lives. Source for tweaks: `src/lib/tweak-presentation.ts`. Engine IDs and categories don't
change, so the journal's history stays valid.

## Tweaks

Sections are headings on the page and set the order inside each tab. Anything this PC can't run moves to an Unavailable
section, always last.

### Tweaks › Core

| Section | Tweak |
| --- | --- |
| Core | `performance.foreground-boost`, `gaming.mmcss-games-priority`, `performance.last-access-off`, `gaming.game-dvr-off`, `windows.storage-sense-off`, `windows.window-animations-off` |

Xbox services stay under Services: they're switches that remove a feature, and Core shouldn't suggest turning off
something that breaks Game Pass.

### Tweaks › Privacy

| Section | Tweak |
| --- | --- |
| Windows data | `privacy.diagnostic-data-minimum`, `privacy.activity-history-off`, `privacy.feedback-requests-off`, `privacy.advertising-id-off`, `privacy.app-launch-tracking-off`, `privacy.language-list-opt-out`, `privacy.online-speech-off`, `privacy.inking-typing-collection-off` |
| App permissions | `privacy.app-location-off`, `privacy.app-account-info-off`, `privacy.app-diagnostics-off`, `privacy.app-notification-access-off`, `privacy.background-apps-off` |

### Hardware, Network and Advanced

| Tab | Tweak |
| --- | --- |
| Hardware › GPU | `gaming.mpo-off` |
| Hardware › CPU | `gaming.system-responsiveness`, `power.power-throttling-off` |
| Hardware › RAM | `performance.kernel-in-ram` |
| Hardware › Peripherals | `windows.mouse-acceleration-off`, `windows.keyboard-repeat-fast`, `windows.sticky-keys-shortcut-off`, `power.usb-selective-suspend-off` |
| Hardware › Storage | `performance.short-names-off` |
| Network › Tweaks | `network.delivery-optimization-http`, `network.prefer-ipv4`, `network.smb-throttling-off` |
| Network › Network Priority | `network.qos-*`, one per game |
| Advanced › Security | `network.llmnr-off`, `network.icmp-redirect-off`, `performance.fault-tolerant-heap-off` |
| Advanced › Devices | `windows.autoplay-off`, `windows.device-metadata-off`, `windows.rdp-com-redirect-off`, `windows.rdp-pnp-redirect-off` |

Core keeps its six tweaks: moving any to Hardware would leave Core empty on PCs where the rest are unavailable.

### Tweaks › Apps

| Section | Tweak |
| --- | --- |
| Microsoft Edge | `performance.edge-background-mode-off`, `performance.edge-startup-boost-off`, `privacy.edge-shopping-off`, `windows.edge-recommendations-off`, `windows.edge-sidebar-off` |

### Tweaks › Services

| Section | Tweak |
| --- | --- |
| Printing | `services.print-spooler-off`, `services.print-notify-off` |
| Bluetooth and sensors | `services.bluetooth-off`, `services.sensor-off`, `services.sensor-monitoring-off` |
| Xbox | `services.xbox-auth-off`, `services.xbox-save-off` |
| Other | `services.maps-broker-off`, `services.wap-push-off`, `services.compat-assistant-off`, `services.sync-host-off` |

## Library

1,221 tweaks in 12 tabs, listed in [OPTIMISATION_CATEGORIES.md](OPTIMISATION_CATEGORIES.md). A tweak's tab and section come from
its `category` and `section`. Tweaks with no `section` stay on Tweaks, Hardware, Network or Advanced.

## Game Mode presets

Presets reference tweaks; each tweak's card stays in its primary location, on Tweaks. Configure lists all seven tweaks
below. Those that need a restart are off by default, because they can't help a game that's already running.

| Profile | Adds | Includes |
| --- | --- | --- |
| Responsive | Favour the game you're playing | `performance.foreground-boost`, `gaming.mmcss-games-priority` |
| Quiet background | Less running behind the game | Responsive, plus `performance.edge-background-mode-off`, `performance.edge-startup-boost-off`, `privacy.background-apps-off` |
| Everything | Recording and animations off too | Quiet background, plus `gaming.game-dvr-off`, `windows.window-animations-off` |

`privacy.background-apps-off`, `gaming.game-dvr-off` and `windows.window-animations-off` need Windows Pro or higher;
on Home a profile marks them unavailable and still works with the rest.

## Everything else

| Feature | Primary location |
| --- | --- |
| Live usage, per-thread load, 5 min history | Monitoring |
| Specifications, drive capacity | Hardware |
| Startup entries | Startup |
| Folder sizes | PC Cleaner |
| Journal, undo | Restore Centre |
| System report, Windows tools | Tools |
| Update Centre, automatic check, Home notice preference | Settings › Updates |
| Version, licences | Settings › About |
| Privileges, log folder, journal state | Settings › General |
| Compact sidebar, reduce motion, theme | Settings › Appearance |

## Merged, moved, removed

| Change | Reason |
| --- | --- |
| Hardware's live charts moved to Monitoring | One purpose per page |
| Home's stat tiles moved to Monitoring | Duplicated Home's chart |
| Home's full hardware card reduced to four lines | Detail lives on Hardware |
| Home's health tile replaced by active profile | Health already has a panel |
| Home's hardware card removed | The chart names the device; detail lives on Hardware |
| Home's health panel moved into a dialog | Anything wrong still shows at the top of Home |
| Home's Recommended list replaced by Quickstart and the action button | Both are derived from the same tweaks |
| Home's gaming profile tile removed | Game Mode has its own state |
| Settings › Notifications merged into Updates | It held one update preference |
| Settings' Version row moved to About | Next to the licences |
| Card indicators for privacy, administrator and undo removed | The tab says privacy; review says admin; the switch undoes |

No user-facing capability was removed. Preferences keep their stored keys (`2vx-ui`), so existing settings carry over.
