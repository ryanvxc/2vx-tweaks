use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.activity-history-off",
        name: "Turn off activity history",
        description: "Stops Windows recording and uploading the apps, files and sites you use.",
        details: "Sets three documented policies to 0 under HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\System: \
                  EnableActivityFeed, PublishUserActivities and UploadUserActivities. Windows stops publishing \
                  activities and stops uploading them. Microsoft lists these policies for Pro and above, so this \
                  isn't offered on Home.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange {
            min_build: 17_134,
            max_build: None,
        },
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#enableactivityfeed",
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#publishuseractivities",
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#uploaduseractivities",
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
        ],
    },
    settings: &[
        Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "EnableActivityFeed",
            0,
        ),
        Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "PublishUserActivities",
            0,
        ),
        Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "UploadUserActivities",
            0,
        ),
    ],
};
