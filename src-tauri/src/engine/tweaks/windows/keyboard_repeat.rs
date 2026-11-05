use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.keyboard-repeat-fast",
        name: "Fastest key repeat",
        description: "Held keys repeat sooner and faster, using the quickest rates Windows offers.",
        details: "Sets the shortest repeat delay and the fastest repeat rate, the same as the sliders in Keyboard Properties. Useful for typing, coding and holding keys in games. Some people find it too quick at first.",
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
            "https://learn.microsoft.com/en-us/dotnet/api/system.windows.forms.systeminformation.keyboardspeed",
            "https://learn.microsoft.com/en-us/dotnet/api/system.windows.forms.systeminformation.keyboarddelay",
        ],
    },
    settings: &[
        Setting::string(
            Hive::CurrentUser,
            r"Control Panel\Keyboard",
            "KeyboardDelay",
            "0",
        ),
        Setting::string(
            Hive::CurrentUser,
            r"Control Panel\Keyboard",
            "KeyboardSpeed",
            "31",
        ),
    ],
};
