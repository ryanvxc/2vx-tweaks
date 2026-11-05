//! Windows tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "windows.startup-delay",
            Windows,
            "Startup and shutdown",
            "Startup app delay",
            "Removes the wait before startup apps run.",
            BATLEZ,
            Observed,
        )
        .impact(Medium),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "StartupDelayInMSec",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.idle-wait",
            Windows,
            "Startup and shutdown",
            "Startup idle wait",
            "Stops waiting for idle before startup apps.",
            KUBAAM,
            Community,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "WaitForIdleState",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.auto-end",
            Windows,
            "Startup and shutdown",
            "End hung tasks",
            "Closes hung apps at shutdown.",
            TAIRIKU,
            Documented,
        ),
        settings: &[hkcu_str(r"Control Panel\Desktop", "AutoEndTasks", "1")],
    },
    SettingsTweak {
        meta: lib(
            "windows.hung-timeout",
            Windows,
            "Startup and shutdown",
            "Hung app timeout",
            "Treats apps as hung sooner.",
            TAIRIKU,
            Documented,
        ),
        settings: &[hkcu_str(r"Control Panel\Desktop", "HungAppTimeout", "2000")],
    },
    SettingsTweak {
        meta: lib(
            "windows.kill-timeout",
            Windows,
            "Startup and shutdown",
            "App kill timeout",
            "Waits less for apps at shutdown.",
            TAIRIKU,
            Documented,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "WaitToKillAppTimeout",
            "2000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.kill-service",
            Windows,
            "Startup and shutdown",
            "Service kill timeout",
            "Waits less for services at shutdown.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SYSTEM\CurrentControlSet\Control",
            "WaitToKillServiceTimeout",
            "2000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hooks-timeout",
            Windows,
            "Startup and shutdown",
            "Low-level hooks timeout",
            "Limits how long input hooks may take.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm_str(
            r"SYSTEM\CurrentControlSet\Control",
            "LowLevelHooksTimeout",
            "1000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.restart-apps",
            Windows,
            "Startup and shutdown",
            "Restart apps after sign-in",
            "Stops apps reopening.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "RestartApps",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.auto-restart",
            Windows,
            "Startup and shutdown",
            "Auto restart on crash",
            "Stays on the blue screen so you can read it.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\CrashControl",
            "AutoReboot",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.logon-animation",
            Windows,
            "Startup and shutdown",
            "Lock screen blur",
            "Stops the blurred sign-in background.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableAcrylicBackgroundOnLogon",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.account-details",
            Windows,
            "Startup and shutdown",
            "Sign-in account details",
            "Hides your email at sign-in.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "BlockUserFromShowingAccountDetailsOnSignin",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.lock-notifications",
            Windows,
            "Startup and shutdown",
            "Lock screen notifications",
            "Stops notifications when locked.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableLockScreenAppNotifications",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.fast-user-switch",
            Windows,
            "Startup and shutdown",
            "Switch user on lock screen",
            "Hides the switch user option.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\UserSwitch",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sign-in-mode",
            Windows,
            "Startup and shutdown",
            "Boot to desktop",
            "Goes to the desktop after sign-in.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
            "SignInMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.smart-card-remove",
            Windows,
            "Startup and shutdown",
            "Smart card removal",
            "Locks the PC when a card is removed.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "ScRemoveOption",
            "2",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cached-logons",
            Windows,
            "Startup and shutdown",
            "Cached sign-ins",
            "Stops caching domain sign-ins.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Domain PCs can't sign in offline.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "CachedLogonsCount",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.visual-fx",
            Windows,
            "Visual effects",
            "Visual effects: best performance",
            "Uses the best performance preset.",
            BATLEZ,
            Community,
        )
        .impact(Medium),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
            "VisualFXSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.menu-delay",
            Windows,
            "Visual effects",
            "Menu show delay",
            "Opens menus faster.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(r"Control Panel\Desktop", "MenuShowDelay", "50")],
    },
    SettingsTweak {
        meta: lib(
            "windows.min-animate",
            Windows,
            "Visual effects",
            "Minimise animation",
            "Turns off window animation.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop\WindowMetrics",
            "MinAnimate",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.drag-full",
            Windows,
            "Visual effects",
            "Show window contents while dragging",
            "Drags an outline instead.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(r"Control Panel\Desktop", "DragFullWindows", "0")],
    },
    SettingsTweak {
        meta: lib(
            "windows.smooth-scroll",
            Windows,
            "Visual effects",
            "Smooth scrolling",
            "Turns off smooth scrolling.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(r"Control Panel\Desktop", "SmoothScroll", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.font-smoothing",
            Windows,
            "Visual effects",
            "Font smoothing",
            "Sets font smoothing to ClearType.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu_str(r"Control Panel\Desktop", "FontSmoothing", "2"),
            hkcu(r"Control Panel\Desktop", "FontSmoothingType", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.font-gamma",
            Windows,
            "Visual effects",
            "Font gamma",
            "Clears the gamma override.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Control Panel\Desktop", "FontSmoothingGamma", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.foreground-lock",
            Windows,
            "Visual effects",
            "Foreground lock",
            "Lets apps take focus at once.",
            BATLEZ,
            Community,
        )
        .medium(),
        settings: &[hkcu(r"Control Panel\Desktop", "ForegroundLockTimeout", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.foreground-flash",
            Windows,
            "Visual effects",
            "Foreground flash",
            "Stops taskbar flashing.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "ForegroundFlashCount",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.window-arrange",
            Windows,
            "Visual effects",
            "Window arrangement",
            "Keeps Snap on.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "WindowArrangementActive",
            "1",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.lock-auto",
            Windows,
            "Visual effects",
            "Lock screen auto-lock",
            "Turns off the wallpaper auto-lock.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "LockScreenAutoLockActive",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dpi-96",
            Windows,
            "Visual effects",
            "Custom DPI",
            "Sets 100% scaling.",
            BATLEZ,
            Observed,
        )
        .medium()
        .sign_out(),
        settings: &[
            hkcu(r"Control Panel\Desktop", "LogPixels", 96),
            hkcu(r"Control Panel\Desktop", "Win8DpiScaling", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-alpha",
            Windows,
            "Visual effects",
            "Translucent selection",
            "Turns off translucent selection.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewAlphaSelect",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-shadow",
            Windows,
            "Visual effects",
            "Icon label shadows",
            "Turns off icon shadows.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewShadow",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-watermark",
            Windows,
            "Visual effects",
            "Folder watermarks",
            "Turns off folder watermarks.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewWatermark",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-animations",
            Windows,
            "Visual effects",
            "Taskbar animations",
            "Turns off taskbar animations.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAnimations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.icons-only",
            Windows,
            "Visual effects",
            "Icons instead of thumbnails",
            "Shows icons only.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "IconsOnly",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-desktop",
            Windows,
            "Visual effects",
            "Peek at desktop",
            "Turns off Peek.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "DisablePreviewDesktop",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.transparency",
            Windows,
            "Visual effects",
            "Transparency",
            "Turns off transparency.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "EnableTransparency",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dark-apps",
            Windows,
            "Visual effects",
            "Dark mode for apps",
            "Uses dark mode in apps.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "AppsUseLightTheme",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dark-system",
            Windows,
            "Visual effects",
            "Dark mode for Windows",
            "Uses dark mode in Windows.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "SystemUsesLightTheme",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.accent-taskbar",
            Windows,
            "Visual effects",
            "Accent colour on taskbar",
            "Stops accent colour on the taskbar.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "ColorPrevalence",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sound-scheme",
            Windows,
            "Visual effects",
            "No sounds",
            "Uses the No Sounds scheme.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(r"AppEvents\Schemes", "", ".None")],
    },
    SettingsTweak {
        meta: lib(
            "windows.beep",
            Windows,
            "Visual effects",
            "Error beep",
            "Turns off the error beep.",
            KUBAAM,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Sound", "Beep", "no")],
    },
    SettingsTweak {
        meta: lib(
            "windows.color-filter",
            Windows,
            "Visual effects",
            "Colour filter",
            "Turns off colour filters.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"Software\Microsoft\ColorFiltering", "Active", 0),
            hkcu(r"Software\Microsoft\ColorFiltering", "HotkeyEnabled", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.narrator-key",
            Windows,
            "Visual effects",
            "Narrator shortcut",
            "Stops Win+Ctrl+Enter opening Narrator.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Narrator\NoRoam",
            "WinEnterLaunchEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.live-captions",
            Windows,
            "Visual effects",
            "Live captions at sign-in",
            "Stops Live captions starting.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\LiveCaptions",
            "StartLiveCaptionsOnLogin",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.voice-access",
            Windows,
            "Visual effects",
            "Voice access at sign-in",
            "Stops Voice access starting.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VoiceAccess",
                "StartVoiceAccessOnLogin",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VoiceAccess",
                "VoiceAccessState",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.launch-pc",
            Windows,
            "Explorer",
            "Open to This PC",
            "Opens Explorer on This PC.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "LaunchTo",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-ext",
            Windows,
            "Explorer",
            "Show file extensions",
            "Shows extensions.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "HideFileExt",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-hidden",
            Windows,
            "Explorer",
            "Show hidden files",
            "Shows hidden files.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Hidden",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-system",
            Windows,
            "Explorer",
            "Show system files",
            "Shows protected system files.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSuperHidden",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.separate-process",
            Windows,
            "Explorer",
            "Folders in separate process",
            "Opens folders in their own process.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SeparateProcess",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.desktop-process",
            Windows,
            "Explorer",
            "Desktop in separate process",
            "Keeps the desktop in its own process.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "DesktopProcess",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.full-path",
            Windows,
            "Explorer",
            "Full path in title",
            "Shows the full path.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState",
            "FullPath",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.nav-expand",
            Windows,
            "Explorer",
            "Expand to open folder",
            "Stops the nav pane expanding.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "NavPaneExpandToCurrentFolder",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.map-drive-btn",
            Windows,
            "Explorer",
            "Map drive button",
            "Hides the map drive button.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "MapNetDrvBtn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sharing-wizard",
            Windows,
            "Explorer",
            "Sharing wizard",
            "Turns off the sharing wizard.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SharingWizardOn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.compressed-color",
            Windows,
            "Explorer",
            "Colour for compressed files",
            "Turns off blue compressed names.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCompColor",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.info-tip",
            Windows,
            "Explorer",
            "File tips",
            "Turns off hover tips.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowInfoTip",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-handlers",
            Windows,
            "Explorer",
            "Preview handlers",
            "Turns off preview handlers.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowPreviewHandlers",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sync-ads",
            Windows,
            "Explorer",
            "OneDrive ads in Explorer",
            "Stops sync provider notifications.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSyncProviderNotifications",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-frequent",
            Windows,
            "Explorer",
            "Frequent folders",
            "Hides frequent folders.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowFrequent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                "ShowFrequent",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-recent",
            Windows,
            "Explorer",
            "Recent files",
            "Hides recent files.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowRecent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                "ShowRecent",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-cloud-recent",
            Windows,
            "Explorer",
            "Cloud files in Quick access",
            "Hides cloud files.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowCloudFilesInQuickAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-recommended",
            Windows,
            "Explorer",
            "Recommendations in Explorer",
            "Hides recommendations.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowRecommendations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-quick-access",
            Windows,
            "Explorer",
            "Quick access",
            "Hides Quick access.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowQuickAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-pane",
            Windows,
            "Explorer",
            "Preview pane",
            "Hides the preview pane.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowPreviewPane",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.thumb-cache",
            Windows,
            "Explorer",
            "Thumbnail cache",
            "Stops caching thumbnails.",
            BATLEZ,
            Community,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbnailCache",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbnailCache",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.confirm-all",
            Windows,
            "Explorer",
            "Do this for all",
            "Clears the copy dialog default.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\OperationStatusManager",
            "ConfirmationCheckBoxDoForAll",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.enthusiast",
            Windows,
            "Explorer",
            "Detailed copy dialog",
            "Shows detail in the copy dialog.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\OperationStatusManager",
            "EnthusiastMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.folder-type",
            Windows,
            "Explorer",
            "Generic folder views",
            "Stops Explorer guessing folder types.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags\AllFolders\Shell",
            "FolderType",
            "NotSpecified",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.autocomplete",
            Windows,
            "Explorer",
            "Autocomplete append",
            "Stops appending in the address bar.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoComplete",
            "Append Completion",
            "No",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cached-icons",
            Windows,
            "Explorer",
            "Icon cache size",
            "Raises the icon cache.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "Max Cached Icons",
            "2000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hub-mode",
            Windows,
            "Explorer",
            "Compact Explorer hub",
            "Hides Home from File Explorer.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                "HubMode",
                1,
            ),
            hklm(r"SOFTWARE\Windows\CurrentVersion\Explorer", "HubMode", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.previous-versions",
            Windows,
            "Explorer",
            "Previous Versions tab",
            "Hides the tab.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "NoPreviousVersionsPage",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.desktop-icons",
            Windows,
            "Explorer",
            "Hide desktop icons",
            "Hides every desktop icon.",
            TAIRIKU,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "HideIcons",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Shell\Bags\1\Desktop",
                "FFlags",
                1075839525,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.aic-anywhere",
            Windows,
            "Explorer",
            "Install apps from anywhere",
            "Allows apps from any source.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "AicEnabled",
            "Anywhere",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.no-autoplay",
            Windows,
            "Explorer",
            "AutoPlay for this user",
            "Turns off AutoPlay.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
            "DisableAutoplay",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.autoplay-nonvolume",
            Windows,
            "Explorer",
            "AutoPlay for non-volumes",
            "Stops AutoPlay for phones and cameras.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoAutoplayfornonVolume",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.no-autorun",
            Windows,
            "Explorer",
            "AutoRun policy",
            "Stops programs running from media.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoAutorun",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.rename-wizards",
            Windows,
            "Explorer",
            "Web tasks",
            "Removes Publish to Web.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoPublishingWizard",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoWebServices",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.prints-wizard",
            Windows,
            "Explorer",
            "Online prints wizard",
            "Removes online printing.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoOnlinePrintsWizard",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.open-with-web",
            Windows,
            "Explorer",
            "Web search in Open With",
            "Stops searching the web for apps.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoInternetOpenWith",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.open-with-store",
            Windows,
            "Explorer",
            "Store in Open With",
            "Stops suggesting the Store.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoUseStoreOpenWith",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.low-disk",
            Windows,
            "Explorer",
            "Low disk warnings",
            "Stops the low disk check.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoLowDiskSpaceChecks",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoLowDiskSpaceChecks",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.recent-docs",
            Windows,
            "Explorer",
            "Recent documents",
            "Stops tracking recent documents.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsHistory",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsHistory",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.recent-nethood",
            Windows,
            "Explorer",
            "Recent shares",
            "Stops adding shares to Network.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "Norecentdocsnethood",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.send-to-drives",
            Windows,
            "Explorer",
            "Send To drives",
            "Hides drives in Send To.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoDrivesInSendToMenu",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoDrivesInSendToMenu",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.net-connect",
            Windows,
            "Explorer",
            "Connect menu",
            "Hides Connect in Start.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoNetConnectDisconnect",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoNetConnectDisconnect",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.active-desktop",
            Windows,
            "Explorer",
            "Active Desktop",
            "Turns off Active Desktop.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "ForceActiveDesktopOn",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoActiveDesktop",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoActiveDesktopChanges",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop",
                "NoAddingComponents",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop",
                "NoComponents",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.shell-security",
            Windows,
            "Explorer",
            "Shell extension security",
            "Only loads approved shell extensions.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Unapproved context menu items stop loading.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "EnforceShellExtensionSecurity",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.home-namespace",
            Windows,
            "Explorer",
            "Linux in This PC",
            "Hides Linux from This PC.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Classes\CLSID\{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}",
            "System.IsPinnedToNameSpaceTree",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.non-enum",
            Windows,
            "Explorer",
            "Hide Documents folder",
            "Hides shared documents.",
            KUBAAM,
            Community,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum",
            "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.file-zone",
            Windows,
            "Explorer",
            "Downloaded file zones",
            "Keeps the downloaded-file mark.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "SaveZoneInformation",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "SaveZoneInformation",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.scan-attachment",
            Windows,
            "Explorer",
            "Scan attachments",
            "Scans downloads with your antivirus.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "ScanWithAntiVirus",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "ScanWithAntiVirus",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-left",
            Windows,
            "Taskbar and Start",
            "Align taskbar left",
            "Moves the Start button left.",
            TAIRIKU,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAl",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-view",
            Windows,
            "Taskbar and Start",
            "Task View button",
            "Hides Task View.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowTaskViewButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.widgets",
            Windows,
            "Taskbar and Start",
            "Widgets button",
            "Hides Widgets.",
            BATLEZ,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarDa",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.chat",
            Windows,
            "Taskbar and Start",
            "Chat button",
            "Hides Chat.",
            BATLEZ,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarMn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.copilot-button",
            Windows,
            "Taskbar and Start",
            "Copilot button",
            "Hides Copilot.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCopilotButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cortana-button",
            Windows,
            "Taskbar and Start",
            "Cortana button",
            "Hides Cortana.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCortanaButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.search-box",
            Windows,
            "Taskbar and Start",
            "Search box",
            "Hides the taskbar search box.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Search",
            "SearchboxTaskbarMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.people",
            Windows,
            "Taskbar and Start",
            "People band",
            "Hides People.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People",
            "PeopleBand",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.end-task",
            Windows,
            "Taskbar and Start",
            "End task in taskbar",
            "Adds End task to taskbar menus.",
            BATLEZ,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            "TaskbarEndTask",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.tray-all",
            Windows,
            "Taskbar and Start",
            "All tray icons",
            "Shows every tray icon.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "EnableAutoTray",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.small-icons",
            Windows,
            "Taskbar and Start",
            "Small taskbar icons",
            "Uses small icons.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarSmallIcons",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-glom",
            Windows,
            "Taskbar and Start",
            "Never combine",
            "Never combines taskbar buttons.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarGlomLevel",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-flash",
            Windows,
            "Taskbar and Start",
            "Taskbar flashing",
            "Stops flashing.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarFlashing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-size",
            Windows,
            "Taskbar and Start",
            "Lock taskbar size",
            "Locks the taskbar.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarSizeMove",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-groups",
            Windows,
            "Taskbar and Start",
            "Snap groups",
            "Stops grouping windows.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "EnableTaskGroups",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.compact-mode",
            Windows,
            "Taskbar and Start",
            "Compact Explorer",
            "Uses compact spacing.",
            BATLEZ,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "UseCompactMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-flyout",
            Windows,
            "Taskbar and Start",
            "Snap layouts flyout",
            "Stops the flyout.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableSnapAssistFlyout",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableSnapAssistFlyoutPreview",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-bar",
            Windows,
            "Taskbar and Start",
            "Snap bar",
            "Stops the top-of-screen bar.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "EnableSnapBar",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-suggest",
            Windows,
            "Taskbar and Start",
            "Snap suggestions",
            "Stops suggesting windows.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SnapAssist",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.alt-tab",
            Windows,
            "Taskbar and Start",
            "Alt+Tab Edge tabs",
            "Shows windows only.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "MultiTaskingAltTabFilter",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.balloon-tips",
            Windows,
            "Taskbar and Start",
            "Balloon tips",
            "Turns off balloon tips.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableBalloonTips",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableBalloonTips",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.start-recommend",
            Windows,
            "Taskbar and Start",
            "Start recommendations",
            "Hides Recommended.",
            TAIRIKU,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_IrisRecommendations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.start-docs",
            Windows,
            "Taskbar and Start",
            "Recent documents in Start",
            "Stops tracking documents.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackDocs",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.start-pinned",
            Windows,
            "Taskbar and Start",
            "Pinned section",
            "Hides pinned apps.",
            BATLEZ,
            Observed,
        )
        .medium()
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Start",
            "ShowPinnedSection",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.pin-taskbar",
            Windows,
            "Taskbar and Start",
            "Taskbar pinning",
            "Blocks taskbar pins.",
            BATLEZ,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoPinningToTaskbar",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notification-center",
            Windows,
            "Taskbar and Start",
            "Notification centre",
            "Turns it off.",
            BATLEZ,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableNotificationCenter",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.tablet-mode",
            Windows,
            "Taskbar and Start",
            "Tablet mode",
            "Stays in desktop mode.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                "TabletMode",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                "TabletModeBatteryThreshold",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-sound",
            Windows,
            "Taskbar and Start",
            "Notification sounds",
            "Mutes notifications.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
            "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-critical",
            Windows,
            "Taskbar and Start",
            "Critical toasts on lock",
            "Hides critical toasts when locked.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
            "NOC_GLOBAL_SETTING_ALLOW_CRITICAL_TOASTS",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-all",
            Windows,
            "Taskbar and Start",
            "Toast notifications",
            "Turns off notifications.",
            BATLEZ,
            Community,
        )
        .medium(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
                "NOC_GLOBAL_SETTING_ALLOW_TOASTS",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\PushNotifications",
                "ToastEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-security",
            Windows,
            "Taskbar and Start",
            "Security notifications",
            "Hides Security and Maintenance toasts.",
            TAIRIKU,
            Observed,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.SecurityAndMaintenance",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-startup",
            Windows,
            "Taskbar and Start",
            "Startup app notifications",
            "Shows when an app adds itself to startup.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.StartupApp",
            "Enabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-edge",
            Windows,
            "Taskbar and Start",
            "Edge notification ranking",
            "Stops Edge in Action Center.",
            TAIRIKU,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\MSEdge",
                "ShowInActionCenter",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\MSEdge",
                "Rank",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-outlook",
            Windows,
            "Taskbar and Start",
            "Outlook notification ranking",
            "Stops Outlook in Action Center.",
            TAIRIKU,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Microsoft.OutlookForWindows_8wekyb3d8bbwe!Microsoft.OutlookforWindows",
                "ShowInActionCenter",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Microsoft.OutlookForWindows_8wekyb3d8bbwe!Microsoft.OutlookforWindows",
                "Rank",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.app-assoc-toast",
            Windows,
            "Taskbar and Start",
            "Default app nags",
            "Stops Edge asking to be default.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.htm",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.html",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_http",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_https",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.pdf",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgePDF_.pdf",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.console-vt",
            Windows,
            "System",
            "Virtual terminal",
            "Turns on ANSI colours in the console.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu("Console", "VirtualTerminalLevel", 1)],
    },
    SettingsTweak {
        meta: lib(
            "windows.developer-sideload",
            Windows,
            "System",
            "Sideload apps",
            "Allows trusted app sideloading.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock",
            "AllowAllTrustedApps",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.developer-mode",
            Windows,
            "System",
            "Developer mode",
            "Turns on developer mode.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock",
            "AllowDevelopmentWithoutDevLicense",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.maintenance",
            Windows,
            "System",
            "Automatic maintenance",
            "Turns off automatic maintenance.",
            BATLEZ,
            Observed,
        )
        .medium()
        .impact(Medium)
        .warn("Disk trims and defrags stop unless you run them.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance",
            "MaintenanceDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.storage-sense-policy",
            Windows,
            "System",
            "Storage Sense",
            "Turns off Storage Sense.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                "AllowStorageSenseGlobal",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                "AllowStorageSenseTemporaryFilesCleanup",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
                "01",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.file-history",
            Windows,
            "System",
            "File History policy",
            "Turns off File History's schedule.",
            ALCHEMY,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\FileHistory",
            "ActiveHoursEnd",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.remote-assistance",
            Windows,
            "System",
            "Remote Assistance",
            "Turns off Remote Assistance.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
                "fAllowToGetHelp",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
                "fAllowFullControl",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "fAllowToGetHelp",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "fAllowUnsolicited",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "fAllowUnsolicitedFullControl",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.rdp-off",
            Windows,
            "System",
            "Remote Desktop",
            "Denies Remote Desktop connections.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Connecting to this PC with Remote Desktop stops working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
            "fDenyTSConnections",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.ts-enabled",
            Windows,
            "System",
            "Terminal Services flag",
            "Clears the Terminal Services flags.",
            TAIRIKU,
            Community,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSUserEnabled",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Terminal Server",
                "TSUserEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSAppCompat",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.dcom",
            Windows,
            "System",
            "DCOM",
            "Turns off DCOM.",
            TAIRIKU,
            Observed,
        )
        .medium()
        .warn("Some management tools and old apps stop working.")
        .admin(),
        settings: &[hklm_str(r"SOFTWARE\Microsoft\Ole", "EnableDCOM", "N")],
    },
    SettingsTweak {
        meta: lib(
            "windows.jit-debug",
            Windows,
            "System",
            "Script JIT debugger",
            "Turns off the script debugger.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows Script\Settings",
            "JITDebug",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.wsh",
            Windows,
            "System",
            "Windows Script Host",
            "Turns off .vbs and .js scripts.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Installers and tools that use scripts stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows Script Host\Settings",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.aedebug",
            Windows,
            "System",
            "Automatic debugger",
            "Stops debuggers starting on crashes.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\AeDebug",
            "Auto",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cert-padding",
            Windows,
            "System",
            "Certificate padding check",
            "Turns on stricter signature checks.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Cryptography\Wintrust\Config",
                "EnableCertPaddingCheck",
                "1",
            ),
            hklm_str(
                r"SOFTWARE\Wow6432Node\Microsoft\Cryptography\Wintrust\Config",
                "EnableCertPaddingCheck",
                "1",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.smart-screen-note",
            Windows,
            "System",
            "Smart card provider",
            "Sets smart card sign-in rules.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowCertificatesWithNoEKU",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowIntegratedUnblock",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowSignatureOnlyKeys",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.device-coinstallers",
            Windows,
            "System",
            "Device co-installers",
            "Stops device app downloads.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Installer",
            "DisableCoInstallers",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.tcg-disabled",
            Windows,
            "System",
            "TCG security activation",
            "Stops activating self-encrypting drives.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\EnhancedStorageDevices",
            "TCGSecurityActivationDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.shutdown-tracker",
            Windows,
            "System",
            "Shutdown Event Tracker",
            "Removes the shutdown reason prompt.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Reliability",
                "ShutdownReasonOn",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Reliability",
                "ShutdownReasonUI",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.always-elevated",
            Windows,
            "System",
            "Always install elevated",
            "Makes sure this risky setting is off.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Installer",
            "AlwaysInstallElevated",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dep-explorer",
            Windows,
            "System",
            "DEP for Explorer",
            "Keeps Data Execution Prevention on Explorer.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoDataExecutionPrevention",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.heap-termination",
            Windows,
            "System",
            "Help pane DEP",
            "Keeps DEP on Help.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableHHDEP",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.credentials-delegation",
            Windows,
            "System",
            "Protected credentials",
            "Stops delegating protected credentials.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CredentialsDelegation",
            "AllowProtectedCreds",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.biometric-antispoof",
            Windows,
            "System",
            "Biometric anti-spoofing",
            "Turns on enhanced anti-spoofing.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Biometrics\Credential Provider",
            "EnhancedAntiSpoofing",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.winrm-client",
            Windows,
            "System",
            "WinRM basic auth",
            "Turns off basic auth.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WinRM\Client",
                "AllowBasic",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "AllowBasic",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.winrs",
            Windows,
            "System",
            "Remote shell",
            "Turns off remote shell.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\WinRM\Service\WinRS",
            "AllowRemoteShellAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.ie-ssl3",
            Windows,
            "System",
            "SSL 3 fallback",
            "Blocks SSL 3 fallback.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\Internet Settings",
            "EnableSSL3Fallback",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cert-errors",
            Windows,
            "System",
            "Certificate error override",
            "Stops ignoring certificate errors.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\Internet Settings",
            "PreventIgnoreCertErrors",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.zone-launch",
            Windows,
            "System",
            "Launch unsafe files prompt",
            "Prompts for programs from the Internet zone.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Zones\3",
                "1806",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Internet Settings\Zones\3",
                "1806",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.mi-layout",
            Windows,
            "System",
            "Optimal layout",
            "Stops auto layout tracking.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\OptimalLayout",
            "EnableAutoLayout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hdr-gaming",
            Windows,
            "System",
            "Auto HDR",
            "Turns off Auto HDR.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "EnableHDRForGaming",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "UseHDRForGaming",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.hdr-playback",
            Windows,
            "System",
            "HDR video playback",
            "Turns off HDR playback.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "EnableHDRForPlayback",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableHDRVideoAutoBrightness",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableHDRVideoAutoBrightness",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSDRtoHDRVideoExpansion",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSDRtoHDRVideoExpansion",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.colour-correction",
            Windows,
            "System",
            "Automatic colour correction",
            "Turns off colour correction.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ColorManagement",
                "EnableColorCorrection",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ColorManagement",
                "EnableColorCorrection",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.media-frameserver",
            Windows,
            "System",
            "Media Foundation frame server",
            "Turns off frame server mode.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some webcam apps may stop working.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows Media Foundation\Platform",
                "EnableFrameServerMode",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows Media Foundation\Platform",
                "EnableFrameServerMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.account-picture",
            Windows,
            "System",
            "Account pictures",
            "Hides account pictures.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\AccountPicture",
            "AccountPictureDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.settings-account",
            Windows,
            "System",
            "Your account page",
            "Hides Your account in Settings.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowYourAccount",
            "value",
            0,
        )],
    },
];
