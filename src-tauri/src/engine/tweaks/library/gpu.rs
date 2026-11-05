//! Gpu tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "gpu.tdr-delay",
            Gpu,
            "Windows graphics",
            "GPU timeout",
            "Waits 10 seconds before resetting the GPU.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "TdrDelay",
            10,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.tdr-ddi",
            Gpu,
            "Windows graphics",
            "GPU DDI timeout",
            "Waits 10 seconds on driver calls.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "TdrDdiDelay",
            10,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.overlays",
            Gpu,
            "Windows graphics",
            "Hardware overlays",
            "Turns off driver overlays.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "DisableOverlays",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.miracast",
            Gpu,
            "Windows graphics",
            "Miracast support",
            "Stops Miracast.",
            BATLEZ,
            Observed,
        )
        .warn("Wireless displays stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "PlatformSupportMiracast",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.preemption",
            Gpu,
            "Windows graphics",
            "GPU preemption",
            "Turns off preemption in the scheduler.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
            "EnablePreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.vsync-idle",
            Gpu,
            "Windows graphics",
            "V-Sync idle timeout",
            "Removes the V-Sync idle delay.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
            "VsyncIdleTimeout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.monitor-latency",
            Gpu,
            "Windows graphics",
            "Monitor latency tolerance",
            "Lowers how much delay Windows allows.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "MonitorLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "MonitorLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
                "MonitorLatencyTolerance",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.monitor-refresh-latency",
            Gpu,
            "Windows graphics",
            "Refresh latency tolerance",
            "Lowers allowed refresh delay.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.tdr-async",
            Gpu,
            "Windows graphics",
            "TDR async reset",
            "Resets the GPU without blocking.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
            "TdrResetFromTimeoutAsync",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-mode-animation",
            Gpu,
            "Windows graphics",
            "Mode change animation",
            "Removes the fade when display modes change.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\Dwm",
            "ForceDisableModeChangeAnimation",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-independent-flip",
            Gpu,
            "Windows graphics",
            "Independent flip",
            "Keeps independent flip on.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableIndependentFlip",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-advanced-flip",
            Gpu,
            "Windows graphics",
            "Advanced direct flip",
            "Turns off advanced direct flip.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableAdvancedDirectFlip",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-prediction",
            Gpu,
            "Windows graphics",
            "Interaction prediction",
            "Turns off input output prediction.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "InteractionOutputPredictionDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-peek",
            Gpu,
            "Windows graphics",
            "Aero Peek",
            "Turns off Peek at desktop.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(r"Software\Microsoft\Windows\DWM", "EnableAeroPeek", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-hibernate",
            Gpu,
            "Windows graphics",
            "Thumbnail hibernation",
            "Stops saving taskbar thumbnails.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\DWM",
            "AlwaysHibernateThumbnails",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-locking",
            Gpu,
            "Windows graphics",
            "DWM memory locking",
            "Stops DWM locking memory.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableLockingMemory",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-shadows",
            Gpu,
            "Windows graphics",
            "DWM projected shadows",
            "Turns off projected shadows.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "DisableProjectedShadows",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "DisableProjectedShadowsRendering",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-hologram",
            Gpu,
            "Windows graphics",
            "Holographic compositor",
            "Turns off the holographic compositor.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableHologramCompositor",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-drawlist",
            Gpu,
            "Windows graphics",
            "Draw list caching",
            "Turns off draw list caching.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableDrawListCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-effect-cache",
            Gpu,
            "Windows graphics",
            "Effect caching",
            "Keeps effect caching on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableEffectCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-megarects",
            Gpu,
            "Windows graphics",
            "Mega rects",
            "Keeps mega rects on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableMegaRects",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-cpu-clipping",
            Gpu,
            "Windows graphics",
            "CPU clipping",
            "Keeps CPU clipping on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableCpuClipping",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-parallel",
            Gpu,
            "Windows graphics",
            "Parallel mode policy",
            "Sets DWM's parallel mode.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "ParallelModePolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-linear",
            Gpu,
            "Windows graphics",
            "Linear resampling",
            "Resamples in linear space.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "ResampleInLinearSpace",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\Windows\DWM", "ResampleModeOverride", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-wet-ink",
            Gpu,
            "Windows graphics",
            "Super wet ink",
            "Turns on super wet ink.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "SuperWetEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-primitive",
            Gpu,
            "Windows graphics",
            "Primitive reordering",
            "Turns off primitive reordering.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnablePrimitiveReordering",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-debug",
            Gpu,
            "Windows graphics",
            "DWM debug fail fast",
            "Turns off debug fail-fast.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SOFTWARE\Microsoft\Windows\DWM", "DebugFailFast", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-shift",
            Gpu,
            "Windows graphics",
            "Animations shift key",
            "Stops Shift slowing animations.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "AnimationsShiftKey",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.wpf-hw",
            Gpu,
            "Windows graphics",
            "WPF hardware acceleration",
            "Runs WPF apps in software.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Microsoft\Avalon.Graphics",
            "DisableHWAcceleration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.colour-calibration",
            Gpu,
            "Windows graphics",
            "Calibration loader",
            "Stops colour calibration updates.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows NT\CurrentVersion\ICM\Calibration",
            "CalibrationUpdateFlag",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.gamma-range",
            Gpu,
            "Windows graphics",
            "GDI gamma range",
            "Widens the colour gamma range.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\ICM",
            "GdiIcmGammaRange",
            256,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.tiff-vk-debug",
            Gpu,
            "Windows graphics",
            "Vulkan loader debug",
            "Silences Vulkan loader logging.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str("Environment", "VK_LOADER_DEBUG", "none")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-pstate",
            Gpu,
            "NVIDIA",
            "Dynamic P-states",
            "Holds the GPU at full clocks.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .impact(High)
        .reboot()
        .desktop()
        .warn("The GPU uses more power at idle.")
        .admin(),
        settings: &[gpu("Nvidia", "DisableDynamicPstate", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-powermizer",
            Gpu,
            "NVIDIA",
            "PowerMizer",
            "Turns off PowerMizer.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PowerMizerEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-perf-level",
            Gpu,
            "NVIDIA",
            "Performance level source",
            "Sets the performance source.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PerfLevelSrc",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-power-saving",
            Gpu,
            "NVIDIA",
            "Driver power saving",
            "Turns off power saving mode.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnablePowerSavingMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-display-power",
            Gpu,
            "NVIDIA",
            "Display power saving",
            "Turns off display power saving.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\NVTweak",
            "DisplayPowerSaving",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-link-state",
            Gpu,
            "NVIDIA",
            "Max link state",
            "Forces full PCIe link speed.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "ForceMaxPerfLinkState",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-dynamic-pstate",
            Gpu,
            "NVIDIA",
            "Dynamic P-state switch",
            "Turns off the driver's dynamic state.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDynamicPState",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-low-latency",
            Gpu,
            "NVIDIA",
            "Low latency boost",
            "Turns on the driver's boost.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "LowLatencyBoost",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-msi",
            Gpu,
            "NVIDIA",
            "NVIDIA MSI",
            "Turns on message-signalled interrupts.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableMSI",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-dma",
            Gpu,
            "NVIDIA",
            "DMA acceleration",
            "Turns on DMA acceleration.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDMAAcceleration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-async-queue",
            Gpu,
            "NVIDIA",
            "Async command queue",
            "Turns on the async queue.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableAsyncCmdQueue",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-p2p",
            Gpu,
            "NVIDIA",
            "Peer-to-peer transfers",
            "Turns on P2P transfers.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableP2PTransfers",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-instrumentation",
            Gpu,
            "NVIDIA",
            "Driver instrumentation",
            "Turns off driver instrumentation.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDriverInstrumentation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-event-tracing",
            Gpu,
            "NVIDIA",
            "Kernel event tracing",
            "Turns off driver event tracing.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableKernelEventTracing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-event-driven",
            Gpu,
            "NVIDIA",
            "Event-driven mode",
            "Sets the driver's event mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EventDrivenMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-polling",
            Gpu,
            "NVIDIA",
            "Completion polling",
            "Polls for completion.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PollForCompletion",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-thread-policy",
            Gpu,
            "NVIDIA",
            "Thread policy",
            "Sets the driver's thread policy.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "ThreadPolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-telemetry-driver",
            Gpu,
            "NVIDIA",
            "Driver telemetry",
            "Stops telemetry in the driver.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
                "EnableNvTelemetry",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\Startup",
                "SendTelemetryData",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableTelemetry",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "NvTelemetryContainer",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-profile-telemetry",
            Gpu,
            "NVIDIA",
            "Profile telemetry",
            "Stops profile telemetry.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvProfile",
            "TelemetryEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-cpl-optin",
            Gpu,
            "NVIDIA",
            "Control Panel feedback",
            "Opts out of Control Panel data.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\NvControlPanel2\Client",
            "OptInOrOutPreference",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-tray",
            Gpu,
            "NVIDIA",
            "NVIDIA tray",
            "Stops the tray at sign-in.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu),
        settings: &[hkcu(
            r"Software\NVIDIA Corporation\NvTray",
            "StartOnLogin",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-ansel",
            Gpu,
            "NVIDIA",
            "Ansel",
            "Turns off Ansel.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\Ansel",
            "AnselAllowed",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-freestyle",
            Gpu,
            "NVIDIA",
            "Freestyle",
            "Turns off game filters.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\GeForceExperience",
            "EnableFreestyle",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rebar",
            Gpu,
            "NVIDIA",
            "Resizable BAR flag",
            "Sets the driver's Resizable BAR flag.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvDriver",
            "ResizableBar",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-camera",
            Gpu,
            "NVIDIA",
            "Nvfcamera",
            "Turns off the NVIDIA capture camera.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvfCamera",
            "Enable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-perf-sdk",
            Gpu,
            "NVIDIA",
            "Performance SDK",
            "Turns off the performance SDK.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\PerformanceSdk",
            "EnablePerfSdk",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-stereo",
            Gpu,
            "NVIDIA",
            "Stereo 3D",
            "Turns off 3D Vision.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\NVIDIA Corporation\Global\Stereo3D",
                "StereoEnable",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\Global\Stereo3D",
                "StereoDisplayMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-vsr",
            Gpu,
            "NVIDIA",
            "RTX Video Super Resolution",
            "Turns off video upscaling.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableRTXVideoSuperResolution",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableRTXVideoQualityEnhancement",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableSuperResolution",
                0,
            ),
            hklm(r"SOFTWARE\NVIDIA Corporation\NvVSR", "EnableVSR", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-gpudebugger",
            Gpu,
            "NVIDIA",
            "GPU debugger",
            "Turns off the GPU debugger hooks.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\NVIDIA Corporation\GPUDebugger", "DisableLoad", 1),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\GPUDebugger",
                "EnableInterface",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\GPUDebugger",
                "EnableRemoteDebugger",
                0,
            ),
            hklm(r"SOFTWARE\NVIDIA Corporation\GPUDebugger", "LogMask", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rm-cache",
            Gpu,
            "NVIDIA",
            "RM registry caching",
            "Turns off RM registry caching.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "RmDisableRegistryCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-l2-compression",
            Gpu,
            "NVIDIA",
            "L2 compression",
            "Turns off RM L2 compression.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "RMDisablePostL2Compression",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-per-core-dpc",
            Gpu,
            "NVIDIA",
            "Per-core DPC",
            "Runs NVIDIA DPCs on every core.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\NVAPI",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\NVTweak",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-compute",
            Gpu,
            "NVIDIA",
            "Compute preemption",
            "Turns off compute preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "ComputePreemption",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption",
            Gpu,
            "NVIDIA",
            "Driver preemption",
            "Turns off driver preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "DisablePreemption",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnablePreemption",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-cuda",
            Gpu,
            "NVIDIA",
            "CUDA preemption",
            "Turns off CUDA preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "DisableCudaContextPreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-s3",
            Gpu,
            "NVIDIA",
            "Preemption on sleep",
            "Turns off preemption on sleep.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "DisablePreemptionOnS3S4",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-level",
            Gpu,
            "NVIDIA",
            "Preemption level",
            "Sets the GPU preemption level.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "GPUPreemptionLevel",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-context-isolation",
            Gpu,
            "NVIDIA",
            "Context isolation",
            "Turns off context isolation.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableContextIsolation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-reflex-driver",
            Gpu,
            "NVIDIA",
            "Driver low latency",
            "Turns on low latency mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableLowLatencyMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-async-compute",
            Gpu,
            "NVIDIA",
            "Async compute",
            "Turns on async compute.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableAsynchronousCompute",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-frame-reporting",
            Gpu,
            "NVIDIA",
            "Frame time reporting",
            "Turns off frame time reporting.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableFrameTimeReporting",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-debug-heap",
            Gpu,
            "NVIDIA",
            "GPU debug heap",
            "Turns off the debug heap.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableGPUDebugHeap",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-debug-layer",
            Gpu,
            "NVIDIA",
            "Driver debug layer",
            "Turns off the debug layer.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "EnableDebugLayer",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-logging",
            Gpu,
            "NVIDIA",
            "Driver logging",
            "Turns off driver logs.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogErrorEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogErrorEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogEventEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogEventEntries",
                0,
            ),
            hklm(r"SYSTEM\CurrentControlSet\Services\nvlddmkm", "LogLevel", 0),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogPagingEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogPagingEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogWarningEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogWarningEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "NVAPIDebugLevel",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\NVAPI",
                "NVAPIDebugLevel",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-hda-d3cold",
            Gpu,
            "NVIDIA",
            "HD Audio D3 cold",
            "Sets HD Audio low power state.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "EnableHDAudioD3Cold",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-contiguous",
            Gpu,
            "NVIDIA",
            "Contiguous system memory",
            "Prefers contiguous memory.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "PreferSystemMemoryContiguous",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-tdr",
            Gpu,
            "NVIDIA",
            "NVIDIA TDR values",
            "Sets the driver's own TDR values.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "TdrDelay",
                10,
            ),
            hklm(r"SYSTEM\CurrentControlSet\Services\nvlddmkm", "TdrLevel", 3),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-hdcp",
            Gpu,
            "NVIDIA",
            "HDCP key",
            "Clears the RM HDCP key flag.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .warn("Protected video may stop playing.")
        .admin(),
        settings: &[gpu("Nvidia", "RMHdcpKeyglobZero", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-ulps",
            Gpu,
            "AMD",
            "Ultra low power state",
            "Turns off ULPS.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .impact(Medium)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "EnableUlps", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-power-gating",
            Gpu,
            "AMD",
            "Power gating",
            "Turns off power gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "DisablePowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dmacopy",
            Gpu,
            "AMD",
            "DMA copy",
            "Turns off DMA copy.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableDMACopy", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-block-write",
            Gpu,
            "AMD",
            "Block write",
            "Turns off block write.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableBlockWrite", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-uvd-gating",
            Gpu,
            "AMD",
            "UVD power gating",
            "Turns off video decoder gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableUVDPowerGatingDynamic", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vce-gating",
            Gpu,
            "AMD",
            "VCE power gating",
            "Turns off video encoder gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableVCEPowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-drmdma-gating",
            Gpu,
            "AMD",
            "DRM DMA gating",
            "Turns off DRM DMA gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableDrmdmaPowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-sclk-sleep",
            Gpu,
            "AMD",
            "Engine clock deep sleep",
            "Turns off deep sleep.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "PP_SclkDeepSleepDisable", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-thermal-throttle",
            Gpu,
            "AMD",
            "Thermal auto throttling",
            "Turns off auto throttling.",
            BATLEZ,
            Community,
        )
        .high()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .warn("The card relies on its own limits to avoid overheating.")
        .admin(),
        settings: &[gpu("Amd", "PP_ThermalAutoThrottlingEnable", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-gpu-down",
            Gpu,
            "AMD",
            "GPU power down",
            "Turns off GPU power down.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "PP_GPUPowerDownEnabled", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-stutter",
            Gpu,
            "AMD",
            "Stutter mode",
            "Sets memory stutter mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "StutterMode", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-color-depth",
            Gpu,
            "AMD",
            "Auto colour depth",
            "Stops colour depth dropping.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "AutoColorDepthReduction_NA", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-rs-overlay",
            Gpu,
            "AMD",
            "Radeon overlay",
            "Turns off the Radeon overlay.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu_str("Amd", "AllowRSOverlay", "false")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-skins",
            Gpu,
            "AMD",
            "Radeon skins",
            "Turns off Radeon skins.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu_str("Amd", "AllowSkins", "false")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-snapshot",
            Gpu,
            "AMD",
            "Radeon snapshots",
            "Turns off Radeon snapshots.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu("Amd", "AllowSnapshot", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-subscription",
            Gpu,
            "AMD",
            "Radeon subscription",
            "Turns off Radeon subscription prompts.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu("Amd", "AllowSubscription", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-cross-gpu",
            Gpu,
            "AMD",
            "Cross-GPU display",
            "Turns off cross-GPU display.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "KMD_EnableCrossGpuDisplaySupport", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-usage-tracking",
            Gpu,
            "AMD",
            "AMD usage tracking",
            "Stops AMD install tracking.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\AMD\Install", "UsageTracking", 0),
            hklm(r"SOFTWARE\WOW6432Node\AMD\Install", "UsageTracking", 0),
            hklm(r"SOFTWARE\AMD\Install", "AUEP", 0),
            hklm(r"SOFTWARE\WOW6432Node\AMD\Install", "AUEP", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-auto-update",
            Gpu,
            "AMD",
            "AMD auto update",
            "Stops AMD Software updating itself.",
            BATLEZ,
            Observed,
        )
        .hw(AmdGpu)
        .medium()
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\Install", "AutoUpdate", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dvr",
            Gpu,
            "AMD",
            "AMD recording",
            "Turns off AMD's recorder.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu),
        settings: &[
            hkcu(r"Software\AMD\DVR", "DvrEnabled", 0),
            hkcu(r"Software\AMD\DVR", "PerformanceMonitorOpacityWA", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-chill",
            Gpu,
            "AMD",
            "Radeon Chill",
            "Turns off Radeon Chill.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdwddmg",
            "ChillEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-rsr",
            Gpu,
            "AMD",
            "Radeon Super Resolution",
            "Turns off RSR.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableRSR",
                0,
            ),
            hklm(r"SOFTWARE\AMD\CN", "EnableRSR", 0),
            hklm(r"SOFTWARE\AMD\CN", "EnableRadeonSuperResolution", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-frtc",
            Gpu,
            "AMD",
            "Frame rate target",
            "Turns off Frame Rate Target Control.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdkmdag",
            "EnableFRTC",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-boost-cn",
            Gpu,
            "AMD",
            "Core Performance Boost",
            "Sets the CN boost flag.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "CorePerformanceBoost", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-power-scheme",
            Gpu,
            "AMD",
            "AMD power scheme",
            "Sets the CN power scheme.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "PowerScheme", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-fsr",
            Gpu,
            "AMD",
            "FidelityFX flag",
            "Turns off the CN FSR flag.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\AMD\CN",
            "EnableFidelityFXSuperResolution",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vrs",
            Gpu,
            "AMD",
            "Variable rate shading flag",
            "Turns off the CN VRS flag.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "EnableVariableRateShading", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-apu-segment",
            Gpu,
            "AMD",
            "APU shared segment",
            "Sets the A+I shared segment.",
            ALCHEMY,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\services\amdkmdap",
                "KMD_APlusISharedMiniSegmentOptions",
                7,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\services\amdkmdap",
                "KMD_APlusISharedMiniSegmentSize",
                67108864,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-preemption",
            Gpu,
            "Intel",
            "Intel preemption",
            "Turns off Intel preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx\Parameters",
            "EnablePreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-msaa",
            Gpu,
            "Intel",
            "Intel MSAA",
            "Turns off Intel MSAA.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(r"SYSTEM\CurrentControlSet\Services\igfx", "MSAAEnable", 0),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\igfx",
                "PreRenderAALevel",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-raytracing",
            Gpu,
            "Intel",
            "Intel ray tracing",
            "Turns off Intel ray tracing.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "RayTracingEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-scaling",
            Gpu,
            "Intel",
            "Intel dynamic scaling",
            "Turns on dynamic scaling.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "EnableDynamicScaling",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-sr",
            Gpu,
            "Intel",
            "Intel super resolution",
            "Turns on Intel super resolution.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "SuperResolutionEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-xess",
            Gpu,
            "Intel",
            "Intel XeSS",
            "Turns on the XeSS driver.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "XeSSDriverEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.hags",
            Gpu,
            "Windows graphics",
            "Hardware GPU scheduling",
            "Lets the GPU schedule its own memory.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "HwSchMode",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-ltr",
            Gpu,
            "AMD",
            "Latency tolerance",
            "Sets the driver's PCIe latency tolerance values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "LTRMaxNoSnoopLatency", 1),
            gpu("Amd", "LTRNoSnoopL1Latency", 1),
            gpu("Amd", "LTRSnoopL0Latency", 1),
            gpu("Amd", "LTRSnoopL1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-bgm-ltr",
            Gpu,
            "AMD",
            "Background latency tolerance",
            "Sets the background PCIe latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "BGM_LTRMaxNoSnoopLatencyValue", 1),
            gpu("Amd", "BGM_LTRMaxSnoopLatencyValue", 1),
            gpu("Amd", "BGM_LTRNoSnoopL0Latency", 1),
            gpu("Amd", "BGM_LTRNoSnoopL1Latency", 1),
            gpu("Amd", "BGM_LTRSnoopL0Latency", 1),
            gpu("Amd", "BGM_LTRSnoopL1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dal-latency",
            Gpu,
            "AMD",
            "Display latency",
            "Sets the display engine's latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "DalNBLatencyForUnderFlow", 1),
            gpu("Amd", "DalUrgentLatencyNs", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-pp-latency",
            Gpu,
            "AMD",
            "PowerPlay latency",
            "Sets PowerPlay transition latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "PP_DGBMMMaxTransitionLatencyUvd", 1),
            gpu("Amd", "PP_DGBPMMaxTransitionLatencyGfx", 1),
            gpu("Amd", "PP_RTPMComputeF1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-compute-latency",
            Gpu,
            "AMD",
            "Compute latency",
            "Sets the kernel driver's compute latency.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "KMD_RpmComputeLatency", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-memclock-latency",
            Gpu,
            "AMD",
            "Memory clock switch",
            "Sets the memory clock switch latency.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "memClockSwitchLatency", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vsr",
            Gpu,
            "AMD",
            "Virtual Super Resolution",
            "Turns off Virtual Super Resolution.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "VSRSupportEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableVirtualSuperResolution",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dxr",
            Gpu,
            "AMD",
            "Ray tracing support",
            "Turns off ray tracing in the driver.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .warn("Games that need ray tracing won't start.")
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "DXRSupportEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableRayTracing",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-aa-flags",
            Gpu,
            "AMD",
            "Anti-aliasing flags",
            "Turns off driver anti-aliasing overrides.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EQAAEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "MultiSampleAAEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "OverrideAAEnable",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dynamic-res",
            Gpu,
            "AMD",
            "Dynamic resolution",
            "Turns off the driver's dynamic resolution.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdkmdag",
            "AllowDynamicResolution",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-px-video",
            Gpu,
            "AMD",
            "Video on integrated",
            "Stops video being forced to the integrated GPU.",
            ALCHEMY,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\services\amdkmdap",
            "KMD_PXForceVideoPlaybackToIntegrated",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-thermal-cn",
            Gpu,
            "AMD",
            "CN thermal limit",
            "Sets the CN thermal limit flag to 90.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "ThermalLimit", 90)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-gr535",
            Gpu,
            "NVIDIA",
            "GR535 flag",
            "Turns off the driver's GR535 feature flag.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableGR535",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rid-flags",
            Gpu,
            "NVIDIA",
            "RID feature flags",
            "Sets two driver RID feature flags.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableRID73779",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableRID74361",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-param-flags",
            Gpu,
            "NVIDIA",
            "Driver parameter flags",
            "Turns off four driver parameter flags.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableDDC68",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3086",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3122",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3252",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-scaling-flags",
            Gpu,
            "NVIDIA",
            "Scaling flags",
            "Turns off the driver's scaling flags.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "Scaling",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "ScalingMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.d3d-flip-no-vsync",
            Gpu,
            "Windows graphics",
            "Direct3D 9 flip",
            "Lets Direct3D 9 apps flip without waiting for vsync.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Direct3D", "FlipNoVsync", 1),
            hkcu(r"Software\Microsoft\Direct3D", "FlipNoVsync", 1),
            hklm(r"SOFTWARE\WOW6432Node\Microsoft\Direct3D", "FlipNoVsync", 1),
            hkcu(r"Software\WOW6432Node\Microsoft\Direct3D", "FlipNoVsync", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.ddraw-nonlocal",
            Gpu,
            "Windows graphics",
            "Non-local video memory",
            "Lets old DirectDraw apps use system video memory.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Direct3D", "UseNonLocalVidMem", 1),
            hkcu(r"Software\Microsoft\Direct3D", "UseNonLocalVidMem", 1),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Direct3D",
                "UseNonLocalVidMem",
                1,
            ),
            hkcu(
                r"Software\WOW6432Node\Microsoft\Direct3D",
                "UseNonLocalVidMem",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\DirectDraw", "UseNonLocalVidMem", 1),
            hkcu(r"Software\Microsoft\DirectDraw", "UseNonLocalVidMem", 1),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\DirectDraw",
                "UseNonLocalVidMem",
                1,
            ),
            hkcu(
                r"Software\WOW6432Node\Microsoft\DirectDraw",
                "UseNonLocalVidMem",
                1,
            ),
        ],
    },
];
