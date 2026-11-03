use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.wap-push-off",
        name: "Disable WAP push routing",
        description: "Turns off a service used to route device management messages.",
        details: "Sets the start type of the dmwappushservice service to Disabled (Start = 4 under \
                  HKLM\\SYSTEM\\CurrentControlSet\\Services\\dmwappushservice). It carries device management \
                  messages, which a personal PC doesn't get. Microsoft's guidance on this service was written for \
                  Windows Server 2016 with Desktop Experience, the closest published guidance; there is no \
                  equivalent for Windows 10 or 11. The service stops at the next restart, and undoing this \
                  restores the recorded start type.",
        category: TweakCategory::Services,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some("Work or school device management can fail on this PC."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\dmwappushservice",
            "ImagePath",
        )),
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\dmwappushservice",
        "Start",
        4,
    )],
};
