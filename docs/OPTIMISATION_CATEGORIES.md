# Optimisation categories

How the 1,221 generated tweaks are organised. Every tweak has one primary location, and the 64 tweaks that shipped earlier
keep theirs.

## Structure

There's no separate page for these. A tweak's `category` picks its page and tab (`CATEGORY_TAB` in
`src/lib/tweak-presentation.ts`) and its `section` picks the folding list on that tab. The categories below map to: Windows
and Performance → Tweaks › Core; Privacy, Gaming, Apps, Tasks and Power → the Tweaks tab of that name; CPU, GPU, Memory and
Storage → Hardware; Network → Network › Tweaks; Services → PC Cleaner › Services. Search covers every tab of its page.

| Tab | Tweaks | Sections |
| --- | --- | --- |
| Windows | 162 | 5 |
| Privacy | 196 | 8 |
| Services | 205 | 19 |
| Tasks | 146 | 9 |
| Gaming | 54 | 5 |
| CPU | 39 | 2 |
| GPU | 133 | 4 |
| Memory | 8 | 2 |
| Network | 86 | 6 |
| Applications | 140 | 6 |
| Storage | 21 | 2 |
| Power | 31 | 5 |

## Sections

### Windows (162)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Startup and shutdown | 16 | App kill timeout, Auto restart on crash, Boot to desktop, Cached sign-ins, End hung tasks, Hung app timeout and more |
| Visual effects | 28 | Accent colour on taskbar, Colour filter, Custom DPI, Dark mode for Windows, Dark mode for apps, Error beep and more |
| Explorer | 48 | Active Desktop, AutoPlay for non-volumes, AutoPlay for this user, AutoRun policy, Autocomplete append, Cloud files in Quick access and more |
| Taskbar and Start | 35 | Align taskbar left, All tray icons, Alt+Tab Edge tabs, Balloon tips, Chat button, Compact Explorer and more |
| System | 35 | Account pictures, Always install elevated, Auto HDR, Automatic colour correction, Automatic debugger, Automatic maintenance and more |

### Privacy (196)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Telemetry | 38 | .NET CLI telemetry, Additional crash data, Application compatibility engine, Application inventory, Application telemetry, Commercial data pipeline and more |
| Advertising and suggestions | 27 | Advertising ID policy, Cloud-optimised content, Consumer features, Content delivery, Content slideshow, Feature management and more |
| Copilot and AI | 8 | Click to Do, Copilot on large screens, Paint Cocreator, Paint Image Creator, Paint generative fill, Recall and more |
| Search | 21 | Cloud search, Cortana, Cortana on the lock screen, Index Outlook, Index email attachments, Index encrypted files and more |
| Sync and cross-device | 22 | Automatic MDM enrolment, Clipboard history, Cloud clipboard, Cross-device experiences, Cross-device resume, Device registration and more |
| App permissions | 42 | Account info consent, App diagnostics, Background apps (user), Background spatial, Bluetooth access, Bluetooth sync and more |
| Speech and typing | 20 | Contact harvesting, Enhanced speech audio, Hardware keyboard prediction, Ink collection, Inking and typing data, Inking privacy flag and more |
| System reports | 18 | Advertising setting store, Device metadata, Driver search, Driver updates in Windows Update, Inking store, IntelliCode remote analysis and more |

### Services (205)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Telemetry and diagnostics | 14 | Data Usage, Diagnostic Execution, Diagnostics Hub Collector, GPU Energy Driver, Graphics Performance Monitor, NDIS Capture and more |
| Xbox | 5 | Game Recording User Service, GameInput, Screen Capture Service, Xbox Accessory Management, Xbox Live Networking |
| Bluetooth | 8 | AVCTP, Bluetooth Audio Gateway, Bluetooth User Support, DevQuery Background Discovery, Device Association, Device Association Broker and more |
| Hyper-V | 9 | Hyper-V Data Exchange, Hyper-V Guest Interface, Hyper-V Guest Shutdown, Hyper-V Heartbeat, Hyper-V Host, Hyper-V PowerShell Direct and more |
| Printing and scanning | 4 | Fax, Print Workflow, Still Image Events, Windows Image Acquisition |
| Phone and cellular | 9 | Cellular Time, Local Profile Assistant, Messaging, Payments and NFC, Phone Service, SMS Router and more |
| Network discovery | 17 | AllJoyn Router, Extensible Authentication, Function Discovery Host, Function Discovery Publisher, Link-Layer Topology Discovery, Mobile Hotspot and more |
| Remote access | 13 | File and Printer Sharing, Netlogon, OpenSSH Agent, RPC Locator, Remote Desktop Configuration, Remote Desktop Port Redirector and more |
| VPN | 7 | IKE and AuthIP Keying, IP Helper, IP Translation Config, IPsec Policy Agent, Remote Access Auto Connection, Remote Access Connection Manager and more |
| Location and sensors | 4 | Auto Time Zone Updater, Natural Authentication, Sensor Data, Windows Time |
| Store and apps | 18 | ActiveX Installer, App Readiness, App-V Client, Application Management, Assigned Access, Client License Service and more |
| Mixed reality | 5 | Mixed Reality OpenXR, Perception Simulation, Spatial Data, Volumetric Audio, Windows Perception |
| Smart card and sign-in | 14 | Certificate Propagation, Cloud Identity, ConsentUX, Credential Enrollment, Device Management Enrollment, Encryption Provider Host and more |
| Backup and storage | 17 | Bind Filter, Block Level Backup, File History, File Sync Helper, Offline Files, OneDrive Updater and more |
| Enterprise | 10 | BranchCache, COM+ System Application, Distributed Transactions, Keyboard Filter, Net.Tcp Port Sharing, Network Setup and more |
| Peer networking | 4 | PNRP Name Publication, Peer Grouping, Peer Identity Manager, Peer Name Resolution |
| Search and shell | 15 | Beep Driver, COM+ Event System, Display Enhancement, Display Policy, Font Cache, Human Interface Devices and more |
| Sync and account | 18 | Agent Activation Runtime, Application Layer Gateway, Camera Frame Server, Camera Frame Server Monitor, Clipboard User Service, Connected Devices Platform and more |
| Third-party | 14 | AMD Crash Defender, AMD External Events, AMD Processor Driver, Edge Elevation, Edge Update, Edge Update Manual and more |

