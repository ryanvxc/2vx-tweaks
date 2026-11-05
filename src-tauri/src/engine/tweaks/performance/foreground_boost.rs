use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.foreground-boost",
        name: "Favour the app you're using",
        description: "Gives the active app three times the CPU time of background apps.",
        details: "Sets Win32PrioritySeparation to 38 under \
                  HKLM\\SYSTEM\\CurrentControlSet\\Control\\PriorityControl. That is the value behind Performance \
                  Options > Adjust for best performance of: Programs, documented as short, variable intervals with \
                  a 3:1 foreground ratio. Many PCs already behave this way, so don't expect a visible change.",
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
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/previous-versions/cc976120(v=technet.10)",
            "https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
        38,
    )],
};
