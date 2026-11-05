# Optimisation library audit

What was found in the supplied tweak collection (`pc tweaks`), and what happened to each item. The row-by-row record is
[OPTIMISATION_LIBRARY_AUDIT.csv](OPTIMISATION_LIBRARY_AUDIT.csv): one row per distinct target, 4,702 rows. The tweaks
that came out of it are in [OPTIMISATION_IMPLEMENTATION_STATUS.md](OPTIMISATION_IMPLEMENTATION_STATUS.md), and how they are
organised is in [OPTIMISATION_CATEGORIES.md](OPTIMISATION_CATEGORIES.md).

## Result

| | |
| --- | --- |
| Files read | 98 |
| Operations extracted | 8,615 (7,241 in apply branches; the rest are restore branches) |
| Distinct targets | 4,702 |
| New tweaks in the Library | 1,221 |
| Tweaks already shipped | 64 |
| Tweaks in the app | 1,285 (1,300 from 0.4, with 15 adapter tweaks) |

| Kind | Rows | Implemented | Duplicate | Blocked | Deferred | Excluded |
| --- | --- | --- | --- | --- | --- | --- |
| Registry value | 3,370 | 1,459 | 33 | 253 | 403 | 1,222 |
| Service | 26 | 11 | 2 | 0 | 0 | 13 |
| Scheduled task | 170 | 146 | 0 | 13 | 3 | 8 |
| Power setting | 40 | 36 | 0 | 0 | 4 | 0 |
| Power command | 41 | 0 | 0 | 0 | 19 | 22 |
| fsutil command | 11 | 7 | 0 | 0 | 0 | 4 |
| PowerShell step | 210 | 0 | 0 | 38 | 69 | 103 |
| DISM command | 111 | 0 | 0 | 0 | 101 | 10 |
| Boot setting | 49 | 0 | 0 | 7 | 42 | 0 |
| netsh command | 77 | 0 | 0 | 7 | 58 | 12 |
| Process kill | 63 | 0 | 0 | 0 | 0 | 63 |
| File operation | 331 | 0 | 0 | 0 | 0 | 331 |
| Event log command | 3 | 0 | 0 | 0 | 0 | 3 |
| Key delete | 105 | 0 | 0 | 0 | 0 | 105 |
| Download | 6 | 0 | 0 | 0 | 0 | 6 |
| Time command | 3 | 0 | 0 | 0 | 0 | 3 |
| Empty key | 2 | 0 | 0 | 0 | 0 | 2 |
| Environment variable | 2 | 2 | 0 | 0 | 0 | 0 |
| Permission change | 68 | 0 | 0 | 0 | 0 | 68 |
| BitLocker command | 6 | 0 | 0 | 6 | 0 | 0 |
| Service delete | 1 | 0 | 0 | 0 | 0 | 1 |
| Package install | 6 | 0 | 0 | 0 | 0 | 6 |
| Device command | 1 | 0 | 0 | 0 | 1 | 0 |
| **Total** | **4,702** | **1,661** | **35** | **324** | **700** | **1,982** |

Each row is one distinct target, so a setting written by three scripts is one row with `occurrences` 3. Implemented rows
outnumber tweaks because one tweak can own several targets (a policy and its user-level twin, say).

## Sources

| Source | Operations | Rows | Licence | Notes |
| --- | --- | --- | --- | --- |
| `Batlez_Tweaks/Batlez-Tweaks.bat` v2.7 (github.com/Batlez/Batlez-Tweaks) | 5,336 | 3,062 | Not stated in the file; a public fork lists the Unlicense | Credits imribiy (AMD GPU), Melody (AMD system), Redwan, ADEX, tarekifla, Chicho, gryOS, Hone, Ghost Optimizer, privacy.sexy, Quaked |
| `Windows-Tweaks-main` | 3,062 | 1,532 | MIT, Copyright (c) 2024 Yannick B. | Credits AlchemyTweaks, AveYo, Khorvie Tech, MysticFoxDE, TairikuOkami |
| `Windows-10-11-Gaming-Optimization-Script-main` | 116 | 41 | None in the folder | |
| `Win11_AllInOneTweaks.bat` | 101 | 67 | Not stated in the file | ambry, kubaam |

Further upstream, as credited by those files: TairikuOokami/Windows (MIT), MysticFoxDE/WINDOWS-OPTIMIZATIONS and
AlchemyTweaks/Verified-Tweaks (no licence listed).

