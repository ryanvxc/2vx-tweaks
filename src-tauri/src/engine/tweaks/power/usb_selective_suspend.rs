use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "power.usb-selective-suspend-off",
        name: "Disable USB selective suspend",
        description: "Keeps USB devices powered, which can fix dropouts and slow wake-ups.",
        details: "Windows can cut power to idle USB devices. If a mouse, headset or hub drops out or takes a moment to respond, this keeps it awake. It uses a little more battery on a laptop.",
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
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
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
