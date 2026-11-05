//! Gaming tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "gaming.mouse-queue",
            Gaming,
            "Input",
            "Mouse data queue",
            "Sets the mouse driver's buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Mice with high polling rates can drop input.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters",
            "MouseDataQueueSize",
            20,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.keyboard-queue",
            Gaming,
            "Input",
            "Keyboard data queue",
            "Sets the keyboard driver's buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Fast typing can drop keys.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters",
            "KeyboardDataQueueSize",
            20,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-hover",
            Gaming,
            "Input",
            "Mouse hover time",
            "Shortens hover delay.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseHoverTime", "10")],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-sensitivity",
            Gaming,
            "Input",
            "Mouse sensitivity",
            "Sets the Windows pointer speed to 6 of 11.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseSensitivity", "10")],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-trails",
            Gaming,
            "Input",
            "Mouse trails",
            "Turns off pointer trails.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseTrails", "0")],
    },
    SettingsTweak {
        meta: lib(
            "gaming.filter-keys",
            Gaming,
            "Input",
            "Filter Keys",
            "Turns off Filter Keys and its shortcut.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "Flags",
                "122",
            ),
            hkcu_str(r"Control Panel\Accessibility\FilterKeys", "Flags", "126"),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.keyboard-response",
            Gaming,
            "Input",
            "Keyboard response",
            "Sets faster repeat in the accessibility layer.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "AutoRepeatDelay",
                "200",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "AutoRepeatRate",
                "6",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "BounceTime",
                "0",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "DelayBeforeAcceptance",
                "0",
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last BounceKey Setting",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Delay",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Repeat",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Wait",
                1000,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-keys",
            Gaming,
            "Input",
            "Mouse Keys shortcut",
            "Stops the Mouse Keys shortcut.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Accessibility\MouseKeys",
            "Flags",
            "38",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.toggle-keys",
            Gaming,
            "Input",
            "Toggle Keys shortcut",
            "Stops the Toggle Keys shortcut.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Accessibility\ToggleKeys",
            "Flags",
            "58",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.num-lock",
            Gaming,
            "Input",
            "Num Lock at sign-in",
            "Turns Num Lock on at sign-in.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Keyboard",
            "InitialKeyboardIndicators",
            "2",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.language-hotkey",
            Gaming,
            "Input",
            "Language switch hotkey",
            "Stops Alt+Shift switching language mid-game.",
            TAIRIKU,
            Observed,
        ),
        settings: &[
            hkcu_str(r"Keyboard Layout\Toggle", "Hotkey", "3"),
            hkcu_str(r"Keyboard Layout\Toggle", "Language Hotkey", "3"),
            hkcu_str(r"Keyboard Layout\Toggle", "Layout Hotkey", "3"),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.touch-pointer",
            Gaming,
            "Input",
            "Absolute pointer",
            "Treats absolute pointers as absolute.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\mouhid\Parameters",
                "TreatAbsolutePointerAsAbsolute",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\mouhid\Parameters",
                "TreatAbsoluteAsRelative",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.print-screen",
            Gaming,
            "Input",
            "Print Screen opens Snipping",
            "Gives Print Screen back to apps.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Control Panel\Keyboard",
            "PrintScreenKeyForSnippingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fse-behavior",
            Gaming,
            "Game Bar and capture",
            "Fullscreen optimisations",
            "Asks Windows to prefer exclusive fullscreen.",
            BATLEZ,
            Observed,
        )
        .impact(Medium),
        settings: &[
            hkcu(r"SYSTEM\GameConfigStore", "GameDVR_FSEBehaviorMode", 2),
            hkcu(r"SYSTEM\GameConfigStore", "GameDVR_FSEBehavior", 2),
            hkcu(
                r"SYSTEM\GameConfigStore",
                "GameDVR_HonorUserFSEBehaviorMode",
                1,
            ),
            hkcu(
                r"SYSTEM\GameConfigStore",
                "GameDVR_DXGIHonorFSEWindowsCompatible",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-mode-auto",
            Gaming,
            "Game Bar and capture",
            "Automatic Game Mode",
            "Makes sure Game Mode is on.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"Software\Microsoft\GameBar", "AutoGameModeEnabled", 1),
            hkcu(r"Software\Microsoft\GameBar", "AllowAutoGameMode", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-bar-controller",
            Gaming,
            "Game Bar and capture",
            "Game Bar controller button",
            "Stops the Xbox button opening Game Bar.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\GameBar",
            "UseNexusForGameBarEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-bar-link",
            Gaming,
            "Game Bar and capture",
            "Game Bar links",
            "Stops ms-gamebar pop-ups after removing Game Bar.",
            ALCHEMY,
            Community,
        ),
        settings: &[
            hkcu_str(r"Software\Classes\ms-gamebar", "", "URL:ms-gamebar"),
            hkcu_str(r"Software\Classes\ms-gamebar", "URL Protocol", ""),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audio-capture",
            Gaming,
            "Game Bar and capture",
            "Game audio capture",
            "Stops capturing audio with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "AudioCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cursor-capture",
            Gaming,
            "Game Bar and capture",
            "Game cursor capture",
            "Stops capturing the cursor with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "CursorCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mic-capture",
            Gaming,
            "Game Bar and capture",
            "Game microphone capture",
            "Stops capturing the microphone with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "MicrophoneCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audio-ducking",
            Gaming,
            "Audio",
            "Communications ducking",
            "Stops Windows lowering game audio during calls.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Multimedia\Audio",
            "UserDuckingPreference",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-audio",
            Gaming,
            "Audio",
            "MMCSS audio priority",
            "Raises the audio task's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Priority",
                6,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Clock Rate",
                10000,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Scheduling Category",
                "High",
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Background Only",
                "False",
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Affinity",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-pro-audio",
            Gaming,
            "Audio",
            "MMCSS pro audio",
            "Tunes the Pro Audio task.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Pro Audio",
                "Priority",
                1,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Pro Audio",
                "Scheduling Category",
                "High",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-display",
            Gaming,
            "Audio",
            "MMCSS display",
            "Tunes the display post-processing task.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\DisplayPostProcessing",
                "Priority",
                8,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\DisplayPostProcessing",
                "Scheduling Category",
                "High",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-games-extra",
            Gaming,
            "Audio",
            "MMCSS games extras",
            "Sets background and latency flags for games.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Affinity",
                0,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Background Only",
                "False",
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Latency Sensitive",
                "True",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-lazy",
            Gaming,
            "Audio",
            "MMCSS lazy mode",
            "Sets the MMCSS idle timeout.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            "LazyModeTimeout",
            10000,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audiodg-priority",
            Gaming,
            "Audio",
            "Audio engine priority",
            "Raises Windows audio engine priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\audiodg.exe\PerfOptions",
                "CpuPriorityClass",
                3,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\audiodg.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.nahimic-none",
            Gaming,
            "Audio",
            "Realtek power management",
            "Keeps Realtek audio out of power saving.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Realtek\Audio\RtkNGUI64\PowerMgnt",
            "enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cs2-priority",
            Gaming,
            "Per-game priority",
            "CS2 high priority",
            "Starts Counter-Strike 2 at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\cs2.exe\PerfOptions",
                "CpuPriorityClass",
                3,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\cs2.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-priority",
            Gaming,
            "Per-game priority",
            "Fortnite high priority",
            "Starts Fortnite at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping.exe\PerfOptions",
                "CpuPriorityClass",
                3,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-be-priority",
            Gaming,
            "Per-game priority",
            "Fortnite BattlEye priority",
            "Starts the BattlEye build at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping_BE.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-eac-priority",
            Gaming,
            "Per-game priority",
            "Fortnite EAC priority",
            "Starts the EAC build at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping_EAC.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.roblox-priority",
            Gaming,
            "Per-game priority",
            "Roblox high priority",
            "Starts Roblox at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\RobloxPlayerBeta.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.obs-priority",
            Gaming,
            "Per-game priority",
            "OBS above normal",
            "Starts OBS above normal priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\obs64.exe\PerfOptions",
            "CpuPriorityClass",
            6,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.obs32-priority",
            Gaming,
            "Per-game priority",
            "OBS 32-bit above normal",
            "Starts 32-bit OBS above normal.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\obs32.exe\PerfOptions",
            "CpuPriorityClass",
            6,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.dwm-priority",
            Gaming,
            "Per-game priority",
            "Desktop compositor priority",
            "Raises DWM's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "IoPriority",
                3,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.csrss-priority",
            Gaming,
            "Per-game priority",
            "Client server priority",
            "Raises CSRSS's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "IoPriority",
                3,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cod-vsync",
            Gaming,
            "Game settings",
            "Modern Warfare V-Sync off",
            "Turns off V-Sync in the game's settings.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Activision\ModernWarfare",
            "VerticalSyncEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cod-vram",
            Gaming,
            "Game settings",
            "Modern Warfare video memory",
            "Lowers its video memory scale.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Activision\ModernWarfare",
            "VideoMemoryScale",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-dx",
            Gaming,
            "Game settings",
            "Fortnite prefers D3D11",
            "Stops preferring D3D12.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
            "PreferD3D12",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-low",
            Gaming,
            "Game settings",
            "Fortnite low quality",
            "Sets low quality in the identifiers key.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.AntiAliasingQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.EffectsQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.ShadowQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.TexturesQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.ViewDistanceQuality",
                1,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.bSmoothFrameRate",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-overlay",
            Gaming,
            "Game settings",
            "Steam overlay",
            "Turns off the Steam in-game overlay.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "EnableOverlay", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-gpu-web",
            Gaming,
            "Game settings",
            "Steam web GPU acceleration",
            "Turns off GPU use in Steam's browser.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "GPUAccelWebViews", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-h264",
            Gaming,
            "Game settings",
            "Steam H.264 acceleration",
            "Turns off hardware video in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "H264HWAccel", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-smooth",
            Gaming,
            "Game settings",
            "Steam smooth scrolling",
            "Turns off smooth scrolling in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "SmoothScrollWebViews", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-dwrite",
            Gaming,
            "Game settings",
            "Steam DirectWrite",
            "Turns off DirectWrite in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "DWriteEnable", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-dpi",
            Gaming,
            "Game settings",
            "Steam DPI scaling",
            "Turns off DPI scaling in Steam.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "DPIScaling", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-silent",
            Gaming,
            "Game settings",
            "Steam silent start",
            "Starts Steam minimised.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "SilentStartup", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-updates",
            Gaming,
            "Game settings",
            "Steam auto update",
            "Stops Steam updating itself.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[hkcu(r"Software\Valve\Steam", "AutoUpdateEnabled", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-startup",
            Gaming,
            "Game settings",
            "Steam startup mode",
            "Opens Steam to the small window.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "StartupMode", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-tcp",
            Gaming,
            "Game settings",
            "Steam TCP settings",
            "Sets Steam's TCP values.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"Software\Valve\Steam", "TcpAckFrequency", 1),
            hkcu(r"Software\Valve\Steam", "TcpNoDelay", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.riot-updates",
            Gaming,
            "Game settings",
            "Riot auto update",
            "Stops Riot Client updating games at launch.",
            BATLEZ,
            Community,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Riot Games\Riot Client",
            "autoUpdateOnLaunch",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-dvr-user",
            Gaming,
            "Game Bar and capture",
            "Background recording",
            "Turns off Game DVR for your account.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"System\GameConfigStore", "GameDVR_Enabled", 0),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                "AppCaptureEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-sfio",
            Gaming,
            "Game settings",
            "Games I/O priority",
            "Raises the MMCSS I/O priority of games.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
            "SFIO Priority",
            "High",
        )],
    },
];
