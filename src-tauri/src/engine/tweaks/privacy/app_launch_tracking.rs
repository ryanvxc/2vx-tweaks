use crate::engine::{
    BuildRange, DeviceScope, Hive, Impact, Permission, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.app-launch-tracking-off",
        name: "Stop tracking app launches",
        description: "Turns off \"Let Windows track app launches to improve Start and search results\" for your account.",
        details: "Sets Start_TrackProgs to 0 under HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced, \
                  as Microsoft documents for this setting. Windows stops recording which apps you start, so Start and \
                  search no longer rank results or show \"Most used\" from your launch history. This is the same \
                  switch as Settings > Privacy & security > General. It affects only your account. Sign out and back \
                  in to make sure Explorer picks it up.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        // Listed under the Windows 10, version 1703 options.
        supported_builds: BuildRange {
            min_build: 15_063,
            max_build: None,
        },
        device: DeviceScope::Any,
        recommended: false,
        restart: RestartRequirement::SignOut,
        impact: Impact::Low,
        added_in: "0.1",
        references: &[
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#181-general",
        ],
    },
    settings: &[Setting::dword(
        Hive::CurrentUser,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "Start_TrackProgs",
        0,
    )],
};
