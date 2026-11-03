//! Optimisation engine contracts.
//!
//! Phase 1 defines the contract and the registry only; **no tweaks are registered yet**.
//! Every tweak added later must implement [`Tweak`] in its own module under
//! `engine/tweaks/` and be registered in [`TweakRegistry::with_builtin_tweaks`].
//! See `docs/optimisation-modules.md` for the review checklist a tweak must pass.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::system::os::{OsInfo, WINDOWS_10_FIRST_BUILD};

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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RestartRequirement {
    None,
    /// Explorer or the user session must restart (sign out).
    SignOut,
    Reboot,
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
    /// Stable, unique, kebab-case ID (e.g. `gaming.game-mode`). Never reuse a retired ID.
    pub id: &'static str,
    pub name: &'static str,
    /// One-sentence summary shown in lists.
    pub description: &'static str,
    /// What actually changes and the consequences, shown before applying.
    pub details: &'static str,
    pub category: TweakCategory,
    pub risk: RiskLevel,
    pub permission: Permission,
    pub supported_builds: BuildRange,
    /// Whether the tweak is part of the default recommended set.
    pub recommended: bool,
    pub restart: RestartRequirement,
    /// Documentation the change is based on (Microsoft Learn etc.). Must not be empty.
    pub references: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TweakState {
    Applied,
    NotApplied,
    /// Some but not all of the tweak's settings match the applied configuration.
    Partial,
    /// Detection failed or the setting is controlled by policy.
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Detection {
    pub state: TweakState,
    /// Human-readable current value(s), for display.
    pub current: serde_json::Value,
    /// Value(s) the tweak would set.
    pub target: serde_json::Value,
}

/// One atomic change a tweak would make. Returned by [`Tweak::plan`] for dry runs.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedChange {
    /// e.g. `HKCU\System\GameConfigStore\GameDVR_Enabled`.
    pub target: String,
    pub from: serde_json::Value,
    pub to: serde_json::Value,
}

/// Original configuration captured before applying. Persisted by the backup engine
/// (Phase 2) and handed back to [`Tweak::revert`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub tweak_id: String,
    pub captured_at_ms: u64,
    pub data: serde_json::Value,
}

/// Facts about the running system a tweak may need.
#[derive(Debug, Clone)]
pub struct TweakContext {
    pub os: OsInfo,
    pub is_elevated: bool,
}

/// The contract every optimisation implements.
///
/// Implementations must be side-effect free in `detect`, `plan` and `verify`.
/// `apply` must capture a complete [`Backup`] *before* changing anything, and must
/// leave the system unchanged if it returns an error part-way (roll back what it did).
pub trait Tweak: Send + Sync {
    fn meta(&self) -> &TweakMeta;
    fn detect(&self, ctx: &TweakContext) -> AppResult<Detection>;
    fn plan(&self, ctx: &TweakContext) -> AppResult<Vec<PlannedChange>>;
    fn apply(&self, ctx: &TweakContext) -> AppResult<Backup>;
    fn revert(&self, ctx: &TweakContext, backup: &Backup) -> AppResult<()>;
    /// Re-reads the system and confirms the applied configuration is in effect.
    fn verify(&self, ctx: &TweakContext) -> AppResult<bool>;
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Compatibility {
    Compatible,
    UnsupportedBuild,
    NeedsElevation,
}

pub fn check_compatibility(meta: &TweakMeta, ctx: &TweakContext) -> Compatibility {
    if !meta.supported_builds.contains(ctx.os.build) {
        Compatibility::UnsupportedBuild
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
    /// The registry the app ships with. Empty in Phase 1 by design.
    pub fn with_builtin_tweaks() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tweak: Box<dyn Tweak>) -> AppResult<()> {
        let meta = tweak.meta();
        validate_meta(meta)?;
        if self.tweaks.contains_key(meta.id) {
            return Err(AppError::Internal(format!(
                "duplicate tweak id `{}`",
                meta.id
            )));
        }
        self.tweaks.insert(meta.id, tweak);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&dyn Tweak> {
        self.tweaks.get(id).map(|t| t.as_ref())
    }

    pub fn list(&self) -> impl Iterator<Item = &TweakMeta> {
        self.tweaks.values().map(|t| t.meta())
    }

    pub fn len(&self) -> usize {
        self.tweaks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tweaks.is_empty()
    }
}

fn validate_meta(meta: &TweakMeta) -> AppResult<()> {
    let id_ok = !meta.id.is_empty()
        && meta
            .id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.');
    if !id_ok {
        return Err(AppError::Internal(format!(
            "invalid tweak id `{}`",
            meta.id
        )));
    }
    if meta.references.is_empty() {
        return Err(AppError::Internal(format!(
            "tweak `{}` has no documentation references",
            meta.id
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake(TweakMeta);

    impl Tweak for Fake {
        fn meta(&self) -> &TweakMeta {
            &self.0
        }
        fn detect(&self, _: &TweakContext) -> AppResult<Detection> {
            unimplemented!()
        }
        fn plan(&self, _: &TweakContext) -> AppResult<Vec<PlannedChange>> {
            Ok(vec![])
        }
        fn apply(&self, _: &TweakContext) -> AppResult<Backup> {
            unimplemented!()
        }
        fn revert(&self, _: &TweakContext, _: &Backup) -> AppResult<()> {
            unimplemented!()
        }
        fn verify(&self, _: &TweakContext) -> AppResult<bool> {
            Ok(false)
        }
    }

    fn meta(id: &'static str) -> TweakMeta {
        TweakMeta {
            id,
            name: "Test",
            description: "",
            details: "",
            category: TweakCategory::Gaming,
            risk: RiskLevel::Low,
            permission: Permission::Administrator,
            supported_builds: BuildRange {
                min_build: 22_000,
                max_build: None,
            },
            recommended: false,
            restart: RestartRequirement::None,
            references: &["https://learn.microsoft.com/"],
        }
    }

    fn ctx(build: u32, is_elevated: bool) -> TweakContext {
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
        }
    }

    #[test]
    fn builtin_registry_is_empty_in_phase_one() {
        assert!(TweakRegistry::with_builtin_tweaks().is_empty());
    }

    #[test]
    fn rejects_duplicate_ids() {
        let mut r = TweakRegistry::default();
        r.register(Box::new(Fake(meta("gaming.test")))).unwrap();
        assert!(r.register(Box::new(Fake(meta("gaming.test")))).is_err());
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn rejects_invalid_meta() {
        let mut r = TweakRegistry::default();
        assert!(r.register(Box::new(Fake(meta("Bad ID")))).is_err());
        let mut no_refs = meta("gaming.no-refs");
        no_refs.references = &[];
        assert!(r.register(Box::new(Fake(no_refs))).is_err());
    }

    #[test]
    fn compatibility_checks_build_then_elevation() {
        let m = meta("gaming.test");
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
