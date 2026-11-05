//! Power tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\6738e2c4-e8a5-4a42-b16a-e040e769756e",
                "AC",
            ),
            "power.disk-idle",
            Power,
            "Disk",
            "Keep hard drives spinning",
            "Hard drives keep spinning on mains power, avoiding the wait to spin back up.",
            POWER_DOCS,
            Documented,
        )
        .admin()
        .details("Without this, an idle hard drive spins down and the next read can stall for a few seconds. It has no effect on SSDs. On battery, drives still stop after 10 minutes."),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\6738e2c4-e8a5-4a42-b16a-e040e769756e",
                "AC",
                0,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\6738e2c4-e8a5-4a42-b16a-e040e769756e",
                "DC",
                600,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\0b2d69d7-a2a1-449c-9680-f91c70521c60",
                "AC",
            ),
            "power.ahci-link",
            Power,
            "Disk",
            "SATA link power saving off",
            "Keeps SATA drive links fully awake, avoiding a short delay after idle.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("SATA drives can drop their connection into low power when idle. This keeps the link active, which can avoid a brief pause on the next read and can settle some drive hiccups. It uses slightly more power."),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\0b2d69d7-a2a1-449c-9680-f91c70521c60",
                "AC",
                0,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\0b2d69d7-a2a1-449c-9680-f91c70521c60",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d639518a-e56d-4345-8af2-b9f32fb26109",
                "AC",
            ),
            "power.nvme-primary",
            Power,
            "Disk",
            "NVMe light idle off",
            "Stops an NVMe SSD entering its first low-power state on mains.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("The SSD stays ready rather than dropping into light idle, which can help a few drives that stall after idle. Any speed gain is tiny, and the drive runs a bit warmer. On battery it still idles after 60 seconds."),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d639518a-e56d-4345-8af2-b9f32fb26109",
                "AC",
                0,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d639518a-e56d-4345-8af2-b9f32fb26109",
                "DC",
                60000,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d3d55efd-c1ff-424e-9dc3-441be7833010",
                "AC",
            ),
            "power.nvme-secondary",
            Power,
            "Disk",
            "NVMe deep idle off",
            "Stops an NVMe SSD dropping into its deeper low-power state on mains.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("Deeper idle takes longer to wake from, and some drives handle it badly. This can remove those short stalls. It makes no difference to normal speed, and the drive runs a bit warmer. On battery it still idles after 60 seconds."),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d3d55efd-c1ff-424e-9dc3-441be7833010",
                "AC",
                0,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\d3d55efd-c1ff-424e-9dc3-441be7833010",
                "DC",
                60000,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"2a737441-1930-4402-8d77-b2bebba308a3\d4e98f31-5ffe-4ce1-be31-1b38b384c009",
                "AC",
            ),
            "power.usb3-link",
            Power,
            "USB and PCIe",
            "USB 3 link power saving off",
            "Keeps USB 3 links awake, which can fix devices that drop out or lag after idle.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("USB 3 ports can drop to lower power between transfers. Turning that off helps the few devices that misbehave because of it. It uses a little more power, on mains and on battery."),
        settings: &[
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\d4e98f31-5ffe-4ce1-be31-1b38b384c009",
                "AC",
                0,
            ),
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\d4e98f31-5ffe-4ce1-be31-1b38b384c009",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"501a4d13-42af-4429-9fd1-a8218c268e20\ee12f906-d277-404b-b6da-e5fa1a576df5",
                "AC",
            ),
            "power.pcie-aspm",
            Power,
            "USB and PCIe",
            "PCIe link power saving off",
            "Keeps PCIe links awake on mains, avoiding wake-up delays and glitches.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("PCIe devices such as graphics cards and SSDs can drop their link into low power when idle. Turning that off on mains can fix rare stutter or device dropouts. It uses slightly more power. On battery it still saves power."),
        settings: &[
            power(
                r"501a4d13-42af-4429-9fd1-a8218c268e20\ee12f906-d277-404b-b6da-e5fa1a576df5",
                "AC",
                0,
            ),
            power(
                r"501a4d13-42af-4429-9fd1-a8218c268e20\ee12f906-d277-404b-b6da-e5fa1a576df5",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\29f6c1db-86da-48c5-9fdb-f2b67b1f44da",
                "AC",
            ),
            "power.sleep",
            Power,
            "Sleep and buttons",
            "Never sleep",
            "The PC no longer goes to sleep by itself when left idle.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("Stops a long download, render or game server being cut off by the sleep timer, on mains and on battery. You can still choose Sleep from the Start menu. Power use rises if the PC is left on."),
        settings: &[
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\29f6c1db-86da-48c5-9fdb-f2b67b1f44da",
                "AC",
                0,
            ),
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\29f6c1db-86da-48c5-9fdb-f2b67b1f44da",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\9d7815a6-7ee4-497e-8888-515a05f02364",
                "AC",
            ),
            "power.hibernate",
            Power,
            "Sleep and buttons",
            "Never hibernate",
            "The PC no longer hibernates by itself after sitting idle.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("Keeps a PC available for overnight downloads or remote access. It only changes the idle timer, so you can still hibernate by hand where that's offered. Laptops left idle use more battery."),
        settings: &[
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\9d7815a6-7ee4-497e-8888-515a05f02364",
                "AC",
                0,
            ),
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\9d7815a6-7ee4-497e-8888-515a05f02364",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"4f971e89-eebd-4455-a8de-9e59040e7347\7648efa3-dd9c-4e3e-b566-50f929386280",
                "AC",
            ),
            "power.button",
            Power,
            "Sleep and buttons",
            "Power button does nothing",
            "Pressing the power button by accident no longer shuts the PC down.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .warn("Hold the button to force a shutdown.")
        .admin()
        .details("Useful if the button is easy to knock, or children or pets can reach it. You then shut down from the Start menu. Holding the button still forces the PC off."),
        settings: &[
            power(
                r"4f971e89-eebd-4455-a8de-9e59040e7347\7648efa3-dd9c-4e3e-b566-50f929386280",
                "AC",
                0,
            ),
            power(
                r"4f971e89-eebd-4455-a8de-9e59040e7347\7648efa3-dd9c-4e3e-b566-50f929386280",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"4f971e89-eebd-4455-a8de-9e59040e7347\5ca83367-6e45-459f-a27b-476b1d01c936",
                "AC",
            ),
            "power.lid",
            Power,
            "Sleep and buttons",
            "Keep running with the lid shut",
            "A laptop keeps running on mains with the lid shut; on battery it still sleeps.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin()
        .details("Useful with an external monitor, keyboard and mouse. On battery, closing the lid still puts the laptop to sleep. Watch the heat if you leave it shut while it's working hard."),
        settings: &[
            power(
                r"4f971e89-eebd-4455-a8de-9e59040e7347\5ca83367-6e45-459f-a27b-476b1d01c936",
                "AC",
                0,
            ),
            power(
                r"4f971e89-eebd-4455-a8de-9e59040e7347\5ca83367-6e45-459f-a27b-476b1d01c936",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"7516b95f-f776-4464-8c53-06167f40cc99\3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e",
                "AC",
            ),
            "power.display",
            Power,
            "Display and devices",
            "Display never turns off",
            "The screen stays on until you turn it off, on mains and on battery.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("Handy for dashboards, long reads or watching a download. Windows no longer turns the display off when idle. It costs battery on laptops and can add burn-in risk on OLED screens."),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e",
                "AC",
                0,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"7516b95f-f776-4464-8c53-06167f40cc99\fbd9aa66-9553-4097-ba44-ed6e9d65eab8",
                "AC",
            ),
            "power.adaptive-brightness",
            Power,
            "Display and devices",
            "No adaptive brightness",
            "Screen brightness stays where you set it instead of following the room light.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin()
        .details("Turns off automatic brightness on PCs with a light sensor, mostly laptops. Useful if the screen keeps shifting during games or films. Desktops without a sensor see no change."),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\fbd9aa66-9553-4097-ba44-ed6e9d65eab8",
                "AC",
                0,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\fbd9aa66-9553-4097-ba44-ed6e9d65eab8",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"44f3beca-a7c0-460e-9df2-bb8b99e0cba6\3619c3f2-afb2-4afc-b0e9-e7fef372de36",
                "AC",
            ),
            "power.intel-graphics",
            Power,
            "Display and devices",
            "Full Intel graphics performance",
            "Stops Intel graphics holding back to save power, at the cost of battery life.",
            POWER_DOCS,
            Documented,
        )
        .hw(IntelGpu)
        .admin()
        .details("Sets the Intel graphics power plan to its highest setting, on mains and on battery. It only applies to Intel graphics. The gain is small, and laptops run down faster and can get warmer."),
        settings: &[
            power(
                r"44f3beca-a7c0-460e-9df2-bb8b99e0cba6\3619c3f2-afb2-4afc-b0e9-e7fef372de36",
                "AC",
                2,
            ),
            power(
                r"44f3beca-a7c0-460e-9df2-bb8b99e0cba6\3619c3f2-afb2-4afc-b0e9-e7fef372de36",
                "DC",
                2,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"19cbb8fa-5279-450e-9fac-8a3d5fedd0c1\12bbebe6-58d6-4636-95bb-3217ef867c1a",
                "AC",
            ),
            "power.wifi-power",
            Power,
            "Display and devices",
            "Wi-Fi power saving off",
            "Stops Wi-Fi saving power on battery, which can steady your connection.",
            POWER_DOCS,
            Documented,
        )
        .admin()
        .details("Power saving can add small, uneven delays on Wi-Fi. Windows already uses full power on mains in most plans, so this mainly matters on battery, where the laptop runs down a little faster."),
        settings: &[
            power(
                r"19cbb8fa-5279-450e-9fac-8a3d5fedd0c1\12bbebe6-58d6-4636-95bb-3217ef867c1a",
                "AC",
                0,
            ),
            power(
                r"19cbb8fa-5279-450e-9fac-8a3d5fedd0c1\12bbebe6-58d6-4636-95bb-3217ef867c1a",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "power.fast-startup",
            Power,
            "System",
            "Turn off fast startup",
            "Shutdown fully closes Windows, so each start is a clean one.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin()
        .details("Fast startup saves part of Windows to disk at shutdown. Turning it off can fix drivers that misbehave after a cold start and makes dual-booting safer. Starting the PC takes longer, mostly on hard drives."),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            0,
        )],
    },
];
