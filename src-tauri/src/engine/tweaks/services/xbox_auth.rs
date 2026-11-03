use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.xbox-auth-off",
        name: "Disable Xbox Live Auth Manager",
        description: "Turns off the service that signs you in to Xbox services.",
        details: "Sets the start type of the XblAuthManager service to Disabled (Start = 4 under \
                  HKLM\\SYSTEM\\CurrentControlSet\\Services\\XblAuthManager). Microsoft's guidance is about \
                  servers, where nobody plays games; this PC may differ. Microsoft's guidance on this service was \
                  written for Windows Server 2016 with Desktop Experience, the closest published guidance; there \
                  is no equivalent for Windows 10 or 11. The service stops at the next restart, and undoing this \
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
        warning: Some("Xbox sign-in fails, which breaks the Xbox app and Game Pass games."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\XblAuthManager",
            "ImagePath",
        )),
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\XblAuthManager",
        "Start",
        4,
    )],
};
