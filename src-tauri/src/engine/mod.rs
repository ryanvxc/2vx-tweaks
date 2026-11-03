//! Optimisation engine.
//!
//! Tweaks are **declarative**: a tweak states the [`Setting`]s it wants and the engine
//! (`service`) handles detection, dry-run plans, write-ahead journaling, apply, read-back
//! verification, rollback and revert for every tweak in one place. A tweak cannot run
//! code against the system, so it cannot skip a backup.
//!
//! Every tweak lives in its own module under `engine/tweaks/` and is registered in
//! [`TweakRegistry::with_builtin_tweaks`]. See `docs/optimisation-modules.md` for the
//! review checklist.

pub mod broker;
pub mod journal;
pub mod plan;
pub mod policy;
pub mod service;
pub mod setting;
pub mod store;
mod tweaks;

use std::collections::BTreeMap;

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::system::os::{OsInfo, WINDOWS_10_FIRST_BUILD};

pub use setting::{Hive, RegTarget, RegValue, Setting};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel {
    /// Reversible, user-scoped configuration. Single confirmation.
    Low,
    /// Can change system behaviour. Requires explicit confirmation with consequences shown.
    Moderate,
    /// Potentially disruptive. Detailed warning, restore point, and a second confirmation.
    High,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum TweakCategory {
    Gaming,
    Performance,
    Power,
    Network,
    Privacy,
    Services,
    Windows,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    /// HKCU and per-user settings only.
    User,
    /// HKLM, services, power schemes, or anything system-wide.
    Administrator,
}

/// Ordered by disruption so a plan can report the strongest requirement.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RestartRequirement {
    None,
    /// Explorer or the user session must restart (sign out).
    SignOut,
    Reboot,
}

/// The expected size of a tweak's effect, from its documentation. An estimate shown to set
/// expectations, never a measurement: 2VX does not claim gains it has not measured.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum Impact {
    /// Removes a small annoyance or one data flow; most people won't notice a difference.
    Low,
    /// Noticeable in day-to-day use (fewer background processes, fewer prompts).
    Medium,
    High,
}

/// Which kind of PC a tweak is meant for (battery-powered or not).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeviceScope {
    Any,
    /// Mains-powered PCs only, e.g. tweaks that cost battery life.
    DesktopOnly,
    LaptopOnly,
}

/// Which Windows editions honour a tweak. Microsoft lists many policies for Pro and above only.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum EditionScope {
    Any,
    ProOrHigher,
}

/// Inclusive build range a tweak has been verified on.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuildRange {
    pub min_build: u32,
    pub max_build: Option<u32>,
}

impl BuildRange {
    pub const ALL: BuildRange = BuildRange {
        min_build: WINDOWS_10_FIRST_BUILD,
        max_build: None,
    };

    pub fn contains(&self, build: u32) -> bool {
        build >= self.min_build && self.max_build.is_none_or(|max| build <= max)
    }
}

/// Static description of a tweak. Serialised to the UI as-is.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakMeta {
    /// Stable, unique `category.kebab-name` ID. Never reuse a retired ID: the journal references it.
    pub id: &'static str,
    pub name: &'static str,
    /// One-sentence summary shown in lists (max 160 chars).
    pub description: &'static str,
    /// What actually changes and the consequences, shown before applying.
    pub details: &'static str,
    pub category: TweakCategory,
    pub risk: RiskLevel,
    /// Must match the settings: any HKLM target needs `Administrator`, HKCU-only needs `User`.
    pub permission: Permission,
    pub supported_builds: BuildRange,
    pub device: DeviceScope,
    pub edition: EditionScope,
    /// Whether the tweak is part of the default recommended set. Never for `High` risk.
    pub recommended: bool,
    pub restart: RestartRequirement,
    pub impact: Impact,
    /// The features this can break, shown as a warning on the card. Only for real breakage:
    /// a tweak that merely changes behaviour has `None`.
    pub warning: Option<&'static str>,
    /// A value that must already exist for the tweak to mean anything. Writing `Start` for a
    /// service that isn't installed would create a broken service key, so those name their
    /// `ImagePath` here. Checked by the engine, which can read the system; meta cannot.
    #[serde(skip)]
    pub requires: Option<RegTarget>,
    /// App version (`major.minor`) that first shipped the tweak; the UI labels recent ones "New".
    pub added_in: &'static str,
    /// Documentation the change is based on (Microsoft Learn etc.). `https://` URLs, at least one.
    pub references: &'static [&'static str],
}

/// The contract every optimisation implements.
pub trait Tweak: Send + Sync {
    fn meta(&self) -> &TweakMeta;
    /// The desired end state. Fixed for the tweak (no system access, no context), so it can
    /// be validated at registration and checked by the elevated broker.
    fn settings(&self) -> &[Setting];
}

