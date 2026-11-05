use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "power.power-throttling-off",
        name: "Disable power throttling",
        description: "Stops Windows slowing background apps. Uses more battery on a laptop.",
        details: "Sets PowerThrottlingOff to 1 under HKLM\\SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling, the value behind the Turn off Power Throttling policy. Efficiency mode in Task Manager may still apply to apps that ask for it.",
        category: TweakCategory::Power,
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
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-power",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
        "PowerThrottlingOff",
        1,
    )],
};
