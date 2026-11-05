//! Privacy tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "privacy.commercial-pipeline",
            Privacy,
            "Telemetry",
            "Commercial data pipeline",
            "Stops the commercial diagnostics pipeline.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowCommercialDataPipeline",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.desktop-analytics",
            Privacy,
            "Telemetry",
            "Desktop Analytics",
            "Stops Desktop Analytics processing.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowDesktopAnalyticsProcessing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.device-name",
            Privacy,
            "Telemetry",
            "Device name in telemetry",
            "Keeps your PC's name out of telemetry.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowDeviceNameInTelemetry",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.update-compliance",
            Privacy,
            "Telemetry",
            "Update Compliance data",
            "Stops Update Compliance processing.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowUpdateComplianceProcessing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.wufb-cloud",
            Privacy,
            "Telemetry",
            "WUfB cloud processing",
            "Stops Windows Update for Business reports.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "AllowWUfBCloudProcessing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.onesettings",
            Privacy,
            "Telemetry",
            "OneSettings downloads",
            "Stops remote configuration downloads.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DisableOneSettingsDownloads",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.feedback-notifications",
            Privacy,
            "Telemetry",
            "Feedback notifications",
            "Stops prompts asking for feedback.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DoNotShowFeedbackNotifications",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.limit-enhanced",
            Privacy,
            "Telemetry",
            "Enhanced diagnostic limit",
            "Limits data sent to Windows Analytics.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "LimitEnhancedDiagnosticDataWindowsAnalytics",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.edge-data-optin",
            Privacy,
            "Telemetry",
            "Edge data in telemetry",
            "Keeps Edge browsing out of Windows telemetry.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "MicrosoftEdgeDataOptIn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ceip",
            Privacy,
            "Telemetry",
            "Customer Experience Program",
            "Leaves the improvement program.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\SQMClient\Windows",
                "CEIPEnable",
                0,
            ),
            hkcu(r"Software\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
            hklm(r"SOFTWARE\Microsoft\SQMClient", "UploadDisableFlag", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ait",
            Privacy,
            "Telemetry",
            "Application telemetry",
            "Stops app usage tracking.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "AITEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.inventory",
            Privacy,
            "Telemetry",
            "Application inventory",
            "Stops the inventory collector.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableInventory",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.steps-recorder",
            Privacy,
            "Telemetry",
            "Steps Recorder",
            "Turns off Steps Recorder.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableUAR",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.pca",
            Privacy,
            "Telemetry",
            "Program Compatibility Assistant",
            "Turns off compatibility prompts.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisablePCA",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.compat-engine",
            Privacy,
            "Telemetry",
            "Application compatibility engine",
            "Stops compatibility fixes loading.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Old apps that rely on shims may stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableEngine",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.compat-tab",
            Privacy,
            "Telemetry",
            "Compatibility tab",
            "Hides the Compatibility tab.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisablePropPage",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.mrt-report",
            Privacy,
            "Telemetry",
            "Malicious tool reports",
            "Stops the removal tool sending reports.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\MRT",
            "DontReportInfectionInformation",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.error-report-policy",
            Privacy,
            "Telemetry",
            "Error reporting",
            "Turns off Windows Error Reporting.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                "Disabled",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "Disabled",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting",
                "Disabled",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "DoReport",
                0,
            ),
            hklm(r"SOFTWARE\Microsoft\PCHealth\ErrorReporting", "DoReport", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.error-extra",
            Privacy,
            "Telemetry",
            "Additional crash data",
            "Stops extra data going with crash reports.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "DontSendAdditionalData",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting",
                "DontSendAdditionalData",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.error-ui",
            Privacy,
            "Telemetry",
            "Crash report prompts",
            "Hides crash report dialogs.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting",
                "DontShowUI",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "DontShowUI",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\PCHealth\ErrorReporting", "ShowUI", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.error-logging",
            Privacy,
            "Telemetry",
            "Error report logging",
            "Stops logging reports.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "LoggingDisabled",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting",
                "LoggingDisabled",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.error-consent",
            Privacy,
            "Telemetry",
            "Error report consent",
            "Sets consent to ask first.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting\Consent",
                "DefaultConsent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting\Consent",
                "DefaultConsent",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting\Consent",
                "DefaultOverrideBehavior",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Windows Error Reporting\Consent",
                "DefaultOverrideBehavior",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.handwriting-reports",
            Privacy,
            "Telemetry",
            "Handwriting error reports",
            "Stops handwriting reports.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports",
            "PreventHandwritingErrorReports",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.handwriting-data",
            Privacy,
            "Telemetry",
            "Handwriting data sharing",
            "Stops sharing handwriting samples.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
            "PreventHandwritingDataSharing",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ie-sqm",
            Privacy,
            "Telemetry",
            "Internet Explorer CEIP",
            "Leaves the IE improvement program.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Internet Explorer\SQM",
                "DisableCustomerImprovementProgram",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\Internet Explorer\SQM", "OptedIn", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ie-logging",
            Privacy,
            "Telemetry",
            "InPrivate logging",
            "Stops IE InPrivate logging.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Internet Explorer\Safety\PrivacIE",
            "DisableLogging",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.messenger-ceip",
            Privacy,
            "Telemetry",
            "Messenger CEIP",
            "Leaves the Messenger program.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Messenger\Client",
            "CEIP",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.media-player-usage",
            Privacy,
            "Telemetry",
            "Media Player usage tracking",
            "Stops Media Player tracking.",
            POLICY_DOCS,
            Documented,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\MediaPlayer\Preferences",
            "UsageTracking",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.toast-diagtrack",
            Privacy,
            "Telemetry",
            "Diagnostic prompt",
            "Marks the diagnostics prompt as seen.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack",
            "ShowedToastAtLevel",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.experimentation",
            Privacy,
            "Telemetry",
            "Windows experiments",
            "Blocks Microsoft's feature experiments.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\PolicyManager\current\device\System",
            "AllowExperimentation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.autologger-diagtrack",
            Privacy,
            "Telemetry",
            "Diagtrack listener",
            "Stops the telemetry trace at boot.",
            TAIRIKU,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\WMI\Autologger\AutoLogger-Diagtrack-Listener",
                "Start",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\WMI\Autologger\Diagtrack-Listener",
                "Start",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.autologger-diaglog",
            Privacy,
            "Telemetry",
            "DiagLog trace",
            "Stops the DiagLog trace at boot.",
            TAIRIKU,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\WMI\Autologger\DiagLog",
            "Start",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.autologger-sqm",
            Privacy,
            "Telemetry",
            "SQM trace",
            "Stops the SQM trace at boot.",
            TAIRIKU,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\WMI\AutoLogger\SQMLogger",
            "Start",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.autologger-wifi",
            Privacy,
            "Telemetry",
            "Wi-Fi session trace",
            "Stops the Wi-Fi trace at boot.",
            TAIRIKU,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\WMI\Autologger\WiFiSession",
            "Start",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.dotnet-telemetry",
            Privacy,
            "Telemetry",
            ".NET CLI telemetry",
            "Opts out of .NET CLI telemetry.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str("Environment", "DOTNET_CLI_TELEMETRY_OPTOUT", "1")],
    },
    SettingsTweak {
        meta: lib(
            "privacy.powershell-telemetry",
            Privacy,
            "Telemetry",
            "PowerShell telemetry",
            "Opts out of PowerShell telemetry.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str("Environment", "POWERSHELL_TELEMETRY_OPTOUT", "1")],
    },
    SettingsTweak {
        meta: lib(
            "privacy.consumer-features",
            Privacy,
            "Advertising and suggestions",
            "Consumer features",
            "Stops Windows installing suggested apps.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                "DisableWindowsConsumerFeatures",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                "DisableConsumerFeatures",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cloud-optimised",
            Privacy,
            "Advertising and suggestions",
            "Cloud-optimised content",
            "Stops cloud-delivered content.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableCloudOptimizedContent",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.soft-landing",
            Privacy,
            "Advertising and suggestions",
            "Tips and tricks",
            "Stops Windows tips.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableSoftLanding",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.tailored-policy",
            Privacy,
            "Advertising and suggestions",
            "Tailored experiences",
            "Stops ads built on diagnostic data.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                "DisableTailoredExperiencesWithDiagnosticData",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spotlight",
            Privacy,
            "Advertising and suggestions",
            "Windows Spotlight",
            "Turns off Spotlight everywhere.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightFeatures",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spotlight-action",
            Privacy,
            "Advertising and suggestions",
            "Spotlight on Action Center",
            "Stops suggestions in Action Center.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightOnActionCenter",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spotlight-lock",
            Privacy,
            "Advertising and suggestions",
            "Spotlight on lock screen",
            "Stops lock screen suggestions.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightOnLockScreen",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spotlight-settings",
            Privacy,
            "Advertising and suggestions",
            "Spotlight in Settings",
            "Stops suggestions in Settings.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            "DisableWindowsSpotlightOnSettings",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spotlight-welcome",
            Privacy,
            "Advertising and suggestions",
            "Welcome experience",
            "Stops the post-update welcome page.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                "DisableWindowsSpotlightWindowsWelcomeExperience",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement",
                "ScoobeSystemSettingEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-preinstalled",
            Privacy,
            "Advertising and suggestions",
            "Preinstalled apps",
            "Stops preinstalled suggested apps.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "PreInstalledAppsEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "PreInstalledAppsEverEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "OemPreInstalledAppsEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-silent",
            Privacy,
            "Advertising and suggestions",
            "Silent app installs",
            "Stops apps installing themselves.",
            BATLEZ,
            Community,
        )
        .impact(Medium),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SilentInstalledAppsEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContentEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-lockscreen",
            Privacy,
            "Advertising and suggestions",
            "Lock screen tips",
            "Stops lock screen facts.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "RotatingLockScreenEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "RotatingLockScreenOverlayEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Lock Screen",
                "SlideshowEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-settings",
            Privacy,
            "Advertising and suggestions",
            "Suggestions in Settings",
            "Stops suggestions in Settings.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338393Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-353694Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-353696Enabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-tips",
            Privacy,
            "Advertising and suggestions",
            "Tips while you use Windows",
            "Stops tips as you use Windows.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338389Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SoftLandingEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-start",
            Privacy,
            "Advertising and suggestions",
            "Suggestions in Start",
            "Stops suggestions in Start.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SubscribedContent-338388Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                "SystemPaneSuggestionsEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-welcome",
            Privacy,
            "Advertising and suggestions",
            "Welcome suggestions",
            "Stops welcome page suggestions.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-310093Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-lock-fun",
            Privacy,
            "Advertising and suggestions",
            "Fun facts",
            "Stops fun facts on the lock screen.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338387Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-mru",
            Privacy,
            "Advertising and suggestions",
            "Spotlight history",
            "Stops another suggestion feed.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-314559Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-feature-mgmt",
            Privacy,
            "Advertising and suggestions",
            "Feature management",
            "Stops feature promotions.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "FeatureManagementEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-content",
            Privacy,
            "Advertising and suggestions",
            "Content delivery",
            "Stops all content delivery.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "ContentDeliveryAllowed",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-slideshow",
            Privacy,
            "Advertising and suggestions",
            "Content slideshow",
            "Stops suggested slideshows.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SlideshowEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-rest",
            Privacy,
            "Advertising and suggestions",
            "Other suggestion channels",
            "Stops the remaining suggestion feeds.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-88000326Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.lock-creative",
            Privacy,
            "Advertising and suggestions",
            "Lock screen creative",
            "Clears Spotlight's creative id.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Software\Microsoft\Windows\CurrentVersion\Lock Screen\Creative",
            "CreativeId",
            "",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.online-tips",
            Privacy,
            "Advertising and suggestions",
            "Online tips",
            "Stops online tips in Settings.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "AllowOnlineTips",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.meet-now",
            Privacy,
            "Advertising and suggestions",
            "Meet Now",
            "Hides Meet Now.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "HideSCAMeetNow",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "HideSCAMeetNow",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.news-interests",
            Privacy,
            "Advertising and suggestions",
            "News and interests",
            "Turns off the feed.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Dsh",
                "AllowNewsAndInterests",
                0,
            ),
            hkcu(
                r"Software\Microsoft\PolicyManager\default\NewsAndInterests\AllowNewsAndInterests",
                "value",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.copilot-policy",
            Privacy,
            "Copilot and AI",
            "Windows Copilot",
            "Turns off Copilot.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
                "TurnOffWindowsCopilot",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\Shell\Copilot",
                "IsCopilotAvailable",
                0,
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\Shell\Copilot",
                "CopilotDisabledReason",
                "FeatureIsDisabled",
            ),
            hkcu(
                r"Software\Microsoft\Windows\Shell\Copilot\BingChat",
                "IsUserEligible",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.copilot-large",
            Privacy,
            "Copilot and AI",
            "Copilot on large screens",
            "Stops Copilot opening by itself.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\Shell\Copilot",
            "AutoOpenCopilotLargeScreens",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.recall",
            Privacy,
            "Copilot and AI",
            "Recall",
            "Turns off Recall.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "AllowRecallEnablement",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "DisableAIDataAnalysis",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.click-to-do",
            Privacy,
            "Copilot and AI",
            "Click to Do",
            "Turns off Click to Do.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "DisableClickToDo",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SmartActionPlatform\SmartClipboard",
                "Disabled",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.paint-cocreator",
            Privacy,
            "Copilot and AI",
            "Paint Cocreator",
            "Turns off Cocreator in Paint.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableCocreator",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.paint-fill",
            Privacy,
            "Copilot and AI",
            "Paint generative fill",
            "Turns off generative fill.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableGenerativeFill",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.paint-creator",
            Privacy,
            "Copilot and AI",
            "Paint Image Creator",
            "Turns off Image Creator.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableImageCreator",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.video-ai",
            Privacy,
            "Copilot and AI",
            "Video AI enhancement",
            "Stops automatic video enhancement.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSuperResolutionAutoEnhancement",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSuperResolutionAutoEnhancement",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableVideoContrastEnhancement",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableVideoContrastEnhancement",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.web-search",
            Privacy,
            "Search",
            "Web results in Search",
            "Keeps Search local.",
            POLICY_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "ConnectedSearchUseWeb",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "DisableWebSearch",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "BingSearchEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
                "DisableWebSearch",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.web-search-metered",
            Privacy,
            "Search",
            "Web search on metered",
            "Stops web results on metered networks.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "ConnectedSearchUseWebOverMeteredConnections",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-privacy",
            Privacy,
            "Search",
            "Search privacy level",
            "Sends less with search.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "ConnectedSearchPrivacy",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-location",
            Privacy,
            "Search",
            "Search location",
            "Stops Search using location.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "AllowSearchToUseLocation",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "AllowSearchToUseLocation",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cortana",
            Privacy,
            "Search",
            "Cortana",
            "Turns off Cortana.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "AllowCortana",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\Experience\AllowCortana",
                "value",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "CortanaEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "CanCortanaBeEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "CortanaConsent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Windows Search",
                "CortanaConsent",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
                "CortanaInAmbientMode",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "VoiceShortcut",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cortana-lock",
            Privacy,
            "Search",
            "Cortana on the lock screen",
            "Stops Cortana above the lock screen.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowCortanaAboveLock",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cloud-search",
            Privacy,
            "Search",
            "Cloud search",
            "Stops searching cloud content.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "AllowCloudSearch",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                "IsMSACloudSearchEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                "IsAADCloudSearchEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-history",
            Privacy,
            "Search",
            "Search history",
            "Stops device search history.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                "IsDeviceSearchHistoryEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "DeviceHistoryEnabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Search",
                "HistoryViewEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                "DisableSearchHistory",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-highlights",
            Privacy,
            "Search",
            "Search highlights",
            "Turns off search highlights.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                "EnableDynamicContentInWSB",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                "IsDynamicSearchBoxEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-box-suggestions",
            Privacy,
            "Search",
            "Search box suggestions",
            "Stops suggestions in Explorer's box.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableSearchBoxSuggestions",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.safe-search",
            Privacy,
            "Search",
            "SafeSearch",
            "Turns off SafeSearch in Windows Search.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
            "SafeSearchMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-background",
            Privacy,
            "Search",
            "Search background toggle",
            "Stops Search running in the background.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Search",
            "BackgroundAppGlobalToggle",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.search-disable",
            Privacy,
            "Search",
            "Search feature",
            "Turns off the search box entirely.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Start and taskbar search stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableSearch",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.remote-queries",
            Privacy,
            "Search",
            "Remote search queries",
            "Stops remote queries of the index.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "PreventRemoteQueries",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.indexer-backoff",
            Privacy,
            "Search",
            "Indexer back-off",
            "Lets indexing run while you work.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableIndexerBackoff",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-encrypted",
            Privacy,
            "Search",
            "Index encrypted files",
            "Keeps encrypted files out of the index.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AllowIndexingEncryptedStoresOrItems",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-language",
            Privacy,
            "Search",
            "Language detection",
            "Stops auto-detecting indexed languages.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "AlwaysUseAutoLangDetection",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-removable",
            Privacy,
            "Search",
            "Index removable drives",
            "Keeps removable drives out of the index.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "DisableRemovableDriveIndexing",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-attachments",
            Privacy,
            "Search",
            "Index email attachments",
            "Stops indexing attachments.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "PreventIndexingEmailAttachments",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-outlook",
            Privacy,
            "Search",
            "Index Outlook",
            "Stops indexing Outlook.",
            POLICY_DOCS,
            Documented,
        )
        .warn("Outlook search gets slower.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "PreventIndexingOutlook",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.index-public",
            Privacy,
            "Search",
            "Index public folders",
            "Stops indexing public folders.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
            "PreventIndexingPublicFolders",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.settings-sync",
            Privacy,
            "Sync and cross-device",
            "Settings sync",
            "Stops syncing settings to your account.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync",
            "SyncPolicy",
            5,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-accessibility",
            Privacy,
            "Sync and cross-device",
            "Sync accessibility",
            "Stops syncing accessibility settings.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Accessibility",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-apps",
            Privacy,
            "Sync and cross-device",
            "Sync apps",
            "Stops syncing app data.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\AppSync",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-browser",
            Privacy,
            "Sync and cross-device",
            "Sync browser",
            "Stops syncing browser settings.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\BrowserSettings",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-credentials",
            Privacy,
            "Sync and cross-device",
            "Sync passwords",
            "Stops syncing passwords.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-theme",
            Privacy,
            "Sync and cross-device",
            "Sync theme",
            "Stops syncing your theme.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\DesktopTheme",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-language",
            Privacy,
            "Sync and cross-device",
            "Sync language",
            "Stops syncing language settings.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Language",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-packages",
            Privacy,
            "Sync and cross-device",
            "Sync package state",
            "Stops syncing package state.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\PackageState",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-personal",
            Privacy,
            "Sync and cross-device",
            "Sync personalisation",
            "Stops syncing personalisation.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Personalization",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-start",
            Privacy,
            "Sync and cross-device",
            "Sync Start layout",
            "Stops syncing Start.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\StartLayout",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-windows",
            Privacy,
            "Sync and cross-device",
            "Sync Windows settings",
            "Stops syncing other settings.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Windows",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.clipboard-history",
            Privacy,
            "Sync and cross-device",
            "Clipboard history",
            "Turns off Win+V.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Win+V history stops working.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "AllowClipboardHistory",
                0,
            ),
            hkcu(r"Software\Microsoft\Clipboard", "EnableClipboardHistory", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.clipboard-cloud",
            Privacy,
            "Sync and cross-device",
            "Cloud clipboard",
            "Stops syncing the clipboard.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "AllowCrossDeviceClipboard",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Clipboard",
                "CloudClipboardAutomaticUpload",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cross-device",
            Privacy,
            "Sync and cross-device",
            "Cross-device experiences",
            "Turns off cross-device features.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CrossDevice",
                "AllowCrossDevice",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CrossDevice",
                "EnableCrossDevice",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CrossDevice",
                "ResumeAllowed",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cross-resume",
            Privacy,
            "Sync and cross-device",
            "Cross-device resume",
            "Stops resuming from your phone.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume",
                "Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume",
                "UsePerformanceMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdp",
            Privacy,
            "Sync and cross-device",
            "Shared experiences",
            "Stops apps continuing on other devices.",
            TAIRIKU,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CDP",
                "CdpSessionUserAuthzPolicy",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CDP",
                "RomeSdkChannelUserAuthzPolicy",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.nearby",
            Privacy,
            "Sync and cross-device",
            "Nearby sharing",
            "Turns off Nearby sharing.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\CDP",
            "NearShareChannelUserAuthzPolicy",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.mobility",
            Privacy,
            "Sync and cross-device",
            "Phone Link suggestion",
            "Stops linking prompts.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Mobility",
            "OptedIn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.find-my-device",
            Privacy,
            "Sync and cross-device",
            "Find My Device",
            "Turns off Find My Device.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("A lost PC can't be located.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\FindMyDevice",
                "AllowFindMyDevice",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Settings\FindMyDevice",
                "LocationSyncEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ms-account",
            Privacy,
            "Sync and cross-device",
            "Microsoft accounts",
            "Blocks Microsoft account sign-in.",
            POLICY_DOCS,
            Documented,
        )
        .high()
        .warn("Store, OneDrive and Xbox sign-in stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\MicrosoftAccount",
            "DisableUserAuth",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.device-registration",
            Privacy,
            "Sync and cross-device",
            "Device registration",
            "Stops joining Microsoft cloud directories.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\DeviceRegistration",
            "DisableRegistration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.mdm-enrol",
            Privacy,
            "Sync and cross-device",
            "Automatic MDM enrolment",
            "Stops enrolling in device management.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\MDM",
                "AutoEnrollMDM",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\MDM",
                "DisableRegistration",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-activity",
            Privacy,
            "App permissions",
            "Motion and activity",
            "Blocks apps reading activity.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-diagnostics",
            Privacy,
            "App permissions",
            "App diagnostics",
            "Blocks apps reading other apps.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-appointments",
            Privacy,
            "App permissions",
            "Calendar",
            "Blocks apps reading your calendar.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-bg-spatial",
            Privacy,
            "App permissions",
            "Background spatial",
            "Blocks spatial data in the background.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\backgroundSpatialPerception",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-bluetooth",
            Privacy,
            "App permissions",
            "Bluetooth access",
            "Blocks apps using Bluetooth.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetooth",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-bluetooth-sync",
            Privacy,
            "App permissions",
            "Bluetooth sync",
            "Blocks apps syncing over Bluetooth.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-files",
            Privacy,
            "App permissions",
            "Full file system access",
            "Blocks apps reading all files.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some Store apps lose file access.")
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-chat",
            Privacy,
            "App permissions",
            "Messaging",
            "Blocks apps reading messages.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-contacts",
            Privacy,
            "App permissions",
            "Contacts",
            "Blocks apps reading contacts.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-documents",
            Privacy,
            "App permissions",
            "Documents",
            "Blocks apps reading Documents.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-downloads",
            Privacy,
            "App permissions",
            "Downloads",
            "Blocks apps reading Downloads.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsFolder",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsFolder",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-email",
            Privacy,
            "App permissions",
            "Email",
            "Blocks apps reading email.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-gaze",
            Privacy,
            "App permissions",
            "Eye tracking",
            "Blocks apps using eye tracking.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-capture",
            Privacy,
            "App permissions",
            "Screen capture",
            "Blocks apps capturing the screen.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Store recording apps stop working.")
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\graphicsCaptureProgrammatic",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\graphicsCaptureProgrammatic",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\graphicsCaptureProgrammatic\NonPackaged",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-capture-border",
            Privacy,
            "App permissions",
            "Borderless capture",
            "Blocks capture without a border.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\graphicsCaptureWithoutBorder",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\graphicsCaptureWithoutBorder",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-hid",
            Privacy,
            "App permissions",
            "Human interface devices",
            "Blocks apps reading HID devices.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\humanInterfaceDevice",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-presence",
            Privacy,
            "App permissions",
            "Presence sensing",
            "Blocks apps using presence sensors.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\humanPresence",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-microphone",
            Privacy,
            "App permissions",
            "Microphone",
            "Blocks apps using the microphone.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Voice chat stops working.")
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                "Value",
                "Deny",
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone\NonPackaged",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-music",
            Privacy,
            "App permissions",
            "Music",
            "Blocks apps reading Music.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musicLibrary",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musicLibrary",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-phone",
            Privacy,
            "App permissions",
            "Phone calls",
            "Blocks apps making calls.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-phone-history",
            Privacy,
            "App permissions",
            "Call history",
            "Blocks apps reading call history.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-pictures",
            Privacy,
            "App permissions",
            "Pictures",
            "Blocks apps reading Pictures.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-radios",
            Privacy,
            "App permissions",
            "Radios",
            "Blocks apps controlling radios.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-serial",
            Privacy,
            "App permissions",
            "Serial devices",
            "Blocks apps using serial ports.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\serialCommunication",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\serialCommunication",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-spatial",
            Privacy,
            "App permissions",
            "Spatial perception",
            "Blocks apps reading spatial data.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\spatialPerception",
            "Value",
            "Deny",
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-tasks",
            Privacy,
            "App permissions",
            "Tasks",
            "Blocks apps reading tasks.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-notifications",
            Privacy,
            "App permissions",
            "Notification access",
            "Blocks apps reading notifications.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-videos",
            Privacy,
            "App permissions",
            "Videos",
            "Blocks apps reading Videos.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-camera",
            Privacy,
            "App permissions",
            "Camera",
            "Blocks apps using the camera.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Video calls stop working.")
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perm-wifi",
            Privacy,
            "App permissions",
            "Wi-Fi data",
            "Blocks apps reading Wi-Fi data.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\wifiData",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\wifiData",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.camera-policy",
            Privacy,
            "App permissions",
            "Camera policy",
            "Turns off the camera for all users.",
            POLICY_DOCS,
            Documented,
        )
        .high()
        .warn("Every app loses the camera.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Camera",
            "AllowCamera",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.camera-effects",
            Privacy,
            "App permissions",
            "Camera effects",
            "Stops camera effects.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\Camera\AllowCameraEffects",
                "value",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\Experience\AllowVideoFilters",
                "value",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.lock-camera",
            Privacy,
            "App permissions",
            "Lock screen camera",
            "Turns off the camera on the lock screen.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Personalization",
            "NoLockScreenCamera",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.location-policy",
            Privacy,
            "App permissions",
            "Location policy",
            "Turns off location for all users.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Maps and Find My Device stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableLocation",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sensors-policy",
            Privacy,
            "App permissions",
            "Sensors policy",
            "Turns off sensors.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Auto-rotate and adaptive brightness stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            "DisableSensors",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ie-geolocation",
            Privacy,
            "App permissions",
            "IE geolocation",
            "Stops websites asking for location in IE.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Internet Explorer\Geolocation",
                "BlockAllWebsites",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Internet Explorer\Geolocation",
                "BlockAllWebsites",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sensor-permission",
            Privacy,
            "App permissions",
            "Location sensor override",
            "Clears the location sensor state.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}",
            "SensorPermissionState",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.background-global",
            Privacy,
            "App permissions",
            "Background apps (user)",
            "Stops Store apps in the background.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
            "GlobalUserDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.push-policy",
            Privacy,
            "App permissions",
            "Cloud notifications policy",
            "Stops notifications from the cloud.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\PushNotifications",
                "NoCloudApplicationNotification",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications",
                "NoToastApplicationNotification",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.input-personalization",
            Privacy,
            "Speech and typing",
            "Online speech policy",
            "Turns off typing and speech learning.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\InputPersonalization",
            "AllowInputPersonalization",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ink-collection",
            Privacy,
            "Speech and typing",
            "Ink collection",
            "Stops learning from handwriting.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                "RestrictImplicitInkCollection",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                "RestrictImplicitTextCollection",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.harvest-contacts",
            Privacy,
            "Speech and typing",
            "Contact harvesting",
            "Stops learning from contacts.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\InputPersonalization\TrainedDataStore",
            "HarvestContacts",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.privacy-policy-input",
            Privacy,
            "Speech and typing",
            "Inking privacy flag",
            "Clears the accepted privacy flag.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Personalization\Settings",
            "AcceptedPrivacyPolicy",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.speech-model",
            Privacy,
            "Speech and typing",
            "Speech model updates",
            "Stops speech model downloads.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Speech",
                "AllowSpeechModelUpdate",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Preferences",
                "ModelDownloadAllowed",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.voice-activation",
            Privacy,
            "Speech and typing",
            "Voice activation",
            "Stops apps listening for keywords.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Preferences",
                "VoiceActivationOn",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Speech_OneCore\Preferences",
                "VoiceActivationDefaultOn",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                "AgentActivationEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.voice-lock",
            Privacy,
            "Speech and typing",
            "Voice on the lock screen",
            "Stops voice activation when locked.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Preferences",
                "VoiceActivationEnableAboveLockscreen",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                "AgentActivationOnLockScreenEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.enhanced-audio",
            Privacy,
            "Speech and typing",
            "Enhanced speech audio",
            "Stops enhanced speech audio.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Speech_OneCore\AudioProcessing",
                "EnableEnhancedAudio",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Speech_OneCore\AudioProcessing",
                "EnableEnhancedAudio",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.voice-typing",
            Privacy,
            "Speech and typing",
            "Voice typing",
            "Turns off voice typing.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Input\Settings",
            "VoiceTypingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.typing-insights",
            Privacy,
            "Speech and typing",
            "Typing insights",
            "Turns off typing insights.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Input\Settings",
            "InsightsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.text-prediction",
            Privacy,
            "Speech and typing",
            "Hardware keyboard prediction",
            "Turns off text suggestions.",
            TAIRIKU,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Input\Settings",
                "EnableHwkbTextPrediction",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Input\Settings",
                "MultilingualEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.tipc",
            Privacy,
            "Speech and typing",
            "Inking and typing data",
            "Stops sending typing data.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Input\TIPC", "Enabled", 0),
            hkcu(r"Software\Microsoft\Input\TIPC", "Enabled", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.autocorrect",
            Privacy,
            "Speech and typing",
            "Touch keyboard autocorrect",
            "Turns off autocorrect.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\TabletTip\1.7",
            "EnableAutocorrection",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.spellcheck-touch",
            Privacy,
            "Speech and typing",
            "Touch keyboard spelling",
            "Turns off spell check.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\TabletTip\1.7",
            "EnableSpellchecking",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.touch-auto",
            Privacy,
            "Speech and typing",
            "Touch keyboard auto-show",
            "Stops it appearing in desktop mode.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\TabletTip\1.7",
            "EnableDesktopModeAutoInvoke",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.touch-button",
            Privacy,
            "Speech and typing",
            "Touch keyboard button",
            "Hides the taskbar button.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\TabletTip\1.7",
            "TipbandDesiredVisibility",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.touch-keyboard",
            Privacy,
            "Speech and typing",
            "Touch keyboard policy",
            "Turns off the touch keyboard.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Touch typing stops working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\TabletPC",
            "DisableTouchKeyboard",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.input-preload",
            Privacy,
            "Speech and typing",
            "Input app preload",
            "Stops preloading the input app.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Input",
            "IsInputAppPreloadEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ink-workspace",
            Privacy,
            "Speech and typing",
            "Windows Ink Workspace",
            "Turns off Ink Workspace.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\WindowsInkWorkspace",
                "AllowWindowsInkWorkspace",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\PenWorkspace",
                "PenWorkspaceButtonDesiredVisibility",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.snipping-tool",
            Privacy,
            "Speech and typing",
            "Snipping Tool policy",
            "Turns off the Snipping Tool.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Win+Shift+S stops working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
            "DisableSnippingTool",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cpss-advertising",
            Privacy,
            "System reports",
            "Advertising setting store",
            "Turns off the setting's store copy.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\CPSS\Store\AdvertisingInfo",
                "Value",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                "Enabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cpss-ink",
            Privacy,
            "System reports",
            "Inking store",
            "Turns off the setting's store copy.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\CPSS\Store\ImproveInkingAndTyping",
            "Value",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.push-to-install",
            Privacy,
            "System reports",
            "PushToInstall policy",
            "Stops apps being pushed from the web.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\PushToInstall",
            "DisablePushToInstall",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.kms-ticket",
            Privacy,
            "System reports",
            "KMS online validation",
            "Stops sending activation data to KMS.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows NT\CurrentVersion\Software Protection Platform",
            "NoGenTicket",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.map-traffic",
            Privacy,
            "System reports",
            "Maps traffic",
            "Stops Maps using the network unprompted.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Maps",
                "AllowUntriggeredNetworkTrafficOnSettingsPage",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\Maps",
                "AutoDownloadAndUpdateMapData",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.media-player-codecs",
            Privacy,
            "System reports",
            "Media Player codec download",
            "Stops codec downloads.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\WindowsMediaPlayer",
            "PreventCodecDownload",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.wmdrm",
            Privacy,
            "System reports",
            "WMDRM online",
            "Stops DRM licence updates.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\WMDRM",
            "DisableOnline",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.device-metadata",
            Privacy,
            "System reports",
            "Device metadata",
            "Stops downloading device metadata.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata",
            "PreventDeviceMetadataFromNetwork",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.driver-search",
            Privacy,
            "System reports",
            "Driver search",
            "Stops searching Windows Update for drivers.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("New hardware needs drivers installed by hand.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching",
                "DontSearchWindowsUpdate",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching",
                "SearchOrderConfig",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\DriverSearching",
                "DontPromptForWindowsUpdate",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.exclude-drivers",
            Privacy,
            "System reports",
            "Driver updates in Windows Update",
            "Keeps drivers out of quality updates.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("You update drivers yourself.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\current\device\Update",
                "ExcludeWUDriversInQualityUpdate",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\PolicyManager\default\Update\ExcludeWUDriversInQualityUpdate",
                "value",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.scheduled-diagnostics",
            Privacy,
            "System reports",
            "Scheduled diagnostics",
            "Stops scheduled troubleshooting.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\ScheduledDiagnostics",
                "EnabledExecution",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\ScheduledDiagnostics",
                "EnabledExecution",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.online-troubleshooters",
            Privacy,
            "System reports",
            "Online troubleshooters",
            "Stops downloading troubleshooters.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\ScriptedDiagnosticsProvider\Policy",
                "EnableQueryRemoteServer",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\ScriptedDiagnosticsProvider\Policy",
                "DisableQueryRemoteServer",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.perftrack",
            Privacy,
            "System reports",
            "PerfTrack",
            "Stops the responsiveness tracker.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}",
            "ScenarioExecutionEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.vs-feedback",
            Privacy,
            "System reports",
            "Visual Studio feedback",
            "Turns off feedback tools.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\VisualStudio\Feedback",
                "DisableEmailInput",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\VisualStudio\Feedback",
                "DisableFeedbackDialog",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\VisualStudio\Feedback",
                "DisableScreenshotCapture",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.vs-intellicode",
            Privacy,
            "System reports",
            "IntelliCode remote analysis",
            "Stops sending code to Microsoft.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\VisualStudio\IntelliCode",
                "DisableRemoteAnalysis",
                1,
            ),
            hkcu(
                r"Software\Microsoft\VSCommon\16.0\IntelliCode",
                "DisableRemoteAnalysis",
                1,
            ),
            hkcu(
                r"Software\Microsoft\VSCommon\17.0\IntelliCode",
                "DisableRemoteAnalysis",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.vs-sqm",
            Privacy,
            "System reports",
            "Visual Studio CEIP",
            "Leaves the VS improvement program.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Microsoft\VisualStudio\SQM", "OptIn", 0),
            hklm(r"SOFTWARE\Microsoft\VSCommon\14.0\SQM", "OptIn", 0),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\14.0\SQM",
                "OptIn",
                0,
            ),
            hklm(r"SOFTWARE\Microsoft\VSCommon\15.0\SQM", "OptIn", 0),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM",
                "OptIn",
                0,
            ),
            hklm(r"SOFTWARE\Microsoft\VSCommon\16.0\SQM", "OptIn", 0),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM",
                "OptIn",
                0,
            ),
            hklm(r"SOFTWARE\Microsoft\VSCommon\17.0\SQM", "OptIn", 0),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM",
                "OptIn",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.vs-telemetry",
            Privacy,
            "System reports",
            "Visual Studio telemetry",
            "Turns off VS telemetry.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\VisualStudio\Telemetry",
            "TurnOffSwitch",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.vscode-telemetry",
            Privacy,
            "System reports",
            "VS Code telemetry flags",
            "Sets VS Code's telemetry keys.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\VSCode",
                "telemetry.enableCrashReporter",
                0,
            ),
            hkcu(r"Software\Microsoft\VSCode", "telemetry.enableTelemetry", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ad-id-policy",
            Privacy,
            "Advertising and suggestions",
            "Advertising ID policy",
            "Turns the advertising ID off for every account.",
            POLICY_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
            "DisabledByGroupPolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.mic-policy",
            Privacy,
            "App permissions",
            "Microphone for apps",
            "Denies Store apps the microphone by policy.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Voice chat and dictation in Store apps stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
            "LetAppsAccessMicrophone",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.location-consent",
            Privacy,
            "App permissions",
            "Location consent",
            "Denies location to apps on this PC.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Maps, weather and Find my device lose your location.")
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.account-consent",
            Privacy,
            "App permissions",
            "Account info consent",
            "Denies apps your name and picture.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm_str(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                "Value",
                "Deny",
            ),
            hkcu_str(
                r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                "Value",
                "Deny",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.telemetry-machine",
            Privacy,
            "Telemetry",
            "Telemetry level",
            "Sets this PC's telemetry level to the lowest.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
            "AllowTelemetry",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.steps-recorder-log",
            Privacy,
            "Telemetry",
            "Steps Recorder log",
            "Turns off the Steps Recorder event log.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\WINEVT\Channels\Microsoft-Windows-Application-Experience/Steps-Recorder",
            "Enabled",
            0,
        )],
    },
];
