//! Adapter Tuner: the NDIS standardized keywords Microsoft documents, on this PC's primary adapter
//! (`Hive::Adapter`). Each needs the driver to list the keyword under `Ndi\Params`, because a value
//! a driver never reads does nothing, so `requires` keeps every tweak to drivers that offer it.
//!
//! All of them write "0", which the documentation lists as Disabled. Everything is a restart, because
//! read-back proves the registry changed: a driver reads these when it starts, and only the Device
//! Manager page also restarts it. `policy` holds the matching keyword lists.

use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Evidence, HardwareScope, Hive, Impact, Permission,
    RegTarget, RestartRequirement, RiskLevel, Setting, SettingsTweak, TweakCategory, TweakMeta,
};

const ETHERNET: &str = "Ethernet adapter";
const WIFI: &str = "Wi-Fi adapter";

const POWER: &[&str] = &[
    "https://learn.microsoft.com/en-us/windows-hardware/drivers/network/standardized-inf-keywords-for-power-management",
];
const ENUMERATION: &[&str] =
    &["https://learn.microsoft.com/en-us/windows-hardware/drivers/network/enumeration-keywords"];
const OFFLOAD: &[&str] = &[
    "https://learn.microsoft.com/en-us/windows-hardware/drivers/network/enumeration-keywords",
    "https://learn.microsoft.com/en-us/windows-hardware/drivers/network/using-registry-values-to-enable-and-disable-task-offloading",
];
const SELECTIVE_SUSPEND: &[&str] = &[
    "https://learn.microsoft.com/en-us/windows-hardware/drivers/network/standardized-inf-keywords-for-ndis-selective-suspend",
];
const COALESCING: &[&str] = &[
    "https://learn.microsoft.com/en-us/windows-hardware/drivers/network/standardized-inf-keywords-for-packet-coalescing",
];

const STOPS_WOL: Option<&str> = Some("Wake-on-LAN stops working.");
const STOPS_WAKE: Option<&str> = Some("Network traffic can no longer wake this PC.");

// `requires` holds a `Cow`, so a const builder can't set it afterwards; it has to be an argument.
#[allow(clippy::too_many_arguments)]
const fn adapter(
    requires: RegTarget,
    id: &'static str,
    section: &'static str,
    name: &'static str,
    description: &'static str,
    details: &'static str,
    references: &'static [&'static str],
    warning: Option<&'static str>,
) -> TweakMeta {
    TweakMeta {
        id,
        name,
        description,
        details,
        category: TweakCategory::Network,
        risk: RiskLevel::Moderate,
        permission: Permission::Administrator,
        supported_builds: BuildRange::ALL,
        device: DeviceScope::Any,
        edition: EditionScope::Any,
        recommended: false,
        restart: RestartRequirement::Reboot,
        impact: Impact::Low,
        warning,
        requires: Some(requires),
        added_in: "0.4",
        section: Some(section),
        evidence: Evidence::Documented,
        hardware: HardwareScope::Any,
        references,
    }
}

