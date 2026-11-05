use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "network.smb-throttling-off",
        name: "Disable SMB throttling",
        description: "Stops Windows limiting transfers to network drives on slow links.",
        details: "Sets DisableBandwidthThrottling to 1 under LanmanWorkstation\\Parameters. Microsoft documents that this allows higher throughput on high-latency connections. It does nothing for games or browsing.",
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
            "https://learn.microsoft.com/en-us/windows-server/administration/performance-tuning/role/file-server/",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
        "DisableBandwidthThrottling",
        1,
    )],
};
