use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.storage-sense-off",
        name: "Turn off Storage Sense",
        description: "Stops Windows deleting temporary files and old downloads by itself.",
        details: "Sets 01 to 0 under \
                  HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StorageSense\\Parameters\\StoragePolicy, the \
                  value Microsoft gives in its Azure Virtual Desktop image guide. That guide covers Enterprise, \
                  and the matching policy is Pro and above, so this isn't offered on Home.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: Some("Windows stops freeing disk space automatically when it runs low."),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/azure/virtual-desktop/set-up-customize-master-image#disable-storage-sense",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
        "01",
        0,
    )],
};
