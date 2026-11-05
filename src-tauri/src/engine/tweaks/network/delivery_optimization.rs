use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.delivery-optimization-http",
        name: "Updates from Microsoft only",
        description: "Stops Windows sharing update files with other PCs.",
        details: "Sets the Delivery Optimization download mode to 0, HTTP only: peer-to-peer caching is off and downloads come from the original source. It doesn't touch Windows Update itself.",
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
