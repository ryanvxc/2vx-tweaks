# Feature categorisation

Where every shipped feature lives. Source for tweaks: `src/lib/tweak-presentation.ts`. Engine IDs and categories don't
change, so the journal's history stays valid.

## Tweaks

### Tweaks › Windows

| Section | Tweak |
| --- | --- |
| Responsiveness | `performance.foreground-boost`, `windows.window-animations-off` |
| Storage | `performance.last-access-off`, `windows.storage-sense-off` |

### Tweaks › Privacy

| Section | Tweak |
| --- | --- |
| Windows data | `privacy.diagnostic-data-minimum`, `privacy.activity-history-off`, `privacy.feedback-requests-off`, `privacy.advertising-id-off`, `privacy.app-launch-tracking-off`, `privacy.language-list-opt-out`, `privacy.online-speech-off`, `privacy.inking-typing-collection-off` |
| App permissions | `privacy.app-location-off`, `privacy.app-account-info-off`, `privacy.app-diagnostics-off`, `privacy.app-notification-access-off`, `privacy.background-apps-off` |

### Tweaks › Apps

| Section | Tweak |
| --- | --- |
| Microsoft Edge | `performance.edge-background-mode-off`, `performance.edge-startup-boost-off`, `privacy.edge-shopping-off`, `windows.edge-recommendations-off`, `windows.edge-sidebar-off` |

All five Edge settings now sit together, where before they were spread over three tabs.

### Tweaks › Services

| Section | Tweak |
| --- | --- |
| Printing | `services.print-spooler-off`, `services.print-notify-off` |
| Bluetooth and sensors | `services.bluetooth-off`, `services.sensor-off`, `services.sensor-monitoring-off` |
| Xbox | `services.xbox-auth-off`, `services.xbox-save-off` |
| Other | `services.maps-broker-off`, `services.wap-push-off`, `services.compat-assistant-off`, `services.sync-host-off` |

### Gaming

| Section | Tweak |
| --- | --- |
| Gaming settings | `gaming.mmcss-games-priority`, `gaming.game-dvr-off` |

Xbox services stay under Services: they're switches that remove a feature, and Gaming shouldn't suggest turning off
something that breaks Game Pass.

## Gaming profiles

Profiles reference tweaks; each tweak's card stays in its primary location.

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
| Privileges, log folder, review rule, Quickstart preference | Settings › General |
| Compact sidebar, reduce motion, theme | Settings › Appearance |
| Journal state, network use, safety rules | Settings › Security |

## Merged, moved, removed

| Change | Reason |
| --- | --- |
| Hardware's live charts moved to Monitoring | One purpose per page |
| Home's stat tiles moved to Monitoring | Duplicated Home's chart |
| Home's full hardware card reduced to four lines | Detail lives on Hardware |
| Home's health tile replaced by active profile | Health already has a panel |
| Settings › Notifications merged into Updates | It held one update preference |
| Settings' Version row moved to About | Next to the licences |
| Card indicators for privacy, administrator and undo removed | The tab says privacy; review says admin; the switch undoes |

No user-facing capability was removed. Preferences keep their stored keys (`2vx-ui`), so existing settings carry over.
