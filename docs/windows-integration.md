# Windows integrations

| Data | Source | Crate | Elevation | Notes |
|---|---|---|---|---|
| OS name, DisplayVersion, build, UBR, edition | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` | `windows-registry` | No | `ProductName` reads "Windows 10" on 11; normalised by build ≥ 22000 |
| CPU brand, cores, frequency, usage | sysinfo (`NtQuerySystemInformation`/PDH) | `sysinfo` 0.40 | No | `frequency` is usually the base clock |
| RAM, page file | sysinfo (`GlobalMemoryStatusEx`) | `sysinfo` | No | |
| Disks, SSD/HDD | sysinfo (volume + `IOCTL_STORAGE_QUERY_PROPERTY`) | `sysinfo` | No | `kind` may be `unknown` for USB/SATA bridges |
| GPU model, dedicated VRAM | sysinfo (DXGI `IDXGIFactory6`) | `sysinfo` | No | |
| GPU utilisation | sysinfo (PDH `\GPU Engine(*)\Utilization Percentage`) | `sysinfo` | No | **Summed over all engines**, clamped to 100 by us. Differs from Task Manager |
| GPU used VRAM | not collected | | | DXGI `QueryVideoMemoryInfo` is per-process. Planned (backlog P1-3): PDH `\GPU Adapter Memory(*)\Dedicated Usage` |
| Network throughput | sysinfo (`GetIfTable2`) | `sysinfo` | No | Summed across interfaces, including virtual ones |
| Motherboard | sysinfo (SMBIOS) | `sysinfo` | No | |
| Elevation | `OpenProcessToken` + `GetTokenInformation(TokenElevation)` | `windows` 0.62 | No | |
| Battery present | `GetSystemPowerStatus` | `windows` | No | `None` when unknown; drives `DeviceScope` |
| Registry read/write (engine) | `windows-registry` | `windows-registry` | HKLM writes, and `HKCU\Software\Policies` (admin-only ACL) | Through `SettingStore`; HKLM via the broker; HKCU policy keys are not used |
| Elevated helper | `ShellExecuteExW("runas")` self-relaunch + named pipe | `windows` | Prompts once per batch | See `security.md` § Privilege |
| CPU layout, L2/L3 cache | `GetLogicalProcessorInformationEx` | `windows` | No | Top `EfficiencyClass` is the performance cores; one class means one kind of core |
| Memory slots and modules | SMBIOS Type 16 and 17 from `GetSystemFirmwareTable('RSMB')` | `windows` | No | Parsed by offset, skipping the serial number and asset tag. Firmware placeholders read as empty |
| Memory compression | A running "Memory Compression" process | `sysinfo` | No | An inference, and `null` if the process list can't be read |
| Physical drives | `IOCTL_STORAGE_QUERY_PROPERTY` (device, seek penalty, TRIM), `IOCTL_DISK_GET_DRIVE_GEOMETRY_EX` on `\\.\PhysicalDriveN` | `windows` | No | Opened with zero access. USB bridges often report no media type or TRIM |
| Drive health | `MSFT_PhysicalDisk.HealthStatus` over WMI COM, `ROOT\Microsoft\Windows\Storage` | `windows` | No | Empty when it can't be read. Status 5 ("Unknown") reads as empty. On its own thread with a 6 s limit, because WMI has none for connecting or starting a query. One stuck query keeps that thread, and later ones come back empty until it ends |
| TRIM on or off | `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\DisableDeleteNotification` | `windows-registry` | No | System-wide, not per drive |
| Devices, GPU drivers, adapters | `CM_Locate_DevNodeW`, `CM_Get_DevNode_Status`, class keys | `windows`, `windows-registry` | No | A leftover driver key is ignored: only a present device counts. Interrupt support is `DEVPKEY_PciDevice_InterruptSupport` |
| Service states | `EnumServicesStatusExW`, `QueryServiceConfigW`, `QueryServiceConfig2W`, read-only handles | `windows` | No | Start type comes from the registry. A per-user template counts as running if a session copy does |
| Shortcut targets | `IShellLinkW::GetPath` after `IPersistFile::Load(STGM_READ)` | `windows` | No | Never `Resolve`, which can search the disk and rewrite the shortcut |
| Program details | `GetDriveTypeW`, then `GetFileVersionInfoW` | `windows` | No | Only on a fixed or removable drive. A share or mapped drive reads as Unknown, unopened. A local link that points at a share isn't caught |
| Start-up tasks | `ITaskService`, every folder but `\Microsoft` | `windows` | No | Sign-in and start-up triggers only. Listed, never switched from Autoruns. A folder that can't be read is left out and named, and the rest still list |
| Ping | `GetBestRoute2` for the gateway, then `GetIpForwardTable2` when that route is on-link, as under a full-tunnel VPN; `IcmpSendEcho2` | `windows` | No | One echo per call, 1 s timeout, ping's default 32 bytes, user-started. A bufferbloat test makes one call a second to each target for 40 s. See `security.md` |
| Windows tools, log folder | `GetSystemDirectoryW` + `ShellExecuteW` | `windows` | No | Fixed list in `links.rs`. Off the main thread, on an STA of its own. A tool's own manifest can still make Windows prompt, and declining reads as `elevation_required` |
| Report export | `IFileSaveDialog` on its own STA thread | `windows` | No | Cancelling returns nothing. Written beside the file and renamed over it, so a failed save keeps the old one |
| Open at sign-in | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, value `2VX Tweaks`, and `...\Explorer\StartupApproved\Run`, read only | `windows-registry` | No | Off by default. If Task Manager has it off the value still exists, Settings says so, and only Task Manager can turn it back on |

## Tasks, power plans, display adapters and network adapters (`engine/system_store.rs`, `engine/store.rs`)

Library tweaks can target more than the registry, without starting another program:

- **Scheduled tasks**: `ITaskService` over COM, reading and setting a registered task's `Enabled`.
- **Power plans**: `PowerGetActiveScheme`, then `PowerReadACValueIndex`, `PowerReadDCValueIndex` and their write twins on the
  active plan.
- **Display adapters**: the display class key of the matching vendor, found from each adapter's `MatchingDeviceId`.
- **Network adapters**: the Net class key of the primary physical adapter of a kind: the connected one, or the first
  listed. Virtual adapters don't count, and a keyword only shows if the driver lists it under `Ndi\Params`. A driver
  reads these when it starts, and only Device Manager's Advanced page also restarts it, so these tweaks need a restart.
  The primary adapter can change between two reads, so a change records which instance it reached (`security.md`,
  rule 10). The same goes for a display adapter.

## Restore points, drives and firmware (0.6)

- `system/wmi.rs` is the one WMI client, on its own COM thread. `__PATH` only comes back with `SELECT *`, so a class whose
  methods are called is read that way. It was named `Wmi::open` rather than `connect` because the network guard bans that name.
- Restore points (`engine/restore_points.rs`): `root\default:SystemRestore`. `Enumerate` lists them and `CreateRestorePoint`
  makes one. Windows skips a new point inside its frequency window and still reports success, so `created` compares the
  newest sequence number before and after. The on and off state comes from `Windows NT\CurrentVersion\SystemRestore`, and
  the policy key under `SOFTWARE\Policies` shows when an administrator blocked it.
- Drives (`engine/drives.rs`): `root\Microsoft\Windows\Storage:MSFT_Volume`, `DriveType = 3`, `Optimize` with no switches
  so Windows picks by media. Listing works unelevated, and running needs the broker.
- Repairs (`engine/repair.rs`): `sfc.exe` and `dism.exe` with constant arguments. `sfc` writes UTF-16 when its output isn't
  a console, and `dism` writes the console code page.
- Firmware (`system/firmware.rs`): `GetFirmwareType`, `HARDWARE\DESCRIPTION\System\BIOS`, `Tbsi_GetDeviceInfo` for TPM,
  `IsProcessorFeaturePresent(PF_VIRT_FIRMWARE_ENABLED)`. Resizable BAR has no API, so it's read from the resources Windows
  allocated to the display adapter: a window of 1 GB or more means it's working, and without it the window is 256 MB.
- Timer hold (`system/devices.rs`): `NtSetTimerResolution`, per process, gone when the app closes.
- Restart (`system/power.rs`): `SE_SHUTDOWN_NAME` enabled for this process, then an unforced `ExitWindowsEx`.

## Windows 10 vs 11

- Windows 10 reached end of support on 14 Oct 2025, but the app still supports it (build ≥ 10240).
- Several gaming settings (e.g. HAGS, Auto HDR, the "Optimizations for windowed games" setting) exist only on newer
  builds. Encode this in each tweak's `supported_builds`.
- Elevation: the app runs as a standard user. HKLM batches go through the elevated broker (`engine/broker.rs`,
  see `security.md`). Its real UAC path still needs a VM check (backlog P0-7).
