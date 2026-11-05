use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RegTarget, RestartRequirement,
    RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "services.compat-assistant-off",
        name: "Disable Compatibility Assistant",
        description: "Stops Windows watching the programs you run and offering compatibility fixes.",
        details: "The assistant tracks programs and suggests fixes when an older one misbehaves. Turning it off stops that tracking and the pop-ups. The saving is tiny, and an old program that needs a fix won't be offered one.",
        category: TweakCategory::Services,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some("Windows stops offering compatibility fixes for older programs."),
        requires: Some(RegTarget::new(
            Hive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\PcaSvc",
            "ImagePath",
        )),
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/security/windows-services/security-guidelines-for-disabling-system-services-in-windows-server",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\PcaSvc",
        "Start",
        4,
    )],
};
