use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.inking-typing-collection-off",
        name: "Stop typing and ink learning",
        description: "Stops Windows building a personal dictionary from what you type and write.",
        details: "Windows collects words you type and write to improve suggestions and handwriting recognition. This turns that off, so suggestions may become less accurate. Takes effect after signing out.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: true,
        restart: RestartRequirement::SignOut,
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
            r"Software\Microsoft\InputPersonalization",
            "RestrictImplicitTextCollection",
            1,
        ),
        Setting::dword(
            Hive::CurrentUser,
            r"Software\Microsoft\InputPersonalization",
            "RestrictImplicitInkCollection",
            1,
        ),
    ],
};
