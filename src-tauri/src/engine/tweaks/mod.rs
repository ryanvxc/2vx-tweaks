//! The built-in tweak catalogue: one tweak per file, grouped by category.
//!
//! Each tweak must pass the checklist in `docs/optimisation-modules.md` before it is added
//! here; [`register_all`] runs every one through the metadata gate.

mod adapter;
mod gaming;
mod library;
mod network;
mod performance;
mod power;
mod privacy;
mod services;
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
    privacy::online_speech::TWEAK,
    privacy::inking_typing::TWEAK,
    privacy::feedback_requests::TWEAK,
    privacy::diagnostic_data::TWEAK,
    privacy::activity_history::TWEAK,
    privacy::background_apps::TWEAK,
    privacy::app_location::TWEAK,
    privacy::app_diagnostics::TWEAK,
    privacy::app_account_info::TWEAK,
    privacy::app_notifications::TWEAK,
    gaming::mmcss_games::TWEAK,
    gaming::game_dvr::TWEAK,
    performance::last_access::TWEAK,
    windows::window_animations::TWEAK,
    services::compat_assistant::TWEAK,
    services::sync_host::TWEAK,
    services::print_spooler::TWEAK,
    services::diagtrack::TWEAK,
    services::geolocation::TWEAK,
    gaming::mpo::TWEAK,
    gaming::system_responsiveness::TWEAK,
    power::power_throttling::TWEAK,
    performance::kernel_in_ram::TWEAK,
    windows::mouse_acceleration::TWEAK,
    windows::keyboard_repeat::TWEAK,
    windows::sticky_keys_shortcut::TWEAK,
    power::usb_selective_suspend::TWEAK,
    performance::short_names::TWEAK,
    network::delivery_optimization::TWEAK,
    network::prefer_ipv4::TWEAK,
    network::smb_throttling::TWEAK,
    network::llmnr::TWEAK,
    network::icmp_redirect::TWEAK,
    performance::fault_tolerant_heap::TWEAK,
    windows::autoplay::TWEAK,
    windows::device_metadata::TWEAK,
    network::qos::CS2,
    network::qos::VALORANT,
    network::qos::FORTNITE,
    network::qos::APEX,
    network::qos::LEAGUE,
    network::qos::OVERWATCH,
    network::qos::ROCKET_LEAGUE,
    network::qos::RAINBOW_SIX,
];

/// Hand-written tweaks that left the catalogue. See `library::retired`.
const RETIRED: &[SettingsTweak] = &[
    performance::foreground_boost::TWEAK,
    services::bluetooth::TWEAK,
    services::maps_broker::TWEAK,
    services::print_notify::TWEAK,
    services::sensor::TWEAK,
    services::sensor_monitoring::TWEAK,
    services::wap_push::TWEAK,
    services::xbox_auth::TWEAK,
    services::xbox_save::TWEAK,
    windows::rdp_com_redirect::TWEAK,
    windows::rdp_pnp_redirect::TWEAK,
    windows::storage_sense::TWEAK,
];

