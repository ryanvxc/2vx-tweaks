use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.llmnr-off",
        name: "Disable LLMNR",
        description: "Stops Windows resolving names over the local network, a common spoofing target.",
        details: "Sets EnableMulticast to 0 under the DNS Client policy key, the Turn off multicast name resolution policy, which is on in Microsoft's security baselines. Names on your own network may stop resolving unless DNS or mDNS handles them.",
        category: TweakCategory::Network,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
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
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-dnsclient",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
        "EnableMulticast",
        0,
    )],
};
