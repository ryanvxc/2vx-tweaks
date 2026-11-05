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
            "Turn off hard disk after",
            "Never spins disks down.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
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
            "AHCI link power",
            "Keeps SATA links active.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "AC",
            ),
            "power.nvme-noppme",
            Power,
            "Disk",
            "NVMe NOPPME",
            "Turns off non-operational power states on NVMe.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "AC",
                1,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "DC",
                1,
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
            "NVMe primary idle timeout",
            "Stops the first NVMe idle transition.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
            "NVMe secondary idle timeout",
            "Stops the deeper NVMe idle transition.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "AC",
            ),
            "power.usb-selective",
            Power,
            "USB and PCIe",
            "USB selective suspend (plan)",
            "Keeps USB devices powered on this plan.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "AC",
                0,
            ),
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "DC",
                1,
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
            "USB 3 link power",
            "Keeps USB 3 links active.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "AC",
            ),
            "power.usb-ioc",
            Power,
            "USB and PCIe",
            "USB IOC on all TDs",
            "Interrupts on every USB transfer.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .desktop()
        .admin(),
        settings: &[
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "AC",
                1,
            ),
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "DC",
                1,
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
            "PCIe link power",
            "Keeps PCIe links active when plugged in.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
            "Stops the PC sleeping on its own.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
            "Stops the PC hibernating on its own.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "AC",
            ),
            "power.away-mode",
            Power,
            "Sleep and buttons",
            "Away mode",
            "Allows media-style away mode.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "AC",
                1,
            ),
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "AC",
            ),
            "power.standby-network",
            Power,
            "Sleep and buttons",
            "Network in standby",
            "Keeps the network on in standby.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "AC",
                1,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "AC",
            ),
            "power.device-idle",
            Power,
            "Sleep and buttons",
            "Device idle policy",
            "Favours performance over idle savings.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "AC",
                0,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "AC",
            ),
            "power.personality",
            Power,
            "Sleep and buttons",
            "Plan personality",
            "Marks the plan as high performance.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "AC",
                1,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "DC",
                1,
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
            "Stops the power button shutting down.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .warn("Hold the button to force a shutdown.")
        .admin(),
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
            "Lid close when plugged in",
            "Keeps running with the lid closed on mains.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
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
            "Keeps the screen on.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
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
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "AC",
            ),
            "power.brightness",
            Power,
            "Display and devices",
            "Full display brightness",
            "Uses full brightness on this plan.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "AC",
                100,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "DC",
                100,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "AC",
            ),
            "power.dim-brightness",
            Power,
            "Display and devices",
            "No dimming",
            "Keeps full brightness when dimmed.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "AC",
                100,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "DC",
                100,
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
            "Adaptive brightness",
            "Stops brightness following the room.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
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
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "AC",
            ),
            "power.gpu-preference",
            Power,
            "Display and devices",
            "GPU preference policy",
            "Prefers the high-performance GPU.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "AC",
                2,
            ),
            power(
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "DC",
                2,
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
            "Intel graphics performance",
            "Sets Intel graphics to maximum performance.",
            POWER_DOCS,
            Documented,
        )
        .hw(IntelGpu)
        .admin(),
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
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "AC",
            ),
            "power.timer-frequency",
            Power,
            "Display and devices",
            "JavaScript timer frequency",
            "Runs browser timers at full rate.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "AC",
                1,
            ),
            power(
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "DC",
                1,
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
            "Wi-Fi power saving",
            "Runs Wi-Fi at full power.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
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
            "power.platform-aoac",
            Power,
            "System",
            "Modern Standby override",
            "Asks firmware for S3 sleep on supported PCs.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Sleep may stop working if the firmware has no S3.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "PlatformAoAcOverride",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.cs-enabled",
            Power,
            "System",
            "Connected standby",
            "Turns off connected standby.",
            BATLEZ,
            Community,
        )
        .medium()
        .reboot()
        .warn("Sleep may stop working if the firmware has no S3.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "CsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.fast-startup",
            Power,
            "System",
            "Fast startup",
            "Stops hybrid shutdown.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
            "HiberbootEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.startup-sound",
            Power,
            "System",
            "Startup sound",
            "Turns off the startup sound.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\EditionOverrides",
            "UserSetting_DisableStartupSound",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.custom-setup",
            Power,
            "System",
            "Plan customisation at setup",
            "Lets setup create default plans.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "CustomizeDuringSetup",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.platform-role",
            Power,
            "System",
            "Platform role",
            "Treats this PC as a desktop for power policy.",
            BATLEZ,
            Observed,
        )
        .medium()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "PlatformRole",
            1,
        )],
    },
];
