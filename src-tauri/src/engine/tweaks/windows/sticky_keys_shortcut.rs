use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.sticky-keys-shortcut-off",
        name: "Turn off Sticky Keys shortcut",
        description: "Pressing Shift five times no longer brings up the Sticky Keys prompt.",
        details: "A common interruption in games that lean on Shift. Sticky Keys itself still works if you turn it on in Accessibility settings.",
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
            "https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-stickykeys",
        ],
    },
    settings: &[Setting::string(
        Hive::CurrentUser,
        r"Control Panel\Accessibility\StickyKeys",
        "Flags",
        "506",
    )],
};
