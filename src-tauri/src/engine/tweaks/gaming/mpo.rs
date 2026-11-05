use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "gaming.mpo-off",
        name: "Disable multiplane overlay",
        description: "Fixes flicker and stutter in desktop apps on some NVIDIA setups.",
        details: "Sets OverlayTestMode to 5 under HKLM\\SOFTWARE\\Microsoft\\Windows\\Dwm, the change in the mpo_disable.reg file NVIDIA published for flicker when resizing windows. Multiplane overlay is a display feature that saves power, so leave it on unless you have the problem.",
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
        references: &["https://nvidia.custhelp.com/app/answers/detail/a_id/5157"],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\Dwm",
        "OverlayTestMode",
        5,
    )],
};
