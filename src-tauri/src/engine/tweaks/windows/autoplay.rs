use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.autoplay-off",
        name: "Turn off AutoPlay",
        description: "Nothing opens or runs by itself when you plug in a drive or insert a disc.",
        details: "Sets NoDriveTypeAutoRun to 255 under the Explorer policy key, which turns AutoPlay off for every drive type.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/troubleshoot/windows-client/shell-experience/issues-autoplay-disabled-group-policy",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoDriveTypeAutoRun",
        255,
    )],
};
