use crate::engine::{
    BuildRange, DeviceScope, Hive, Impact, Permission, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "windows.edge-sidebar-off",
        name: "Hide the Microsoft Edge sidebar",
        description: "Removes the launcher bar on the right side of Edge so pages get the full window width.",
        details: "Sets the documented Edge policy HubsSidebarEnabled to 0 (HKLM\\SOFTWARE\\Policies\\Microsoft\\Edge). The sidebar is then never shown, whatever the Edge setting says. From Edge 141 the Copilot toolbar icon has its own policy and is not affected by this one. Applies without restarting Edge. Because this is a machine policy, Edge shows the option as managed and may show \"Your browser is managed by your organization\". Reverting removes the value; the empty policy key is left in place.",
        category: TweakCategory::Windows,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/hubssidebarenabled",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "HubsSidebarEnabled",
        0,
    )],
};
