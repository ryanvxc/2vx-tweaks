//! Storage tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "storage.long-paths",
            Storage,
            "File system",
            "Long file paths",
            "Lets apps that support it open paths longer than the old 260-character limit.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Deep folders, often from developer tools or game mods, can hit the old limit and fail. This lifts it for apps built to handle long paths. File Explorer and some older programs still keep the limit.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "LongPathsEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.trim",
            Storage,
            "File system",
            "SSD TRIM",
            "Makes sure Windows tells your SSD which deleted blocks it can erase.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("TRIM is on by default, so most PCs see no change and the card simply shows it is on. It matters on cloned or upgraded installs and after some disk tools switch it off, because an SSD without TRIM slows down as it fills. Hard drives ignore it.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "DisableDeleteNotification",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.registry-backup",
            Storage,
            "File system",
            "Periodic registry backup",
            "Brings back the regular registry backup that Windows stopped making by default.",
            TAIRIKU,
            Documented,
        )
        .reboot()
        .admin()
        .details("Since Windows 10 version 1803 the automatic registry copy is no longer made. This turns it back on so recovery tools have a recent copy. It uses a little disk space and needs the scheduled backup task left on."),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager",
            "EnablePeriodicBackup",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.reserved",
            Storage,
            "File system",
            "No reserved storage",
            "Asks Windows to stop holding back disk space for updates.",
            BATLEZ,
            Documented,
        )
        .medium()
        .reboot()
        .warn("Updates need free space you keep clear.")
        .admin()
        .details("Meant for small SSDs. It mimics Microsoft's DISM setting, and 2VX hasn't confirmed the space comes back on every build. It can take until the next update to show, and large updates can fail on a nearly full drive."),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                "ShippedWithReserves",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\MiscPolicyInfo",
                "ShippedWithReserves",
                2,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\PassedPolicy",
                "ShippedWithReserves",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                "MiscPolicyInfo",
                2,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                "PassedPolicy",
                0,
            ),
        ],
    },
];
