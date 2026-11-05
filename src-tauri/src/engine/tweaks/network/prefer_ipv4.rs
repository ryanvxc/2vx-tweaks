use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.prefer-ipv4",
        name: "Prefer IPv4 over IPv6",
        description: "Uses IPv4 when a site offers both IPv4 and IPv6.",
        details: "Microsoft's advice if IPv6 causes trouble, rather than disabling it. It can fix slow connections or timeouts on networks with poor IPv6 routing. If IPv6 works well for you, there is no reason to change it.",
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
