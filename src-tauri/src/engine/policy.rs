//! Compile-time safety policy for registry targets.
//!
//! Every target is checked here: when a tweak registers, when a plan is built, and again
//! inside the elevated broker before it writes. The denylist covers security features
//! the project rules forbid touching (Defender, Firewall, Windows Update, UAC,
//! SmartScreen, VBS/HVCI, Secure Boot, LSA, CPU mitigations) plus hives that are never legitimate
//! targets. The same rules cover scheduled tasks, whose paths are checked like keys.

use crate::engine::setting::{Hive, RegTarget, RegValue, Setting};
use crate::error::{AppError, AppResult};

const MAX_KEY_LEN: usize = 255;
const MAX_VALUE_NAME_LEN: usize = 255;

/// Key prefixes (case-insensitive, segment-aligned) that may never be written.
const DENIED_KEYS: &[(Hive, &str)] = &[
    // Whole hives that are never a tweak target.
    (Hive::LocalMachine, "SAM"),
    (Hive::LocalMachine, "SECURITY"),
    (Hive::LocalMachine, "BCD00000000"),
    // Microsoft Defender.
    (Hive::LocalMachine, r"SOFTWARE\Microsoft\Windows Defender"),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows Defender Security Center",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows Defender",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows Advanced Threat Protection",
    ),
    // Windows Firewall.
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\WindowsFirewall",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Services\SharedAccess",
    ),
    // Windows Update.
    (Hive::LocalMachine, r"SOFTWARE\Microsoft\WindowsUpdate"),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate",
    ),
    // UAC and other system policies.
    (
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
    ),
    (
        Hive::CurrentUser,
        r"Software\Microsoft\Windows\CurrentVersion\Policies\System",
    ),
    // VBS / HVCI / Credential Guard, LSA protection, Secure Boot.
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard",
    ),
    (Hive::LocalMachine, r"SYSTEM\CurrentControlSet\Control\Lsa"),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\SecureBoot",
    ),
    // Scheduled tasks that keep the same features working.
    (Hive::ScheduledTask, r"\Microsoft\Windows\Windows Defender"),
    (Hive::ScheduledTask, r"\Microsoft\Windows\ExploitGuard"),
    (Hive::ScheduledTask, r"\Microsoft\Windows\WindowsUpdate"),
    (
        Hive::ScheduledTask,
        r"\Microsoft\Windows\UpdateOrchestrator",
    ),
    (Hive::ScheduledTask, r"\Microsoft\Windows\WaaSMedic"),
    (Hive::ScheduledTask, r"\Microsoft\Windows\AppID"),
    (Hive::ScheduledTask, r"\Microsoft\Windows\TPM"),
    (
        Hive::ScheduledTask,
        r"\Microsoft\Windows\Windows Filtering Platform",
    ),
    (Hive::ScheduledTask, r"\Microsoft\Windows\SystemRestore"),
    // Code integrity.
    (Hive::LocalMachine, r"SYSTEM\CurrentControlSet\Control\CI"),
    // SmartScreen for Store apps and Windows Security notifications.
    (
        Hive::CurrentUser,
        r"Software\Microsoft\Windows\CurrentVersion\AppHost",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows Security Health",
    ),
    (
        Hive::CurrentUser,
        r"Software\Microsoft\Windows Security Health",
    ),
];

/// Individual values that may never be written, where the key itself holds unrelated settings.
const DENIED_VALUES: &[(Hive, &str, &str)] = &[
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "EnableSmartScreen",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
        "SmartScreenEnabled",
    ),
    // CPU vulnerability mitigations, ASLR, SEHOP and exploit protection.
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "FeatureSettings",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "FeatureSettingsOverride",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "FeatureSettingsOverrideMask",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "MoveImages",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "MitigationOptions",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "MitigationAuditOptions",
    ),
    (
        Hive::LocalMachine,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "DisableExceptionChainValidation",
    ),
    // Microsoft Edge policies share one key; its SmartScreen and security policies are protected.
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "SmartScreenEnabled",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "SmartScreenPuaEnabled",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "PreventSmartScreenPromptOverride",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "PreventSmartScreenPromptOverrideForFiles",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "SmartScreenForTrustedDownloadsEnabled",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "TyposquattingCheckerEnabled",
    ),
    (
        Hive::LocalMachine,
        r"SOFTWARE\Policies\Microsoft\Edge",
        "SitePerProcess",
    ),
];

/// Services whose configuration (start type included) is protected.
// The DNS client's cache sizes and lifetimes: tuning them leaves the service running, which is
// what protecting it is for.
const TUNABLE_SERVICE_KEY: &str = r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters";

