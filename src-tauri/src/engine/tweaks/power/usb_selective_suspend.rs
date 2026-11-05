use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "power.usb-selective-suspend-off",
        name: "Disable USB selective suspend",
        description: "Keeps USB devices powered, a fix for dropouts. Uses more battery.",
        details: "Sets DisableSelectiveSuspend to 1 under HKLM\\SYSTEM\\CurrentControlSet\\Services\\USB, for every USB controller. Microsoft documents this as a workaround for devices that stop working, so turn it on if yours do.",
        category: TweakCategory::Power,
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
            "https://support.microsoft.com/en-us/help/968132/usb-devices-may-stop-functioning-correctly-when-multiple-devices-conne",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\USB",
        "DisableSelectiveSuspend",
        1,
    )],
};
