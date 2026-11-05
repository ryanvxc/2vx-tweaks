use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "performance.kernel-in-ram",
        name: "Keep kernel code in RAM",
        description: "Stops Windows paging kernel code out to disk. Uses a little more memory.",
        details: "Sets DisablePagingExecutive to 1 under Memory Management. Microsoft documents that this keeps kernel-mode code from being paged and that it raises memory use. On a PC with plenty of RAM the kernel is rarely paged anyway, so don't expect much.",
        category: TweakCategory::Performance,
        risk: RiskLevel::Low,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: None,
        requires: None,
        added_in: "0.2",
        references: &[
            "https://learn.microsoft.com/en-us/windows/win32/api/evntrace/ns-evntrace-enable_trace_parameters",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "DisablePagingExecutive",
        1,
    )],
};
