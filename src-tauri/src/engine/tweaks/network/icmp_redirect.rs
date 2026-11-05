use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.icmp-redirect-off",
        name: "Ignore ICMP redirects",
        description: "Stops a device on the network changing this PC's routes with a redirect packet.",
        details: "Sets EnableICMPRedirect to 0 under Tcpip\\Parameters, as Microsoft's hardening guide recommends, so Windows doesn't create host routes from ICMP redirects.",
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
            "https://learn.microsoft.com/en-us/security-updates/windowsupdateservices/appendixelistofsecuritysettings-21741280",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
        "EnableICMPRedirect",
        0,
    )],
};
