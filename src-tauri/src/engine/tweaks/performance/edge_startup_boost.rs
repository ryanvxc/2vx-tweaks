use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.edge-startup-boost-off",
        name: "Turn off Microsoft Edge Startup boost",
        description: "Stops Edge from starting in the background when you sign in and from staying loaded after its \
                      last window closes.",
        details: "Sets the documented Edge policy StartupBoostEnabled to 0 (HKLM\\SOFTWARE\\Policies\\Microsoft\\Edge). \
                  Edge processes no longer start at sign-in or stay resident after the last window closes, so they \
                  use no memory until you open Edge; the first Edge window then opens a little slower. Because this \
                  is a machine policy, edge://settings shows the option as managed and Edge may show \"Your browser \
                  is managed by your organization\". Takes effect the next time Edge starts. Reverting removes the \
                  value; the empty policy key is left in place.",
        category: TweakCategory::Performance,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Medium,
        warning: None,
        requires: None,
        added_in: "0.1",
        references: &[
            "https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/startupboostenabled",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "StartupBoostEnabled",
        0,
    )],
};