const PROTECTED_SERVICES: &[&str] = &[
    "WinDefend",
    "WdNisSvc",
    "WdNisDrv",
    "WdFilter",
    "BDESVC",
    "WdBoot",
    "Sense",
    "SecurityHealthService",
    "wscsvc",
    "mpssvc",
    "BFE",
    "wuauserv",
    "WaaSMedicSvc",
    "UsoSvc",
    "BITS",
    "TrustedInstaller",
    "EventLog",
    "WdDevFlt",
    "MsSecFlt",
    "webthreatdefsvc",
    "webthreatdefusersvc",
    "SgrmBroker",
    "SamSs",
    "KeyIso",
    "VaultSvc",
    "RpcSs",
    "RpcEptMapper",
    "DcomLaunch",
    "LSM",
    "PlugPlay",
    "Power",
    "ProfSvc",
    "BrokerInfrastructure",
    "CoreMessagingRegistrar",
    "gpsvc",
    "Schedule",
    "SystemEventsBroker",
    "StateRepository",
    "UserManager",
    "Winmgmt",
    "CryptSvc",
    "Dhcp",
    "Dnscache",
    "nsi",
    "AudioEndpointBuilder",
    "Audiosrv",
    "AppXSvc",
    "msiserver",
    "TokenBroker",
    "DeviceInstall",
    "EFS",
];

/// Checks a target against the denylist and basic shape rules.
pub fn check_target(target: &RegTarget) -> AppResult<()> {
    let key = target.key.as_ref();
    let shape_ok = !key.is_empty()
        && key.len() <= MAX_KEY_LEN
        && target.value.len() <= MAX_VALUE_NAME_LEN
        && (!key.starts_with('\\') || target.hive == Hive::ScheduledTask)
        && !key.ends_with('\\')
        && !key.contains(r"\\")
        && !key.chars().any(char::is_control)
        && !target.value.chars().any(char::is_control);
    if !shape_ok {
        return Err(AppError::Policy(format!(
            "malformed registry target `{target}`"
        )));
    }

    for (hive, prefix) in DENIED_KEYS {
        if *hive == target.hive && has_key_prefix(key, prefix) {
            return Err(AppError::Policy(format!(
                "`{target}` is a protected location"
            )));
        }
    }
    for (hive, denied_key, value) in DENIED_VALUES {
        if *hive == target.hive
            && key.eq_ignore_ascii_case(denied_key)
            && target.value.eq_ignore_ascii_case(value)
        {
            return Err(AppError::Policy(format!(
                "`{target}` is a protected setting"
            )));
        }
    }
    if target.hive == Hive::LocalMachine {
        for service in PROTECTED_SERVICES {
            let service_key = format!(r"SYSTEM\CurrentControlSet\Services\{service}");
            if has_key_prefix(key, &service_key) && !has_key_prefix(key, TUNABLE_SERVICE_KEY) {
                return Err(AppError::Policy(format!(
                    "`{target}` belongs to a protected service ({service})"
                )));
            }
        }
    }
    Ok(())
}

/// Checks the data a setting writes, for targets that accept only some values.
pub fn check_value(setting: &Setting) -> AppResult<()> {
    let target = &setting.target;
    let ok = match (target.hive, &setting.value) {
        (Hive::ScheduledTask, Some(RegValue::Dword(0 | 1))) => target.value == "Enabled",
        (Hive::ScheduledTask, _) => false,
        (Hive::PowerPlan, Some(RegValue::Dword(_))) => matches!(target.value.as_ref(), "AC" | "DC"),
        (Hive::PowerPlan, _) => false,
        (Hive::Gpu, _) => {
            let vendor = target.key.split('\\').next();
            matches!(vendor, Some("Nvidia" | "Amd" | "Intel"))
        }
        _ => true,
    };
    if ok {
        Ok(())
    } else {
        Err(AppError::Policy(format!(
            "`{target}` can't hold that value"
        )))
    }
}

