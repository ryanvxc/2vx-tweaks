use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.online-speech-off",
        name: "Turn off online speech recognition",
        description: "Stops sending your voice to Microsoft's cloud speech service.",
        details: "Sets HasAccepted to 0 under \
                  HKCU\\Software\\Microsoft\\Speech_OneCore\\Settings\\OnlineSpeechPrivacy, as Microsoft documents \
                  in section 18.6 of its connections guide. This is the switch in Settings > Privacy & security > \
                  Speech. It affects only your account.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: Some("Voice typing and other cloud speech features stop working."),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
        ],
    },
    settings: &[Setting::dword(
        Hive::CurrentUser,
        r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
        "HasAccepted",
        0,
    )],
};
