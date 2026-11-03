use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "gaming.mmcss-games-priority",
        name: "Raise game priority in MMCSS",
        description: "Gives games that register with the Multimedia Class Scheduler a higher thread priority.",
        details: "Sets Priority to 6 and Scheduling Category to High under the MMCSS Games task \
                  (HKLM\\SOFTWARE\\Microsoft\\Windows \
                  NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games), both documented values. It only \
                  reaches games that register their threads with MMCSS, and many don't. Those threads then run \
                  above the Audio task's Medium category. 2VX hasn't measured the effect on any game.",
        category: TweakCategory::Gaming,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Medium,
        warning: None,
        requires: None,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service",
        ],
    },
    settings: &[
        Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
            "Priority",
            6,
        ),
        Setting::string(
            Hive::LocalMachine,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
            "Scheduling Category",
            "High",
        ),
    ],
};
