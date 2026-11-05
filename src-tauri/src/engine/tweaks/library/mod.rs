//! The tweak library: tweaks from community packs and vendors, one table entry each.
//!
//! The tweaks in the folders above have a file each because each was written and reviewed on its
//! own. The library is too large for that, so an entry states only what differs from the default:
//! low risk, low impact, no restart, any PC. It goes through the same gate and the same engine.
//! `evidence` says how much is known about each setting, and the UI shows it.

use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Evidence, Evidence::*, HardwareScope, HardwareScope::*,
    Hive, Impact, Impact::*, Permission, RegTarget, RestartRequirement, RiskLevel, Setting,
    SettingsTweak, TweakCategory, TweakCategory::*, TweakMeta,
};

mod apps;
mod cpu;
mod gaming;
mod gpu;
mod memory;
mod network;
mod power;
mod privacy;
mod retired;
mod services;
mod storage;
mod tasks;
mod visuals;
mod windows;

pub const ALL: &[&[SettingsTweak]] = &[
    apps::TWEAKS,
    cpu::TWEAKS,
    gaming::TWEAKS,
    gpu::TWEAKS,
    memory::TWEAKS,
    network::TWEAKS,
    power::TWEAKS,
    privacy::TWEAKS,
    services::TWEAKS,
    storage::TWEAKS,
    tasks::TWEAKS,
    visuals::TWEAKS,
    windows::TWEAKS,
];

pub const RETIRED: &[SettingsTweak] = retired::TWEAKS;

type Refs = &'static [&'static str];

// Where a tweak came from. Settings are facts, so nothing here copies the packs' code; the links
// credit them and let a reader see the original.
const BATLEZ: Refs = &["https://github.com/PrasyIkuzo/Batlez-Tweaks"];
const TAIRIKU: Refs = &["https://github.com/TairikuOokami/Windows"];
const ALCHEMY: Refs = &["https://github.com/AlchemyTweaks"];
const KUBAAM: Refs = &["https://github.com/kubaam"];

// Documentation for tweaks marked `Documented`.
const POLICY_DOCS: Refs = &[
    "https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-configuration-service-provider",
];
const POWER_DOCS: Refs = &[
    "https://learn.microsoft.com/en-us/windows-hardware/customize/power-settings/configure-power-settings",
];
const CHROME_DOCS: Refs = &["https://chromeenterprise.google/policies/"];
const EDGE_DOCS: Refs = &["https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies"];
const FIREFOX_DOCS: Refs = &["https://mozilla.github.io/policy-templates/"];
const TASKBAR_DOCS: Refs = &[
    "https://support.microsoft.com/en-us/windows/experience/personalization/customize-the-taskbar-in-windows",
];
const START_DOCS: Refs = &[
    "https://support.microsoft.com/en-gb/windows/experience/personalization/customize-the-windows-start-menu",
];
const COLOUR_DOCS: Refs = &[
    "https://support.microsoft.com/en-us/windows/experience/personalization/personalize-your-colors-in-windows",
];
const DESKTOP_DOCS: Refs = &[
    "https://support.microsoft.com/en-us/windows/experience/personalization/customize-the-desktop-icons-in-windows",
];
const EXPLORER_DOCS: Refs = &[
    "https://support.microsoft.com/en-us/windows/experience/fileexplorer/file-explorer-in-windows",
];
const BRAVE_DOCS: Refs = &["https://support.brave.com/hc/en-us/articles/360039248271-Group-Policy"];

const fn hklm(key: &'static str, name: &'static str, data: u32) -> Setting {
    Setting::dword(Hive::LocalMachine, key, name, data)
}
const fn hkcu(key: &'static str, name: &'static str, data: u32) -> Setting {
    Setting::dword(Hive::CurrentUser, key, name, data)
}
const fn hklm_str(key: &'static str, name: &'static str, data: &'static str) -> Setting {
    Setting::string(Hive::LocalMachine, key, name, data)
}
const fn hkcu_str(key: &'static str, name: &'static str, data: &'static str) -> Setting {
    Setting::string(Hive::CurrentUser, key, name, data)
}
const fn hklm_absent(key: &'static str, name: &'static str) -> Setting {
    Setting::absent(Hive::LocalMachine, key, name)
}
const fn hkcu_absent(key: &'static str, name: &'static str) -> Setting {
    Setting::absent(Hive::CurrentUser, key, name)
}
const fn gpu(key: &'static str, name: &'static str, data: u32) -> Setting {
    Setting::dword(Hive::Gpu, key, name, data)
}
const fn gpu_str(key: &'static str, name: &'static str, data: &'static str) -> Setting {
    Setting::string(Hive::Gpu, key, name, data)
}
/// Turns a scheduled task off.
const fn task(path: &'static str) -> Setting {
    Setting::dword(Hive::ScheduledTask, path, "Enabled", 0)
}
const fn power(key: &'static str, mode: &'static str, data: u32) -> Setting {
    Setting::dword(Hive::PowerPlan, key, mode, data)
}

