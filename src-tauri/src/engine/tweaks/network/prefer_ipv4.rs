use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.prefer-ipv4",
        name: "Prefer IPv4 over IPv6",
        description: "Uses IPv4 when a site offers both. Microsoft's advice over disabling IPv6.",
        details: "Sets DisabledComponents to 0x20 under Tcpip6\\Parameters. IPv6 stays on. Worth trying only if your IPv6 route is worse than your IPv4 one.",
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
            "https://learn.microsoft.com/en-us/troubleshoot/windows-server/networking/configure-ipv6-in-windows",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters",
        "DisabledComponents",
        32,
    )],
};
