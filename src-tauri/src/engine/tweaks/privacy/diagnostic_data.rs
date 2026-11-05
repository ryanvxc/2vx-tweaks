use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

pub const TWEAK: SettingsTweak = SettingsTweak {
    meta: TweakMeta {
        id: "privacy.diagnostic-data-minimum",
        name: "Send the least diagnostic data",
        description: "Sets the Windows diagnostic data policy to its lowest level.",
        details: "Sets AllowTelemetry to 0 under HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection, as \
                  Microsoft documents in section 18.16 of its connections guide. Only Enterprise and Education \
                  turn diagnostic data fully off at this value; on other editions Windows treats it as the \
                  Required level. Windows Update and Microsoft Defender are not touched.",
        category: TweakCategory::Privacy,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::ProOrHigher,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning: Some(
            "Windows Insider builds stop being offered: they need optional diagnostic data.",
        ),
        requires: None,
        added_in: "0.2",
        section: None,
        evidence: crate::engine::Evidence::Documented,
        hardware: crate::engine::HardwareScope::Any,
        references: &[
            "https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services",
            "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-system#allowtelemetry",
        ],
    },
    settings: &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
        0,
    )],
};
