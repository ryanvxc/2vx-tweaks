use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.geolocation-off",
        name: "Disable Geolocation Service",
        description: "Cuts off location for all apps and Windows by stopping the service that provides it.",
        details: "A firmer block than the per-app location switches. Find my device and automatic time zone stop working, as does anything that locates you, such as local weather. On a desktop with a fixed time zone you are unlikely to miss it.",
        category: TweakCategory::Services,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some("Location, Find my device and automatic time zone stop working."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\lfsvc",
            "ImagePath",
        )),
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\lfsvc",
        "Start",
        4,
    )],
};
