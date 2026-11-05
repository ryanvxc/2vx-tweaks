//! Cpu tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7",
                "AC",
            ),
            "cpu.boost-mode",
            Cpu,
            "Power plan",
            "Aggressive boost mode",
            "Lets the CPU reach its turbo speeds sooner while plugged in.",
            POWER_DOCS,
            Documented,
        )
        .admin()
        .details("Sets turbo behaviour on the active power plan to aggressive on mains and normal on battery. The difference is small and some plans already use it. Expect slightly more heat and power use."),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7",
                "AC",
                2,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\893dee8e-2bef-41e0-89c6-b55d0929964c",
                "AC",
            ),
            "cpu.min-state",
            Cpu,
            "Power plan",
            "Raise minimum CPU speed",
            "Stops the CPU dropping to its lowest speed while plugged in.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .desktop()
        .admin()
        .details("Raises the minimum processor state to 85% on mains and leaves battery at 5%. Clocks respond a little faster after idle, at the cost of more idle power and heat. Many people will not notice a difference."),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\893dee8e-2bef-41e0-89c6-b55d0929964c",
                "AC",
                85,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\893dee8e-2bef-41e0-89c6-b55d0929964c",
                "DC",
                5,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\36687f9e-e3a5-4dbf-b1dc-15eb381c6863",
                "AC",
            ),
            "cpu.epp",
            Cpu,
            "Power plan",
            "Energy preference: performance",
            "Tells the CPU to favour speed over power saving while plugged in.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .desktop()
        .admin()
        .details("Sets the energy preference on the active plan to full performance on mains. Clocks rise faster and fall more slowly, so the PC uses more power and runs a little warmer. It needs a CPU that manages its own clocks."),
        settings: &[power(
            r"54533251-82be-4824-96c1-47b60b740d00\36687f9e-e3a5-4dbf-b1dc-15eb381c6863",
            "AC",
            0,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583",
                "AC",
            ),
            "cpu.parking-min",
            Cpu,
            "Power plan",
            "Keep all CPU cores awake",
            "Keeps all CPU cores awake while plugged in instead of parking spare ones.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin()
        .details("Sets minimum unparked cores to 100% on mains and 10% on battery. It avoids a small delay when parked cores wake, but uses more idle power. Leave it alone on chips that depend on parking.")
        .warn("Dual-CCD Ryzen X3D chips use core parking to keep games on the cache cores."),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583",
                "DC",
                10,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "cpu.global-timer",
            Cpu,
            "Scheduler and timers",
            "Make timer requests global",
            "Makes one app's high-precision timer request apply to the whole PC.",
            BATLEZ,
            Observed,
        )
        .admin()
        .reboot()
        .details("Brings back the older shared timer behaviour on Windows 11, so a game or timer tool's request reaches every app. Frame limiters may be slightly more even, but idle power use rises. Windows 11 only, and it needs a restart."),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "GlobalTimerResolutionRequests",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.wpbt",
            Windows,
            "Startup and shutdown",
            "Block firmware-installed apps",
            "Stops firmware making Windows run a vendor program at every boot.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Vendor tools installed by firmware stop being added.")
        .admin()
        .details("PC makers can use a firmware table to have Windows run their program at startup, which can bring back vendor software you removed. This turns that off. Microsoft does not document the switch.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "DisableWpbtExecution",
            1,
        )],
    },
];
