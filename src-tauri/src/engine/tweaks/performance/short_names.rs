use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.short-names-off",
        name: "Stop creating 8.3 file names",
        description: "New files skip the old DOS-style short name, saving work in big folders.",
        details: "Sets NtfsDisable8dot3NameCreation to 1 on every NTFS volume. Names that already exist stay. A very old program that only understands short names can have trouble with new files.",
        category: TweakCategory::Performance,
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
            "https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/fsutil-8dot3name",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\FileSystem",
        "NtfsDisable8dot3NameCreation",
        1,
    )],
};
