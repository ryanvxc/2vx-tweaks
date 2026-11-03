use crate::engine::{
    BuildRange, DeviceScope, Hive, Impact, Permission, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.edge-shopping-off",
        name: "Turn off Shopping in Microsoft Edge",
        description: "Stops Edge looking up coupons, rebates and prices from other retailers whenever you visit a shop.",
        details: "Sets the documented Edge policy EdgeShoppingAssistantEnabled to 0 (HKLM\\SOFTWARE\\Policies\\Microsoft\\Edge). With Shopping on, Edge fetches coupons for the current retailer and prices from other retailers from a Microsoft server while you browse shops. With it off, price comparison, coupons, rebates and express checkout are no longer offered. Applies without restarting Edge. Because this is a machine policy, Edge shows the option as managed and may show \"Your browser is managed by your organization\". Reverting removes the value; the empty policy key is left in place.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        recommended: true,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/edgeshoppingassistantenabled",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "EdgeShoppingAssistantEnabled",
        0,
    )],
};
