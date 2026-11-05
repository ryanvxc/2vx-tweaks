use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.diagtrack-off",
        name: "Disable Telemetry Service",
        description: "Stops the background service that sends diagnostic data to Microsoft.",
        details: "The Connected User Experiences and Telemetry service runs all the time and uploads the usage and reliability data Windows collects. This is the most direct way to stop that. Insider builds need it, and problem reports may not send.",
        category: TweakCategory::Services,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some("Windows Insider builds and problem reports can stop working."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\DiagTrack",
            "ImagePath",
        )),
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
            "https://learn.microsoft.com/en-us/windows/privacy/configure-windows-diagnostic-data-in-your-organization",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\DiagTrack",
        "Start",
        4,
    )],
};