/// The usual shape of a tweak: metadata plus a fixed list of settings.
pub struct SettingsTweak {
    pub meta: TweakMeta,
    pub settings: &'static [Setting],
}

impl Tweak for SettingsTweak {
    fn meta(&self) -> &TweakMeta {
        &self.meta
    }
    fn settings(&self) -> &[Setting] {
        self.settings
    }
}

/// Facts about the running system a tweak's applicability depends on.
#[derive(Debug, Clone)]
pub struct TweakContext {
    pub os: OsInfo,
    pub is_elevated: bool,
    /// `None` when the power status could not be read; device-scoped tweaks are then allowed.
    pub has_battery: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Compatibility {
    Compatible,
    UnsupportedBuild,
    UnsupportedEdition,
    WrongDevice,
    /// What the tweak configures isn't on this PC.
    NotInstalled,
    /// Applicable, but applying will show a UAC prompt.
    NeedsElevation,
}

impl Compatibility {
    /// Whether the tweak can't be applied here. `NeedsElevation` only adds a prompt.
    pub fn blocks(self) -> bool {
        !matches!(
            self,
            Compatibility::Compatible | Compatibility::NeedsElevation
        )
    }
}

/// Home is the `Core` family (`Core`, `CoreN`, `CoreSingleLanguage`, `CoreCountrySpecific`).
fn is_home_edition(edition_id: Option<&str>) -> bool {
    edition_id.is_some_and(|e| {
        e.get(..4)
            .is_some_and(|head| head.eq_ignore_ascii_case("core"))
    })
}

pub fn check_compatibility(meta: &TweakMeta, ctx: &TweakContext) -> Compatibility {
    let device_ok = !matches!(
        (meta.device, ctx.has_battery),
        (DeviceScope::DesktopOnly, Some(true)) | (DeviceScope::LaptopOnly, Some(false))
    );
    if !meta.supported_builds.contains(ctx.os.build) {
        Compatibility::UnsupportedBuild
    } else if meta.edition == EditionScope::ProOrHigher
        && is_home_edition(ctx.os.edition_id.as_deref())
    {
        Compatibility::UnsupportedEdition
    } else if !device_ok {
        Compatibility::WrongDevice
    } else if meta.permission == Permission::Administrator && !ctx.is_elevated {
        Compatibility::NeedsElevation
    } else {
        Compatibility::Compatible
    }
}

#[derive(Default)]
pub struct TweakRegistry {
    tweaks: BTreeMap<&'static str, Box<dyn Tweak>>,
}

impl TweakRegistry {
    /// The registry the app ships with.
    ///
    /// # Panics
    /// If a built-in tweak fails the metadata gate. That is a build defect, caught by the
    /// unit tests and `tests/policy.rs`, never a runtime condition.
    pub fn with_builtin_tweaks() -> Self {
        let mut registry = Self::default();
        if let Err(e) = tweaks::register_all(&mut registry) {
            panic!("built-in tweak catalogue is invalid: {e}");
        }
        registry
    }

