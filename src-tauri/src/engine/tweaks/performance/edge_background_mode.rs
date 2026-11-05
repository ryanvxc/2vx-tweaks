use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.edge-background-mode-off",
        name: "Stop Microsoft Edge running in the background",
        description: "Stops Edge running after you close its last window.",
        details: "Sets the documented Edge policy BackgroundModeEnabled to 0 (HKLM\\SOFTWARE\\Policies\\Microsoft\\Edge). \
                  Closing the last Edge window then ends Edge, including extensions and web apps that would otherwise \
                  keep running (for example to show notifications). Edge already defaults to off for most users; this \
                  makes it fixed. Because this is a machine policy, edge://settings shows the option as managed and \
                  Edge may show \"Your browser is managed by your organization\". Takes effect the next time Edge \
                  starts. Reverting removes the value; the empty policy key is left in place.",
        category: TweakCategory::Performance,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.1",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/backgroundmodeenabled",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "BackgroundModeEnabled",
        0,
    )],
};