### Tasks (146)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Telemetry | 27 | Application Impact Telemetry, Bluetooth CEIP, CEIP Consolidator, CEIP Uploader, Compatibility Appraiser, Compatibility Appraiser Exp and more |
| Flighting and experiments | 5 | Feature Usage Flushing, Feature Usage Receiver, Feature Usage Reporting, OneSettings Refresh, Reconcile Features |
| Diagnostics | 11 | Autochk Proxy, Diagnostic Resolution Host, Disk Diagnostic Collector, Disk Diagnostic Resolver, Disk Footprint Diagnostics, Full Memory Diagnostic and more |
| Maps and location | 4 | Location Action Dialog, Location Notifications, Maps Toast, Maps Update |
| Family and account | 13 | Automatic Device Join, Cellular Provisioning, Cloud Experience Host, Family Safety Monitor, Family Safety Refresh, Family Safety Refresh (legacy) and more |
| Input sync | 15 | Cross Device Resume, Cross Device Resume Trigger, Input Local User Sync, Input Settings Restore, Language Components Install, Language Components Removal and more |
| Maintenance | 35 | .NET NGEN, .NET NGEN 64-bit, .NET NGEN 64-bit Critical, .NET NGEN Critical, App Link Verifier, Component Clean-up and more |
| Network | 13 | Forced Time Sync, Mobile Broadband Discovery, Mobile Broadband Notifications, NCSI Proxy Detection, NLA Wi-Fi Task, Scheduled Time Sync and more |
| Third-party | 23 | AMD Install Launcher, AMD Link Update, AMD Modify Link Update, AMD Recording at Startup, AMD Software at Startup, AMD User Experience and more |

### Gaming (54)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Input | 13 | Absolute pointer, Filter Keys, Keyboard data queue, Keyboard response, Language switch hotkey, Mouse Keys shortcut and more |
| Game Bar and capture | 8 | Automatic Game Mode, Background recording, Fullscreen optimisations, Game Bar controller button, Game Bar links, Game audio capture and more |
| Audio | 8 | Audio engine priority, Communications ducking, MMCSS audio priority, MMCSS display, MMCSS games extras, MMCSS lazy mode and more |
| Per-game priority | 9 | CS2 high priority, Client server priority, Desktop compositor priority, Fortnite BattlEye priority, Fortnite EAC priority, Fortnite high priority and more |
| Game settings | 16 | Fortnite low quality, Fortnite prefers D3D11, Games I/O priority, Modern Warfare V-Sync off, Modern Warfare video memory, Riot auto update and more |

### CPU (39)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Power plan | 11 | Active cooling policy, Aggressive boost mode, Boost policy, Core parking max cores, Core parking min cores, Energy preference: performance and more |
| Scheduler and timers | 28 | DPC threshold, DPC timeout, DPC watchdog offset, DPC watchdog period, Debug poll interval, Deep I/O coalescing and more |

### GPU (133)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Windows graphics | 35 | Advanced direct flip, Aero Peek, Animations shift key, CPU clipping, Calibration loader, DWM debug fail fast and more |
| NVIDIA | 52 | Ansel, Async command queue, Async compute, CUDA preemption, Completion polling, Compute preemption and more |
| AMD | 40 | AMD auto update, AMD power scheme, AMD recording, AMD usage tracking, APU shared segment, Anti-aliasing flags and more |
| Intel | 6 | Intel MSAA, Intel XeSS, Intel dynamic scaling, Intel preemption, Intel ray tracing, Intel super resolution |

