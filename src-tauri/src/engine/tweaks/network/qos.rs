//! Network Priority: one policy-based QoS rule per game. Each is a tweak of its own, so it gets the
//! usual review, journal and undo. The games are a fixed table because commands never take a
//! process name.

use crate::engine::{
    BuildRange, DeviceScope, EditionScope, Hive, Impact, Permission, RestartRequirement, RiskLevel,
    Setting, SettingsTweak, TweakCategory, TweakMeta,
};

/// `id:` stays on its own line in each row: the frontend's test reads the catalogue with a regex.
macro_rules! qos_policy {
    ($tweak:ident, id: $id:literal, game: $game:literal, exe: $exe:literal,) => {
        pub const $tweak: SettingsTweak = SettingsTweak {
            meta: TweakMeta {
                id: $id,
                name: $game,
                description: concat!("Asks for high priority for ", $game, ". Not all networks honour it."),
                details: "Sets a Windows QoS policy that marks the game's packets with DSCP 46. It only helps if Windows applies the policy on your network and your router or ISP acts on the mark. Windows may skip it outside a domain and many home routers ignore it, so you may see no change.",
                category: TweakCategory::Network,
                risk: RiskLevel::Low,
                permission: Permission::Administrator,
                supported_builds: BuildRange::ALL,
                device: DeviceScope::Any,
                edition: EditionScope::Any,
                recommended: false,
                restart: RestartRequirement::Reboot,
                impact: Impact::Low,
                warning: None,
                requires: None,
                added_in: "0.2",
                section: None,
                evidence: crate::engine::Evidence::Documented,
                hardware: crate::engine::HardwareScope::Any,
                references: &[
                    "https://learn.microsoft.com/en-us/microsoftteams/qos-in-teams-clients",
                    "https://learn.microsoft.com/en-us/windows-server/networking/technologies/qos/qos-policy-manage",
                ],
            },
            settings: &[
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Version", "1.0"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Application Name", $exe),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Protocol", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Local Port", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Local IP", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Local IP Prefix Length", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Remote Port", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Remote IP", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Remote IP Prefix Length", "*"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "DSCP Value", "46"),
                Setting::string(Hive::LocalMachine, concat!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\2VX ", $game), "Throttle Rate", "-1"),
            ],
        };
    };
}

qos_policy!(
    CS2,
    id: "network.qos-cs2",
    game: "Counter-Strike 2",
    exe: "cs2.exe",
);
qos_policy!(
    VALORANT,
    id: "network.qos-valorant",
    game: "VALORANT",
    exe: "VALORANT-Win64-Shipping.exe",
);
qos_policy!(
    FORTNITE,
    id: "network.qos-fortnite",
    game: "Fortnite",
    exe: "FortniteClient-Win64-Shipping.exe",
);
qos_policy!(
    APEX,
    id: "network.qos-apex",
    game: "Apex Legends",
    exe: "r5apex.exe",
);
qos_policy!(
    LEAGUE,
    id: "network.qos-league",
    game: "League of Legends",
    exe: "League of Legends.exe",
);
qos_policy!(
    OVERWATCH,
    id: "network.qos-overwatch",
    game: "Overwatch 2",
    exe: "Overwatch.exe",
);
qos_policy!(
    ROCKET_LEAGUE,
    id: "network.qos-rocket-league",
    game: "Rocket League",
    exe: "RocketLeague.exe",
);
qos_policy!(
    RAINBOW_SIX,
    id: "network.qos-rainbow-six",
    game: "Rainbow Six Siege",
    exe: "RainbowSix.exe",
);
