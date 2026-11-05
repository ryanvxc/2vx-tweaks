use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.app-launch-tracking-off",
        name: "Stop tracking app launches",
        description: "Stops Windows noting which apps you open to personalise Start and search.",
        details: "App launches normally feed the most-used list in Start and the ranking in search. After this, Windows stops logging them, so that list stops updating. Takes effect after signing out.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        // Listed under the Windows 10, version 1703 options.
        supported_builds: BuildRange {
            min_build: 15_063,
            max_build: None,
        },
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::SignOut,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.1",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
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
