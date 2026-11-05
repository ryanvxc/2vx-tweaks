//! Privacy tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "privacy.onesettings",
            Privacy,
            "Telemetry",
            "No OneSettings downloads",
            "Stops Windows downloading settings that adjust its data collection.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("OneSettings lets Microsoft change some diagnostic behaviour remotely, without an update. This blocks those downloads. There's no visible change in normal use."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
            "DisableOneSettingsDownloads",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.ceip",
            Privacy,
            "Telemetry",
            "No Experience Program",
            "Takes this PC out of Microsoft's older program for collecting usage data.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("This is the official opt-out for the Customer Experience Improvement Program. Windows diagnostic data has mostly replaced it, so the effect is small, but the remaining collector stops sending data."),
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
            "No application telemetry",
            "Stops Windows tracking how apps run and are used for compatibility reporting.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Application Telemetry records which programs you use and how they behave, as part of Windows diagnostics. Turning it off reduces that data and has no visible effect on your apps."),
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
            "No application inventory",
            "Stops Windows listing your installed apps, files and devices to send to Microsoft.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("The Inventory Collector records the software and hardware on your PC for compatibility checks. Turning it off reduces that data. Windows may have less to go on when checking a feature update."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
            "DisableInventory",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.mrt-report",
            Privacy,
            "Telemetry",
            "No removal tool reports",
            "Stops the Malicious Software Removal Tool sending infection reports to Microsoft.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("The tool still runs after Windows updates and still removes what it finds. This only stops it reporting the results, so your protection doesn't change."),
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
            "No error reporting",
            "Stops Windows sending Microsoft a report when an app or Windows crashes.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Crash reports can include details of what the app was doing. With reporting off, nothing is sent. You lose any solutions Windows would offer for a crash."),
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
            "privacy.autologger-diagtrack",
            Privacy,
            "Telemetry",
            "No telemetry boot trace",
            "Stops the trace that collects telemetry events for Microsoft from the start of boot.",
            TAIRIKU,
            Observed,
        )
        .reboot()
        .admin()
        .details("This early trace feeds the Windows diagnostics service and writes a log file to disk. Turning it off reduces what is captured at startup. Takes effect after a restart."),
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
            "privacy.dotnet-telemetry",
            Privacy,
            "Telemetry",
            "No .NET CLI telemetry",
            "Stops the .NET developer tools sending Microsoft usage data when you run them.",
            BATLEZ,
            Community,
        )
        .details("Only relevant if you have the .NET SDK installed and use its command line tools. This sets Microsoft's own opt-out. Apps built with .NET aren't affected."),
        settings: &[hkcu_str("Environment", "DOTNET_CLI_TELEMETRY_OPTOUT", "1")],
    },
    SettingsTweak {
        meta: lib(
            "privacy.powershell-telemetry",
            Privacy,
            "Telemetry",
            "No PowerShell telemetry",
            "Stops PowerShell 7 sending Microsoft basic usage data when it starts.",
            BATLEZ,
            Community,
        )
        .details("Only relevant if you've installed PowerShell 7 or later; the built-in Windows PowerShell doesn't send this data. This sets Microsoft's own opt-out."),
        settings: &[hkcu_str("Environment", "POWERSHELL_TELEMETRY_OPTOUT", "1")],
    },
    SettingsTweak {
        meta: lib(
            "privacy.tailored-policy",
            Privacy,
            "Advertising and suggestions",
            "No tailored experiences",
            "Stops Microsoft using your diagnostic data to tailor tips, ads and recommendations.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Tailored experiences use your diagnostic data to personalise what Windows suggests. This turns that off. Windows still collects data at your diagnostic level, but won't use it this way."),
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
            "privacy.spotlight-welcome",
            Privacy,
            "Advertising and suggestions",
            "No finish setup prompts",
            "Stops full-screen prompts asking you to finish setting up your PC.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Windows can ask you to link a phone, try Microsoft 365 or set up OneDrive after updates. This turns those prompts off so you go straight to your desktop. Nothing is removed."),
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
            "privacy.cdm-silent",
            Privacy,
            "Advertising and suggestions",
            "No silent app installs",
            "Stops Windows quietly installing sponsored apps and games.",
            BATLEZ,
            Community,
        )
        .details("Windows can add promoted apps to your account without asking, often after a big update. This turns off that delivery. Apps already installed stay until you remove them."),
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
            "privacy.cdm-settings",
            Privacy,
            "Advertising and suggestions",
            "No suggestions in Settings",
            "Stops the Settings app showing suggested content and app promotions.",
            BATLEZ,
            Community,
        )
        .details("Settings pages can carry Microsoft suggestions, including promotions for its own services. This removes them. All the normal options stay where they are."),
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
            "No tips while you use Windows",
            "Stops pop-ups with tips and suggestions while you use Windows.",
            BATLEZ,
            Observed,
        )
        .details("Windows shows notifications about features and Microsoft services based on how you use the PC. This turns them off. You can still find help when you want it."),
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
            "No suggestions in Start",
            "Removes suggested apps and promotions from the Start menu.",
            BATLEZ,
            Community,
        )
        .details("Stops Windows using Start to advertise apps and Microsoft services. Your pinned apps and files aren't affected. On Windows 11 the Recommended section has its own Visuals card."),
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
            "No post-update welcome page",
            "Stops the full-screen page showing what's new after some Windows updates.",
            BATLEZ,
            Community,
        )
        .details("After updates and sometimes at sign-in, Windows can open a page promoting new features and Microsoft services. This turns it off, so you go straight to your desktop."),
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
            "No lock screen fun facts",
            "Removes the fun facts, tips and offers written over your lock screen picture.",
            BATLEZ,
            Community,
        )
        .details("Windows can overlay text and links from Microsoft on the lock screen, some of them promotions. This turns that off and leaves your picture clean. It doesn't change the picture itself."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            "SubscribedContent-338387Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.online-tips",
            Privacy,
            "Advertising and suggestions",
            "No online tips",
            "Stops the Settings app fetching online tips and help from Microsoft.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Settings pages can pull tips and help links from the internet. This keeps them offline and removes those panels. Every setting stays where it is."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "AllowOnlineTips",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.copilot-policy",
            Privacy,
            "Copilot and AI",
            "Turn off Windows Copilot",
            "Turns off the Copilot built into Windows and removes its taskbar entry.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Uses Microsoft's policy to switch off Windows Copilot. Microsoft has deprecated this policy and the newer Copilot app may not be covered. If Copilot still opens, uninstall the app as well."),
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
            "privacy.recall",
            Privacy,
            "Copilot and AI",
            "Turn off Recall",
            "Removes Recall, which saves regular snapshots of your screen to search later.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Recall stores snapshots of what you do on the PC. This turns it off and stops it being turned on. It only exists on Copilot+ PCs, so other PCs see no change."),
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
            "Turn off Click to Do",
            "Turns off the AI overlay that scans what's on screen so you can act on it.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Click to Do analyses a screenshot of what you are viewing to offer actions on text and pictures. It only exists on Copilot+ PCs, so other PCs see no change."),
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
            "privacy.paint-fill",
            Privacy,
            "Copilot and AI",
            "No Paint generative fill",
            "Removes Paint's AI generative fill tool, for anyone who doesn't want it.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Generative fill uses AI to add or change part of a picture from a text prompt. This hides the tool. The rest of Paint is unchanged."),
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
            "No Paint Image Creator",
            "Removes Paint's Image Creator, which sends your prompts to Microsoft.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Image Creator generates pictures from text using an online service. Turning it off hides the tool, which suits anyone who doesn't want AI features in Paint. The rest of Paint is unchanged."),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
            "DisableImageCreator",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.web-search",
            Privacy,
            "Search",
            "No web results in Search",
            "Keeps Windows Search to your PC, with no Bing results or web queries.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("What you type in Start stays on this PC instead of going to Bing, and results are local only. You lose quick web answers. Recent Windows 11 builds may still show some web content."),
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
            "privacy.search-location",
            Privacy,
            "Search",
            "No location in Search",
            "Stops Windows Search using this PC's location for its results.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Search can use your location for local results such as weather and places. This stops that, whatever your other location settings are. It matters little if web results are already off."),
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
            "privacy.cloud-search",
            Privacy,
            "Search",
            "No cloud search",
            "Stops Windows Search looking through OneDrive, Outlook and cloud content.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Search can include items from your Microsoft or work account. This keeps results to this PC, so your account content isn't pulled into local searches. Files synced to the PC are still found."),
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
            "No search history",
            "Stops Windows keeping a record of what you search for on this PC.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Device search history improves results based on past searches. Turning it off stops that record, and past searches no longer appear in the search box. Results may be a little less personal."),
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
            "privacy.settings-sync",
            Privacy,
            "Sync and cross-device",
            "No settings sync",
            "Stops Windows syncing your settings to your Microsoft account.",
            BATLEZ,
            Observed,
        )
        .details("Turns off settings sync as a whole, so your preferences stay on this PC and aren't copied to other devices. Moving to a new PC won't bring your settings with it."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync",
            "SyncPolicy",
            5,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.sync-credentials",
            Privacy,
            "Sync and cross-device",
            "No password sync",
            "Stops Windows syncing saved passwords to your Microsoft account.",
            BATLEZ,
            Observed,
        )
        .details("Keeps Windows' saved passwords on this PC rather than syncing them between devices. Use this if you want sync for other settings but not passwords."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.clipboard-history",
            Privacy,
            "Sync and cross-device",
            "No clipboard history",
            "Stops Windows keeping a list of what you've copied, including any passwords.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("Win+V history stops working.")
        .admin()
        .details("Without clipboard history, only the last item you copied is kept. This helps if you often copy sensitive text. The Win+V list stops working."),
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
            "No cloud clipboard",
            "Stops what you copy being sent to the cloud and shared with your other devices.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Clipboard sync can upload copied text and images through your Microsoft account. Turning it off keeps them on this PC. You can't paste between devices, but normal copy and paste still works."),
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
            "privacy.cdp",
            Privacy,
            "Sync and cross-device",
            "No shared experiences",
            "Stops apps on other devices connecting to this PC or continuing tasks on it.",
            TAIRIKU,
            Observed,
        )
        .details("Shared experiences let apps hand tasks between your devices through your Microsoft account. Turning it off limits what this PC shares with them. Cross-device app features stop working."),
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
            "Turn off Nearby sharing",
            "Stops this PC being discoverable for sharing files with nearby devices.",
            TAIRIKU,
            Observed,
        )
        .details("Nearby sharing sends files and links to devices around you over Bluetooth and Wi-Fi. Turning it off means nearby devices can't send to this PC. It's already off on many PCs."),
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
            "No Phone Link suggestions",
            "Stops Windows suggesting you link your phone to this PC.",
            TAIRIKU,
            Observed,
        )
        .details("Turns off the prompts that promote Phone Link and using your mobile device with Windows. If you already use Phone Link, it carries on working."),
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
            "Turn off Find My Device",
            "Stops this PC regularly saving its location to your Microsoft account.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("A lost PC can't be located.")
        .admin()
        .details("Find My Device lets you locate a lost PC or lock it remotely. Turning it off stops that location tracking. It's most useful on laptops, so think before using this on one you carry around."),
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
            "privacy.perm-appointments",
            Privacy,
            "App permissions",
            "Block calendar access",
            "Stops apps reading the calendar and appointments stored in Windows.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("This is the Calendar switch in Windows privacy settings, set off for the PC. It mainly affects Store apps. Apps that rely on Windows' calendar store can't show your events."),
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
            "privacy.perm-files",
            Privacy,
            "App permissions",
            "Block full file system access",
            "Stops Store apps reading every file you can reach, not just their own folders.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some Store apps lose file access.")
        .admin()
        .details("A few Store apps ask for access to your whole file system. This denies it for the PC. Apps can still open files you pick. Desktop programs are not limited by this setting."),
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
            "privacy.perm-contacts",
            Privacy,
            "App permissions",
            "Block contacts access",
            "Stops apps reading the contacts stored in Windows.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("This is the Contacts switch in Windows privacy settings, set off for the PC. It mainly affects Store apps. Apps can't use your address book until you undo this."),
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
            "Block Documents access",
            "Stops Store apps reading your Documents folder without you choosing a file.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Apps can still open a file you pick yourself. This only removes their blanket access to the folder. Desktop programs are not limited by this setting.")
        .warn("Store apps that browse your Documents folder lose access."),
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
            "Block Downloads access",
            "Stops Store apps reading your Downloads folder without you choosing a file.",
            TAIRIKU,
            Observed,
        )
        .admin()
        .details("Apps can still open a file you pick yourself. This only removes their blanket access to the folder. Desktop programs are not limited by this setting.")
        .warn("Store apps that browse your Downloads folder lose access."),
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
            "Block email access",
            "Stops apps reading the email accounts and messages stored in Windows.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("This is the Email switch in Windows privacy settings, set off for the PC. It only affects apps using Windows' own mail store, not web mail or Outlook from Office."),
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
            "privacy.perm-capture",
            Privacy,
            "App permissions",
            "Block screen capture access",
            "Stops apps taking screenshots of or recording your screen through Windows.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin()
        .details("Apps can't capture your screen or other windows through the Windows capture feature. It's a strong privacy limit but it can affect recording and streaming. Undo it if a capture app shows a black window.")
        .warn("Apps that record or share your screen can stop working."),
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
            "privacy.perm-microphone",
            Privacy,
            "App permissions",
            "Block microphone access",
            "Turns the microphone off for every app, the same as the main switch in Settings.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Voice chat stops working.")
        .admin()
        .details("No app can use the microphone, including desktop programs such as game voice chat. Only worth it if you never use a microphone on this PC."),
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
            "privacy.perm-pictures",
            Privacy,
            "App permissions",
            "Block Pictures access",
            "Stops Store apps reading your Pictures folder without you choosing a file.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Apps can still open a file you pick yourself. This only removes their blanket access to the folder. Desktop programs are not limited by this setting.")
        .warn("Store apps that browse your Pictures folder lose access."),
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
            "privacy.perm-videos",
            Privacy,
            "App permissions",
            "Block Videos access",
            "Stops Store apps reading your Videos folder without you choosing a file.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Apps can still open a file you pick yourself. This only removes their blanket access to the folder. Desktop programs are not limited by this setting.")
        .warn("Store apps that browse your Videos folder lose access."),
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
            "Block camera access",
            "Turns the camera off for every app, the same as the main switch in Settings.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Video calls stop working.")
        .admin()
        .details("No app can use the camera, including desktop programs. A strong choice if you never use a webcam on this PC. You can undo it whenever you need a video call."),
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
            "privacy.lock-camera",
            Privacy,
            "App permissions",
            "No lock screen camera",
            "Stops anyone using the camera from the lock screen without signing in.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Some tablets and laptops can open the camera from the lock screen. This turns that off. On a PC with no lock screen camera there's no change, and the camera works as normal once you sign in."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Personalization",
            "NoLockScreenCamera",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.voice-activation",
            Privacy,
            "Speech and typing",
            "No voice activation",
            "Stops apps listening for wake words such as voice assistant keywords.",
            BATLEZ,
            Community,
        )
        .admin()
        .details("Voice activation keeps an app listening through the microphone for its keyword. This turns it off for all apps, so assistants only respond when you open them yourself."),
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
            "privacy.tipc",
            Privacy,
            "Speech and typing",
            "No inking and typing data",
            "Stops Windows sending samples of your typing and handwriting to Microsoft.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("This is the setting for improving inking and typing recognition, separate from your personal dictionary. Turning it off stops that contribution and doesn't change how typing works."),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Input\TIPC", "Enabled", 0),
            hkcu(r"Software\Microsoft\Input\TIPC", "Enabled", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.push-to-install",
            Privacy,
            "System reports",
            "No remote app installs",
            "Stops apps being installed on this PC remotely from the Store website.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Anyone signed in to your Microsoft account can start an install from the web. This blocks that route. Installing apps from the Store on this PC still works."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\PushToInstall",
            "DisablePushToInstall",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.driver-search",
            Privacy,
            "System reports",
            "No automatic driver downloads",
            "Stops Windows fetching drivers and device software for new hardware by itself.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("New hardware needs drivers installed by hand.")
        .admin()
        .details("Useful if you want to keep a particular driver, such as your graphics driver. Windows stops looking online when hardware is connected, so you install new drivers yourself."),
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
            "No Windows Update drivers",
            "Stops Windows Update replacing your drivers during monthly updates.",
            POLICY_DOCS,
            Documented,
        )
        .medium()
        .warn("You update drivers yourself.")
        .admin()
        .details("Helps if updates keep replacing a driver you chose, such as a graphics driver. Windows security updates are not affected. You keep your drivers up to date yourself."),
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
            "privacy.vs-sqm",
            Privacy,
            "System reports",
            "No Visual Studio feedback",
            "Takes Visual Studio out of its program for collecting usage data.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Only relevant if Visual Studio is installed. This is the documented opt-out and it covers several versions. The editor works the same."),
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
            "privacy.ad-id-policy",
            Privacy,
            "Advertising and suggestions",
            "Block the advertising ID",
            "Turns the advertising ID off for every account on this PC and locks the setting.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Apps can't use your advertising ID to link your activity for targeted ads. As a policy, it applies to all users and greys out the switch in Settings. You will still see ads, just less targeted ones."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
            "DisabledByGroupPolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.location-consent",
            Privacy,
            "App permissions",
            "Turn off location",
            "Turns off location for the whole PC, the same as the main switch in Settings.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Maps, weather and Find my device lose your location.")
        .admin()
        .details("No app or Windows feature can ask for your location, including desktop programs that use Windows location services. Weather, maps and automatic time zone will need manual input."),
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
];