No script was run and no script code was copied. What 2VX keeps from these sources is registry paths, value names and
values, which are facts about Windows. Names, descriptions and warnings are written for 2VX, and every change goes through
the Rust engine.

## Method

1. All 12 `.bat`, 1 `.cmd`, 1 `.ps1` and 77 `.reg` files were parsed into operations without executing anything. A
   power plan file that one script downloads from a chat link was not fetched.
2. Operations were reduced to distinct targets, case-insensitively, keeping only apply branches.
3. Each value name was searched for in Windows' own binaries, driver files and policy definitions on a reference PC.
   That gives the evidence tier below.
4. Each target was given one owner in the tweak registry. `TweakRegistry::register` refuses a second owner, even for the
   same value, so an exact duplicate can't ship.
5. Every target was then given a status and a concrete reason.
6. The current ACL of every library HKLM key was read on a reference PC. Tweaks whose key Administrators can't write were
   removed.

## Evidence tiers

Every library tweak is labelled, and the label shows in its details.

| Tier | Meaning | Tweaks |
| --- | --- | --- |
| Documented | Microsoft or the vendor documents it | 351 |
| Found in Windows | Not documented, but the value name is in a Windows or driver file | 710 |
| Community | Only the source scripts vouch for it, or the driver isn't on the reference PC | 160 |

Community tweaks stay out of the presets.

## Why items were not implemented

### Blocked (324)

| Rows | Reason |
| --- | --- |
| 202 | Security feature: project safety rule |
| 50 | Core Windows service: turning it off breaks Windows |
| 36 | Security or update protection: project safety rule |
| 21 | Cryptography or credential policy: changes what's trusted, not how fast Windows runs |
| 8 | BitLocker: security feature |
| 7 | Firewall rule edit: project safety rule |

### Deferred (700)

| Rows | Reason |
| --- | --- |
| 206 | Written to every device class; no evidence Windows reads it there |
| 101 | Windows feature or capability change; needs the DISM API and can't be undone by a value |
| 88 | Undocumented NVIDIA start-up switches in a (Default) value; what each one does can't be confirmed |
| 70 | Path computed by the script at run time (per device or per game); commands never take a free-form path |
| 58 | Needs CIM or per-adapter access; the backend never spawns netsh |
| 42 | Boot configuration: needs its own reviewed module with a high-risk confirmation |
| 28 | Script step with no registry equivalent; the backend never spawns PowerShell |
| 21 | Per-adapter or CIM setting; the backend never spawns PowerShell |
| 18 | Store app removal; PC Cleaner's Uninstall tab handles a fixed list instead |
| 17 | Creates or switches a power plan; settings are applied to the active plan instead |
| 13 | Binary value; the engine stores DWORD, QWORD and string values only |
| 12 | Changes the limits of a power setting rather than its value; the plan API can't restore them |
| 11 | Catalog entry numbers differ per PC; disabling the wrong provider breaks name resolution |
| 4 | Setting isn't present on every plan or CPU; needs a per-PC check before it can be offered |
| 3 | Feature-flag override; the ids change between Windows builds |
| 3 | The task name differs on every install; commands never take a free-form name |
| 2 | Hibernation also resizes hiberfil.sys, which takes the power API, not a registry write |
| 2 | Memory compression needs CIM; the backend never spawns PowerShell |
| 1 | Disables a device; needs SetupAPI, and the source needs a third-party devcon.exe |

### Excluded (1,982)

