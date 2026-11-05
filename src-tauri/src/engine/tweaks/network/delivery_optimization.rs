use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.delivery-optimization-http",
        name: "Updates from Microsoft only",
        description: "Stops Windows sharing update files with other PCs on the internet or your network.",
        details: "Windows downloads updates from Microsoft only and no longer uploads them to other PCs, which can free up upload bandwidth. On a home network with several PCs, each one will download updates itself.",
        category: TweakCategory::Network,
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
            "https://learn.microsoft.com/en-us/windows/deployment/do/waas-delivery-optimization-reference",
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-deliveryoptimization",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",
        "DODownloadMode",
        0,
    )],
};
