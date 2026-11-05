//! Services tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RemoteRegistry",
                "ImagePath",
            ),
            "services.remote-registry",
            Services,
            "Remote access",
            "Disable Remote Registry",
            "Makes sure nobody on the network can reach this PC's registry remotely.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Remote Registry is normally off on Windows 10 and 11, so most PCs see no change. This keeps it off if another program or admin tool turned it on. Only remote management tools that need it are affected.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RemoteRegistry",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WebClient",
                "ImagePath",
            ),
            "services.webclient",
            Services,
            "Remote access",
            "Disable WebClient (WebDAV)",
            "Closes a route attackers use to make Windows leak sign-in details.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("The service lets Windows open WebDAV folders. Microsoft suggests disabling it where unused, because rogue servers can use it to capture sign-in credentials. Most home PCs never use WebDAV. It's a precaution, so you won't see a difference day to day.")
        .warn("Mapped WebDAV drives stop working.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WebClient",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\LanmanServer",
                "ImagePath",
            ),
            "services.lanman-server",
            Services,
            "Remote access",
            "Disable File and Printer Sharing",
            "Stops this PC serving files and printers, so other machines can't reach it that way.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Shared folders and printers on this PC stop working.")
        .admin()
        .details("Good for a PC that shares nothing, as it removes a common attack route on a local network. Shared folders and printers on this PC stop working, including the hidden admin shares. You can still open shares on other PCs.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WSearch",
                "ImagePath",
            ),
            "services.wsearch",
            Services,
            "Search and shell",
            "Disable Search indexing",
            "Stops file indexing in the background, at the cost of slower, thinner file searches.",
            KUBAAM,
            Observed,
        )
        .medium()
        .warn("Start and Explorer search get slower and less complete.")
        .admin()
        .details("Indexing reads your files when the PC is idle, which can be noticeable on an older hard drive and barely matters on an SSD. Start still finds apps, but file and Outlook searches get slower or incomplete.")
        .impact(Medium)
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WSearch",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\CDPSvc",
                "ImagePath",
            ),
            "services.cdp",
            Services,
            "Sync and account",
            "Disable Connected Devices",
            "Stops the always-on background service behind Phone Link and Nearby Sharing.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Nearby Sharing and Phone Link stop working.")
        .admin()
        .details("Worth it if you never link a phone or share between PCs. It stops a service that normally runs all the time, and every cross-device feature built on it. The resource saving itself is small.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CDPSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NahimicService",
                "ImagePath",
            ),
            "services.nahimic",
            Services,
            "Third-party",
            "Disable Nahimic",
            "Stops Nahimic audio effects, a reported cause of game crashes and stutter.",
            BATLEZ,
            Observed,
        )
        .warn("Nahimic audio effects stop working.")
        .admin()
        .details("Nahimic comes with many MSI boards and laptops and some other brands, and it runs all the time. If you get trouble in games, turning it off is a common first step. Its sound effects and app stop working, and a driver update may restore it.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NahimicService",
            "Start",
            4,
        )],
    },
];