| Rows | Reason |
| --- | --- |
| 611 | Owned by TrustedInstaller; Administrators can't write it without taking ownership |
| 331 | Deletes or renames files; 2VX deletes only inside PC Cleaner's allowlist |
| 260 | No effect: no Windows or driver file on the reference PC contains this value name |
| 105 | Deletes a whole key tree; the engine backs up values, not trees |
| 97 | Read-only helper in the script; nothing to implement |
| 68 | Takes ownership or rewrites permissions on system files |
| 63 | Shell layout or a context-menu entry, not an optimisation |
| 63 | Ends a running process; tweaks take effect on restart instead |
| 43 | Debug or test switch; not an optimisation and unsafe on a normal PC |
| 38 | Part of removing Edge or WebView2 |
| 36 | Only sets the Windows default, or only deletes a value a normal PC doesn't have |
| 30 | Regional format, not an optimisation; the right value depends on the user's locale |
| 25 | Admin-only key; a per-user tweak can't write it, and the machine-wide policy is used where one exists |
| 20 | Kernel function name, not a setting; the source is writing a name Windows never reads as a registry value |
| 17 | Only shows a hidden setting in Power Options |
| 13 | Force-installs third-party browser extensions |
| 12 | Only stopped for the session in the source; a stop doesn't last past a restart |
| 12 | Repair or read-only command, not a lasting setting |
| 11 | Per-install leftover from the author's own PC, not a setting |
| 10 | Repair or clean-up run, not a lasting setting |
| 9 | Admin-only key or another user's hive |
| 8 | Rewrites Windows' boot or sign-in chain |
| 7 | Forces software rendering: harmful |
| 6 | Stops a program from starting |
| 6 | Downloads or runs code from the internet |
| 6 | Downloads and runs a third-party binary |
| 6 | Installs third-party software |
| 4 | Forces the slow reference rasterizer: harmful |
| 4 | Administrators can't write this service's key; the source used TrustedInstaller |
| 4 | Read-only check |
| 3 | Wi-Fi Sense was removed from Windows in 2016 |
| 3 | Another user's hive |
| 3 | Loop variable in the script; each service or task it loops over has its own row |
| 3 | Needed by Windows: turning it off breaks sound events, Task Manager or text input |
| 3 | Reapplies the active plan; the engine's power write does this itself |
| 3 | Wipes Windows event logs |
| 3 | Changes time servers, not an optimisation |
| 2 | Hides the clock or the desktop: harmful to the interface |
| 2 | Adds a program to start-up |
| 2 | Blocks shell extensions by CLSID; breaks the Explorer feature that owns them |
| 2 | Per-install Wi-Fi data, not a setting |
| 2 | Security feature in the browser |
| 2 | The source writes the literal text %NUMBER_OF_PROCESSORS%, not a number |
| 2 | Not a setting Windows reads at this key: the name appears only in unrelated code |
| 2 | Wrong path in the source; the real Edge update tasks sit at the task root and have their own rows |
| 2 | Creates an empty key with no value |
| 1 | Personal preference with no single right value |
| 1 | Holds the user's own per-app graphics choices; overwriting it would lose them |
| 1 | Changes a game's server address, not an optimisation |
| 1 | Records a hotspot state, not a setting |
| 1 | Store package bookkeeping, not a setting |
| 1 | Only useful alongside DISM component cleanup, which is deferred |
| 1 | Proxy policy, not an optimisation |
| 1 | Per-install QoS id, not a setting |
| 1 | Boot driver policy: not an optimisation |
| 1 | Storage write-protect policy: risks data on removable drives |
| 1 | Misspelt service name in the source; no such service exists |
| 1 | Runtime state Windows rewrites itself |
| 1 | Per-install path, not a setting |
| 1 | Rewrites a service's dependencies: breaks Wi-Fi start-up |
| 1 | Windows creates and removes this task itself |
| 1 | Only sets the Windows default: TRIM is already on |
| 1 | Only sets the Windows default |
| 1 | Deletes a service; no way back |

Some "Blocked" rows are the project's safety rule at work: Defender, Firewall, Windows Update, UAC, SmartScreen, Memory
integrity, Secure Boot, CPU mitigations, System Restore, BitLocker and core Windows services are never turned off as an
optimisation. Those rows are rated Critical in the CSV.

"Deferred" rows are legitimate but need something the engine doesn't have yet, usually CIM or a boot-configuration module,
because the backend never spawns processes. They're the natural next batches.

Values with no effect were checked on one reference PC with an NVIDIA driver. AMD driver values could not be checked that
way, so they were kept as Community tweaks rather than dropped.

## Compatibility of the library

| Scope | Tweaks |
| --- | --- |
| Any hardware | 1,105 |
| NVIDIA GPU only | 58 |
| AMD GPU only | 48 |
| Intel GPU only | 7 |
| Intel CPU only | 0 |
| AMD CPU only | 3 |
| Desktop only | 32 |
| Laptop only | 4 |
| Windows 11 only | 8 |

A tweak for other hardware reads "Made for other hardware" and can't be applied. GPU tweaks are written to the matching
vendor's display adapter key, so an NVIDIA tweak can't reach an AMD adapter on a hybrid PC.