/// Segment-aligned, case-insensitive prefix match: `A\B` matches `A\B` and `A\B\C`, not `A\BC`.
fn has_key_prefix(key: &str, prefix: &str) -> bool {
    // `get` rather than slicing: a non-ASCII key must not panic on a char boundary.
    match key.get(..prefix.len()) {
        Some(head) if head.eq_ignore_ascii_case(prefix) => {
            key.len() == prefix.len() || key.as_bytes()[prefix.len()] == b'\\'
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(hive: Hive, key: &'static str, value: &'static str) -> RegTarget {
        RegTarget::new(hive, key, value)
    }

    #[test]
    fn scheduled_tasks_stay_off_security_paths() {
        for path in [
            r"\Microsoft\Windows\Windows Defender\Windows Defender Scheduled Scan",
            r"\Microsoft\Windows\WindowsUpdate\Scheduled Start",
            r"\Microsoft\Windows\UpdateOrchestrator\Schedule Scan",
            r"\Microsoft\Windows\ExploitGuard\ExploitGuard MDM policy Refresh",
            r"\Microsoft\Windows\AppID\SmartScreenSpecific",
            r"\Microsoft\Windows\TPM\Tpm-Maintenance",
            r"\Microsoft\Windows\SystemRestore\SR",
        ] {
            assert!(
                check_target(&t(Hive::ScheduledTask, path, "Enabled")).is_err(),
                "{path}"
            );
        }
        assert!(
            check_target(&t(
                Hive::ScheduledTask,
                r"\Microsoft\Windows\Maps\MapsUpdateTask",
                "Enabled"
            ))
            .is_ok()
        );
    }

    #[test]
    fn core_services_and_mitigations_stay_protected() {
        for service in ["RpcSs", "Schedule", "Winmgmt", "AppXSvc", "SgrmBroker"] {
            let key: &'static str =
                Box::leak(format!(r"SYSTEM\CurrentControlSet\Services\{service}").into_boxed_str());
            assert!(check_target(&t(Hive::LocalMachine, key, "Start")).is_err());
        }
        assert!(
            check_target(&t(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "FeatureSettingsOverride"
            ))
            .is_err()
        );
        let dns = r"SYSTEM\CurrentControlSet\Services\Dnscache";
        assert!(check_target(&t(Hive::LocalMachine, dns, "Start")).is_err());
        let cache = r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters";
        assert!(check_target(&t(Hive::LocalMachine, cache, "MaxCacheTtl")).is_ok());
    }

    #[test]
    fn non_registry_values_keep_their_shape() {
        let task = Setting::dword(Hive::ScheduledTask, r"\Microsoft\X", "Enabled", 2);
        assert!(check_value(&task).is_err());
        let string = Setting::string(Hive::PowerPlan, "a\\b", "AC", "1");
        assert!(check_value(&string).is_err());
        let wrong_name = Setting::dword(Hive::PowerPlan, "a\\b", "Battery", 1);
        assert!(check_value(&wrong_name).is_err());
        assert!(check_value(&Setting::dword(Hive::PowerPlan, "a\\b", "DC", 1)).is_ok());
        assert!(check_value(&Setting::dword(Hive::Gpu, "Driver", "X", 1)).is_err());
        assert!(check_value(&Setting::dword(Hive::Gpu, r"Amd\UMD", "X", 1)).is_ok());
    }

    #[test]
    fn allows_ordinary_targets() {
        assert!(
            check_target(&t(
                Hive::CurrentUser,
                r"System\GameConfigStore",
                "GameDVR_Enabled"
            ))
            .is_ok()
        );
        assert!(
            check_target(&t(
                Hive::LocalMachine,
                r"SOFTWARE\Microsoft\Windows\Dwm",
                "X"
            ))
            .is_ok()
        );
    }

    #[test]
    fn denies_security_locations_case_insensitively() {
        for target in [
            t(
                Hive::LocalMachine,
                r"software\policies\microsoft\windows defender\Real-Time Protection",
                "X",
            ),
            t(
                Hive::LocalMachine,
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",
                "NoAutoUpdate",
            ),
            t(
                Hive::LocalMachine,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "EnableLUA",
            ),
            t(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios",
                "X",
            ),
            t(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WinDefend",
                "Start",
            ),
            t(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wuauserv",
                "Start",
            ),
            t(
                Hive::LocalMachine,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "EnableSmartScreen",
            ),
            t(
                Hive::LocalMachine,
                r"SOFTWARE\Policies\Microsoft\Edge",
                "smartscreenenabled",
            ),
            t(Hive::LocalMachine, "SAM", "X"),
        ] {
            assert!(
                matches!(check_target(&target), Err(AppError::Policy(_))),
                "{target}"
            );
        }
    }

    #[test]
    fn prefix_match_is_segment_aligned() {
        // `Services\BITSX` is not `Services\BITS`.
        assert!(
            check_target(&t(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\BITSX",
                "Start"
            ))
            .is_ok()
        );
        // The same key under HKCU is not the HKLM policy.
        assert!(
            check_target(&t(
                Hive::CurrentUser,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "EnableSmartScreen"
            ))
            .is_ok()
        );
    }

    #[test]
    fn rejects_malformed_targets() {
        for key in [
            "",
            r"\Software\X",
            r"Software\X\",
            r"Software\\X",
            "Soft\u{0}ware",
        ] {
            let target = RegTarget {
                hive: Hive::CurrentUser,
                key: key.to_string().into(),
                value: "V".into(),
            };
            assert!(check_target(&target).is_err(), "{key:?}");
        }
    }
}
