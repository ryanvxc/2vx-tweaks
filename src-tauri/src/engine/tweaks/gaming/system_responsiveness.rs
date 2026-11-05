use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "gaming.system-responsiveness",
        name: "Less CPU for background tasks",
        description: "Lets games and media that use MMCSS have 90% of the CPU instead of 80%.",
        details: "Sets SystemResponsiveness to 10 under the MMCSS SystemProfile key. Microsoft documents it as the share of CPU guaranteed to low-priority tasks, in tens, defaulting to 20. It only matters for programs that register with MMCSS, and 2VX hasn't measured the effect on any game.",
        category: TweakCategory::Gaming,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
        10,
    )],
};
