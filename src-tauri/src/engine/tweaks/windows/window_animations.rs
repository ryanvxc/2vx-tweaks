use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.window-animations-off",
        name: "Turn off window animations",
        description: "Stops the animations when windows open, minimise and maximise.",
        details: "Sets the \"Do not allow window animations\" policy (DisallowAnimations = 1 under \
                  HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DWM). Microsoft says it needs a sign-out to apply, \
                  and lists it for Pro and above, so it isn't offered on Home.",
        category: TweakCategory::Windows,
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
        section: None,
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
