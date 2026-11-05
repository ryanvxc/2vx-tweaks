//! Storage tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "storage.long-paths",
            Storage,
            "File system",
            "Long file paths",
            "Lets apps use paths beyond 260 characters.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "LongPathsEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.memoryusage",
            Storage,
            "File system",
            "NTFS memory usage",
            "Gives NTFS more paged pool for caching.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsMemoryUsage",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.mft-zone",
            Storage,
            "File system",
            "MFT zone",
            "Reserves more disk for the file table.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsMftZoneReservation",
            4,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.no-efs",
            Storage,
            "File system",
            "NTFS encryption",
            "Turns off the Encrypting File System.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .medium()
        .warn("EFS encrypted files can't be opened.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsDisableEncryption",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.no-paging-encryption",
            Storage,
            "File system",
            "Page file encryption",
            "Keeps the page file unencrypted.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Policies",
            "NtfsEncryptPagingFile",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.trim",
            Storage,
            "File system",
            "SSD TRIM",
            "Makes sure TRIM is on.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "DisableDeleteNotification",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.boot-optimize",
            Storage,
            "File system",
            "Boot file optimisation",
            "Keeps boot files together on hard drives.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Dfrg\BootOptimizeFunction",
            "Enable",
            "y",
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.autochk-timeout",
            Storage,
            "File system",
            "Disk check countdown",
            "Shortens the boot-time disk check countdown.",
            TAIRIKU,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "AutoChkTimeout",
            5,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.registry-backup",
            Storage,
            "File system",
            "Periodic registry backup",
            "Brings back the regular registry backup.",
            TAIRIKU,
            Documented,
        )
        .reboot()
        .admin(),
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
            "Reserved storage",
            "Stops Windows setting space aside for updates.",
            BATLEZ,
            Documented,
        )
        .medium()
        .reboot()
        .warn("Updates need free space you keep clear.")
        .admin(),
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
    SettingsTweak {
        meta: lib(
            "storage.ahci-dipm",
            Storage,
            "Drives",
            "AHCI device power saving",
            "Stops SATA drives entering low power.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\storahci\Parameters",
            "EnableDipm",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.ahci-hipm",
            Storage,
            "Drives",
            "AHCI host power saving",
            "Stops the SATA link entering low power.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\storahci\Parameters",
            "EnableHipm",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-logging",
            Storage,
            "Drives",
            "NVMe driver logging",
            "Stops the NVMe driver's logs.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
            "EnableLogging",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-queue",
            Storage,
            "Drives",
            "NVMe queue depth",
            "Raises the NVMe I/O queue.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "IoQueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "IoQueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "QueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "QueueDepth",
                64,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-requests",
            Storage,
            "Drives",
            "NVMe request limit",
            "Raises concurrent NVMe requests.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "NumberOfRequests",
                1500,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "NumberOfRequests",
                1500,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-arbitration",
            Storage,
            "Drives",
            "NVMe arbitration burst",
            "Raises commands per arbitration.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ArbitrationBurst",
                256,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ArbitrationBurst",
                256,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-queues",
            Storage,
            "Drives",
            "NVMe submission queues",
            "Sets the NVMe queue count.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "IoSubmissionQueueCount",
                3,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "IoSubmissionQueueCount",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-immediate",
            Storage,
            "Drives",
            "NVMe immediate data",
            "Sends small writes inline.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ImmediateData",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ImmediateData",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-hmb",
            Storage,
            "Drives",
            "NVMe host memory buffer",
            "Sets the memory the drive may borrow.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Drives without DRAM rely on this.")
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "HostMemoryBufferBytes",
                1500,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "HostMemoryBufferBytes",
                1500,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-flush",
            Storage,
            "Drives",
            "NVMe flush",
            "Forces flushes through.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ForceFlush",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ForceFlush",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-timeout",
            Storage,
            "Drives",
            "NVMe I/O timeout",
            "Waits longer before resetting a drive.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device",
            "IoTimeoutValue",
            255,
        )],
    },
];
