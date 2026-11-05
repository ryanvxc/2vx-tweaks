//! Network tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "network.source-routing",
            Network,
            "TCP",
            "Drop source-routed packets",
            "Drops network packets that try to choose their own path through this PC.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Source routing is an old feature attackers can use to get around routing rules. Almost nothing legitimate uses it, so blocking it is a small, safe hardening step from Windows security baselines.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "DisableIPSourceRouting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.name-release",
            Network,
            "TCP",
            "Ignore NetBIOS name release",
            "Stops other devices forcing this PC to give up its NetBIOS name.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("A small hardening step from Windows security baselines that blocks a simple denial-of-service trick on local networks. You won't notice any difference in normal use, and it only matters if NetBIOS is in use.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "NoNameReleaseOnDemand",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.winhttp-wpad",
            Network,
            "Winsock and resolution",
            "Turn off WPAD",
            "Stops Windows searching your network for an automatic proxy configuration.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Networks that rely on automatic proxy setup stop working.")
        .admin()
        .details("WPAD can be abused by a device on the same network to redirect web traffic through it. Home PCs rarely need it, but workplaces and schools that use automatic proxy setup do."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\WinHttp",
            "DisableWpad",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.doh",
            Network,
            "Winsock and resolution",
            "Allow encrypted DNS (DoH)",
            "Lets Windows use encrypted DNS when your DNS server supports it.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Hides your lookups from anyone watching the network between you and your DNS server. If the server lacks support, Windows falls back to normal DNS. You still need a DoH provider set. Windows 11 only."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
            "DoHPolicy",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-neg-ttl",
            Network,
            "DNS cache",
            "Forget failed DNS lookups sooner",
            "Shortens how long Windows remembers a failed website lookup.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Windows remembers when a lookup fails, so a site that was briefly unreachable can keep failing after it recovers. This shortens that wait. Browsers that use their own DNS client may not be affected.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "MaxNegativeCacheTtl",
            300,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-netbios",
            Network,
            "DNS cache",
            "No NetBIOS name lookups",
            "Stops the Windows 11 DNS client using NetBIOS to find names on the network.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("NetBIOS is an old name system that can be spoofed on shared networks. Windows already avoids it on public networks, and this extends that to every network. Very old devices may stop being found by name.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "EnableNetbios",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-signing-require",
            Network,
            "SMB",
            "SMB signing required",
            "Requires signed connections to this PC's shared folders, which blocks tampering.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Older devices may not connect.")
        .admin()
        .details("Protects file shares against relay and tampering attacks on your network. Recent Windows 11 builds do this by default. Old NAS boxes and scanners may fail to connect, and very fast transfers can slow slightly.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "RequireSecuritySignature",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-admin-shares-server",
            Network,
            "SMB",
            "No hidden admin shares",
            "Removes the hidden C$ and ADMIN$ style shares Windows creates automatically.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Some remote management tools need these.")
        .admin()
        .details("They let administrators reach the drive remotely, which an attacker with a stolen password can also use. Most home PCs don't need them, though some remote support and backup tools do.")
        .reboot(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
                "AutoShareServer",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
                "AutoShareWks",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.smb1",
            Network,
            "SMB",
            "Turn off SMB 1",
            "Turns off the old SMB 1 protocol for folders shared from this PC.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("SMB 1 is insecure and was the route used by WannaCry. New Windows installs already leave it out, so this matters on older or upgraded PCs. Very old devices that read this PC's shares will stop connecting.")
        .reboot(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "SMB1",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.throttling-index",
            Network,
            "TCP",
            "No network throttling",
            "Stops Windows limiting network traffic while music or video is playing.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin()
        .details("Multimedia apps can trigger a limit of about 10 packets per millisecond on other traffic. It mainly affects very fast local transfers, so gamers on normal connections rarely notice. Needs a restart."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            "NetworkThrottlingIndex",
            4294967295,
        )],
    },
];
