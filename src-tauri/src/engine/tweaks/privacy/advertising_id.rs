use crate::engine::{
    BuildRange, DeviceScope, Hive, Impact, Permission, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.advertising-id-off",
        name: "Turn off the advertising ID",
        description: "Stops apps using your Windows advertising ID to show you personalised ads across apps.",
        details: "Sets Enabled to 0 under HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo, the value Microsoft documents for turning off the advertising ID for every account on this PC. Apps that ask for the ID no longer get one, so ads in apps are not personalised from it; you see the same number of ads. Reverting restores the previous value. The per-account switch in Settings > Privacy & security > General is not changed.",
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
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#181-general",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
        0,
    )],
};
