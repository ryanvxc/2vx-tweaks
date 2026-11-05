//! Services tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\diagnosticshub.standardcollector.service",
                "ImagePath",
            ),
            "services.diagnostics-hub",
            Services,
            "Telemetry and diagnostics",
            "Diagnostics Hub Collector",
            "Collects data for Visual Studio's diagnostic tools.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\diagnosticshub.standardcollector.service",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\diagsvc",
                "ImagePath",
            ),
            "services.diagsvc",
            Services,
            "Telemetry and diagnostics",
            "Diagnostic Execution",
            "Runs troubleshooters on request.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Windows troubleshooters stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\diagsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TroubleshootingSvc",
                "ImagePath",
            ),
            "services.troubleshooting",
            Services,
            "Telemetry and diagnostics",
            "Recommended Troubleshooting",
            "Offers fixes Windows thinks you need.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TroubleshootingSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WerSvc",
                "ImagePath",
            ),
            "services.wersvc",
            Services,
            "Telemetry and diagnostics",
            "Windows Error Reporting",
            "Sends crash reports to Microsoft.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WerSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wercplsupport",
                "ImagePath",
            ),
            "services.wercpl",
            Services,
            "Telemetry and diagnostics",
            "Problem Reports Control Panel",
            "Shows and clears saved problem reports.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wercplsupport",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\pla",
                "ImagePath",
            ),
            "services.pla",
            Services,
            "Telemetry and diagnostics",
            "Performance Logs and Alerts",
            "Collects performance counters on a schedule.",
            BATLEZ,
            Observed,
        )
        .warn("Performance Monitor data collectors stop working.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\pla", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PerfHost",
                "ImagePath",
            ),
            "services.perfhost",
            Services,
            "Telemetry and diagnostics",
            "Performance Counter Host",
            "Lets 32-bit tools read 64-bit performance counters.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PerfHost",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wmiApSrv",
                "ImagePath",
            ),
            "services.wmiapsrv",
            Services,
            "Telemetry and diagnostics",
            "WMI Performance Adapter",
            "Serves performance data to WMI clients.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wmiApSrv",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Wecsvc",
                "ImagePath",
            ),
            "services.wecsvc",
            Services,
            "Telemetry and diagnostics",
            "Windows Event Collector",
            "Collects events from other PCs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Wecsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\GraphicsPerfSvc",
                "ImagePath",
            ),
            "services.graphicsperf",
            Services,
            "Telemetry and diagnostics",
            "Graphics Performance Monitor",
            "Records GPU performance for Task Manager history.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\GraphicsPerfSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\GpuEnergyDrv",
                "ImagePath",
            ),
            "services.gpu-energy",
            Services,
            "Telemetry and diagnostics",
            "GPU Energy Driver",
            "Estimates GPU power use for Windows.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\GpuEnergyDrv",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DusmSvc",
                "ImagePath",
            ),
            "services.dusmsvc",
            Services,
            "Telemetry and diagnostics",
            "Data Usage",
            "Counts network use per app.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Data limits and the Data usage page stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DusmSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\sysmon",
                "ImagePath",
            ),
            "services.sysmon",
            Services,
            "Telemetry and diagnostics",
            "System Monitor Driver",
            "Logs system activity if Sysmon is installed.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Security logging from Sysmon stops.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\sysmon",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NdisCap",
                "ImagePath",
            ),
            "services.ndiscap",
            Services,
            "Telemetry and diagnostics",
            "NDIS Capture",
            "Filter driver used by network packet capture.",
            BATLEZ,
            Observed,
        )
        .warn("Network captures with pktmon or netsh trace stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NdisCap",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\XboxNetApiSvc",
                "ImagePath",
            ),
            "services.xbox-net",
            Services,
            "Xbox",
            "Xbox Live Networking",
            "Handles Xbox multiplayer connections.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Xbox Live multiplayer stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\XboxNetApiSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\XboxGipSvc",
                "ImagePath",
            ),
            "services.xbox-gip",
            Services,
            "Xbox",
            "Xbox Accessory Management",
            "Runs Xbox controllers and accessories.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Xbox accessories may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\XboxGipSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\GameInputSvc",
                "ImagePath",
            ),
            "services.game-input",
            Services,
            "Xbox",
            "GameInput",
            "Input service used by some games.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some controllers and Microsoft games may lose input.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\GameInputSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\BcastDVRUserService",
                "ImagePath",
            ),
            "services.bcast-dvr",
            Services,
            "Xbox",
            "Game Recording User Service",
            "Records and broadcasts gameplay.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\BcastDVRUserService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\CaptureService",
                "ImagePath",
            ),
            "services.capture",
            Services,
            "Xbox",
            "Screen Capture Service",
            "Lets apps capture the screen.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some recording and screen sharing apps stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CaptureService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\BluetoothUserService",
                "ImagePath",
            ),
            "services.bluetooth-user",
            Services,
            "Bluetooth",
            "Bluetooth User Support",
            "Per-user Bluetooth features.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Bluetooth features may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\BluetoothUserService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\BTAGService",
                "ImagePath",
            ),
            "services.btag",
            Services,
            "Bluetooth",
            "Bluetooth Audio Gateway",
            "Phone-call audio over Bluetooth.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Bluetooth headset calls stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\BTAGService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\BthAvctpSvc",
                "ImagePath",
            ),
            "services.avctp",
            Services,
            "Bluetooth",
            "AVCTP",
            "Remote controls for Bluetooth audio.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Bluetooth audio controls stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\BthAvctpSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DeviceAssociationService",
                "ImagePath",
            ),
            "services.device-assoc",
            Services,
            "Bluetooth",
            "Device Association",
            "Pairs wireless devices.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Pairing new wireless devices stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DeviceAssociationService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DeviceAssociationBrokerSvc",
                "ImagePath",
            ),
            "services.device-assoc-broker",
            Services,
            "Bluetooth",
            "Device Association Broker",
            "Lets apps pair devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DeviceAssociationBrokerSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DevicePickerUserSvc",
                "ImagePath",
            ),
            "services.device-picker",
            Services,
            "Bluetooth",
            "Device Picker",
            "Shows the device picker in apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DevicePickerUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DevicesFlowUserSvc",
                "ImagePath",
            ),
            "services.devices-flow",
            Services,
            "Bluetooth",
            "Devices Flow",
            "Connects wireless displays and audio.",
            BATLEZ,
            Observed,
        )
        .warn("Wireless display and Cast may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DevicesFlowUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DevQueryBroker",
                "ImagePath",
            ),
            "services.devquery",
            Services,
            "Bluetooth",
            "DevQuery Background Discovery",
            "Lets apps find network devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DevQueryBroker",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\HvHost",
                "ImagePath",
            ),
            "services.hvhost",
            Services,
            "Hyper-V",
            "Hyper-V Host",
            "Performance helpers for virtual machines.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Hyper-V, WSL 2 and Windows Sandbox can lose performance.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\HvHost",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicguestinterface",
                "ImagePath",
            ),
            "services.vmic-guest",
            Services,
            "Hyper-V",
            "Hyper-V Guest Interface",
            "Guest channel for Hyper-V VMs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicguestinterface",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicheartbeat",
                "ImagePath",
            ),
            "services.vmic-heartbeat",
            Services,
            "Hyper-V",
            "Hyper-V Heartbeat",
            "Reports VM health to the host.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicheartbeat",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmickvpexchange",
                "ImagePath",
            ),
            "services.vmic-kvp",
            Services,
            "Hyper-V",
            "Hyper-V Data Exchange",
            "Shares VM data with the host.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmickvpexchange",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicrdv",
                "ImagePath",
            ),
            "services.vmic-rdv",
            Services,
            "Hyper-V",
            "Hyper-V Remote Desktop",
            "Remote desktop into a Hyper-V VM.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicrdv",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicshutdown",
                "ImagePath",
            ),
            "services.vmic-shutdown",
            Services,
            "Hyper-V",
            "Hyper-V Guest Shutdown",
            "Lets the host shut down a VM.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicshutdown",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmictimesync",
                "ImagePath",
            ),
            "services.vmic-time",
            Services,
            "Hyper-V",
            "Hyper-V Time Sync",
            "Syncs VM time with the host.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmictimesync",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicvmsession",
                "ImagePath",
            ),
            "services.vmic-session",
            Services,
            "Hyper-V",
            "Hyper-V PowerShell Direct",
            "PowerShell into a VM.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicvmsession",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vmicvss",
                "ImagePath",
            ),
            "services.vmic-vss",
            Services,
            "Hyper-V",
            "Hyper-V Shadow Copy",
            "Backs up a running VM.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\vmicvss",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PrintWorkflowUserSvc",
                "ImagePath",
            ),
            "services.print-workflow",
            Services,
            "Printing and scanning",
            "Print Workflow",
            "Runs print apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PrintWorkflowUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\StiSvc",
                "ImagePath",
            ),
            "services.stisvc",
            Services,
            "Printing and scanning",
            "Windows Image Acquisition",
            "Talks to scanners and cameras.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Scanners stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\StiSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WiaRpc",
                "ImagePath",
            ),
            "services.wiarpc",
            Services,
            "Printing and scanning",
            "Still Image Events",
            "Launches apps when a scanner button is pressed.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WiaRpc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Fax",
                "ImagePath",
            ),
            "services.fax",
            Services,
            "Printing and scanning",
            "Fax",
            "Sends and receives faxes.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\Fax", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PhoneSvc",
                "ImagePath",
            ),
            "services.phonesvc",
            Services,
            "Phone and cellular",
            "Phone Service",
            "Manages telephony state.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PhoneSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TapiSrv",
                "ImagePath",
            ),
            "services.tapisrv",
            Services,
            "Phone and cellular",
            "Telephony",
            "Telephony API for apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TapiSrv",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SmsRouter",
                "ImagePath",
            ),
            "services.smsrouter",
            Services,
            "Phone and cellular",
            "SMS Router",
            "Routes text messages to apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SmsRouter",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MessagingService",
                "ImagePath",
            ),
            "services.messaging",
            Services,
            "Phone and cellular",
            "Messaging",
            "Supports text messaging.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MessagingService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WwanSvc",
                "ImagePath",
            ),
            "services.wwansvc",
            Services,
            "Phone and cellular",
            "WWAN AutoConfig",
            "Manages mobile broadband.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .warn("Mobile broadband stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WwanSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wlpasvc",
                "ImagePath",
            ),
            "services.wlpasvc",
            Services,
            "Phone and cellular",
            "Local Profile Assistant",
            "Manages eSIM profiles.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wlpasvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\autotimesvc",
                "ImagePath",
            ),
            "services.autotime",
            Services,
            "Phone and cellular",
            "Cellular Time",
            "Sets the time from mobile networks.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\autotimesvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SEMgrSvc",
                "ImagePath",
            ),
            "services.semgr",
            Services,
            "Phone and cellular",
            "Payments and NFC",
            "Manages secure elements for payments.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SEMgrSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WalletService",
                "ImagePath",
            ),
            "services.wallet",
            Services,
            "Phone and cellular",
            "Wallet",
            "Stores payment details for the Wallet app.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WalletService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\fdPHost",
                "ImagePath",
            ),
            "services.fdphost",
            Services,
            "Network discovery",
            "Function Discovery Host",
            "Finds devices on the network.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("PCs and printers may stop appearing in Network.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\fdPHost",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FDResPub",
                "ImagePath",
            ),
            "services.fdrespub",
            Services,
            "Network discovery",
            "Function Discovery Publisher",
            "Announces this PC on the network.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("This PC stops appearing in Network.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FDResPub",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SSDPSRV",
                "ImagePath",
            ),
            "services.ssdp",
            Services,
            "Network discovery",
            "SSDP Discovery",
            "Finds UPnP devices.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("UPnP devices and some media casting stop being found.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SSDPSRV",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\upnphost",
                "ImagePath",
            ),
            "services.upnphost",
            Services,
            "Network discovery",
            "UPnP Device Host",
            "Hosts UPnP devices on this PC.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\upnphost",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\lltdsvc",
                "ImagePath",
            ),
            "services.lltdsvc",
            Services,
            "Network discovery",
            "Link-Layer Topology Discovery",
            "Draws the network map.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\lltdsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\lmhosts",
                "ImagePath",
            ),
            "services.lmhosts",
            Services,
            "Network discovery",
            "TCP/IP NetBIOS Helper",
            "NetBIOS name resolution.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Older network shares may stop being found.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\lmhosts",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NcdAutoSetup",
                "ImagePath",
            ),
            "services.ncdautosetup",
            Services,
            "Network discovery",
            "Network Devices Auto-Setup",
            "Installs network devices it finds.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NcdAutoSetup",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wcncsvc",
                "ImagePath",
            ),
            "services.wcncsvc",
            Services,
            "Network discovery",
            "Windows Connect Now",
            "Configures Wi-Fi with WPS.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wcncsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WFDSConMgrSvc",
                "ImagePath",
            ),
            "services.wfds",
            Services,
            "Network discovery",
            "Wi-Fi Direct Connection Manager",
            "Connects to Wi-Fi Direct devices.",
            BATLEZ,
            Observed,
        )
        .warn("Wi-Fi Direct and some wireless displays stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WFDSConMgrSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\QWAVE",
                "ImagePath",
            ),
            "services.qwave",
            Services,
            "Network discovery",
            "Quality Windows Audio Video",
            "Prioritises media streams on home networks.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\QWAVE", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\icssvc",
                "ImagePath",
            ),
            "services.icssvc",
            Services,
            "Network discovery",
            "Mobile Hotspot",
            "Shares this PC's internet.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Mobile hotspot stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\icssvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SNMPTRAP",
                "ImagePath",
            ),
            "services.sharedaccess-note",
            Services,
            "Network discovery",
            "SNMP Trap",
            "Receives SNMP messages.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SNMPTRAP",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\dot3svc",
                "ImagePath",
            ),
            "services.dot3svc",
            Services,
            "Network discovery",
            "Wired AutoConfig",
            "802.1X login on wired networks.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Networks that require 802.1X login need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\dot3svc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\EapHost",
                "ImagePath",
            ),
            "services.eaphost",
            Services,
            "Network discovery",
            "Extensible Authentication",
            "Network login with EAP.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Enterprise Wi-Fi and some VPNs need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\EapHost",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NcbService",
                "ImagePath",
            ),
            "services.ncb",
            Services,
            "Network discovery",
            "Network Connection Broker",
            "Lets Store apps wait for network events.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NcbService",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NcaSvc",
                "ImagePath",
            ),
            "services.ncasvc",
            Services,
            "Network discovery",
            "Network Connectivity Assistant",
            "Reports DirectAccess status.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NcaSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TermService",
                "ImagePath",
            ),
            "services.termservice",
            Services,
            "Remote access",
            "Remote Desktop Services",
            "Accepts remote desktop connections.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Connecting to this PC with Remote Desktop stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TermService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SessionEnv",
                "ImagePath",
            ),
            "services.session-env",
            Services,
            "Remote access",
            "Remote Desktop Configuration",
            "Sets up remote desktop sessions.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SessionEnv",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\UmRdpService",
                "ImagePath",
            ),
            "services.umrdp",
            Services,
            "Remote access",
            "Remote Desktop Port Redirector",
            "Redirects devices over remote desktop.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\UmRdpService",
            "Start",
            4,
        )],
    },
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
            "Remote Registry",
            "Lets others edit this PC's registry.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
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
                r"SYSTEM\CurrentControlSet\Services\WinRM",
                "ImagePath",
            ),
            "services.winrm",
            Services,
            "Remote access",
            "Windows Remote Management",
            "Remote PowerShell and management.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\WinRM", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RemoteAccess",
                "ImagePath",
            ),
            "services.remote-access",
            Services,
            "Remote access",
            "Routing and Remote Access",
            "Runs this PC as a router or VPN server.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RemoteAccess",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RpcLocator",
                "ImagePath",
            ),
            "services.rpclocator",
            Services,
            "Remote access",
            "RPC Locator",
            "Legacy RPC name service.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RpcLocator",
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
            "WebClient",
            "Maps WebDAV drives.",
            BATLEZ,
            Observed,
        )
        .admin(),
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
            "File and Printer Sharing",
            "Shares this PC's files and printers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Shared folders and printers on this PC stop working.")
        .admin(),
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
                r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation",
                "ImagePath",
            ),
            "services.lanman-workstation",
            Services,
            "Remote access",
            "SMB Client",
            "Connects to shared folders.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Network drives and shared folders stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LanmanWorkstation",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Netlogon",
                "ImagePath",
            ),
            "services.netlogon",
            Services,
            "Remote access",
            "Netlogon",
            "Signs in to a domain.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Signing in to a work domain stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Netlogon",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\seclogon",
                "ImagePath",
            ),
            "services.seclogon",
            Services,
            "Remote access",
            "Secondary Logon",
            "Powers Run as different user.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Run as different user stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\seclogon",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\ssh-agent",
                "ImagePath",
            ),
            "services.ssh-agent",
            Services,
            "Remote access",
            "OpenSSH Agent",
            "Holds SSH keys.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\ssh-agent",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RasMan",
                "ImagePath",
            ),
            "services.rasman",
            Services,
            "VPN",
            "Remote Access Connection Manager",
            "Manages VPN and dial-up.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("VPNs built on Windows need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RasMan",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RasAuto",
                "ImagePath",
            ),
            "services.rasauto",
            Services,
            "VPN",
            "Remote Access Auto Connection",
            "Dials when an app needs a remote address.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RasAuto",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SstpSvc",
                "ImagePath",
            ),
            "services.sstp",
            Services,
            "VPN",
            "Secure Socket Tunneling",
            "SSTP VPN connections.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("SSTP VPNs stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SstpSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\IKEEXT",
                "ImagePath",
            ),
            "services.ikeext",
            Services,
            "VPN",
            "IKE and AuthIP Keying",
            "IPsec keys for VPNs.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("IKEv2 and L2TP VPNs stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\IKEEXT",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PolicyAgent",
                "ImagePath",
            ),
            "services.policy-agent",
            Services,
            "VPN",
            "IPsec Policy Agent",
            "Enforces IPsec rules.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("IPsec policies stop being enforced.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PolicyAgent",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\iphlpsvc",
                "ImagePath",
            ),
            "services.iphlpsvc",
            Services,
            "VPN",
            "IP Helper",
            "IPv6 transition tunnels.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("IPv6 tunnels and some VPNs stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\iphlpsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc",
                "ImagePath",
            ),
            "services.ipxlat",
            Services,
            "VPN",
            "IP Translation Config",
            "Configures IPv4 to IPv6 translation.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SensorDataService",
                "ImagePath",
            ),
            "services.sensor-data",
            Services,
            "Location and sensors",
            "Sensor Data",
            "Delivers data from sensors.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SensorDataService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\tzautoupdate",
                "ImagePath",
            ),
            "services.tzautoupdate",
            Services,
            "Location and sensors",
            "Auto Time Zone Updater",
            "Sets the time zone from your location.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\tzautoupdate",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\W32Time",
                "ImagePath",
            ),
            "services.w32time",
            Services,
            "Location and sensors",
            "Windows Time",
            "Keeps the clock in sync.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("The clock can drift if nothing else syncs it.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\W32Time",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NaturalAuthentication",
                "ImagePath",
            ),
            "services.natural-auth",
            Services,
            "Location and sensors",
            "Natural Authentication",
            "Dynamic lock and other presence signals.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NaturalAuthentication",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\InstallService",
                "ImagePath",
            ),
            "services.install-service",
            Services,
            "Store and apps",
            "Store Install Service",
            "Installs Store apps.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Microsoft Store installs need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\InstallService",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PushToInstall",
                "ImagePath",
            ),
            "services.push-to-install",
            Services,
            "Store and apps",
            "PushToInstall",
            "Installs apps pushed from another device.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PushToInstall",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\LicenseManager",
                "ImagePath",
            ),
            "services.license-manager",
            Services,
            "Store and apps",
            "Windows License Manager",
            "Licences for Store apps.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Store apps may stop opening.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LicenseManager",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\ClipSVC",
                "ImagePath",
            ),
            "services.clipsvc",
            Services,
            "Store and apps",
            "Client License Service",
            "Store licensing.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Store apps may stop opening.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\ClipSVC",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AppReadiness",
                "ImagePath",
            ),
            "services.app-readiness",
            Services,
            "Store and apps",
            "App Readiness",
            "Prepares apps at first sign-in.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AppReadiness",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DsmSvc",
                "ImagePath",
            ),
            "services.dsmsvc",
            Services,
            "Store and apps",
            "Device Setup Manager",
            "Downloads device software.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("New hardware may need software installed by hand.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DsmSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AppMgmt",
                "ImagePath",
            ),
            "services.app-mgmt",
            Services,
            "Store and apps",
            "Application Management",
            "Installs software deployed by policy.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AppMgmt",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AxInstSV",
                "ImagePath",
            ),
            "services.axinst",
            Services,
            "Store and apps",
            "ActiveX Installer",
            "Installs ActiveX controls.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AxInstSV",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AppVClient",
                "ImagePath",
            ),
            "services.appv",
            Services,
            "Store and apps",
            "App-V Client",
            "Runs streamed apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AppVClient",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\EntAppSvc",
                "ImagePath",
            ),
            "services.ent-app",
            Services,
            "Store and apps",
            "Enterprise App Management",
            "Installs work apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\EntAppSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RetailDemo",
                "ImagePath",
            ),
            "services.retail-demo",
            Services,
            "Store and apps",
            "Retail Demo",
            "Runs store demo mode.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RetailDemo",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wisvc",
                "ImagePath",
            ),
            "services.wisvc",
            Services,
            "Store and apps",
            "Windows Insider",
            "Runs the Insider Program.",
            BATLEZ,
            Observed,
        )
        .warn("Windows Insider builds stop working.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\wisvc", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\LxpSvc",
                "ImagePath",
            ),
            "services.lxp",
            Services,
            "Store and apps",
            "Language Experience",
            "Installs language packs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LxpSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\embeddedmode",
                "ImagePath",
            ),
            "services.embedded-mode",
            Services,
            "Store and apps",
            "Embedded Mode",
            "Runs background apps on IoT devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\embeddedmode",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AssignedAccessManagerSvc",
                "ImagePath",
            ),
            "services.assigned-access",
            Services,
            "Store and apps",
            "Assigned Access",
            "Kiosk mode.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AssignedAccessManagerSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DialogBlockingService",
                "ImagePath",
            ),
            "services.dialog-blocking",
            Services,
            "Store and apps",
            "Dialog Blocking",
            "Blocks dialogs on kiosks.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DialogBlockingService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WarpJITSvc",
                "ImagePath",
            ),
            "services.warp-jit",
            Services,
            "Store and apps",
            "WarpJIT",
            "JIT for software rendering.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WarpJITSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\StorSvc",
                "ImagePath",
            ),
            "services.storsvc",
            Services,
            "Store and apps",
            "Storage Service",
            "Manages storage settings and app installs.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\StorSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MixedRealityOpenXRSvc",
                "ImagePath",
            ),
            "services.openxr",
            Services,
            "Mixed reality",
            "Mixed Reality OpenXR",
            "Runtime for Windows Mixed Reality.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MixedRealityOpenXRSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\perceptionsimulation",
                "ImagePath",
            ),
            "services.perception-sim",
            Services,
            "Mixed reality",
            "Perception Simulation",
            "Simulates sensors for mixed reality.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\perceptionsimulation",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\spectrum",
                "ImagePath",
            ),
            "services.spectrum",
            Services,
            "Mixed reality",
            "Windows Perception",
            "Spatial input and holographic rendering.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\spectrum",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SharedRealitySvc",
                "ImagePath",
            ),
            "services.shared-reality",
            Services,
            "Mixed reality",
            "Spatial Data",
            "Shares spatial data for mixed reality.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SharedRealitySvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\VacSvc",
                "ImagePath",
            ),
            "services.vacsvc",
            Services,
            "Mixed reality",
            "Volumetric Audio",
            "Spatial audio for mixed reality.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\VacSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SCardSvr",
                "ImagePath",
            ),
            "services.scardsvr",
            Services,
            "Smart card and sign-in",
            "Smart Card",
            "Reads smart cards.",
            BATLEZ,
            Observed,
        )
        .warn("Smart card sign-in stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SCardSvr",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\ScDeviceEnum",
                "ImagePath",
            ),
            "services.scdevice",
            Services,
            "Smart card and sign-in",
            "Smart Card Enumeration",
            "Finds smart card readers.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\ScDeviceEnum",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SCPolicySvc",
                "ImagePath",
            ),
            "services.scpolicy",
            Services,
            "Smart card and sign-in",
            "Smart Card Removal Policy",
            "Locks the PC when a card is removed.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SCPolicySvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\CertPropSvc",
                "ImagePath",
            ),
            "services.certprop",
            Services,
            "Smart card and sign-in",
            "Certificate Propagation",
            "Copies certificates from smart cards.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CertPropSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WbioSrvc",
                "ImagePath",
            ),
            "services.wbio",
            Services,
            "Smart card and sign-in",
            "Windows Biometric",
            "Fingerprint and face sign-in.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Windows Hello fingerprint and face sign-in stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WbioSrvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NgcCtnrSvc",
                "ImagePath",
            ),
            "services.ngc-ctnr",
            Services,
            "Smart card and sign-in",
            "Passport Container",
            "Holds Windows Hello keys.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Windows Hello PIN sign-in needs this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NgcCtnrSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NgcSvc",
                "ImagePath",
            ),
            "services.ngc",
            Services,
            "Smart card and sign-in",
            "Microsoft Passport",
            "Windows Hello credentials.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Windows Hello PIN sign-in needs this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NgcSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wlidsvc",
                "ImagePath",
            ),
            "services.wlidsvc",
            Services,
            "Smart card and sign-in",
            "Microsoft Account Sign-in",
            "Microsoft account sign-in.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Microsoft account sign-in and the Store need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wlidsvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\CredentialEnrollmentManagerUserSvc",
                "ImagePath",
            ),
            "services.cred-enroll",
            Services,
            "Smart card and sign-in",
            "Credential Enrollment",
            "Stores credentials for sign-in prompts.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CredentialEnrollmentManagerUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\ConsentUxUserSvc",
                "ImagePath",
            ),
            "services.consent-ux",
            Services,
            "Smart card and sign-in",
            "ConsentUX",
            "Prompts for connecting devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\ConsentUxUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\cloudidsvc",
                "ImagePath",
            ),
            "services.cloud-id",
            Services,
            "Smart card and sign-in",
            "Cloud Identity",
            "Cloud identity for enterprise.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\cloudidsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DmEnrollmentSvc",
                "ImagePath",
            ),
            "services.dmenroll",
            Services,
            "Smart card and sign-in",
            "Device Management Enrollment",
            "Joins device management.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DmEnrollmentSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\LPlatSvc",
                "ImagePath",
            ),
            "services.lpasvc-note",
            Services,
            "Smart card and sign-in",
            "Policy Platform Agent",
            "Enterprise policy service.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\LPlatSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WEPHOSTSVC",
                "ImagePath",
            ),
            "services.wepsvc",
            Services,
            "Smart card and sign-in",
            "Encryption Provider Host",
            "Hosts third-party encryption.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WEPHOSTSVC",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SDRSVC",
                "ImagePath",
            ),
            "services.sdrsvc",
            Services,
            "Backup and storage",
            "Windows Backup",
            "Runs Windows Backup.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Windows Backup stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\SDRSVC",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\wbengine",
                "ImagePath",
            ),
            "services.wbengine",
            Services,
            "Backup and storage",
            "Block Level Backup",
            "Block-level backup engine.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\wbengine",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\fhsvc",
                "ImagePath",
            ),
            "services.fhsvc",
            Services,
            "Backup and storage",
            "File History",
            "Backs up your files.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("File History stops working.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\fhsvc", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\VSS",
                "ImagePath",
            ),
            "services.vss",
            Services,
            "Backup and storage",
            "Volume Shadow Copy",
            "Makes backups of files in use.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Backups and System Restore need this.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\VSS", "Start", 3)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\swprv",
                "ImagePath",
            ),
            "services.swprv",
            Services,
            "Backup and storage",
            "Shadow Copy Provider",
            "Manages shadow copies.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Backups and System Restore need this.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\swprv", "Start", 3)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\vds",
                "ImagePath",
            ),
            "services.vds",
            Services,
            "Backup and storage",
            "Virtual Disk",
            "Manages disks and volumes.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Disk Management needs this.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\vds", "Start", 3)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\defragsvc",
                "ImagePath",
            ),
            "services.defragsvc",
            Services,
            "Backup and storage",
            "Optimize Drives",
            "Trims and defragments drives.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Scheduled drive trimming needs this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\defragsvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\smphost",
                "ImagePath",
            ),
            "services.smphost",
            Services,
            "Backup and storage",
            "Storage Spaces SMP",
            "Manages Storage Spaces.",
            BATLEZ,
            Observed,
        )
        .warn("Storage Spaces management stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\smphost",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TieringEngineService",
                "ImagePath",
            ),
            "services.tiering",
            Services,
            "Backup and storage",
            "Storage Tiers",
            "Moves data between storage tiers.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TieringEngineService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\svsvc",
                "ImagePath",
            ),
            "services.svsvc",
            Services,
            "Backup and storage",
            "Spot Verifier",
            "Verifies suspected file system errors.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\svsvc", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MSiSCSI",
                "ImagePath",
            ),
            "services.msiscsi",
            Services,
            "Backup and storage",
            "iSCSI Initiator",
            "Connects to iSCSI storage.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MSiSCSI",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\CscService",
                "ImagePath",
            ),
            "services.csc",
            Services,
            "Backup and storage",
            "Offline Files",
            "Keeps network files available offline.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CscService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\workfolderssvc",
                "ImagePath",
            ),
            "services.workfolders",
            Services,
            "Backup and storage",
            "Work Folders",
            "Syncs work files.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\workfolderssvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FileSyncHelper",
                "ImagePath",
            ),
            "services.file-sync-helper",
            Services,
            "Backup and storage",
            "File Sync Helper",
            "Helps sync apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FileSyncHelper",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\OneDrive Updater Service",
                "ImagePath",
            ),
            "services.onedrive-updater",
            Services,
            "Backup and storage",
            "OneDrive Updater",
            "Keeps OneDrive updated.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\OneDrive Updater Service",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\storqosflt",
                "ImagePath",
            ),
            "services.storqos",
            Services,
            "Backup and storage",
            "Storage QoS Filter",
            "Storage quality of service for VMs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\storqosflt",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\bindflt",
                "ImagePath",
            ),
            "services.bindflt",
            Services,
            "Backup and storage",
            "Bind Filter",
            "File redirection for containers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Containers and some Store games stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\bindflt",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MSDTC",
                "ImagePath",
            ),
            "services.msdtc",
            Services,
            "Enterprise",
            "Distributed Transactions",
            "Coordinates database transactions.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\MSDTC", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\KtmRm",
                "ImagePath",
            ),
            "services.ktmrm",
            Services,
            "Enterprise",
            "Transaction Manager Bridge",
            "Links transactions to MSDTC.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\KtmRm", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\COMSysApp",
                "ImagePath",
            ),
            "services.comsysapp",
            Services,
            "Enterprise",
            "COM+ System Application",
            "Manages COM+ components.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\COMSysApp",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PeerDistSvc",
                "ImagePath",
            ),
            "services.branchcache",
            Services,
            "Enterprise",
            "BranchCache",
            "Caches content from other PCs on the network.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PeerDistSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\shpamsvc",
                "ImagePath",
            ),
            "services.shared-pc",
            Services,
            "Enterprise",
            "Shared PC Accounts",
            "Manages accounts on shared PCs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\shpamsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\UevAgentService",
                "ImagePath",
            ),
            "services.uev",
            Services,
            "Enterprise",
            "User Experience Virtualization",
            "Roams settings in enterprises.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\UevAgentService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WManSvc",
                "ImagePath",
            ),
            "services.wman",
            Services,
            "Enterprise",
            "Windows Management Service",
            "Device management tasks.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WManSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NetSetupSvc",
                "ImagePath",
            ),
            "services.netsetup",
            Services,
            "Enterprise",
            "Network Setup",
            "Installs network drivers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NetSetupSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MsKeyboardFilter",
                "ImagePath",
            ),
            "services.keyboard-filter",
            Services,
            "Enterprise",
            "Keyboard Filter",
            "Blocks key combinations on kiosks.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MsKeyboardFilter",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NetTcpPortSharing",
                "ImagePath",
            ),
            "services.net-tcp-sharing",
            Services,
            "Enterprise",
            "Net.Tcp Port Sharing",
            "Shares TCP ports for .NET apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NetTcpPortSharing",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\p2pimsvc",
                "ImagePath",
            ),
            "services.p2pimsvc",
            Services,
            "Peer networking",
            "Peer Identity Manager",
            "Identity for peer networking.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\p2pimsvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\p2psvc",
                "ImagePath",
            ),
            "services.p2psvc",
            Services,
            "Peer networking",
            "Peer Grouping",
            "Multi-party peer networking.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\p2psvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PNRPAutoReg",
                "ImagePath",
            ),
            "services.pnrpauto",
            Services,
            "Peer networking",
            "PNRP Name Publication",
            "Publishes this PC's peer name.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PNRPAutoReg",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PNRPsvc",
                "ImagePath",
            ),
            "services.pnrpsvc",
            Services,
            "Peer networking",
            "Peer Name Resolution",
            "Peer name resolution.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PNRPsvc",
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
            "Windows Search",
            "Builds the file search index.",
            KUBAAM,
            Observed,
        )
        .medium()
        .impact(High)
        .warn("Start and Explorer search get slower and less complete.")
        .admin(),
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
                r"SYSTEM\CurrentControlSet\Services\ShellHWDetection",
                "ImagePath",
            ),
            "services.shell-hw",
            Services,
            "Search and shell",
            "Shell Hardware Detection",
            "Runs AutoPlay.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\ShellHWDetection",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Themes",
                "ImagePath",
            ),
            "services.themes",
            Services,
            "Search and shell",
            "Themes",
            "Draws the modern window style.",
            BATLEZ,
            Observed,
        )
        .high()
        .impact(Medium)
        .warn("Windows falls back to a basic look.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Themes",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TextInputManagementService",
                "ImagePath",
            ),
            "services.text-input",
            Services,
            "Search and shell",
            "Text Input Management",
            "Keyboard and language input.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Typing in Start and modern apps can stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TextInputManagementService",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\TabletInputService",
                "ImagePath",
            ),
            "services.tablet-input",
            Services,
            "Search and shell",
            "Touch Keyboard and Handwriting",
            "Runs the touch keyboard.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("The touch keyboard and emoji panel stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\TabletInputService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\hidserv",
                "ImagePath",
            ),
            "services.hidserv",
            Services,
            "Search and shell",
            "Human Interface Devices",
            "Hotkeys on keyboards and remotes.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Media keys may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\hidserv",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FontCache",
                "ImagePath",
            ),
            "services.fontcache",
            Services,
            "Search and shell",
            "Font Cache",
            "Speeds up font loading.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FontCache",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FontCache3.0.0.0",
                "ImagePath",
            ),
            "services.fontcache3",
            Services,
            "Search and shell",
            "WPF Font Cache",
            "Speeds up font loading in WPF apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FontCache3.0.0.0",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DispBrokerDesktopSvc",
                "ImagePath",
            ),
            "services.disp-broker",
            Services,
            "Search and shell",
            "Display Policy",
            "Manages display layouts.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DispBrokerDesktopSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DisplayEnhancementService",
                "ImagePath",
            ),
            "services.display-enhance",
            Services,
            "Search and shell",
            "Display Enhancement",
            "Adaptive brightness.",
            BATLEZ,
            Observed,
        )
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DisplayEnhancementService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\SENS",
                "ImagePath",
            ),
            "services.sens",
            Services,
            "Search and shell",
            "System Event Notification",
            "Tells apps about sign-in and network events.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\SENS", "Start", 3)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\EventSystem",
                "ImagePath",
            ),
            "services.event-system",
            Services,
            "Search and shell",
            "COM+ Event System",
            "Delivers events to components.",
            BATLEZ,
            Observed,
        )
        .high()
        .warn("Sign-in and many apps rely on this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\EventSystem",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MCPManagementService",
                "ImagePath",
            ),
            "services.biometric-mcp",
            Services,
            "Search and shell",
            "MCP Management",
            "Agent connector management.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MCPManagementService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Beep",
                "ImagePath",
            ),
            "services.beep",
            Services,
            "Search and shell",
            "Beep Driver",
            "Motherboard speaker beeps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\Beep", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\cbdhsvc",
                "ImagePath",
            ),
            "services.cbdhsvc",
            Services,
            "Sync and account",
            "Clipboard User Service",
            "Clipboard history and sync.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Clipboard history (Win+V) stops working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\cbdhsvc",
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
            "Connected Devices Platform",
            "Phone Link, Nearby Sharing and shared experiences.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Nearby Sharing and Phone Link stop working.")
        .admin(),
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
                r"SYSTEM\CurrentControlSet\Services\CDPUserSvc",
                "ImagePath",
            ),
            "services.cdp-user",
            Services,
            "Sync and account",
            "Connected Devices User",
            "Per-user part of connected devices.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Nearby Sharing and Phone Link stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\CDPUserSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\DsSvc",
                "ImagePath",
            ),
            "services.dssvc",
            Services,
            "Sync and account",
            "Data Sharing",
            "Brokers data between apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\DsSvc", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\UnistoreSvc",
                "ImagePath",
            ),
            "services.unistore",
            Services,
            "Sync and account",
            "User Data Storage",
            "Stores contacts and calendar.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Mail and Calendar stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\UnistoreSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\UserDataSvc",
                "ImagePath",
            ),
            "services.userdata",
            Services,
            "Sync and account",
            "User Data Access",
            "Lets apps read contacts and calendar.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Mail and Calendar stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\UserDataSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\PimIndexMaintenanceSvc",
                "ImagePath",
            ),
            "services.pim-index",
            Services,
            "Sync and account",
            "Contact Indexing",
            "Indexes contacts for search.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\PimIndexMaintenanceSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AarSvc",
                "ImagePath",
            ),
            "services.aarsvc",
            Services,
            "Sync and account",
            "Agent Activation Runtime",
            "Activates voice assistants.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AarSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WpnService",
                "ImagePath",
            ),
            "services.wpn",
            Services,
            "Sync and account",
            "Push Notifications",
            "Delivers notifications.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Notifications need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WpnService",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WpnUserService",
                "ImagePath",
            ),
            "services.wpn-user",
            Services,
            "Sync and account",
            "Push Notifications User",
            "Per-user notifications.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Notifications need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WpnUserService",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WpcMonSvc",
                "ImagePath",
            ),
            "services.wpc-mon",
            Services,
            "Sync and account",
            "Parental Controls",
            "Applies family safety rules.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WpcMonSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FrameServer",
                "ImagePath",
            ),
            "services.frameserver",
            Services,
            "Sync and account",
            "Camera Frame Server",
            "Shares a camera between apps.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Cameras need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FrameServer",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\FrameServerMonitor",
                "ImagePath",
            ),
            "services.frameserver-monitor",
            Services,
            "Sync and account",
            "Camera Frame Server Monitor",
            "Watches the camera service.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\FrameServerMonitor",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WMPNetworkSvc",
                "ImagePath",
            ),
            "services.wmp-network",
            Services,
            "Sync and account",
            "Media Player Sharing",
            "Shares media libraries.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WMPNetworkSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WPDBusEnum",
                "ImagePath",
            ),
            "services.wpd-bus",
            Services,
            "Sync and account",
            "Portable Devices",
            "Connects phones and MP3 players.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Phones may stop appearing in Explorer.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WPDBusEnum",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\ALG",
                "ImagePath",
            ),
            "services.alg",
            Services,
            "Sync and account",
            "Application Layer Gateway",
            "Plug-ins for Internet Connection Sharing.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\ALG", "Start", 4)],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NPSMSvc",
                "ImagePath",
            ),
            "services.npsm",
            Services,
            "Sync and account",
            "Now Playing",
            "Media controls for Windows.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NPSMSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\UdkUserSvc",
                "ImagePath",
            ),
            "services.udk",
            Services,
            "Sync and account",
            "Shell Experience Host User",
            "Shell components.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\UdkUserSvc",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AMD Crash Defender Service",
                "ImagePath",
            ),
            "services.amd-crash",
            Services,
            "Third-party",
            "AMD Crash Defender",
            "Collects AMD driver crash data.",
            BATLEZ,
            Observed,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AMD Crash Defender Service",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AMD External Events Utility",
                "ImagePath",
            ),
            "services.amd-external",
            Services,
            "Third-party",
            "AMD External Events",
            "Handles AMD display events.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(AmdGpu)
        .warn("FreeSync and Radeon hotkeys can stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AMD External Events Utility",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AmdPPM",
                "ImagePath",
            ),
            "services.amd-ppm",
            Services,
            "Third-party",
            "AMD Processor Driver",
            "Power driver for Ryzen.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(AmdCpu)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AmdPPM",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NvProfileUpdater64",
                "ImagePath",
            ),
            "services.nv-profile-updater",
            Services,
            "Third-party",
            "NVIDIA Profile Updater",
            "Updates NVIDIA game profiles.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NvProfileUpdater64",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NvTelemetry",
                "ImagePath",
            ),
            "services.nv-telemetry",
            Services,
            "Third-party",
            "NVIDIA Telemetry",
            "Sends usage data to NVIDIA.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NvTelemetry",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\NvContainerLocalSystem",
                "ImagePath",
            ),
            "services.nv-container",
            Services,
            "Third-party",
            "NVIDIA Display Container",
            "Runs NVIDIA Control Panel.",
            BATLEZ,
            Observed,
        )
        .high()
        .hw(NvidiaGpu)
        .warn("NVIDIA Control Panel and laptop GPU switching need this.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NvContainerLocalSystem",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\edgeupdate",
                "ImagePath",
            ),
            "services.edge-update",
            Services,
            "Third-party",
            "Edge Update",
            "Updates Edge.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Edge stops getting security updates when this can't run.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\edgeupdate",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\edgeupdatem",
                "ImagePath",
            ),
            "services.edge-updatem",
            Services,
            "Third-party",
            "Edge Update Manual",
            "Updates Edge on demand.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Edge stops getting security updates when this can't run.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\edgeupdatem",
            "Start",
            3,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MicrosoftEdgeElevationService",
                "ImagePath",
            ),
            "services.edge-elevation",
            Services,
            "Third-party",
            "Edge Elevation",
            "Elevates Edge when needed.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MicrosoftEdgeElevationService",
            "Start",
            3,
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
            "Nahimic",
            "Audio effects from MSI and others.",
            BATLEZ,
            Observed,
        )
        .warn("Nahimic audio effects stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\NahimicService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\RtkAudioService",
                "ImagePath",
            ),
            "services.rtk-audio",
            Services,
            "Third-party",
            "Realtek Audio",
            "Realtek audio control.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Realtek audio settings may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\RtkAudioService",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Realtek Audio Universal Service",
                "ImagePath",
            ),
            "services.rtk-universal",
            Services,
            "Third-party",
            "Realtek Audio Universal",
            "Realtek audio control.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Realtek audio settings may stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\Realtek Audio Universal Service",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WavesSysSvc",
                "ImagePath",
            ),
            "services.waves",
            Services,
            "Third-party",
            "Waves Audio",
            "Audio effects for laptops.",
            BATLEZ,
            Observed,
        )
        .warn("Waves audio effects stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\WavesSysSvc",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AJRouter",
                "ImagePath",
            ),
            "services.ajrouter",
            Services,
            "Network discovery",
            "AllJoyn Router",
            "Connects nearby smart devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AJRouter",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\MicrosoftSearchInBing",
                "ImagePath",
            ),
            "services.search-in-bing",
            Services,
            "Search and shell",
            "Microsoft Search in Bing",
            "Sends address bar searches through Bing.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\MicrosoftSearchInBing",
            "Start",
            4,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AMDRyzenMasterDriverV19",
                "ImagePath",
            ),
            "services.amd-ryzen-master",
            Services,
            "Third-party",
            "Ryzen Master driver",
            "Sets the Ryzen Master driver to start on demand.",
            BATLEZ,
            Observed,
        )
        .hw(AmdCpu)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\AMDRyzenMasterDriverV19",
            "Start",
            3,
        )],
    },
];
