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
            "Lets the CPU boost sooner on the active plan.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
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
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "AC",
            ),
            "cpu.boost-policy",
            Cpu,
            "Power plan",
            "Boost policy",
            "Raises how much the CPU may boost.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "DC",
                60,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "AC",
            ),
            "cpu.max-state",
            Cpu,
            "Power plan",
            "Maximum processor state",
            "Lets the CPU reach full speed.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "DC",
                90,
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
            "Minimum processor state",
            "Keeps the CPU at a higher floor when plugged in.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .impact(Medium)
        .desktop()
        .admin(),
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
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "AC",
            ),
            "cpu.idle-disable",
            Cpu,
            "Power plan",
            "Processor idle disable",
            "Stops C-states so the CPU never sleeps.",
            POWER_DOCS,
            Documented,
        )
        .high()
        .impact(Medium)
        .desktop()
        .warn("The CPU runs hot and draws full idle power.")
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "AC",
                1,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\94d3a615-a899-4ac5-ae2b-e4d8f634367f",
                "AC",
            ),
            "cpu.cooling-active",
            Cpu,
            "Power plan",
            "Active cooling policy",
            "Speeds up fans before slowing the CPU.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
        settings: &[power(
            r"54533251-82be-4824-96c1-47b60b740d00\94d3a615-a899-4ac5-ae2b-e4d8f634367f",
            "AC",
            1,
        )],
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
            "Tells the CPU to favour speed over power.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .desktop()
        .admin(),
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
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "AC",
            ),
            "cpu.parking-max",
            Cpu,
            "Power plan",
            "Core parking max cores",
            "Allows every core to stay awake.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "DC",
                100,
            ),
        ],
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
            "Core parking min cores",
            "Keeps every core awake when plugged in.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .desktop()
        .admin(),
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
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "AC",
            ),
            "cpu.idle-demote",
            Cpu,
            "Power plan",
            "Idle demote threshold",
            "Moves into deep idle less readily.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "AC",
                5,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "DC",
                30,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "AC",
            ),
            "cpu.interrupt-steering",
            Cpu,
            "Power plan",
            "Interrupt steering mode",
            "Changes how interrupts are shared across cores.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[
            power(
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "AC",
                3,
            ),
            power(
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "DC",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-watchdog",
            Cpu,
            "Scheduler and timers",
            "DPC watchdog period",
            "Removes the DPC watchdog limit.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcWatchdogPeriod",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-timeout",
            Cpu,
            "Scheduler and timers",
            "DPC timeout",
            "Removes the per-DPC time limit.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcTimeout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-watchdog-offset",
            Cpu,
            "Scheduler and timers",
            "DPC watchdog offset",
            "Removes the watchdog profile offset.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcWatchdogProfileOffset",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.ideal-dpc-rate",
            Cpu,
            "Scheduler and timers",
            "Ideal DPC rate",
            "Tunes when DPCs run.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "IdealDpcRate",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.minimum-dpc-rate",
            Cpu,
            "Scheduler and timers",
            "Minimum DPC rate",
            "Tunes how often DPCs drain.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MinimumDpcRate",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.threaded-dpc",
            Cpu,
            "Scheduler and timers",
            "Threaded DPCs",
            "Runs DPCs at normal interrupt level.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "ThreadDpcEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.global-timer",
            Cpu,
            "Scheduler and timers",
            "Global timer requests",
            "Lets one app's timer request apply system-wide.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "GlobalTimerResolutionRequests",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.serialize-timer",
            Cpu,
            "Scheduler and timers",
            "Serialize timer expiration",
            "Runs timers on one core.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "SerializeTimerExpiration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.low-qos-timer",
            Cpu,
            "Scheduler and timers",
            "Low QoS timer resolution",
            "Stops Windows slowing timers for background apps.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DisableLowQosTimerResolution",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.ready-queue",
            Cpu,
            "Scheduler and timers",
            "Shared ready queue size",
            "Changes the size of the scheduler's ready queue.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MaximumSharedReadyQueueSize",
            128,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dynamic-tick-duration",
            Cpu,
            "Scheduler and timers",
            "Dynamic tick duration",
            "Shortens how long the tick can stop.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MaxDynamicTickDuration",
            10,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.per-cpu-clock",
            Cpu,
            "Scheduler and timers",
            "Per-CPU clock ticks",
            "Changes how clock ticks are scheduled.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "EnablePerCpuClockTickScheduling",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.timer-check",
            Cpu,
            "Scheduler and timers",
            "Timer check flags",
            "Turns off extra timer checks.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "TimerCheckFlags",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.debug-poll",
            Cpu,
            "Scheduler and timers",
            "Debug poll interval",
            "Lengthens kernel debugger polling.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "DebugPollInterval",
            1000,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.coalescing",
            Cpu,
            "Scheduler and timers",
            "Timer coalescing",
            "Stops Windows batching timers to save power.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\ModernSleep",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Executive",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                "CoalescingTimerInterval",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "cpu.split-large-caches",
            Cpu,
            "Scheduler and timers",
            "Split large caches",
            "Treats large L3 caches as separate groups.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "SplitLargeCaches",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.adjust-dpc",
            Cpu,
            "Scheduler and timers",
            "DPC threshold",
            "Tunes when DPC work is rebalanced.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "AdjustDpcThreshold",
            800,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.deep-io-coalescing",
            Cpu,
            "Scheduler and timers",
            "Deep I/O coalescing",
            "Groups I/O completions in deep idle.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "DeepIoCoalescingEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.scheduler-assist",
            Cpu,
            "Scheduler and timers",
            "Scheduler assist override",
            "Overrides a scheduler hint flag.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "SchedulerAssistThreadFlagOverride",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.priority-dpc",
            Cpu,
            "Scheduler and timers",
            "Ideal DPC rate (priority key)",
            "Same idea, under PriorityControl.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "IdealDpcRate",
            800,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.passive-int",
            Cpu,
            "Scheduler and timers",
            "Passive interrupt priority",
            "Raises the priority of passive interrupt workers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\I/O System",
            "PassiveIntRealTimeWorkerPriority",
            18,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.wpbt",
            Cpu,
            "Scheduler and timers",
            "Platform binary table",
            "Stops firmware running a program at boot.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Vendor tools installed by firmware stop being added.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "DisableWpbtExecution",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.tick-power",
            Cpu,
            "Scheduler and timers",
            "Exit latency check",
            "Turns on the power exit latency check.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "ExitLatencyCheckEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.exit-latency",
            Cpu,
            "Scheduler and timers",
            "Exit latency",
            "Lowers the assumed idle exit latency.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "ExitLatency",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.unpark-count",
            Cpu,
            "Scheduler and timers",
            "Initial unpark count",
            "Starts Class 1 cores unparked.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "Class1InitialUnparkCount",
            100,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.energy-estimation",
            Cpu,
            "Scheduler and timers",
            "Energy estimation",
            "Stops Windows estimating power per app.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "EnergyEstimationEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.event-processor",
            Cpu,
            "Scheduler and timers",
            "Power event processor",
            "Stops the power event logger.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "EventProcessorEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.tagged-energy",
            Cpu,
            "Scheduler and timers",
            "Tagged energy logging",
            "Stops per-app energy logs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "DisableTaggedEnergyLogging",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "TelemetryMaxApplication",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "TelemetryMaxTagPerApplication",
                0,
            ),
        ],
    },
];
