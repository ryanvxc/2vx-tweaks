//! Memory tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "memory.svchost-split",
            Memory,
            "Paging and cache",
            "Group background services",
            "Groups background services into fewer processes, which saves a little memory.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Windows normally runs most services separately on PCs with plenty of RAM. This groups them again on PCs with under 32 GB. The catch: one crash can take several services down with it.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control",
            "SvcHostSplitThresholdInKB",
            33554432,
        )],
    },
];
