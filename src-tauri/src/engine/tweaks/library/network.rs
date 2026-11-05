//! Network tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "network.ack-frequency",
            Network,
            "TCP",
            "TCP ACK frequency",
            "Sends ACKs at once rather than batching.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpAckFrequency",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.del-ack",
            Network,
            "TCP",
            "Delayed ACK ticks",
            "Removes delayed ACK.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpDelAckTicks",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.no-delay",
            Network,
            "TCP",
            "TCP no delay",
            "Turns off Nagle's algorithm.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TCPNoDelay",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.ttl",
            Network,
            "TCP",
            "Default TTL",
            "Sets the default packet lifetime to 64.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "DefaultTTL",
            64,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.timed-wait",
            Network,
            "TCP",
            "TIME_WAIT delay",
            "Frees closed ports after 30 seconds.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpTimedWaitDelay",
            30,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.max-user-port",
            Network,
            "TCP",
            "Dynamic port limit",
            "Raises the ephemeral port limit.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "MaxUserPort",
            65534,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.tcp1323",
            Network,
            "TCP",
            "TCP window scaling",
            "Turns on RFC 1323 timestamps and scaling.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "Tcp1323Opts",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.max-retrans",
            Network,
            "TCP",
            "Data retransmissions",
            "Gives up sooner on dead connections.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpMaxDataRetransmissions",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.max-data-retrans",
            Network,
            "TCP",
            "Retransmission limit",
            "Older name for the same limit.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "MaxDataRetransmissions",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.connect-retrans",
            Network,
            "TCP",
            "Connect retransmissions",
            "Fails connection attempts sooner.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpMaxConnectRetransmissions",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dup-acks",
            Network,
            "TCP",
            "Duplicate ACKs",
            "Retransmits after two duplicate ACKs.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpMaxDupAcks",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.rto",
            Network,
            "TCP",
            "Initial retransmission time",
            "Lowers the first retry delay.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "initialRto",
            2000,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.urgent",
            Network,
            "TCP",
            "RFC 1122 urgent pointer",
            "Sets the urgent pointer style.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "TcpUseRFC1122UrgentPointer",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.pmtu",
            Network,
            "TCP",
            "Path MTU discovery",
            "Makes sure path MTU discovery is on.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnablePMTUDiscovery",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.pmtu-bh",
            Network,
            "TCP",
            "Black hole detection",
            "Turns off PMTU black hole detection.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnablePMTUBHDetect",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dead-gw",
            Network,
            "TCP",
            "Dead gateway detection",
            "Fails over from a dead gateway.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnableDeadGWDetect",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dca",
            Network,
            "TCP",
            "Direct cache access",
            "Lets the adapter write to CPU cache.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnableDCA",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.task-offload",
            Network,
            "TCP",
            "Task offload",
            "Keeps offloading work to the adapter.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "DisableTaskOffload",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.free-tcbs",
            Network,
            "TCP",
            "TCP control blocks",
            "Raises the free TCB cache.",
            BATLEZ,
            Community,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "MaxFreeTcbs",
            65535,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.hash-table",
            Network,
            "TCP",
            "TCP hash table",
            "Raises the TCB hash table.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "MaxHashTableSize",
            65536,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.irp-stack",
            Network,
            "TCP",
            "Network IRP stack",
            "Raises the stack size for network I/O.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                "IRPStackSize",
                50,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
                "IRPStackSize",
                20,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.forward-buffers",
            Network,
            "TCP",
            "Forward buffer memory",
            "Sets forward buffer size.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                "ForwardBufferMemory",
                1376256,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                "MaxForwardBufferMemory",
                1376256,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.forward-packets",
            Network,
            "TCP",
            "Forward packets",
            "Sets forward packet count.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                "NumForwardPackets",
                598,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                "MaxNumForwardPackets",
                598,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.mcast-forward",
            Network,
            "TCP",
            "Multicast forwarding",
            "Stops forwarding multicast.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnableMulticastForwarding",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.igmp",
            Network,
            "TCP",
            "IGMP level",
            "Sets how this PC joins multicast groups.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "IGMPLevel",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.media-sense-log",
            Network,
            "TCP",
            "Media sense log",
            "Stops logging cable changes.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "DisableMediaSenseEventLog",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.source-routing",
            Network,
            "TCP",
            "IP source routing",
            "Blocks source-routed packets.",
            BATLEZ,
            Documented,
        )
        .admin(),
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
            "NetBIOS name release",
            "Ignores name release demands.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "NoNameReleaseOnDemand",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.arp-retry",
            Network,
            "TCP",
            "ARP retries",
            "Sets ARP retry count.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "ArpRetryCount",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.arp-single",
            Network,
            "TCP",
            "ARP single route",
            "Sends ARP on one route.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "ArpTRSingleRoute",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.arp-snap",
            Network,
            "TCP",
            "ARP SNAP",
            "Uses Ethernet II for ARP.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "ArpUseEtherSNAP",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.afd-receive",
            Network,
            "Winsock and resolution",
            "Default receive window",
            "Sets Winsock's receive buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AFD\Parameters",
            "DefaultReceiveWindow",
            65536,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.afd-send",
            Network,
            "Winsock and resolution",
            "Default send window",
            "Sets Winsock's send buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AFD\Parameters",
            "DefaultSendWindow",
            65536,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.provider-dns",
            Network,
            "Winsock and resolution",
            "DNS priority",
            "Raises DNS in name resolution order.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\ServiceProvider",
            "DnsPriority",
            6,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.provider-hosts",
            Network,
            "Winsock and resolution",
            "Hosts priority",
            "Raises the hosts file in resolution order.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\ServiceProvider",
            "HostsPriority",
            5,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.provider-local",
            Network,
            "Winsock and resolution",
            "Local priority",
            "Raises local names in resolution order.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\ServiceProvider",
            "LocalPriority",
            4,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.provider-netbt",
            Network,
            "Winsock and resolution",
            "NetBT priority",
            "Sets NetBIOS in resolution order.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\ServiceProvider",
            "NetbtPriority",
            7,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.lmhosts",
            Network,
            "Winsock and resolution",
            "LMHOSTS lookup",
            "Stops reading the LMHOSTS file.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NetBT\Parameters",
            "EnableLMHOSTS",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.rss-base",
            Network,
            "Winsock and resolution",
            "RSS base CPU",
            "Moves network receive work off core 0.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Ndis\Parameters",
            "RssBaseCpu",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.winhttp-wpad",
            Network,
            "Winsock and resolution",
            "WPAD",
            "Stops proxy auto-discovery.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Networks that rely on automatic proxy setup stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\WinHttp",
            "DisableWpad",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.connections-1",
            Network,
            "Winsock and resolution",
            "HTTP/1.0 connections",
            "Allows more connections per server.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "MaxConnectionsPer1_0Server",
            16,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.connections",
            Network,
            "Winsock and resolution",
            "HTTP connections",
            "Allows more connections per server.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "MaxConnectionsPerServer",
            16,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.doh",
            Network,
            "Winsock and resolution",
            "DNS over HTTPS",
            "Requires encrypted DNS when available.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
            "DoHPolicy",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.neg-cache-policy",
            Network,
            "Winsock and resolution",
            "Negative DNS cache (policy)",
            "Limits failed lookups in the cache.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
            "MaxNegativeCacheTtl",
            300,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-table",
            Network,
            "DNS cache",
            "DNS cache table",
            "Raises DNS cache table size.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "CacheHashTableSize",
            384,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-bucket",
            Network,
            "DNS cache",
            "DNS cache bucket",
            "Sets DNS bucket size.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "CacheHashTableBucketSize",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-max-ttl",
            Network,
            "DNS cache",
            "DNS maximum TTL",
            "Sets how long answers stay cached.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "MaxCacheTtl",
            86400,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-neg-ttl",
            Network,
            "DNS cache",
            "DNS negative TTL",
            "Limits how long failures stay cached.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "MaxNegativeCacheTtl",
            300,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-soa-ttl",
            Network,
            "DNS cache",
            "DNS SOA TTL",
            "Limits SOA cache time.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "MaxSOACacheEntryTtlLimit",
            300,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-neg-time",
            Network,
            "DNS cache",
            "Negative cache time",
            "Limits how long failures stay cached.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "NegativeCacheTime",
            300,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-net-failure",
            Network,
            "DNS cache",
            "Network failure cache",
            "Limits how long network failures stay cached.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "NetFailureCacheTime",
            30,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-timeout",
            Network,
            "DNS cache",
            "DNS cache timeout",
            "Sets DNS cache timeout.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "DnsCacheTimeout",
            86400,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-max-size",
            Network,
            "DNS cache",
            "DNS cache size",
            "Raises maximum DNS cache size.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "MaxCacheSize",
            33554432,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.dns-netbios",
            Network,
            "DNS cache",
            "NetBIOS in DNS Client",
            "Turns off NetBIOS lookups.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "EnableNetbios",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-oplocks",
            Network,
            "SMB",
            "SMB oplocks",
            "Stops opportunistic locks.",
            BATLEZ,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "EnableOplocks",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-signing-enable",
            Network,
            "SMB",
            "SMB signing",
            "Offers signing on this PC's shares.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "EnableSecuritySignature",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-signing-require",
            Network,
            "SMB",
            "SMB signing required",
            "Requires signing on this PC's shares.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Older devices may not connect.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "RequireSecuritySignature",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-autodisconnect",
            Network,
            "SMB",
            "SMB auto disconnect",
            "Stops dropping idle clients.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "autodisconnect",
            4294967295,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-admin-shares-server",
            Network,
            "SMB",
            "Admin shares",
            "Stops C$ style shares.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Some remote management tools need these.")
        .admin(),
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
            "SMB 1",
            "Turns off SMB 1 on this PC's shares.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
            "SMB1",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-size",
            Network,
            "SMB",
            "Server size",
            "Optimises the file server for throughput.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\lanmanserver\parameters",
            "Size",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-sharing-delay",
            Network,
            "SMB",
            "Sharing violation delay",
            "Removes the retry wait.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
                "SharingViolationDelay",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\LanmanServer\Parameters",
                "SharingViolationRetries",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.null-sessions",
            Network,
            "SMB",
            "Null session access",
            "Blocks anonymous share access.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanManServer\Parameters",
            "restrictnullsessaccess",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-dir-cache",
            Network,
            "SMB",
            "SMB directory cache",
            "Raises the client's directory cache.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "DirectoryCacheEntriesMax",
            1024,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-file-cache",
            Network,
            "SMB",
            "SMB file info cache",
            "Raises the client's file info cache.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "FileInfoCacheEntriesMax",
            1024,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-large-mtu",
            Network,
            "SMB",
            "SMB large MTU",
            "Keeps large MTU on.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "DisableLargeMtu",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-keepconn",
            Network,
            "SMB",
            "SMB keep connection",
            "Keeps idle SMB connections for 10 minutes.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "KeepConn",
            600,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-max-cmds",
            Network,
            "SMB",
            "SMB max commands",
            "Sets concurrent SMB commands.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "MaxCmds",
            16,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.qos-valorant-launcher",
            Network,
            "QoS",
            "VALORANT launcher QoS tag",
            "Tags valorant.exe traffic with DSCP 46.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Version",
                "1.0",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Application Name",
                "valorant.exe",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Protocol",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Local Port",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Local IP",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Local IP Prefix Length",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Remote Port",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Remote IP",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Remote IP Prefix Length",
                "*",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "DSCP Value",
                "46",
            ),
            hklm_str(
                r"SOFTWARE\Policies\Microsoft\Windows\QoS\VALORANT",
                "Throttle Rate",
                "-1",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.psched-sends",
            Network,
            "QoS",
            "Packet scheduler sends",
            "Raises outstanding sends.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Psched",
            "MaxOutstandingSends",
            65536,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.psched-timer",
            Network,
            "QoS",
            "Packet scheduler timer",
            "Sets the scheduler's timer.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Psched",
            "TimerResolution",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.wifi-sense",
            Network,
            "Discovery and Wi-Fi",
            "Wi-Fi Sense",
            "Stops auto-connecting to suggested hotspots.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots",
                "value",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\WiFi\AllowWiFiHotSpotReporting",
                "value",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config",
                "AutoConnectAllowedOEM",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.hotspot-login",
            Network,
            "Discovery and Wi-Fi",
            "Hotspot auto login",
            "Stops automatic hotspot login.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\WifiNetworkManager\HotspotLogin",
            "IsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.wcn",
            Network,
            "Discovery and Wi-Fi",
            "Windows Connect Now",
            "Stops WPS configuration.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WCN\Registrars",
                "EnableRegistrars",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WCN\UI",
                "DisableWcnUi",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.lm-hosts-wsd",
            Network,
            "Discovery and Wi-Fi",
            "WSD in TCP/IP",
            "Turns off Web Services Discovery hooks.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
            "EnableWsd",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.teredo-mdns",
            Network,
            "Discovery and Wi-Fi",
            "Local name providers",
            "Turns off Winsock name providers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Network name lookups may fail.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WinSock2\Parameters\NameSpace_Catalog5\Catalog_Entries64\000000000001",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.metered-device",
            Network,
            "Discovery and Wi-Fi",
            "Costed network policy",
            "Stops device metadata on metered networks.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"Microsoft\Windows\CurrentVersion\DeviceSetup",
            "CostedNetworkPolicy",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.minimise-connections",
            Network,
            "Discovery and Wi-Fi",
            "Minimise connections",
            "Clears the policy that drops other networks.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm_absent(
            r"SOFTWARE\Policies\Microsoft\Windows\WcmSvc\GroupPolicy",
            "fMinimizeConnections",
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NativeWifiP",
                "ImagePath",
            ),
            "network.ndiscap-note",
            Network,
            "Discovery and Wi-Fi",
            "Native Wi-Fi filter",
            "Sets the Wi-Fi filter driver to manual.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NativeWifiP",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vwififlt",
                "ImagePath",
            ),
            "network.vwifi",
            Network,
            "Discovery and Wi-Fi",
            "Virtual Wi-Fi filter",
            "Sets the hosted network filter to manual.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vwififlt",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NdisVirtualBus",
                "ImagePath",
            ),
            "network.ndisvirtual",
            Network,
            "Discovery and Wi-Fi",
            "NDIS Virtual Bus",
            "Turns off the virtual network bus.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NdisVirtualBus",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NetBIOS",
                "ImagePath",
            ),
            "network.netbios-driver",
            Network,
            "Discovery and Wi-Fi",
            "NetBIOS driver",
            "Turns off the NetBIOS driver.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NetBIOS",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.throttling-index",
            Network,
            "TCP",
            "Network throttling",
            "Stops Windows limiting network traffic while media plays.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            "NetworkThrottlingIndex",
            4294967295,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.smb-client-threads",
            Network,
            "SMB",
            "SMB client threads",
            "Gives the SMB client more worker threads.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters",
            "MaxThreads",
            30,
        )],
    },
    SettingsTweak {
        meta: lib(
            "network.qos-reserve",
            Network,
            "QoS",
            "QoS bandwidth reserve",
            "Stops Windows setting bandwidth aside for QoS traffic.",
            POLICY_DOCS,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Psched",
                "NonBestEffortLimit",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\Psched",
                "NonBestEffortLimit",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "network.do-machine",
            Network,
            "Discovery and Wi-Fi",
            "Delivery Optimization mode",
            "Sets Delivery Optimization to plain downloads.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config",
                "DODownloadMode",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization",
                "SystemSettingsDownloadMode",
                0,
            ),
        ],
    },
];
