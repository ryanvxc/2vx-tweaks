use crate::engine::{
    BuildRange, DeviceScope, Hive, Permission, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.language-list-opt-out",
        name: "Don't share your language list with websites",
        description: "Turns off \"Let websites show me locally relevant content by accessing my language list\" for \
                      your account.",
        details: "Sets HttpAcceptLanguageOptOut to 1 under HKCU\\Control Panel\\International\\User Profile, as Microsoft \
                  documents for this setting. Windows stops offering your installed language list to websites and \
                  apps, which then see only your primary display language and may pick a different language or \
                  region for their content. This is the same switch as Settings > Privacy & security > General. It \
                  affects only your account and applies to apps started afterwards.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        references: &[
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#181-general",
        ],
    },
    settings: &[Setting::dword(
        Hive::CurrentUser,
        r"Control Panel\International\User Profile",
        "HttpAcceptLanguageOptOut",
        1,
    )],
};
