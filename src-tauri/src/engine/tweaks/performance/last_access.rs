use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.last-access-off",
        name: "Stop updating last-access times",
        description: "Stops NTFS noting the time every file or folder is read, saving small disk writes.",
        details: "Windows can update a timestamp each time a file is read. Turning that off saves a small amount of disk work, mostly in large folders. Many modern PCs already handle this automatically.",
        category: TweakCategory::Performance,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some(
            "Backup, archive and cleanup tools that sort by last access can misjudge files.",
        ),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/fsutil-behavior",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\FileSystem",
        "NtfsDisableLastAccessUpdate",
        0x8000_0001,
    )],
};
