use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.feedback-requests-off",
        name: "Stop Windows feedback requests",
        description: "Stops the pop-ups that ask you to rate Windows and send feedback.",
        details: "Sets NumberOfSIUFInPeriod and PeriodInNanoSeconds to 0 under \
                  HKCU\\Software\\Microsoft\\Siuf\\Rules, as Microsoft documents in section 18.16 of its \
                  connections guide. This is \"Feedback frequency: Never\" in Settings. You can still open \
                  Feedback Hub yourself.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: true,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
        ],
    },
    settings: &[
        Setting::dword(
            Hive::CurrentUser,
            r"Software\Microsoft\Siuf\Rules",
            "NumberOfSIUFInPeriod",
            0,
        ),
        Setting::dword(
            Hive::CurrentUser,
            r"Software\Microsoft\Siuf\Rules",
            "PeriodInNanoSeconds",
            0,
        ),
    ],
};
