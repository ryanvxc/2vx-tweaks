use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "gaming.game-dvr-off",
        name: "Turn off Game DVR",
        description: "Stops Windows recording and broadcasting games in the background.",
        details: "Sets the Windows policy that turns off game recording and broadcasting, so Game Bar capture no longer runs. Worth doing if you never use it. Clips, screenshots and broadcasts through Game Bar stop working.",
        category: TweakCategory::Gaming,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Medium,
        warning: Some("Game Bar recording, screenshots and broadcasting stop working."),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-applicationmanagement#allowgamedvr",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
        "AllowGameDVR",
        0,
    )],
};
