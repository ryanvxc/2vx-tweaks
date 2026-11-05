//! Gaming tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "gaming.filter-keys",
            Gaming,
            "Input",
            "No Filter Keys prompt",
            "Stops the Filter Keys prompt appearing when you hold the right Shift key.",
            BATLEZ,
            Observed,
        )
        .details("Holding right Shift for eight seconds normally offers Filter Keys, which is easy to trigger in games. This turns the feature and its shortcut off. Anyone who relies on it for accessibility should leave it on."),
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
            "gaming.toggle-keys",
            Gaming,
            "Input",
            "No Toggle Keys shortcut",
            "Stops holding Num Lock from turning on the Toggle Keys tones.",
            BATLEZ,
            Observed,
        )
        .details("Holding Num Lock for five seconds normally offers Toggle Keys, which beeps when you press a lock key. This turns off that shortcut. The feature stays available in Accessibility settings."),
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
            "Turns Num Lock on as soon as you sign in to your account.",
            BATLEZ,
            Community,
        )
        .details("Saves pressing the key each time you start the PC. It applies to your own account, so the sign-in screen itself may still start with Num Lock off."),
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
            "No language switch hotkey",
            "Stops Alt+Shift switching keyboard language in the middle of a game.",
            TAIRIKU,
            Observed,
        )
        .details("Easy to hit by accident if you have more than one keyboard language or layout. This clears the language and layout hotkeys. You can still switch from the taskbar or with Win+Space.")
        .warn("Alt+Shift no longer switches keyboard language."),
        settings: &[
            hkcu_str(r"Keyboard Layout\Toggle", "Hotkey", "3"),
            hkcu_str(r"Keyboard Layout\Toggle", "Language Hotkey", "3"),
            hkcu_str(r"Keyboard Layout\Toggle", "Layout Hotkey", "3"),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.print-screen",
            Gaming,
            "Input",
            "Print Screen skips Snipping Tool",
            "Print Screen copies the screen as it used to, without opening Snipping Tool.",
            BATLEZ,
            Observed,
        )
        .details("Windows 11 opens the snipping bar when you press the key. Turning that off lets games and screenshot apps use Print Screen again. You can still open snipping with Win+Shift+S."),
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
            "Prefer exclusive fullscreen",
            "Asks games to use true exclusive fullscreen instead of Windows' optimised mode.",
            BATLEZ,
            Observed,
        )
        .details("Can help older games that stutter or misbehave with fullscreen optimisations. Most modern games gain nothing. Alt-tab gets slower in exclusive mode and overlays may not appear."),
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
            "Makes sure Game Mode is switched on for games that support it.",
            BATLEZ,
            Observed,
        )
        .details("Game Mode favours the game and holds back Windows Update restarts and driver installs while you play. It is on by default, so this only matters if something turned it off. The effect on most PCs is small."),
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
            "No Game Bar controller button",
            "Stops the Xbox button on your controller opening Game Bar.",
            BATLEZ,
            Observed,
        )
        .details("Handy if you press the button by accident or use it in another launcher. Game Bar still opens with Win+G. It is the same switch as the one in Settings under Gaming."),
        settings: &[hkcu(
            r"Software\Microsoft\GameBar",
            "UseNexusForGameBarEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audio-ducking",
            Gaming,
            "Audio",
            "No volume drop during calls",
            "Stops Windows lowering game and music volume when a call starts.",
            BATLEZ,
            Observed,
        )
        .details("Windows can turn other sounds down during calls. This sets it to do nothing, so your game stays at the level you chose. It matches the Do nothing option on the Communications tab in Sound settings."),
        settings: &[hkcu(
            r"Software\Microsoft\Multimedia\Audio",
            "UserDuckingPreference",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cs2-priority",
            Gaming,
            "Per-game priority",
            "CS2 high priority",
            "Runs Counter-Strike 2 at a higher priority than background apps.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Starts the game at High priority every time. It only matters when the CPU is fully busy, so many PCs will see little or no change. Voice chat or streaming can stutter if the game takes all the CPU."),
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
            "Runs Fortnite at a higher priority than background apps.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Starts the game at High priority every time. It only matters when the CPU is fully busy, so many PCs will see little or no change. Voice chat or streaming can stutter if the game takes all the CPU."),
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
            "gaming.roblox-priority",
            Gaming,
            "Per-game priority",
            "Roblox high priority",
            "Runs Roblox at a higher priority than background apps.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Starts the game at High priority every time. It only matters when the CPU is fully busy, so many PCs will see little or no change. Voice chat or streaming can stutter if the game takes all the CPU."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\RobloxPlayerBeta.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
];
