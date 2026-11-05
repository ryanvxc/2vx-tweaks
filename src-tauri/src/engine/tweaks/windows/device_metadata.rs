use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.device-metadata-off",
        name: "Stop downloading device apps",
        description: "Windows no longer fetches apps and icons from the internet for new hardware.",
        details: "Sets PreventDeviceMetadataFromNetwork to 1, the Prevent device metadata retrieval from the Internet policy. Drivers still install.",
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
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-deviceinstallation",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata",
        "PreventDeviceMetadataFromNetwork",
        1,
    )],
};
