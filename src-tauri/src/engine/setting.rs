//! The typed vocabulary for system changes.
//!
//! A tweak never touches the OS itself: it declares the [`Setting`]s it wants and the
//! engine does detection, backup, apply, verification and revert for all tweaks the same
//! way. Most are registry values, service start types included (`Services\<name>\Start`). A few
//! things Windows keeps elsewhere look the same to the engine: see the non-registry [`Hive`]s, which
//! `store` routes to their own APIs so backup, verify and revert stay in one place.

use std::borrow::Cow;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Hive {
    /// HKEY_CURRENT_USER: per-user, no elevation needed.
    CurrentUser,
    /// HKEY_LOCAL_MACHINE: system-wide, needs administrator rights to write.
    LocalMachine,
    /// Not a registry hive: `key` is a Task Scheduler path, `value` is `Enabled`, data is 0 or 1.
    ScheduledTask,
    /// Not a registry hive: `key` is `<subgroup GUID>\<setting GUID>` in the active power plan and
    /// `value` is `AC` or `DC`.
    PowerPlan,
    /// A vendor's GPU driver key under `Control\Class`, which scripts hardcode as `0000` though it
    /// can be any index, and a PC with two GPUs has one per adapter. `key` is `Nvidia`, `Amd` or
    /// `Intel`, optionally followed by `\<subkey>`.
    Gpu,
    /// A physical network adapter's driver key under `Control\Class`, whose index differs per PC.
    /// `key` is `Ethernet` or `Wifi`, resolved to that kind's primary adapter, and `value` is one of
    /// the documented NDIS keywords in `policy`. Only a `requires` check reads below it.
    Adapter,
}

impl Hive {
    pub fn short_name(self) -> &'static str {
        match self {
            Hive::CurrentUser => "HKCU",
            Hive::LocalMachine => "HKLM",
            Hive::ScheduledTask => "TASK",
            Hive::PowerPlan => "POWER",
            Hive::Gpu => "GPU",
            Hive::Adapter => "ADAPTER",
        }
    }

    pub fn needs_elevation(self) -> bool {
        self != Hive::CurrentUser
    }
}

/// One registry value: hive + key path + value name.
///
/// `Cow` lets tweak definitions be `const` while values read back from the journal or
/// the elevated broker are owned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RegTarget {
    pub hive: Hive,
    /// Path below the hive, backslash-separated, no leading or trailing backslash.
    pub key: Cow<'static, str>,
    /// Value name. Empty means the key's default value.
    pub value: Cow<'static, str>,
}

impl RegTarget {
    pub const fn new(hive: Hive, key: &'static str, value: &'static str) -> Self {
        RegTarget {
            hive,
            key: Cow::Borrowed(key),
            value: Cow::Borrowed(value),
        }
    }

    /// Case-insensitive identity, matching how Windows compares key and value names.
    pub fn same_as(&self, other: &RegTarget) -> bool {
        self.hive == other.hive
            && self.key.eq_ignore_ascii_case(&other.key)
            && self.value.eq_ignore_ascii_case(&other.value)
    }
}

impl fmt::Display for RegTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = if self.value.is_empty() {
            "(Default)"
        } else {
            &self.value
        };
        write!(f, "{}\\{}\\{}", self.hive.short_name(), self.key, value)
    }
}

/// Registry data types the engine can capture and restore exactly.
/// Anything else (binary, multi-string…) is refused rather than restored lossily.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum RegValue {
    Dword(u32),
    Qword(u64),
    String(Cow<'static, str>),
    ExpandString(Cow<'static, str>),
}

impl fmt::Display for RegValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegValue::Dword(v) => write!(f, "{v} (REG_DWORD)"),
            RegValue::Qword(v) => write!(f, "{v} (REG_QWORD)"),
            RegValue::String(s) => write!(f, "\"{s}\" (REG_SZ)"),
            RegValue::ExpandString(s) => write!(f, "\"{s}\" (REG_EXPAND_SZ)"),
        }
    }
}

/// The state a tweak wants for one target. `value: None` means "the value must not exist".
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub target: RegTarget,
    pub value: Option<RegValue>,
}

impl Setting {
    pub const fn dword(hive: Hive, key: &'static str, value: &'static str, data: u32) -> Self {
        Setting {
            target: RegTarget::new(hive, key, value),
            value: Some(RegValue::Dword(data)),
        }
    }

    pub const fn string(
        hive: Hive,
        key: &'static str,
        value: &'static str,
        data: &'static str,
    ) -> Self {
        Setting {
            target: RegTarget::new(hive, key, value),
            value: Some(RegValue::String(Cow::Borrowed(data))),
        }
    }

    pub const fn absent(hive: Hive, key: &'static str, value: &'static str) -> Self {
        Setting {
            target: RegTarget::new(hive, key, value),
            value: None,
        }
    }
}

/// Display form for an optional value, as shown in plans and history.
pub fn describe(value: &Option<RegValue>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "(not set)".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_display_and_case_insensitive_identity() {
        let a = RegTarget::new(Hive::CurrentUser, r"Software\Test", "Value");
        let b = RegTarget::new(Hive::CurrentUser, r"SOFTWARE\test", "VALUE");
        assert_eq!(a.to_string(), r"HKCU\Software\Test\Value");
        assert!(a.same_as(&b));
        assert!(!a.same_as(&RegTarget::new(
            Hive::LocalMachine,
            r"Software\Test",
            "Value"
        )));
        assert_eq!(
            RegTarget::new(Hive::LocalMachine, "K", "").to_string(),
            r"HKLM\K\(Default)"
        );
    }

    #[test]
    fn values_round_trip_through_json() {
        for v in [
            RegValue::Dword(1),
            RegValue::Qword(u64::MAX),
            RegValue::String("a\"b".into()),
            RegValue::ExpandString("%TEMP%".into()),
        ] {
            let json = serde_json::to_string(&v).unwrap();
            assert_eq!(serde_json::from_str::<RegValue>(&json).unwrap(), v);
        }
        assert_eq!(
            serde_json::to_value(RegValue::Dword(3)).unwrap(),
            serde_json::json!({ "type": "dword", "data": 3 })
        );
    }
}
