# Windows integrations

| Data | Source | Crate | Elevation | Notes |
|---|---|---|---|---|
| OS name, DisplayVersion, build, UBR, edition | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` | `windows-registry` | No | `ProductName` reads "Windows 10" on 11; normalised by build ≥ 22000 |
| CPU brand, cores, frequency, usage | sysinfo (`NtQuerySystemInformation`/PDH) | `sysinfo` 0.40 | No | `frequency` is usually the base clock |
| RAM, page file | sysinfo (`GlobalMemoryStatusEx`) | `sysinfo` | No | |
| Disks, SSD/HDD | sysinfo (volume + `IOCTL_STORAGE_QUERY_PROPERTY`) | `sysinfo` | No | `kind` may be `unknown` for USB/SATA bridges |
| GPU model, dedicated VRAM | sysinfo (DXGI `IDXGIFactory6`) | `sysinfo` | No | |
| GPU utilisation | sysinfo (PDH `\GPU Engine(*)\Utilization Percentage`) | `sysinfo` | No | **Summed over all engines**, clamped to 100 by us. Differs from Task Manager |
| GPU used VRAM | not collected | | | DXGI `QueryVideoMemoryInfo` is per-process. Phase 2: PDH `\GPU Adapter Memory(*)\Dedicated Usage` |
| Network throughput | sysinfo (`GetIfTable2`) | `sysinfo` | No | Summed across interfaces, including virtual ones |
| Motherboard | sysinfo (SMBIOS) | `sysinfo` | No | |
| Elevation | `OpenProcessToken` + `GetTokenInformation(TokenElevation)` | `windows` 0.62 | No | |

## Planned integrations (not implemented)

- Power plans: `PowerGetActiveScheme` / `PowerSetActiveScheme` (powrprof), not `powercfg` text parsing.
- Restore points: `SRSetRestorePointW` (srclient), which needs elevation and System Protection enabled.
- Startup entries: Run/RunOnce keys, Startup folders, `StartupApproved` keys (the mechanism Task Manager uses).
- Services: `QueryServiceConfigW`, `ChangeServiceConfigW`, for explained, individual start-type changes only.

## Windows 10 vs 11

- Windows 10 reached end of support on 14 Oct 2025, but the app still supports it (build ≥ 10240).
- Several gaming settings (e.g. HAGS, Auto HDR, the "Optimizations for windowed games" setting) exist only on newer
  builds. Encode this in each tweak's `supported_builds`.
- Elevation: the app runs as a standard user. It needs a separate elevated helper or relaunch for admin tweaks.
  That design is decided in Phase 2 (see `security.md`).
