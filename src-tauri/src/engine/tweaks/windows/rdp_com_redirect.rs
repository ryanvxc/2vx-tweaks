use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.rdp-com-redirect-off",
        name: "Block COM port redirection",
        description: "Remote Desktop sessions on this PC can't use COM ports on the PC connecting in.",
        details: "Sets fDisableCcm to 1, the Do not allow COM port redirection policy. It changes nothing unless someone connects to this PC with Remote Desktop.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-terminalserver",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
        "fDisableCcm",
        1,
    )],
};
