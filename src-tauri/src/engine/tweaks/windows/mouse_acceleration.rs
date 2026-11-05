use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.mouse-acceleration-off",
        name: "Disable mouse acceleration",
        description: "Pointer travel matches how far you move the mouse, whatever the speed.",
        details: "Same as unticking Enhance pointer precision in Mouse settings. Aiming and muscle memory get more consistent, which many gamers prefer. You may need to raise pointer speed or DPI to compensate.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::SignOut,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow",
            "https://techshelps.github.io/MSDN/DNWINNT/HTML/D1E/S8545.HTM",
        ],
    },
    settings: &[
        Setting::string(Hive::CurrentUser, r"Control Panel\Mouse", "MouseSpeed", "0"),
        Setting::string(
            Hive::CurrentUser,
            r"Control Panel\Mouse",
            "MouseThreshold1",
            "0",
        ),
        Setting::string(
            Hive::CurrentUser,
            r"Control Panel\Mouse",
            "MouseThreshold2",
            "0",
        ),
    ],
};
