use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.inking-typing-collection-off",
        name: "Stop learning from typing and inking",
        description: "Stops Windows learning from what you type and write.",
        details: "Sets RestrictImplicitTextCollection and RestrictImplicitInkCollection to 1 under \
                  HKCU\\Software\\Microsoft\\InputPersonalization, as Microsoft documents in section 18.21 of its \
                  connections guide. It affects only your account. Sign out and back in so input services pick it \
                  up.",
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
