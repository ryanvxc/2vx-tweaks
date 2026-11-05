use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.edge-recommendations-off",
        name: "Stop Microsoft Edge feature suggestions",
        description: "Stops Edge's feature pop-ups and banners.",
        details: "Sets the documented Edge policy ShowRecommendationsEnabled to 0 (HKLM\\SOFTWARE\\Policies\\Microsoft\\Edge). Edge then stops showing in-browser assistance notifications, for example the prompt to try vertical tabs when many tabs are open. Features keep working; only the suggestions stop. Applies without restarting Edge. Because this is a machine policy, Edge shows the option as managed and may show \"Your browser is managed by your organization\". Reverting removes the value; the empty policy key is left in place.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: true,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/showrecommendationsenabled",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "ShowRecommendationsEnabled",
        0,
    )],
};