### Memory (8)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Paging and cache | 4 | Keep page file at shutdown, Large system cache, Memory page combining, Service grouping threshold |
| Prefetch | 4 | Network Data Usage driver, Prefetcher, Superfetch parameter, Superfetch tracing |

### Network (86)

| Section | Tweaks | Examples |
| --- | --- | --- |
| TCP | 32 | ARP SNAP, ARP retries, ARP single route, Black hole detection, Connect retransmissions, Data retransmissions and more |
| Winsock and resolution | 13 | DNS over HTTPS, DNS priority, Default receive window, Default send window, HTTP connections, HTTP/1.0 connections and more |
| DNS cache | 10 | DNS SOA TTL, DNS cache bucket, DNS cache size, DNS cache table, DNS cache timeout, DNS maximum TTL and more |
| SMB | 15 | Admin shares, Null session access, SMB 1, SMB auto disconnect, SMB client threads, SMB directory cache and more |
| QoS | 4 | Packet scheduler sends, Packet scheduler timer, QoS bandwidth reserve, VALORANT launcher QoS tag |
| Discovery and Wi-Fi | 12 | Costed network policy, Delivery Optimization mode, Hotspot auto login, Local name providers, Minimise connections, NDIS Virtual Bus and more |

### Applications (140)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Edge | 45 | Chromium Edge upgrade, Edge Acrobat button, Edge Bing ads, Edge Collections, Edge Copilot page access, Edge Discover and more |
| Chrome | 34 | Chrome AI search, Chrome Cleanup, Chrome DoH, Chrome Lens, Chrome Privacy Sandbox, Chrome Safe Browsing reports and more |
| Firefox | 19 | Captive portal check, Extension recommendations, Firefox DoH, Firefox accounts, Firefox crash reporter, Firefox default agent and more |
| Brave, Opera and Vivaldi | 12 | Brave Safe Browsing reports, Brave background mode, Brave hardware acceleration, Brave metrics, Brave spellcheck, Opera GX background mode and more |
| Office | 12 | Acrobat Office add-ins, Classic Outlook, New Outlook migration, New Outlook toggle, Office feedback, Office telemetry and more |
| Other apps | 18 | Adobe telemetry, Adobe update manager, App Installer experiments, App Installer sources, CCleaner Health Check, CCleaner data and more |

### Storage (21)

| Section | Tweaks | Examples |
| --- | --- | --- |
| File system | 10 | Boot file optimisation, Disk check countdown, Long file paths, MFT zone, NTFS encryption, NTFS memory usage and more |
| Drives | 11 | AHCI device power saving, AHCI host power saving, NVMe I/O timeout, NVMe arbitration burst, NVMe driver logging, NVMe flush and more |

### Power (31)

| Section | Tweaks | Examples |
| --- | --- | --- |
| Disk | 5 | AHCI link power, NVMe NOPPME, NVMe primary idle timeout, NVMe secondary idle timeout, Turn off hard disk after |
| USB and PCIe | 4 | PCIe link power, USB 3 link power, USB IOC on all TDs, USB selective suspend (plan) |
| Sleep and buttons | 8 | Away mode, Device idle policy, Lid close when plugged in, Network in standby, Never hibernate, Never sleep and more |
| Display and devices | 8 | Adaptive brightness, Display never turns off, Full display brightness, GPU preference policy, Intel graphics performance, JavaScript timer frequency and more |
| System | 6 | Connected standby, Fast startup, Modern Standby override, Plan customisation at setup, Platform role, Startup sound |

## Risk

| Level | Meaning | In the app |
| --- | --- | --- |
| Low | Ordinary and reversible | Nothing shown |
| Medium | Can affect a particular app, feature or hardware | "Medium risk" |
| High | Can disrupt common setups | "High risk", plus a warning that names what breaks |
| Critical | Destructive, hard to recover, or weakens security or data integrity | Not shipped |

## Presets

Presets are named filters over the catalogue, defined in `src/lib/tweak-presets.ts`, on Tweaks › Presets. A preset only includes tweaks that
are Low risk, carry no warning, and aren't Community tier. Each one selects its tweaks into the normal review, and nothing
applies without that review.

| Preset | Includes |
| --- | --- |
| General | Windows, Memory, Storage |
| Gaming | Gaming |
| Privacy | Privacy |
| This hardware | Tweaks made for a specific GPU or CPU, when this PC has it |
| Apps | Applications |

## Adding a tweak

1. Add it under `src-tauri/src/engine/tweaks/library/`, with a section and an evidence tier.
2. Give it a real source in `references`. A Community tweak says so.
3. Use a target nothing else owns. `TweakRegistry::register` refuses a second owner.
4. Run `npm run check:all`. `library_tweaks_read_well_in_a_row` checks that the text fits.
