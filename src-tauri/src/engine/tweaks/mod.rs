//! The built-in tweak catalogue: one tweak per file, grouped by category.
//!
//! Each tweak must pass the checklist in `docs/optimisation-modules.md` before it is added
//! here; [`register_all`] runs every one through the metadata gate.

mod performance;
mod privacy;
mod windows;

use crate::engine::{SettingsTweak, TweakRegistry};
use crate::error::AppResult;

const ALL: &[SettingsTweak] = &[
    performance::edge_startup_boost::TWEAK,
    performance::edge_background_mode::TWEAK,
    privacy::app_launch_tracking::TWEAK,
    privacy::language_list_opt_out::TWEAK,
    privacy::edge_shopping::TWEAK,
    privacy::advertising_id::TWEAK,
    windows::edge_sidebar::TWEAK,
    windows::edge_recommendations::TWEAK,
];

pub fn register_all(registry: &mut TweakRegistry) -> AppResult<()> {
    for tweak in ALL {
        registry.register(Box::new(SettingsTweak {
            meta: tweak.meta.clone(),
            settings: tweak.settings,
        }))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::engine::journal::Journal;
    use crate::engine::service::{Engine, InProcess, TweakState};
    use crate::engine::store::{MemoryStore, SettingStore};
    use crate::engine::tests::ctx;
    use crate::engine::{Hive, Permission, RegTarget, RegValue};
    use crate::error::AppResult;

    struct Shared(Arc<MemoryStore>);
    impl SettingStore for Shared {
        fn read(&self, t: &RegTarget) -> AppResult<Option<RegValue>> {
            self.0.read(t)
        }
        fn write(&self, t: &RegTarget, v: Option<&RegValue>) -> AppResult<()> {
            self.0.write(t, v)
        }
    }

    fn engine(store: Arc<MemoryStore>) -> Engine {
        Engine::new(
            TweakRegistry::with_builtin_tweaks(),
            Box::new(Shared(store)),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, false)),
        )
    }

    #[test]
    fn catalogue_registers_with_user_and_admin_tweaks() {
        let registry = TweakRegistry::with_builtin_tweaks();
        assert_eq!(registry.len(), ALL.len());
        let perms: Vec<_> = registry.list().map(|m| m.permission).collect();
        assert!(perms.contains(&Permission::User));
        assert!(perms.contains(&Permission::Administrator));
        // HKCU\Software\Policies is admin-only, so a "User" tweak must not target it.
        for tweak in registry.iter() {
            for s in tweak.settings() {
                assert!(
                    !(s.target.hive == Hive::CurrentUser
                        && s.target
                            .key
                            .to_ascii_lowercase()
                            .starts_with(r"software\policies")),
                    "{}",
                    s.target
                );
            }
        }
    }

    #[test]
    fn plans_match_the_documented_values() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone());
        let ids: Vec<String> = ALL.iter().map(|t| t.meta.id.to_string()).collect();
        let plan = e.plan(&ids).unwrap();

        let to = |key: &str, value: &str| {
            plan.changes
                .iter()
                .find(|c| c.target.key == key && c.target.value == value)
                .map(|c| (c.from.clone(), c.to.clone()))
                .unwrap_or_else(|| panic!("{key}\\{value} not planned"))
        };
        let off = (None, Some(RegValue::Dword(0)));
        assert_eq!(
            to(r"SOFTWARE\Policies\Microsoft\Edge", "StartupBoostEnabled"),
            off
        );
        assert_eq!(
            to(r"SOFTWARE\Policies\Microsoft\Edge", "BackgroundModeEnabled"),
            off
        );
        assert_eq!(
            to(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "Start_TrackProgs"
            ),
            off
        );
        assert_eq!(
            to(
                r"Control Panel\International\User Profile",
                "HttpAcceptLanguageOptOut"
            ),
            (None, Some(RegValue::Dword(1)))
        );
        for value in [
            "EdgeShoppingAssistantEnabled",
            "HubsSidebarEnabled",
            "ShowRecommendationsEnabled",
        ] {
            assert_eq!(to(r"SOFTWARE\Policies\Microsoft\Edge", value), off);
        }
        assert_eq!(
            to(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                "Enabled"
            ),
            off
        );
        assert_eq!(plan.changes.len(), ALL.len());
        assert!(plan.needs_elevation);
    }

    #[test]
    fn apply_then_revert_round_trips() {
        let store = Arc::new(MemoryStore::default());
        let e = engine(store.clone());
        let plan = e
            .plan(&["privacy.app-launch-tracking-off".to_string()])
            .unwrap();
        let outcome = e.apply(&plan.id).unwrap();
        let status = |e: &Engine| {
            e.tweak_statuses()
                .into_iter()
                .find(|s| s.meta.id == "privacy.app-launch-tracking-off")
                .unwrap()
                .state
        };
        assert_eq!(status(&e), TweakState::Applied);

        e.revert(outcome.snapshot_id.unwrap()).unwrap();
        assert_eq!(status(&e), TweakState::NotApplied);
    }
}