pub fn register_all(registry: &mut TweakRegistry) -> AppResult<()> {
    let tables = [ALL, adapter::ALL]
        .into_iter()
        .chain(library::ALL.iter().copied());
    for tweak in tables.flatten() {
        registry.register(Box::new(SettingsTweak {
            meta: tweak.meta.clone(),
            settings: tweak.settings,
        }))?;
    }
    for tweak in RETIRED
        .iter()
        .chain(adapter::RETIRED)
        .chain(library::RETIRED)
    {
        registry.register_retired(Box::new(SettingsTweak {
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
    use crate::engine::journal::SnapshotStatus;
    use crate::engine::service::{Engine, InProcess, TweakState};
    use crate::engine::store::{MemoryStore, SettingStore};
    use crate::engine::tests::ctx;
    use crate::engine::{
        Compatibility, EditionScope, HardwareScope, Hive, Permission, RegTarget, RegValue,
        RiskLevel,
    };
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

    fn engine_on(store: Arc<MemoryStore>, edition: Option<&'static str>) -> Engine {
        Engine::new(
            TweakRegistry::with_builtin_tweaks(),
            Box::new(Shared(store)),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(move || {
                let mut c = ctx(26_100, false);
                c.os.edition_id = edition.map(String::from);
                c
            }),
        )
    }

    fn engine(store: Arc<MemoryStore>) -> Engine {
        engine_on(store, None)
    }

    fn library() -> impl Iterator<Item = &'static SettingsTweak> {
        library::ALL.iter().copied().flatten()
    }

    /// A value the tweak doesn't write, so a revert to a Windows default would be caught.
    fn original(tweak_writes: Option<&RegValue>) -> RegValue {
        match tweak_writes {
            Some(RegValue::Dword(v)) => RegValue::Dword(u32::from(*v == 0)),
            Some(_) => RegValue::String("what this PC had".into()),
            None => RegValue::Dword(7),
        }
    }

    #[test]
    fn library_tweaks_apply_once_and_restore_what_the_pc_had() {
        let mut tested = 0;
        for (has_battery, cpu) in [(false, "GenuineIntel"), (true, "AuthenticAMD")] {
            let store = Arc::new(MemoryStore::default());
            for t in library() {
                if let Some(required) = &t.meta.requires {
                    store.write(required, Some(&RegValue::Dword(1))).unwrap();
                }
                for s in t.settings {
                    let was = original(s.value.as_ref());
                    store.write(&s.target, Some(&was)).unwrap();
                }
            }
            let e = Engine::new(
                TweakRegistry::with_builtin_tweaks(),
                Box::new(Shared(store.clone())),
                Box::new(InProcess),
                Journal::open_in_memory(),
                Box::new(move || {
                    let mut c = ctx(26_100, true);
                    c.has_battery = Some(has_battery);
                    c.gpu_vendors = vec![0x10de, 0x1002, 0x8086];
                    c.cpu_vendor = Some(cpu.into());
                    c
                }),
            );
            let usable: std::collections::BTreeSet<String> = e
                .tweak_statuses()
                .into_iter()
                .filter(|s| s.compatibility == Compatibility::Compatible)
                .map(|s| s.meta.id.to_string())
                .collect();

            for t in library().filter(|t| usable.contains(t.meta.id)) {
                let id = vec![t.meta.id.to_string()];
                let read = |t: &SettingsTweak| {
                    t.settings
                        .iter()
                        .map(|s| store.read(&s.target).unwrap())
                        .collect::<Vec<_>>()
                };
                let before = read(t);

                let plan = e.plan(&id).unwrap();
                assert_eq!(plan.changes.len(), t.settings.len(), "{}", t.meta.id);
                let out = e.apply(&plan.id).unwrap();
                assert_eq!(out.status, SnapshotStatus::Completed, "{}", t.meta.id);
                for s in t.settings {
                    assert_eq!(store.read(&s.target).unwrap(), s.value, "{}", s.target);
                }
                assert!(e.plan(&id).unwrap().changes.is_empty(), "{}", t.meta.id);

                let out = e.revert_tweak(t.meta.id).unwrap();
                assert_eq!(out.status, SnapshotStatus::Completed, "{}", t.meta.id);
                assert_eq!(read(t), before, "{} didn't restore", t.meta.id);
                tested += 1;
            }
        }
        assert!(tested > 400, "only {tested} library tweaks ran");
    }

    #[test]
    fn adapter_tweaks_stay_hidden_until_the_driver_lists_them_then_restore_what_it_had() {
        let store = Arc::new(MemoryStore::default());
        let e = Engine::new(
            TweakRegistry::with_builtin_tweaks(),
            Box::new(Shared(store.clone())),
            Box::new(InProcess),
            Journal::open_in_memory(),
            Box::new(|| ctx(26_100, true)),
        );
        let flow_control = "network.adapter-eth-flow-control";
        let compatibility = || {
            let all = e.tweak_statuses();
            let status = all.into_iter().find(|s| s.meta.id == flow_control);
            status.unwrap().compatibility
        };
        assert_eq!(compatibility(), Compatibility::NotInstalled);

        let keyword = RegTarget::new(Hive::Adapter, "Ethernet", "*FlowControl");
        let listed = RegTarget::new(
            Hive::Adapter,
            r"Ethernet\Ndi\Params\*FlowControl",
            "default",
        );
        // 3 is both directions on, the documented client default, which no tweak writes.
        let had = RegValue::String("3".into());
        store.write(&listed, Some(&had)).unwrap();
        store.write(&keyword, Some(&had)).unwrap();
        assert_eq!(compatibility(), Compatibility::Compatible);

        let plan = e.plan(&[flow_control.to_string()]).unwrap();
        assert_eq!(e.apply(&plan.id).unwrap().status, SnapshotStatus::Completed);
        let off = RegValue::String("0".into());
        assert_eq!(store.read(&keyword).unwrap(), Some(off));
        let out = e.revert_tweak(flow_control).unwrap();
        assert_eq!(out.status, SnapshotStatus::Completed);
        assert_eq!(store.read(&keyword).unwrap(), Some(had));
    }

    #[test]
    fn library_tweaks_read_well_in_a_row() {
        for t in library() {
            let m = &t.meta;
            assert!(m.name.chars().count() <= 40, "{} name", m.id);
            assert!(m.description.chars().count() <= 88, "{} description", m.id);
            assert!(m.section.is_some(), "{} section", m.id);
            assert_eq!(m.id.split('.').count(), 2, "{} id", m.id);
            if m.risk == RiskLevel::High {
                assert!(m.warning.is_some(), "{} needs a warning", m.id);
            }
        }
    }

    /// These five shipped writing Windows' own default, so their switch did nothing.
    #[test]
    fn visuals_write_the_value_their_name_promises() {
        let registry = TweakRegistry::with_builtin_tweaks();
        for (id, value) in [
            ("windows.taskbar-left", 0),
            ("windows.desktop-icons", 1),
            ("windows.enthusiast", 1),
            ("windows.accent-taskbar", 1),
            ("windows.nav-expand", 1),
            ("windows.hub-mode", 1),
        ] {
            let tweak = registry.get(id).unwrap_or_else(|| panic!("{id}"));
            let [setting] = tweak.settings() else {
                panic!("{id} should write one value");
            };
            assert_eq!(setting.value, Some(RegValue::Dword(value)), "{id}");
        }
    }

    #[test]
    fn a_retired_tweak_can_be_neither_listed_nor_planned_but_keeps_its_targets() {
        let registry = TweakRegistry::with_builtin_tweaks();
        assert!(registry.retired().count() >= 999);
        let e = engine(installed());
        let listed: std::collections::BTreeSet<_> =
            e.tweak_statuses().into_iter().map(|s| s.meta.id).collect();
        for id in registry.retired().map(|meta| meta.id) {
            assert!(!listed.contains(id), "{id} is still listed");
            assert!(e.plan(&[id.to_string()]).is_err(), "{id} can be planned");
            let tweak = registry.undoable(id);
            for setting in tweak.unwrap_or_else(|| panic!("{id}")).settings() {
                assert!(registry.declares_target(&setting.target), "{id}");
            }
        }
    }

    #[test]
    fn the_audit_left_a_curated_library() {
        let registry = TweakRegistry::with_builtin_tweaks();
        assert_eq!(registry.len(), 320);
        let visuals = registry
            .list()
            .filter(|m| m.category == crate::engine::TweakCategory::Visuals)
            .count();
        assert_eq!(visuals, 79);
    }

    #[test]
    fn a_card_windows_ignores_is_unavailable_rather_than_a_dead_switch() {
        const WINDOWS_10: u32 = 19045;
        const WINDOWS_11: u32 = 26200;
        let registry = TweakRegistry::with_builtin_tweaks();
        let builds = |id: &str| {
            registry
                .list()
                .find(|m| m.id == id)
                .unwrap_or_else(|| panic!("{id}"))
                .supported_builds
        };
        for id in ["windows.small-icons", "windows.tray-all"] {
            assert!(builds(id).contains(WINDOWS_10), "{id}");
            assert!(!builds(id).contains(WINDOWS_11), "{id}");
        }
        for id in [
            "windows.snap-bar",
            "windows.home-namespace",
            "power.startup-sound",
        ] {
            assert!(!builds(id).contains(WINDOWS_10), "{id}");
            assert!(builds(id).contains(WINDOWS_11), "{id}");
        }
        assert!(!builds("windows.copilot-button").contains(WINDOWS_11));
    }

    #[test]
    fn hardware_tweaks_stay_on_their_hardware() {
        let e = engine(installed());
        for s in e.tweak_statuses() {
            let wrong = matches!(
                s.meta.hardware,
                HardwareScope::AmdGpu | HardwareScope::IntelGpu | HardwareScope::AmdCpu
            );
            assert_eq!(
                s.compatibility == Compatibility::WrongHardware,
                wrong,
                "{}",
                s.meta.id
            );
        }
    }

    #[test]
    fn scheduled_tasks_are_only_turned_off() {
        for t in library() {
            for s in t.settings {
                if s.target.hive == Hive::ScheduledTask {
                    assert_eq!(s.value, Some(RegValue::Dword(0)), "{}", s.target);
                    assert!(t.meta.requires.is_some(), "{}", t.meta.id);
                }
            }
        }
    }

    /// A store where everything the catalogue requires is installed.
    fn installed() -> Arc<MemoryStore> {
        let store = Arc::new(MemoryStore::default());
        for t in ALL.iter().chain(library()) {
            if let Some(required) = &t.meta.requires {
                let image = RegValue::ExpandString("image.exe".into());
                store.write(required, Some(&image)).unwrap();
            }
        }
        store
    }

    fn compatibility(e: &Engine, id: &str) -> Compatibility {
        e.tweak_statuses()
            .into_iter()
            .find(|s| s.meta.id == id)
            .unwrap()
            .compatibility
    }

    #[test]
    fn catalogue_registers_with_user_and_admin_tweaks() {
        let registry = TweakRegistry::with_builtin_tweaks();
        let library: usize = library::ALL.iter().map(|t| t.len()).sum();
        assert_eq!(registry.len(), ALL.len() + adapter::ALL.len() + library);
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
        let e = engine(installed());
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
        let deny = (None, Some(RegValue::Dword(2)));
        for value in ["LetAppsRunInBackground", "LetAppsAccessLocation"] {
            assert_eq!(
                to(r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy", value),
                deny
            );
        }
        assert_eq!(
            to(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Scheduling Category"
            ),
            (None, Some(RegValue::String("High".into())))
        );
        assert_eq!(
            to(
                r"SYSTEM\CurrentControlSet\Control\FileSystem",
                "NtfsDisableLastAccessUpdate"
            ),
            (None, Some(RegValue::Dword(0x8000_0001)))
        );
        assert_eq!(
            to(r"SYSTEM\CurrentControlSet\Services\Spooler", "Start"),
            (None, Some(RegValue::Dword(4)))
        );
        let settings: usize = ALL.iter().map(|t| t.settings.len()).sum();
        assert_eq!(plan.changes.len(), settings);
        assert!(plan.skipped.is_empty());
        assert!(plan.needs_elevation);
    }

    #[test]
    fn service_tweaks_are_single_warned_and_need_the_service() {
        for t in ALL {
            let starts: Vec<_> = t
                .settings
                .iter()
                .filter(|s| {
                    // Start is the value that disables a service. Others under Services\ are that
                    // driver's own parameters, such as Tcpip's EnableICMPRedirect.
                    s.target.value == "Start"
                        && s.target
                            .key
                            .starts_with(r"SYSTEM\CurrentControlSet\Services\")
                })
                .collect();
            if starts.is_empty() {
                continue;
            }
            // One service per tweak: the rules forbid bulk service disabling.
            assert_eq!(t.settings.len(), 1, "{}", t.meta.id);
            assert_eq!(starts[0].value, Some(RegValue::Dword(4)), "{}", t.meta.id);
            assert!(t.meta.warning.is_some(), "{}", t.meta.id);
            let required = t.meta.requires.as_ref().expect(t.meta.id);
            assert_eq!(required.key, starts[0].target.key, "{}", t.meta.id);
        }
    }

    #[test]
    fn missing_service_is_not_installed_and_never_planned() {
        let e = engine(Arc::new(MemoryStore::default()));
        let id = "services.print-spooler-off";
        assert_eq!(compatibility(&e, id), Compatibility::NotInstalled);
        let plan = e.plan(&[id.to_string()]).unwrap();
        assert!(plan.changes.is_empty());
        assert_eq!(plan.skipped[0].reason, Compatibility::NotInstalled);

        let e = engine(installed());
        assert_eq!(compatibility(&e, id), Compatibility::NeedsElevation);
    }

    #[test]
    fn pro_policies_are_blocked_on_home_only() {
        let pro_tweak = ALL
            .iter()
            .find(|t| t.meta.edition == EditionScope::ProOrHigher)
            .unwrap();
        let id = pro_tweak.meta.id;
        for (edition, expected) in [
            ("Core", Compatibility::UnsupportedEdition),
            ("CoreSingleLanguage", Compatibility::UnsupportedEdition),
            ("Professional", Compatibility::NeedsElevation),
            ("Enterprise", Compatibility::NeedsElevation),
        ] {
            let e = engine_on(installed(), Some(edition));
            assert_eq!(compatibility(&e, id), expected, "{edition}");
        }
        // Unknown edition: allowed, like an unknown battery state.
        assert_eq!(
            compatibility(&engine(installed()), id),
            Compatibility::NeedsElevation
        );
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
