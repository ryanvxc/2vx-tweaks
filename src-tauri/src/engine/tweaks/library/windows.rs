//! Windows tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "windows.startup-delay",
            Windows,
            "Startup and shutdown",
            "No startup app delay",
            "Startup apps begin straight after sign-in, without the usual short wait.",
            BATLEZ,
            Observed,
        )
        .details("Windows holds startup apps back for a few seconds so the desktop settles first. Removing that wait gets them running sooner, but the first moments after sign-in can feel busier, especially with many apps."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "StartupDelayInMSec",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.auto-end",
            Windows,
            "Startup and shutdown",
            "Close frozen apps at shutdown",
            "Windows closes frozen apps at shutdown rather than waiting for you to decide.",
            TAIRIKU,
            Documented,
        )
        .details("The app-is-preventing-shutdown screen is skipped, so shutdown doesn't stall. The risk is real: an app still holding unsaved work gets closed without asking, so save first.")
        .warn("Unsaved work in open apps is lost at shutdown."),
        settings: &[hkcu_str(r"Control Panel\Desktop", "AutoEndTasks", "1")],
    },
    SettingsTweak {
        meta: lib(
            "windows.kill-timeout",
            Windows,
            "Startup and shutdown",
            "Shorter app shutdown wait",
            "Shutdown gives apps less time to close before moving on, so it can finish sooner.",
            TAIRIKU,
            Documented,
        )
        .details("Windows waits for open apps at shutdown. This shortens that wait to 2 seconds. It only helps when an app is slow to close, and a slow app saving work may be cut off.")
        .warn("Apps get less time to save their work at shutdown."),
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
            "Shorter service shutdown wait",
            "Gives stuck services less time to stop, so shutdown can finish sooner.",
            TAIRIKU,
            Documented,
        )
        .admin()
        .details("Windows waits for slow services before it shuts down. This shortens that wait to 2 seconds. It only helps when a service is slow, and a service that needs longer to save its work gets cut off.")
        .reboot(),
        settings: &[hklm_str(
            r"SYSTEM\CurrentControlSet\Control",
            "WaitToKillServiceTimeout",
            "2000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.auto-restart",
            Windows,
            "Startup and shutdown",
            "Keep the blue screen on crash",
            "After a crash, the blue screen stays up so you can read the error.",
            TAIRIKU,
            Documented,
        )
        .admin()
        .details("Windows normally restarts straight away, which hides the stop code. This keeps it on screen until you restart by hand. An unattended PC stays stuck on the error rather than coming back on its own.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\CrashControl",
            "AutoReboot",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.narrator-key",
            Gaming,
            "Input",
            "No Narrator shortcut",
            "Stops Win+Ctrl+Enter starting Narrator by accident.",
            BATLEZ,
            Observed,
        )
        .details("Same as the keyboard shortcut switch in Narrator settings. It avoids Narrator suddenly talking during a game. You can still open Narrator from Settings."),
        settings: &[hkcu(
            r"Software\Microsoft\Narrator\NoRoam",
            "WinEnterLaunchEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.separate-process",
            Windows,
            "Explorer",
            "Folders in separate process",
            "Each File Explorer window runs apart, so one crash doesn't take the taskbar with it.",
            BATLEZ,
            Observed,
        )
        .details("Same as Launch folder windows in a separate process in Folder Options. It helps if a bad shell extension keeps crashing Explorer. It uses slightly more memory with several folder windows open."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SeparateProcess",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sync-ads",
            Privacy,
            "Advertising and suggestions",
            "No OneDrive ads in Explorer",
            "Stops OneDrive and storage offers appearing inside File Explorer windows.",
            BATLEZ,
            Observed,
        )
        .details("Same as Show sync provider notifications in Folder Options. It removes the promotional banners Microsoft puts in File Explorer. OneDrive itself keeps working."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSyncProviderNotifications",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.autoplay-nonvolume",
            Windows,
            "Explorer",
            "No AutoPlay for phones",
            "Stops the AutoPlay prompt appearing for phones, cameras and media players.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Plugging in a phone or camera no longer opens the AutoPlay prompt. You can still open the device in File Explorer. This covers devices that the main AutoPlay switch misses."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoAutoplayfornonVolume",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.low-disk",
            Windows,
            "Explorer",
            "No low disk warnings",
            "Stops the warnings that a drive is running out of space.",
            TAIRIKU,
            Documented,
        )
        .admin()
        .details("Useful for a game or backup drive you keep nearly full on purpose. You lose the warning on your system drive too, so keep an eye on free space there."),
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
            "windows.notif-sound",
            Windows,
            "Notifications",
            "Mute notification sounds",
            "Notifications stay silent, so no pings interrupt games or calls.",
            BATLEZ,
            Observed,
        )
        .details("Same as the Allow notifications to play sounds switch in Settings. Pop-ups still appear, just without the sound. Apps with their own sounds may still play them."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
            "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-all",
            Windows,
            "Notifications",
            "Turn off all notifications",
            "Turns off all pop-up notifications, including calls and reminders.",
            BATLEZ,
            Community,
        )
        .medium()
        .details("Same as the main Notifications switch in Settings. Games and work aren't interrupted by pop-ups. You also miss alerts you may want, so Do not disturb can be a gentler choice."),
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
            "windows.notif-startup",
            Windows,
            "Notifications",
            "Startup app notifications",
            "Tells you when an app adds itself to startup, so you can switch it off.",
            TAIRIKU,
            Observed,
        )
        .details("Same as the Startup app notification switch in Settings. It's an early sign of an app making your PC slower to start. It shows one small notification each time this happens."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.StartupApp",
            "Enabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.storage-sense-policy",
            Windows,
            "System",
            "Turn off Storage Sense",
            "Stops Windows deleting temporary files and old downloads by itself.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Storage Sense clears space automatically, which can surprise you if you keep files in Downloads or the Recycle Bin. This turns it off, so cleaning up is your call.")
        .warn("Windows stops freeing disk space automatically when it runs low."),
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
            "windows.remote-assistance",
            Windows,
            "System",
            "Turn off Remote Assistance",
            "Turns off Remote Assistance, so no one can be invited in to view or control this PC.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Remote Assistance lets someone connect to help you after an invitation. If you don't use it, turning it off removes one way in. Quick Assist and other remote tools are separate."),
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
            "Turn off Remote Desktop",
            "Refuses all incoming Remote Desktop connections to this PC.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Connecting to this PC with Remote Desktop stops working.")
        .admin()
        .details("A simple lock for a feature most people never use, even if something turns it on later. Home editions can't accept these connections anyway. Connecting out to other PCs still works."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
            "fDenyTSConnections",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.wsh",
            Windows,
            "System",
            "Turn off Windows Script Host",
            "Stops .vbs and .js script files running, closing a common route for malware.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Installers and tools that use scripts stop working.")
        .admin()
        .details("A script sent by email or download can't run if Windows Script Host is off. That's a real safety gain, but some installers and admin tools rely on these scripts, so undo this if one fails."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows Script Host\Settings",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cert-padding",
            Windows,
            "System",
            "Strict file signature check",
            "Rejects signed files that have extra data hidden after the signature.",
            TAIRIKU,
            Documented,
        )
        .admin()
        .details("Turns on a stricter signature check that Microsoft offers but leaves optional, closing a way to tamper with signed programs. It has no speed cost. A few old installers may then show as unsigned.")
        .warn("A few old installers may show as unsigned."),
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
            "windows.device-coinstallers",
            Windows,
            "System",
            "Block device vendor installers",
            "Stops hardware vendors' installers running by themselves when you plug a device in.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Some mice, keyboards and headsets pull in extra vendor software the first time you connect them. This blocks that step. Drivers still install, but any companion app has to be installed by hand."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Installer",
            "DisableCoInstallers",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.biometric-antispoof",
            Windows,
            "System",
            "Face sign-in anti-spoofing",
            "Makes face sign-in use anti-spoofing checks, so a photo is harder to pass off.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Windows Hello Face then has to pass extra checks against photos and similar tricks. This only matters if you sign in with your face. Cameras that don't support these checks can no longer be used for face sign-in."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Biometrics\Credential Provider",
            "EnhancedAntiSpoofing",
            1,
        )],
    },
];
