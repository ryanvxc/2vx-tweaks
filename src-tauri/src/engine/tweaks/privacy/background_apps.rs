use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.background-apps-off",
        name: "Block background apps",
        description: "Stops Store apps running in the background for every user.",
        details: "Sets the \"Let Windows apps run in the background\" policy to Force Deny (LetAppsRunInBackground \
                  = 2 under HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\AppPrivacy), the most restrictive value \
                  Microsoft documents. Every user on this PC is affected, and the matching switch in Settings > \
                  Privacy & security is locked while the policy is set. An app that is open keeps its access until \
                  it restarts. Microsoft lists this policy for Pro and above, so it isn't offered on Home.",
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
        impact: Impact::Medium,
        warning: Some(
            "Message, email and voicemail notifications and background mail sync can stop working.",
        ),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy#letappsruninbackground",
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
        "LetAppsRunInBackground",
        2,
    )],
};
