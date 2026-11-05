//! Memory tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "memory.large-system-cache",
            Memory,
            "Paging and cache",
            "Large system cache",
            "Favours the file cache over apps.",
            BATLEZ,
            Community,
        )
        .medium()
        .warn("Games can have less memory available.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "LargeSystemCache",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.page-combining",
            Memory,
            "Paging and cache",
            "Memory page combining",
            "Stops Windows merging identical pages.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "DisablePageCombining",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.clear-pagefile",
            Memory,
            "Paging and cache",
            "Keep page file at shutdown",
            "Skips wiping the page file, for faster shutdown.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "ClearPageFileAtShutdown",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.svchost-split",
            Memory,
            "Paging and cache",
            "Service grouping threshold",
            "Groups services into fewer processes.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control",
            "SvcHostSplitThresholdInKB",
            33554432,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.prefetcher",
            Memory,
            "Prefetch",
            "Prefetcher",
            "Stops Windows preloading app files.",
            BATLEZ,
            Observed,
        )
        .medium()
        .impact(Medium)
        .warn("Apps start slower, mostly on hard drives.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "EnablePrefetcher",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.superfetch",
            Memory,
            "Prefetch",
            "Superfetch parameter",
            "Stops memory preloading.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "EnableSuperfetch",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.sf-tracing",
            Memory,
            "Prefetch",
            "Superfetch tracing",
            "Stops Superfetch tracing.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "SfTracingState",
            0,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Ndu",
                "ImagePath",
            ),
            "memory.compat-ndu",
            Memory,
            "Prefetch",
            "Network Data Usage driver",
            "Stops the driver known for non-paged pool growth.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("The Data usage page stops counting.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\Ndu", "Start", 4)],
    },
];
