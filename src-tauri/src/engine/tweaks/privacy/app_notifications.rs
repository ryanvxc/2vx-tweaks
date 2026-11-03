use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.app-notification-access-off",
        name: "Block app access to notifications",
        description: "Stops Store apps reading the notifications on this PC.",
        details: "Sets the \"Let Windows apps access notifications\" policy to Force Deny \
                  (LetAppsAccessNotifications = 2 under HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\AppPrivacy), \
                  the most restrictive value Microsoft documents. Every user on this PC is affected, and the \
                  matching switch in Settings > Privacy & security is locked while the policy is set. An app that \
                  is open keeps its access until it restarts. Microsoft lists this policy for Pro and above, so it \
                  isn't offered on Home.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange {
            min_build: 15_063,
            max_build: None,
        },
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: Some("Apps that mirror or read your notifications stop working."),
        requires: None,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#letappsaccessnotifications",
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
        "LetAppsAccessNotifications",
        2,
    )],
};
