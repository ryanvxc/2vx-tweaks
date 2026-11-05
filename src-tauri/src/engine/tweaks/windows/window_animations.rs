use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.window-animations-off",
        name: "Turn off window animations",
        description: "Stops the animations when windows open, close, minimise and maximise.",
        details: "Windows appear and disappear instantly, which can feel quicker and helps if motion bothers you. Sign out to apply it. It applies to every account on this PC. It covers the desktop's window effects, not animations inside apps.",
        category: TweakCategory::Visuals,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::SignOut,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: Some("Effects and animation"),
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-dwm#dwmdisallowanimations_2",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\DWM",
        "DisallowAnimations",
        1,
    )],
};
