//! Compile-time safety policy for registry targets.
//!
//! Every target is checked here: when a tweak registers, when a plan is built, and again
//! inside the elevated broker before it writes. The denylist covers security features
//! the project rules forbid touching (Defender, Firewall, Windows Update, UAC,
//! SmartScreen, VBS/HVCI, Secure Boot, LSA) plus hives that are never legitimate targets.

use crate::engine::setting::{Hive, RegTarget};
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
const PROTECTED_SERVICES: &[&str] = &[
    "WinDefend",
    "WdNisSvc",
    "WdNisDrv",
    "WdFilter",
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
];

/// Checks a target against the denylist and basic shape rules.
pub fn check_target(target: &RegTarget) -> AppResult<()> {
    let key = target.key.as_ref();
    let shape_ok = !key.is_empty()
        && key.len() <= MAX_KEY_LEN
        && target.value.len() <= MAX_VALUE_NAME_LEN
        && !key.starts_with('\\')
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
            if has_key_prefix(key, &service_key) {
                return Err(AppError::Policy(format!(
                    "`{target}` belongs to a protected service ({service})"
                )));
            }
        }
    }
    Ok(())
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