/// A library tweak with the defaults. Chain the methods below for anything else. The entry itself
/// is a `SettingsTweak` literal in the table, because a const can only keep its `settings` slice
/// alive when the slice is written directly in the initialiser.
const fn lib(
    id: &'static str,
    category: TweakCategory,
    section: &'static str,
    name: &'static str,
    description: &'static str,
    references: Refs,
    evidence: Evidence,
) -> TweakMeta {
    build(
        None,
        id,
        category,
        section,
        name,
        description,
        references,
        evidence,
    )
}

/// A tweak for something that may not exist on this PC: a service, a task or a power setting.
/// `requires` names a value the engine checks first, so it reports "not installed".
// `requires` holds a `Cow`, so a const builder can't set it afterwards; it has to be an argument.
#[allow(clippy::too_many_arguments)]
const fn guarded(
    requires: RegTarget,
    id: &'static str,
    category: TweakCategory,
    section: &'static str,
    name: &'static str,
    description: &'static str,
    references: Refs,
    evidence: Evidence,
) -> TweakMeta {
    build(
        Some(requires),
        id,
        category,
        section,
        name,
        description,
        references,
        evidence,
    )
}

#[allow(clippy::too_many_arguments)]
const fn build(
    requires: Option<RegTarget>,
    id: &'static str,
    category: TweakCategory,
    section: &'static str,
    name: &'static str,
    description: &'static str,
    references: Refs,
    evidence: Evidence,
) -> TweakMeta {
    TweakMeta {
        id,
        name,
        description,
        details: description,
        category,
        risk: RiskLevel::Low,
        permission: Permission::User,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::None,
        impact: Impact::Low,
        warning: None,
        requires,
        added_in: "0.3",
        section: Some(section),
        evidence,
        hardware: HardwareScope::Any,
        references,
    }
}

impl TweakMeta {
    /// The longer text behind the card. Without it the card's line is repeated.
    const fn details(mut self, details: &'static str) -> Self {
        self.details = details;
        self
    }
    const fn added(mut self, version: &'static str) -> Self {
        self.added_in = version;
        self
    }
    const fn admin(mut self) -> Self {
        self.permission = Permission::Administrator;
        self
    }
    const fn medium(mut self) -> Self {
        self.risk = RiskLevel::Moderate;
        self
    }
    const fn high(mut self) -> Self {
        self.risk = RiskLevel::High;
        self
    }
    const fn impact(mut self, impact: Impact) -> Self {
        self.impact = impact;
        self
    }
    const fn sign_out(mut self) -> Self {
        self.restart = RestartRequirement::SignOut;
        self
    }
    const fn reboot(mut self) -> Self {
        self.restart = RestartRequirement::Reboot;
        self
    }
    const fn desktop(mut self) -> Self {
        self.device = DeviceScope::DesktopOnly;
        self
    }
    const fn laptop(mut self) -> Self {
        self.device = DeviceScope::LaptopOnly;
        self
    }
    const fn hw(mut self, hardware: HardwareScope) -> Self {
        self.hardware = hardware;
        self
    }
    const fn builds(mut self, min_build: u32) -> Self {
        self.supported_builds = BuildRange {
            min_build,
            max_build: None,
        };
        self
    }
    /// The last build a setting does anything on. Call after `builds`.
    const fn until(mut self, max_build: u32) -> Self {
        self.supported_builds.max_build = Some(max_build);
        self
    }
    const fn warn(mut self, warning: &'static str) -> Self {
        self.warning = Some(warning);
        self
    }
}
