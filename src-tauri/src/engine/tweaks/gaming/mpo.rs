use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "gaming.mpo-off",
        name: "Disable multiplane overlay",
        description: "Can stop flicker and stutter in desktop apps caused by multiplane overlays.",
        details: "NVIDIA published this workaround for apps that flicker after a driver update. Use it only if you see the problem. Reports differ on whether it still works on Windows 11 24H2 and later.",
        category: TweakCategory::Gaming,
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
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &["https://nvidia.custhelp.com/app/answers/detail/a_id/5157"],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\Dwm",
        "OverlayTestMode",
        5,
    )],
};
