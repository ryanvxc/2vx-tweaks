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

## Planned integrations (not implemented)

- Power plans: `PowerGetActiveScheme` / `PowerSetActiveScheme` (powrprof), not `powercfg` text parsing.
- Restore points: `SRSetRestorePointW` (srclient), which needs elevation and System Protection enabled.
- Startup entries: Run/RunOnce keys (documented) and Startup folders. Disabling removes the Run value and journals
  it (decision P0-8a); Task Manager's undocumented `StartupApproved` keys are not used.
- Services: `QueryServiceConfigW`, `ChangeServiceConfigW`, for explained, individual start-type changes only.

## Windows 10 vs 11

- Windows 10 reached end of support on 14 Oct 2025, but the app still supports it (build ≥ 10240).
- Several gaming settings (e.g. HAGS, Auto HDR, the "Optimizations for windowed games" setting) exist only on newer
  builds. Encode this in each tweak's `supported_builds`.
- Elevation: the app runs as a standard user. HKLM batches go through the elevated broker (`engine/broker.rs`,
  see `security.md`). Its real UAC path still needs a VM check (backlog P0-7).