/// One tweak. A macro, because a const can only keep its `settings` slice alive when the slice is
/// written directly in the initialiser, and the requires key is built from the keyword.
macro_rules! keyword {
    ($kind:literal, $keyword:literal, $id:literal, $section:expr, $name:literal,
     $description:literal, $references:expr, $warning:expr) => {
        SettingsTweak {
            meta: adapter(
                RegTarget::new(
                    Hive::Adapter,
                    concat!($kind, r"\Ndi\Params\", $keyword),
                    "default",
                ),
                concat!("network.adapter-", $id),
                $section,
                $name,
                $description,
                concat!(
                    "Sets ",
                    $keyword,
                    " to 0 on the connected adapter, or the first one listed if none is. The \
                     driver reads it when it starts, so this applies after a restart."
                ),
                $references,
                $warning,
            ),
            settings: &[Setting::string(Hive::Adapter, $kind, $keyword, "0")],
        }
    };
}

pub const ALL: &[SettingsTweak] = &[
    keyword!(
        "Ethernet",
        "*EEE",
        "eth-energy-efficient",
        ETHERNET,
        "Energy-Efficient Ethernet off",
        "Stops the link dropping to low power between packets.",
        POWER,
        None
    ),
    keyword!(
        "Ethernet",
        "*FlowControl",
        "eth-flow-control",
        ETHERNET,
        "Flow control off",
        "Stops the adapter pausing traffic when a link partner is busy.",
        ENUMERATION,
        None
    ),
    keyword!(
        "Ethernet",
        "*InterruptModeration",
        "eth-interrupt-moderation",
        ETHERNET,
        "Interrupt moderation off",
        "Interrupts per packet rather than in batches. Uses more CPU.",
        ENUMERATION,
        None
    ),
    keyword!(
        "Ethernet",
        "*WakeOnMagicPacket",
        "eth-wake-magic",
        ETHERNET,
        "Wake on Magic Packet off",
        "The PC no longer wakes when it receives a Magic Packet.",
        POWER,
        STOPS_WOL
    ),
    keyword!(
        "Ethernet",
        "*WakeOnPattern",
        "eth-wake-pattern",
        ETHERNET,
        "Wake on pattern match off",
        "The PC no longer wakes on matched network traffic.",
        POWER,
        STOPS_WAKE
    ),
    keyword!(
        "Ethernet",
        "*PMARPOffload",
        "eth-arp-offload",
        ETHERNET,
        "ARP offload off",
        "While asleep, the adapter stops answering IPv4 ARP requests.",
        POWER,
        None
    ),
    keyword!(
        "Ethernet",
        "*PMNSOffload",
        "eth-ns-offload",
        ETHERNET,
        "NS offload off",
        "While asleep, the adapter stops answering IPv6 neighbour requests.",
        POWER,
        None
    ),
    keyword!(
        "Ethernet",
        "*LsoV2IPv4",
        "eth-lso-ipv4",
        ETHERNET,
        "Large Send Offload off (IPv4)",
        "The CPU splits large IPv4 sends instead of the adapter.",
        OFFLOAD,
        None
    ),
    keyword!(
        "Ethernet",
        "*LsoV2IPv6",
        "eth-lso-ipv6",
        ETHERNET,
        "Large Send Offload off (IPv6)",
        "The CPU splits large IPv6 sends instead of the adapter.",
        OFFLOAD,
        None
    ),
    keyword!(
        "Ethernet",
        "*SelectiveSuspend",
        "eth-selective-suspend",
        ETHERNET,
        "Selective suspend off",
        "Keeps an idle adapter from entering its low-power state.",
        SELECTIVE_SUSPEND,
        None
    ),
    keyword!(
        "Wifi",
        "*WakeOnMagicPacket",
        "wifi-wake-magic",
        WIFI,
        "Wake on Magic Packet off",
        "The PC no longer wakes when it receives a Magic Packet.",
        POWER,
        STOPS_WOL
    ),
    keyword!(
        "Wifi",
        "*WakeOnPattern",
        "wifi-wake-pattern",
        WIFI,
        "Wake on pattern match off",
        "The PC no longer wakes on matched network traffic.",
        POWER,
        STOPS_WAKE
    ),
    keyword!(
        "Wifi",
        "*PMARPOffload",
        "wifi-arp-offload",
        WIFI,
        "ARP offload off",
        "While asleep, the adapter stops answering IPv4 ARP requests.",
        POWER,
        None
    ),
    keyword!(
        "Wifi",
        "*PMNSOffload",
        "wifi-ns-offload",
        WIFI,
        "NS offload off",
        "While asleep, the adapter stops answering IPv6 neighbour requests.",
        POWER,
        None
    ),
    keyword!(
        "Wifi",
        "*PacketCoalescing",
        "wifi-packet-coalescing",
        WIFI,
        "Packet coalescing off",
        "Hands broadcast packets to Windows at once instead of holding them.",
        COALESCING,
        None
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::RegValue;
    use crate::engine::policy::{ETHERNET_KEYWORDS, WIFI_KEYWORDS};

    fn keywords_of(kind: &str) -> Vec<&'static str> {
        let mut found: Vec<&str> = ALL
            .iter()
            .flat_map(|t| t.settings)
            .filter(|s| s.target.key == kind)
            .map(|s| match &s.target.value {
                std::borrow::Cow::Borrowed(value) => *value,
                std::borrow::Cow::Owned(_) => unreachable!("tweaks are constants"),
            })
            .collect();
        found.sort_unstable();
        found
    }

    #[test]
    fn every_documented_keyword_has_one_tweak_and_nothing_else_does() {
        let sorted = |list: &[&'static str]| {
            let mut list = list.to_vec();
            list.sort_unstable();
            list
        };
        assert_eq!(keywords_of("Ethernet"), sorted(ETHERNET_KEYWORDS));
        assert_eq!(keywords_of("Wifi"), sorted(WIFI_KEYWORDS));
        assert_eq!(ALL.len(), ETHERNET_KEYWORDS.len() + WIFI_KEYWORDS.len());
    }

    #[test]
    fn each_tweak_turns_its_keyword_off_and_needs_the_driver_to_list_it() {
        for tweak in ALL {
            let [setting] = tweak.settings else {
                panic!("{} should set one value", tweak.meta.id);
            };
            assert_eq!(setting.value, Some(RegValue::String("0".into())));
            let requires = tweak.meta.requires.as_ref().unwrap();
            assert_eq!(requires.hive, Hive::Adapter);
            assert_eq!(requires.value, "default");
            assert_eq!(
                requires.key,
                format!(
                    r"{}\Ndi\Params\{}",
                    setting.target.key, setting.target.value
                ),
                "{}",
                tweak.meta.id
            );
            assert_eq!(tweak.meta.restart, RestartRequirement::Reboot);
            assert!(tweak.meta.name.chars().count() <= 33, "{}", tweak.meta.name);
            assert!(
                tweak.meta.description.chars().count() <= 74,
                "{}",
                tweak.meta.id
            );
        }
    }

    #[test]
    fn only_a_stopped_feature_gets_a_warning() {
        let warned: Vec<&str> = ALL
            .iter()
            .filter(|t| t.meta.warning.is_some())
            .map(|t| t.meta.id)
            .collect();
        assert_eq!(
            warned,
            [
                "network.adapter-eth-wake-magic",
                "network.adapter-eth-wake-pattern",
                "network.adapter-wifi-wake-magic",
                "network.adapter-wifi-wake-pattern",
            ]
        );
    }
}
