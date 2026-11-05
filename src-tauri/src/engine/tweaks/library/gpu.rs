//! Gpu tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "gpu.tdr-delay",
            Gpu,
            "Windows graphics",
            "Longer GPU timeout",
            "Gives a busy graphics card 10 seconds before Windows resets the driver.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("The default is 2 seconds, which long render or AI jobs can exceed, causing a black flash and a driver reset. A real GPU hang takes longer to recover from. It won't fix an unstable overclock.")
        .reboot(),
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
            "Longer GPU driver call timeout",
            "Gives graphics driver calls 10 seconds to return before Windows steps in.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Pairs with the GPU timeout for heavy render and compute work, and some creative apps recommend setting both. The default is 5 seconds, and most gamers will never reach it.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "TdrDdiDelay",
            10,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-pstate",
            Gpu,
            "NVIDIA",
            "Hold the GPU at full clocks",
            "Holds the graphics card at full clocks instead of dropping to idle speeds.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .warn("The GPU uses more power at idle.")
        .admin()
        .details("Can stop stutter or flicker caused by clock changes on some setups. The card uses much more power at idle and runs warmer, and driver updates can undo it. NVIDIA's per-game power mode is usually the better choice."),
        settings: &[gpu("Nvidia", "DisableDynamicPstate", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-cpl-optin",
            Gpu,
            "NVIDIA",
            "No NVIDIA Control Panel data",
            "Opts out of usage data collection in the NVIDIA Control Panel.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin()
        .details("Turns off the Control Panel's data sharing preference. It is a small privacy change with no effect on how the driver or games run. The NVIDIA app has its own setting."),
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
            "No NVIDIA tray",
            "Stops the NVIDIA tray icon starting each time you sign in.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .details("Removes an icon many people never use. The NVIDIA Control Panel and your graphics driver keep working as normal."),
        settings: &[hkcu(
            r"Software\NVIDIA Corporation\NvTray",
            "StartOnLogin",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.hags",
            Gpu,
            "Windows graphics",
            "Hardware GPU scheduling",
            "Lets the graphics card schedule its own work instead of Windows.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin()
        .details("The same switch as Hardware-accelerated GPU scheduling in Settings. Some frame generation features need it. Results vary by game and can be slightly better or worse, and it needs a restart."),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "HwSchMode",
            2,
        )],
    },
];
