use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.sensor-monitoring-off",
        name: "Disable Sensor Monitoring",
        description: "Turns off the service that adapts the PC to its sensors, such as ambient light.",
        details: "Sets the start type of the SensrSvc service to Disabled (Start = 4 under \
                  HKLM\\SYSTEM\\CurrentControlSet\\Services\\SensrSvc). It reacts to sensor readings, for example \
                  by changing screen brightness. Microsoft's guidance on this service was written for Windows \
                  Server 2016 with Desktop Experience, the closest published guidance; there is no equivalent for \
                  Windows 10 or 11. The service stops at the next restart, and undoing this restores the recorded \
                  start type.",
        category: TweakCategory::Services,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some("Adaptive brightness and other sensor-driven changes stop working."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\SensrSvc",
            "ImagePath",
        )),
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\SensrSvc",
        "Start",
        4,
    )],
};
