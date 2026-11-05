use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.fault-tolerant-heap-off",
        name: "Disable Fault Tolerant Heap",
        description: "Stops Windows applying crash workarounds to apps that keep crashing, which can slow them.",
        details: "Sets Enabled to 0 under HKLM\\SOFTWARE\\Microsoft\\FTH, as Microsoft documents. FTH is a reliability feature, not a security one. An app that crashes often may crash more without it.",
        category: TweakCategory::Performance,
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
            "https://learn.microsoft.com/en-us/windows/win32/win7appqual/fault-tolerant-heap",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\FTH",
        "Enabled",
        0,
    )],
};