    pub fn register(&mut self, tweak: Box<dyn Tweak>) -> AppResult<()> {
        validate(tweak.as_ref())?;
        let meta = tweak.meta();
        if self.tweaks.contains_key(meta.id) {
            return Err(AppError::Internal(format!(
                "duplicate tweak id `{}`",
                meta.id
            )));
        }
        // Two tweaks wanting different values for one target would undo each other.
        for setting in tweak.settings() {
            for other in self.tweaks.values() {
                if let Some(clash) = other
                    .settings()
                    .iter()
                    .find(|s| s.target.same_as(&setting.target) && s.value != setting.value)
                {
                    return Err(AppError::Internal(format!(
                        "tweak `{}` conflicts with `{}` on `{}`",
                        meta.id,
                        other.meta().id,
                        clash.target
                    )));
                }
            }
        }
        self.tweaks.insert(meta.id, tweak);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&dyn Tweak> {
        self.tweaks.get(id).map(|t| t.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Tweak> {
        self.tweaks.values().map(|t| t.as_ref())
    }

    pub fn list(&self) -> impl Iterator<Item = &TweakMeta> {
        self.iter().map(|t| t.meta())
    }

    /// Whether some registered tweak declares this target. The elevated broker writes nothing else.
    pub fn declares_target(&self, target: &RegTarget) -> bool {
        self.iter()
            .any(|t| t.settings().iter().any(|s| s.target.same_as(target)))
    }

    pub fn len(&self) -> usize {
        self.tweaks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tweaks.is_empty()
    }
}

/// The metadata completeness gate. A tweak that fails here cannot ship.
fn validate(tweak: &dyn Tweak) -> AppResult<()> {
    let meta = tweak.meta();
    let fail = |reason: String| Err(AppError::Internal(format!("tweak `{}`: {reason}", meta.id)));

    let id_ok = meta.id.split_once('.').is_some_and(|(cat, name)| {
        let part = |s: &str| {
            !s.is_empty()
                && !s.starts_with('-')
                && !s.ends_with('-')
                && s.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        };
        part(cat) && part(name)
    });
    if !id_ok {
        return fail("id must be `category.kebab-name`".into());
    }
    if meta.name.trim().is_empty()
        || meta.description.trim().is_empty()
        || meta.details.trim().is_empty()
    {
        return fail("name, description and details are required".into());
    }
    if meta.description.chars().count() > 160 {
        return fail("description is longer than 160 characters".into());
    }
    if meta
        .warning
        .is_some_and(|w| w.trim().is_empty() || w.chars().count() > 160)
    {
        return fail("warning must be 1 to 160 characters".into());
    }
    if meta.references.is_empty() {
        return fail("has no documentation references".into());
    }
    if let Some(bad) = meta.references.iter().find(|r| !r.starts_with("https://")) {
        return fail(format!("reference `{bad}` is not an https URL"));
    }
    let version_ok = meta.added_in.split_once('.').is_some_and(|(major, minor)| {
        [major, minor]
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
    });
    if !version_ok {
        return fail("added_in must be `major.minor`".into());
    }
    if meta.risk == RiskLevel::High && meta.recommended {
        return fail("High-risk tweaks cannot be recommended".into());
    }
    // Cards only flag warnings, so a High-risk tweak has to carry one.
    if meta.risk == RiskLevel::High && meta.warning.is_none() {
        return fail("High-risk tweaks need a warning".into());
    }

    let settings = tweak.settings();
    if settings.is_empty() {
        return fail("declares no settings".into());
    }
    for (i, setting) in settings.iter().enumerate() {
        policy::check_target(&setting.target).or_else(|e| fail(e.to_string()))?;
        if settings[..i]
            .iter()
            .any(|s| s.target.same_as(&setting.target))
        {
            return fail(format!("declares `{}` twice", setting.target));
        }
    }
    let needs_admin = settings.iter().any(|s| s.target.hive.needs_elevation());
    let expected = if needs_admin {
        Permission::Administrator
    } else {
        Permission::User
    };
    if meta.permission != expected {
        return fail(format!("permission must be {expected:?} for its targets"));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub const USER_SETTINGS: &[Setting] = &[
        Setting::dword(Hive::CurrentUser, r"Software\2VX-Test\A", "One", 1),
        Setting::absent(Hive::CurrentUser, r"Software\2VX-Test\A", "Gone"),
    ];
    pub const ADMIN_SETTINGS: &[Setting] = &[Setting::dword(
        Hive::LocalMachine,
        r"SOFTWARE\2VX-Test",
        "Admin",
        2,
    )];

    pub fn meta(id: &'static str) -> TweakMeta {
        TweakMeta {
            id,
            name: "Test",
            description: "A test tweak.",
            details: "Changes test values only.",
            category: TweakCategory::Gaming,
            risk: RiskLevel::Low,
            permission: Permission::User,
            supported_builds: BuildRange {
                min_build: 22_000,
                max_build: None,
            },
            device: DeviceScope::Any,
            edition: EditionScope::Any,
            recommended: false,
            restart: RestartRequirement::None,
            impact: Impact::Low,
            warning: None,
            requires: None,
            added_in: "0.1",
            references: &["https://learn.microsoft.com/"],
        }
    }

    pub fn tweak(meta: TweakMeta, settings: &'static [Setting]) -> Box<dyn Tweak> {
        Box::new(SettingsTweak { meta, settings })
    }

    pub fn ctx(build: u32, is_elevated: bool) -> TweakContext {
        TweakContext {
            os: OsInfo {
                product_name: "Windows".into(),
                display_version: None,
                build,
                ubr: None,
                edition_id: None,
                is_windows_11: build >= 22_000,
                architecture: "x86_64".into(),
            },
            is_elevated,
            has_battery: Some(false),
        }
    }

    #[test]
    fn rejects_duplicate_ids() {
        let mut r = TweakRegistry::default();
        r.register(tweak(meta("gaming.test"), USER_SETTINGS))
            .unwrap();
        assert!(
            r.register(tweak(meta("gaming.test"), USER_SETTINGS))
                .is_err()
        );
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn rejects_conflicting_targets_across_tweaks() {
        const OTHER: &[Setting] = &[Setting::dword(
            Hive::CurrentUser,
            r"software\2vx-test\a",
            "one",
            9,
        )];
        const SAME: &[Setting] = &[Setting::dword(
            Hive::CurrentUser,
            r"Software\2VX-Test\A",
            "One",
            1,
        )];
        let mut r = TweakRegistry::default();
        r.register(tweak(meta("gaming.a"), USER_SETTINGS)).unwrap();
        assert!(r.register(tweak(meta("gaming.b"), OTHER)).is_err());
        // Agreeing on the same value is fine.
        r.register(tweak(meta("gaming.c"), SAME)).unwrap();
    }

    #[test]
    fn metadata_gate() {
        let mut r = TweakRegistry::default();
        let reject = |m: TweakMeta, s: &'static [Setting]| {
            TweakRegistry::default().register(tweak(m, s)).is_err()
        };

        assert!(reject(meta("Bad ID"), USER_SETTINGS));
        assert!(reject(meta("nocategory"), USER_SETTINGS));
        assert!(reject(meta("gaming.-x"), USER_SETTINGS));
        assert!(reject(
            TweakMeta {
                references: &[],
                ..meta("gaming.a")
            },
            USER_SETTINGS
        ));
        assert!(reject(
            TweakMeta {
                references: &["http://example.com"],
                ..meta("gaming.a")
            },
            USER_SETTINGS
        ));
        assert!(reject(
            TweakMeta {
                details: " ",
                ..meta("gaming.a")
            },
            USER_SETTINGS
        ));
        assert!(reject(
            TweakMeta {
                risk: RiskLevel::High,
                recommended: true,
                ..meta("gaming.a")
            },
            USER_SETTINGS
        ));
        assert!(reject(meta("gaming.a"), &[]));
        // Permission must match the hives.
        assert!(reject(meta("gaming.a"), ADMIN_SETTINGS));
        assert!(reject(
            TweakMeta {
                permission: Permission::Administrator,
                ..meta("gaming.a")
            },
            USER_SETTINGS
        ));
        // Protected targets never register.
        const DEFENDER: &[Setting] = &[Setting::dword(
            Hive::LocalMachine,
            r"SOFTWARE\Policies\Microsoft\Windows Defender",
            "DisableAntiSpyware",
            1,
        )];
        assert!(reject(
            TweakMeta {
                permission: Permission::Administrator,
                ..meta("gaming.a")
            },
            DEFENDER
        ));
        // Duplicate target within one tweak.
        const DUP: &[Setting] = &[
            Setting::dword(Hive::CurrentUser, r"Software\X", "V", 1),
            Setting::dword(Hive::CurrentUser, r"SOFTWARE\x", "v", 1),
        ];
        assert!(reject(meta("gaming.a"), DUP));

        r.register(tweak(meta("gaming.ok"), USER_SETTINGS)).unwrap();
        r.register(tweak(
            TweakMeta {
                permission: Permission::Administrator,
                ..meta("gaming.admin")
            },
            ADMIN_SETTINGS,
        ))
        .unwrap();
        assert!(r.declares_target(&RegTarget::new(
            Hive::LocalMachine,
            r"software\2vx-test",
            "ADMIN"
        )));
        assert!(!r.declares_target(&RegTarget::new(
            Hive::LocalMachine,
            r"SOFTWARE\Other",
            "Admin"
        )));
    }

    #[test]
    fn compatibility_checks_build_device_then_elevation() {
        let m = TweakMeta {
            permission: Permission::Administrator,
            ..meta("gaming.test")
        };
        assert_eq!(
            check_compatibility(&m, &ctx(19_045, true)),
            Compatibility::UnsupportedBuild
        );
        assert_eq!(
            check_compatibility(&m, &ctx(26_100, false)),
            Compatibility::NeedsElevation
        );
        assert_eq!(
            check_compatibility(&m, &ctx(26_100, true)),
            Compatibility::Compatible
        );

        let desktop = TweakMeta {
            device: DeviceScope::DesktopOnly,
            ..meta("gaming.test")
        };
        let mut laptop = ctx(26_100, true);
        laptop.has_battery = Some(true);
        assert_eq!(
            check_compatibility(&desktop, &laptop),
            Compatibility::WrongDevice
        );
        laptop.has_battery = None;
        assert_eq!(
            check_compatibility(&desktop, &laptop),
            Compatibility::Compatible
        );
    }

    #[test]
    fn build_range_bounds_are_inclusive() {
        let r = BuildRange {
            min_build: 100,
            max_build: Some(200),
        };
        assert!(r.contains(100) && r.contains(200));
        assert!(!r.contains(99) && !r.contains(201));
    }
}
