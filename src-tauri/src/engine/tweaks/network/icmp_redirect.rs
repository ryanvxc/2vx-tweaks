use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.icmp-redirect-off",
        name: "Ignore ICMP redirects",
        description: "Stops devices on your network changing this PC's routes with redirects.",
        details: "Redirect packets can send your traffic through a different device. Home PCs rarely need them, so this is a low-cost security setting from Windows hardening guides. Complex office networks may rely on them.",
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
