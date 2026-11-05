//! Tweaks that left the catalogue in the 0.5 audit (`docs/VISUALS_SECTION_AUDIT.md`).
//!
//! Nothing here is offered, listed or planned. An entry stays because a release may have applied
//! it: the journal names the tweak, and undo and the broker need its settings. Don't delete one
//! until no supported upgrade path can still have its change on a PC.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "apps.edge-site-info",
            Apps,
            "Edge",
            "Edge site info",
            "Stops sending pages visited.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SendSiteInfoToImproveServices",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-feedback",
            Apps,
            "Edge",
            "Edge feedback",
            "Turns off feedback.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "UserFeedbackAllowed",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-copilot-page",
            Apps,
            "Edge",
            "Edge Copilot page access",
            "Stops Copilot reading pages.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "CopilotPageContext", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "CopilotCDPPageContext",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "DiscoverPageContextEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-bing-chat",
            Apps,
            "Edge",
            "Edge new tab Copilot",
            "Removes Copilot from new tabs.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageBingChatEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-collections",
            Apps,
            "Edge",
            "Edge Collections",
            "Turns off Collections.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeCollectionsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-discover",
            Apps,
            "Edge",
            "Edge Discover",
            "Turns off Discover.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeDiscoverEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-enhance-images",
            Apps,
            "Edge",
            "Edge image enhancement",
            "Stops sending images to enhance.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeEnhanceImagesEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-follow",
            Apps,
            "Edge",
            "Edge Follow",
            "Turns off Follow.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "EdgeFollowEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-games",
            Apps,
            "Edge",
            "Edge games menu",
            "Hides games.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AllowGamesMenu",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-family",
            Apps,
            "Edge",
            "Edge family safety",
            "Turns off family safety.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "FamilySafetySettingsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-insider",
            Apps,
            "Edge",
            "Edge insider promotion",
            "Hides insider promotions.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "MicrosoftEdgeInsiderPromotionEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-spotlight",
            Apps,
            "Edge",
            "Edge Spotlight",
            "Stops Edge Spotlight ideas.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SpotlightExperiencesAndRecommendationsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-signin-ntp",
            Apps,
            "Edge",
            "Edge sign-in prompt",
            "Hides sign-in on new tabs.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SignInCtaOnNtpEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-widget",
            Apps,
            "Edge",
            "Edge desktop widget",
            "Turns off the web widget.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "WebWidgetAllowed", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "WebWidgetIsEnabledOnStartup",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-sidebar",
            Apps,
            "Edge",
            "Edge standalone sidebar",
            "Turns off the sidebar.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "StandaloneHubsSidebarEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-ntp-bg",
            Apps,
            "Edge",
            "Edge new tab background",
            "Uses a plain background.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageAllowedBackgroundTypes",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-quick-links",
            Apps,
            "Edge",
            "Edge quick links",
            "Hides quick links.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageQuickLinksEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-bing-ads",
            Apps,
            "Edge",
            "Edge Bing ads",
            "Suppresses Bing ads.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "BingAdsSuppression",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-related-matches",
            Apps,
            "Edge",
            "Edge related matches",
            "Stops related matches.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "RelatedMatchesCloudServiceEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-address-bar",
            Apps,
            "Edge",
            "Edge Microsoft Search",
            "Stops Microsoft Search in the address bar.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AddressBarMicrosoftSearchInBingProviderEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-autofill-address",
            Apps,
            "Edge",
            "Edge address autofill",
            "Turns off address autofill.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AutofillAddressEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-payment-query",
            Apps,
            "Edge",
            "Edge payment query",
            "Stops sites checking saved payments.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PaymentMethodQueryEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-dnt",
            Apps,
            "Edge",
            "Edge Do Not Track",
            "Sends Do Not Track.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ConfigureDoNotTrack",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-site-safety",
            Apps,
            "Edge",
            "Edge site safety",
            "Stops site safety lookups.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SiteSafetyServicesEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-spellcheck",
            Apps,
            "Edge",
            "Edge spellcheck",
            "Turns off spellcheck.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SpellcheckEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-in-app-support",
            Apps,
            "Edge",
            "Edge in-app support",
            "Turns off in-app support.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "InAppSupportEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-acrobat",
            Apps,
            "Edge",
            "Edge Acrobat button",
            "Hides the Acrobat button.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowAcrobatSubscriptionButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-update-window",
            Apps,
            "Edge",
            "Edge update hours",
            "Blocks Edge updates for 24 hours a day.",
            EDGE_DOCS,
            Documented,
        )
        .high()
        .warn("Edge stops getting security fixes.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "UpdatesSuppressedDurationMin",
                1440,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "UpdatesSuppressedStartHour",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "UpdatesSuppressedStartMin",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-update-period",
            Apps,
            "Edge",
            "Edge update checks",
            "Stops periodic Edge update checks.",
            EDGE_DOCS,
            Documented,
        )
        .high()
        .warn("Edge stops getting security fixes.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "AutoUpdateCheckPeriodMinutes",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "UpdateDefault",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-install-default",
            Apps,
            "Edge",
            "Edge install block",
            "Blocks Edge being reinstalled.",
            EDGE_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "InstallDefault",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                "Install{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-prelaunch",
            Apps,
            "Edge",
            "Legacy Edge prelaunch",
            "Stops prelaunch.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main",
            "AllowPrelaunch",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-tab-preload",
            Apps,
            "Edge",
            "Legacy Edge tab preload",
            "Stops tab preloading.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader",
            "AllowTabPreloading",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-do-not-upgrade",
            Apps,
            "Edge",
            "Chromium Edge upgrade",
            "Blocks the legacy upgrade.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\EdgeUpdate",
            "DoNotUpdateToEdgeWithChromium",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-feedback",
            Apps,
            "Chrome",
            "Chrome feedback",
            "Turns off feedback.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "UserFeedbackAllowed",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-privacy-sandbox",
            Apps,
            "Chrome",
            "Chrome Privacy Sandbox",
            "Turns off ad topics and measurement.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "PrivacySandboxAdMeasurementEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "PrivacySandboxAdTopicsEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "PrivacySandboxSiteEnabledAdsEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "ThirdPartyStoragePartitioningEnabled",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-lens",
            Apps,
            "Chrome",
            "Chrome Lens",
            "Turns off Lens.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "LensRegionSearchEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "AIGenerativeGoogleLensFeatureEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "GoogleSearchSidePanelEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-ai-search",
            Apps,
            "Chrome",
            "Chrome AI search",
            "Turns off AI search.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "AIGenerativeGoogleSearchFeatureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-media",
            Apps,
            "Chrome",
            "Chrome camera and mic",
            "Blocks media requests.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .warn("Video calls in Chrome stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultMediaStreamSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-sensors",
            Apps,
            "Chrome",
            "Chrome sensors",
            "Blocks sensors.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultSensorsSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-bluetooth",
            Apps,
            "Chrome",
            "Chrome Web Bluetooth",
            "Blocks Web Bluetooth.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultWebBluetoothGuardSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-usb",
            Apps,
            "Chrome",
            "Chrome Web USB",
            "Blocks Web USB.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultWebUsbGuardSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-translate",
            Apps,
            "Chrome",
            "Chrome translate",
            "Turns off translate.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "TranslateEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-autofill-address",
            Apps,
            "Chrome",
            "Chrome address autofill",
            "Turns off address autofill.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "AutofillAddressEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-dns",
            Apps,
            "Chrome",
            "Chrome built-in DNS",
            "Uses Windows DNS.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "BuiltInDnsClientEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-doh",
            Apps,
            "Chrome",
            "Chrome DoH",
            "Turns off Chrome's DNS over HTTPS.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Policies\Google\Chrome",
            "DnsOverHttpsMode",
            "off",
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-error-pages",
            Apps,
            "Chrome",
            "Chrome error pages",
            "Stops web lookups for typos.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "AlternateErrorPagesEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-ad-block",
            Apps,
            "Chrome",
            "Chrome ad blocking",
            "Blocks ads on abusive sites.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "AdBlockingModeEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-cleanup",
            Apps,
            "Chrome",
            "Chrome Cleanup",
            "Turns off Chrome Cleanup.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "ChromeCleanupEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "ChromeCleanupReportingEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-cloud-reporting",
            Apps,
            "Chrome",
            "Chrome cloud reporting",
            "Stops cloud reporting.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "CloudReportingEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Policies\Google\Chrome",
                "CloudReportingEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "CloudManagementEnrollmentMandatory",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-cloud-print",
            Apps,
            "Chrome",
            "Chrome cloud print",
            "Stops the print proxy.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "CloudPrintProxyEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-updates",
            Apps,
            "Chrome",
            "Chrome update checks",
            "Stops Chrome updating.",
            CHROME_DOCS,
            Documented,
        )
        .high()
        .warn("Chrome stops getting security fixes.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Update",
                "AutoUpdateCheckPeriodMinutes",
                0,
            ),
            hklm(r"SOFTWARE\Policies\Google\Update", "UpdateDefault", 0),
            hklm(
                r"SOFTWARE\Policies\Google\Update",
                "Update{8A69D345-D564-463C-AFF1-A69D9E530F96}",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-components",
            Apps,
            "Chrome",
            "Chrome component updates",
            "Stops component updates.",
            CHROME_DOCS,
            Documented,
        )
        .high()
        .warn("Widevine, safety lists and other parts stop updating.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "ComponentUpdatesEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-crash",
            Apps,
            "Firefox",
            "Firefox crash reporter",
            "Turns off the crash reporter.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableCrashReporter",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-feedback",
            Apps,
            "Firefox",
            "Firefox feedback",
            "Hides feedback.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableFeedbackCommands",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-accounts",
            Apps,
            "Firefox",
            "Firefox accounts",
            "Turns off Firefox accounts.",
            FIREFOX_DOCS,
            Documented,
        )
        .medium()
        .warn("Firefox sync stops working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableFirefoxAccounts",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-pocket",
            Apps,
            "Firefox",
            "Pocket",
            "Turns off Pocket.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisablePocket",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-private-shortcut",
            Apps,
            "Firefox",
            "Private browsing shortcut",
            "Hides the shortcut.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisablePrivateBrowsingShortcut",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-import",
            Apps,
            "Firefox",
            "Firefox profile import",
            "Turns off import.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableProfileImport",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-refresh",
            Apps,
            "Firefox",
            "Firefox profile refresh",
            "Turns off refresh.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableProfileRefresh",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-password-reveal",
            Apps,
            "Firefox",
            "Firefox password reveal",
            "Hides the reveal option.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisablePasswordReveal",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-recommendations",
            Apps,
            "Firefox",
            "Extension recommendations",
            "Turns them off.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "ExtensionRecommendations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-home",
            Apps,
            "Firefox",
            "Firefox home",
            "Clears the home policy.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(r"SOFTWARE\Policies\Mozilla\Firefox", "FirefoxHome", 0)],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-messaging",
            Apps,
            "Firefox",
            "Firefox messaging",
            "Stops product messages.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "UserMessaging",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-captive",
            Apps,
            "Firefox",
            "Captive portal check",
            "Stops the check.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "CaptivePortal",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-doh",
            Apps,
            "Firefox",
            "Firefox DoH",
            "Turns off DNS over HTTPS.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox\DNSOverHTTPS",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-prediction",
            Apps,
            "Firefox",
            "Firefox network prediction",
            "Stops preloading.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox\NetworkPrediction",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-updates",
            Apps,
            "Firefox",
            "Firefox updates",
            "Stops Firefox updating.",
            FIREFOX_DOCS,
            Documented,
        )
        .high()
        .warn("Firefox stops getting security fixes.")
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Mozilla\Firefox", "DisableAppUpdate", 1),
            hklm(
                r"SOFTWARE\Policies\Mozilla\Firefox",
                "ManualAppUpdateOnly",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-addons-update",
            Apps,
            "Firefox",
            "Firefox system add-on updates",
            "Stops system add-on updates.",
            FIREFOX_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableSystemAddonUpdate",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.brave-metrics",
            Apps,
            "Brave, Opera and Vivaldi",
            "Brave metrics",
            "Stops usage reports.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "MetricsReportingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.brave-hw",
            Apps,
            "Brave, Opera and Vivaldi",
            "Brave hardware acceleration",
            "Keeps hardware acceleration on.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "HardwareAccelerationModeEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.brave-reports",
            Apps,
            "Brave, Opera and Vivaldi",
            "Brave Safe Browsing reports",
            "Stops extended reports.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "SafeBrowsingExtendedReportingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.brave-spellcheck",
            Apps,
            "Brave, Opera and Vivaldi",
            "Brave spellcheck",
            "Stops online spellcheck.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "SpellCheckServiceEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.opera-background",
            Apps,
            "Brave, Opera and Vivaldi",
            "Opera GX background mode",
            "Stops Opera running after close.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Opera Software\Opera GX Stable",
            "BackgroundModeEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.opera-metrics",
            Apps,
            "Brave, Opera and Vivaldi",
            "Opera GX metrics",
            "Stops usage reports.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Opera Software\Opera GX Stable",
                "MetricsReportingEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Opera Software\Opera GX Stable",
                "UserFeedbackAllowed",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.opera-hw",
            Apps,
            "Brave, Opera and Vivaldi",
            "Opera GX hardware acceleration",
            "Keeps hardware acceleration on.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Opera Software\Opera GX Stable",
            "HardwareAccelerationModeEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.opera-search",
            Apps,
            "Brave, Opera and Vivaldi",
            "Opera GX search provider",
            "Enables the default provider policy.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Opera Software\Opera GX Stable",
            "DefaultSearchProviderEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.vivaldi-metrics",
            Apps,
            "Brave, Opera and Vivaldi",
            "Vivaldi metrics",
            "Stops usage reports.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Vivaldi", "MetricsReportingEnabled", 0),
            hklm(r"SOFTWARE\Policies\Vivaldi", "UserFeedbackAllowed", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.vivaldi-prediction",
            Apps,
            "Brave, Opera and Vivaldi",
            "Vivaldi prediction",
            "Stops preloading.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Vivaldi",
            "NetworkPredictionOptions",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.vivaldi-spellcheck",
            Apps,
            "Brave, Opera and Vivaldi",
            "Vivaldi spellcheck",
            "Stops online spellcheck.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Vivaldi",
            "SpellCheckServiceEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-verbose",
            Apps,
            "Office",
            "Office verbose logging",
            "Turns off verbose logs.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Common\ClientTelemetry",
                "VerboseLogging",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Common\ClientTelemetry",
                "VerboseLogging",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\Common\ClientTelemetry",
                "VerboseLogging",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-feedback",
            Apps,
            "Office",
            "Office feedback",
            "Turns off feedback.",
            BATLEZ,
            Documented,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Common\Feedback",
                "Enabled",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Common\Feedback",
                "Enabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-outlook-calendar",
            Apps,
            "Office",
            "Outlook calendar logging",
            "Turns off calendar logs.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Outlook\Options\Calendar",
                "EnableCalendarLogging",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Outlook\Options\Calendar",
                "EnableCalendarLogging",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-outlook-mail",
            Apps,
            "Office",
            "Outlook mail logging",
            "Turns off mail logs.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Outlook\Options\Mail",
                "EnableLogging",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Outlook\Options\Mail",
                "EnableLogging",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-word-logging",
            Apps,
            "Office",
            "Word logging",
            "Turns off Word logs.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Word\Options",
                "EnableLogging",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Word\Options",
                "EnableLogging",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.new-outlook-migrate",
            Apps,
            "Office",
            "New Outlook migration",
            "Stops the automatic switch.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Office\16.0\Outlook\Options\General",
            "DoNewOutlookAutoMigration",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.new-outlook-use",
            Apps,
            "Office",
            "Classic Outlook",
            "Stays on classic Outlook.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Office\16.0\Outlook\Preferences",
            "UseNewOutlook",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-pdfmaker",
            Apps,
            "Office",
            "Acrobat Office add-ins",
            "Stops PDFMaker loading in Office.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Office\Excel\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Office\Excel\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Office\PowerPoint\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Office\PowerPoint\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Office\Word\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Office\Word\Addins\PDFMaker.OfficeAddin",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Office\Outlook\AddIns\AdobeAcroOutlook.SendAsLink",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\Office\Outlook\AddIns\AdobeAcroOutlook.SendAsLink",
                "LoadBehavior",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-vs-adaptors",
            Apps,
            "Office",
            "Visual Studio Office add-ins",
            "Stops adapter add-ins.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Office\Excel\Addins\VS10ExcelAdaptor",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Office\Excel\Addins\VS10ExcelAdaptor",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Office\Word\Addins\VS10WordAdaptor",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Office\Word\Addins\VS10WordAdaptor",
                "LoadBehavior",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-tfc",
            Apps,
            "Office",
            "Team Foundation add-ins",
            "Stops TFS add-ins.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Office\Excel\Addins\TFCOfficeShim.Connect.16",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\Office\Excel\Addins\TFCOfficeShim.Connect.16",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Office\Excel\Addins\TFCOfficeShim.Connect.3",
                "LoadBehavior",
                0,
            ),
            hklm(
                r"SOFTWARE\Wow6432Node\Microsoft\Office\Excel\Addins\TFCOfficeShim.Connect.3",
                "LoadBehavior",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.onedrive-save",
            Apps,
            "Other apps",
            "Save to OneDrive default",
            "Stops saving there by default.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "DisableLibrariesDefaultSaveToOneDrive",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.onedrive-metered",
            Apps,
            "Other apps",
            "OneDrive on metered",
            "Stops syncing on metered networks.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
            "DisableMeteredNetworkFileSync",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.onedrive-personal",
            Apps,
            "Other apps",
            "OneDrive personal sync",
            "Turns off personal sync.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\OneDrive",
            "DisablePersonalSync",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.store-auto-update",
            Apps,
            "Other apps",
            "Store app auto updates",
            "Stops Store apps updating.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("Store apps stop getting fixes.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate",
                "AutoDownload",
                2,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\WindowsStore",
                "AutoDownload",
                2,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.store-remove",
            Apps,
            "Other apps",
            "Microsoft Store",
            "Blocks the Store.",
            BATLEZ,
            Documented,
        )
        .high()
        .warn("The Store and its app updates stop working.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\WindowsStore",
            "RemoveWindowsStore",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.adobe-optout",
            Apps,
            "Other apps",
            "Adobe telemetry",
            "Opts out of Adobe data.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Adobe\Adobe Desktop Common\ADS",
            "OptOut",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.adobe-rum",
            Apps,
            "Other apps",
            "Adobe update manager",
            "Opts out of remote updates.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Adobe\Adobe Desktop Common\RemoteUpdateManager",
            "RemoteUpdateManagerOptin",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.ccleaner-monitoring",
            Apps,
            "Other apps",
            "CCleaner monitoring",
            "Turns off active monitoring.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(r"Software\Piriform\CCleaner", "Monitoring", 0),
            hkcu(r"Software\Piriform\CCleaner", "SystemMonitoring", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.ccleaner-updates",
            Apps,
            "Other apps",
            "CCleaner update checks",
            "Turns off update checks.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(r"Software\Piriform\CCleaner", "UpdateAuto", 0),
            hkcu(r"Software\Piriform\CCleaner", "UpdateBackground", 0),
            hkcu(r"Software\Piriform\CCleaner", "UpdateCheck", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.ccleaner-telemetry",
            Apps,
            "Other apps",
            "CCleaner data",
            "Stops usage data.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Piriform\CCleaner",
            "HelpImproveCCleaner",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.ccleaner-offers",
            Apps,
            "Other apps",
            "CCleaner offers",
            "Turns off offers.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(r"Software\Piriform\CCleaner", "CheckTrialOffer", 0),
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)GetIpmForTrial", 0),
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)QuickCleanIpm", 0),
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)SoftwareUpdaterIpm", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.ccleaner-health",
            Apps,
            "Other apps",
            "CCleaner Health Check",
            "Turns off Health Check.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)HealthCheck", 0),
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)QuickClean", 0),
            hkcu(r"Software\Piriform\CCleaner", "(Cfg)SoftwareUpdater", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.app-installer-exp",
            Apps,
            "Other apps",
            "App Installer experiments",
            "Turns on App Installer features.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
            "EnableExperimentalFeatures",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.app-installer-source",
            Apps,
            "Other apps",
            "App Installer sources",
            "Enables winget sources.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
                "EnableAppInstaller",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
                "EnableDefaultSource",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
                "EnableMicrosoftStoreSource",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
                "EnableMSAppInstallerProtocol",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\AppInstaller",
                "EnableSettings",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "AC",
            ),
            "cpu.boost-policy",
            Cpu,
            "Power plan",
            "Boost policy",
            "Raises how much the CPU may boost.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59",
                "DC",
                60,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "AC",
            ),
            "cpu.max-state",
            Cpu,
            "Power plan",
            "Maximum processor state",
            "Lets the CPU reach full speed.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\bc5038f7-23e0-4960-96da-33abaf5935ec",
                "DC",
                90,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "AC",
            ),
            "cpu.idle-disable",
            Cpu,
            "Power plan",
            "Processor idle disable",
            "Stops C-states so the CPU never sleeps.",
            POWER_DOCS,
            Documented,
        )
        .high()
        .impact(Medium)
        .desktop()
        .warn("The CPU runs hot and draws full idle power.")
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "AC",
                1,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\5d76a2ca-e8c0-402f-a133-2158492d58ad",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\94d3a615-a899-4ac5-ae2b-e4d8f634367f",
                "AC",
            ),
            "cpu.cooling-active",
            Cpu,
            "Power plan",
            "Active cooling policy",
            "Speeds up fans before slowing the CPU.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
        settings: &[power(
            r"54533251-82be-4824-96c1-47b60b740d00\94d3a615-a899-4ac5-ae2b-e4d8f634367f",
            "AC",
            1,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "AC",
            ),
            "cpu.parking-max",
            Cpu,
            "Power plan",
            "Core parking max cores",
            "Allows every core to stay awake.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "AC",
                100,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\ea062031-0e34-4ff1-9b6d-eb1059334028",
                "DC",
                100,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "AC",
            ),
            "cpu.idle-demote",
            Cpu,
            "Power plan",
            "Idle demote threshold",
            "Moves into deep idle less readily.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "AC",
                5,
            ),
            power(
                r"54533251-82be-4824-96c1-47b60b740d00\4b92d758-5a24-4851-a470-815d78aee119",
                "DC",
                30,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "AC",
            ),
            "cpu.interrupt-steering",
            Cpu,
            "Power plan",
            "Interrupt steering mode",
            "Changes how interrupts are shared across cores.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[
            power(
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "AC",
                3,
            ),
            power(
                r"48672f38-7a9a-4bb2-8bf8-3d85be19de4e\2bfc24f9-5ea2-4801-8213-3dbae01aa39d",
                "DC",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-watchdog",
            Cpu,
            "Scheduler and timers",
            "DPC watchdog period",
            "Removes the DPC watchdog limit.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcWatchdogPeriod",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-timeout",
            Cpu,
            "Scheduler and timers",
            "DPC timeout",
            "Removes the per-DPC time limit.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcTimeout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dpc-watchdog-offset",
            Cpu,
            "Scheduler and timers",
            "DPC watchdog offset",
            "Removes the watchdog profile offset.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DpcWatchdogProfileOffset",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.ideal-dpc-rate",
            Cpu,
            "Scheduler and timers",
            "Ideal DPC rate",
            "Tunes when DPCs run.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "IdealDpcRate",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.minimum-dpc-rate",
            Cpu,
            "Scheduler and timers",
            "Minimum DPC rate",
            "Tunes how often DPCs drain.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MinimumDpcRate",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.threaded-dpc",
            Cpu,
            "Scheduler and timers",
            "Threaded DPCs",
            "Runs DPCs at normal interrupt level.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "ThreadDpcEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.serialize-timer",
            Cpu,
            "Scheduler and timers",
            "Serialize timer expiration",
            "Runs timers on one core.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "SerializeTimerExpiration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.low-qos-timer",
            Cpu,
            "Scheduler and timers",
            "Low QoS timer resolution",
            "Stops Windows slowing timers for background apps.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "DisableLowQosTimerResolution",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.ready-queue",
            Cpu,
            "Scheduler and timers",
            "Shared ready queue size",
            "Changes the size of the scheduler's ready queue.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MaximumSharedReadyQueueSize",
            128,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.dynamic-tick-duration",
            Cpu,
            "Scheduler and timers",
            "Dynamic tick duration",
            "Shortens how long the tick can stop.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "MaxDynamicTickDuration",
            10,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.per-cpu-clock",
            Cpu,
            "Scheduler and timers",
            "Per-CPU clock ticks",
            "Changes how clock ticks are scheduled.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "EnablePerCpuClockTickScheduling",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.timer-check",
            Cpu,
            "Scheduler and timers",
            "Timer check flags",
            "Turns off extra timer checks.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "TimerCheckFlags",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.debug-poll",
            Cpu,
            "Scheduler and timers",
            "Debug poll interval",
            "Lengthens kernel debugger polling.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
            "DebugPollInterval",
            1000,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.coalescing",
            Cpu,
            "Scheduler and timers",
            "Timer coalescing",
            "Stops Windows batching timers to save power.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\ModernSleep",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Kernel",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Executive",
                "CoalescingTimerInterval",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                "CoalescingTimerInterval",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "cpu.split-large-caches",
            Cpu,
            "Scheduler and timers",
            "Split large caches",
            "Treats large L3 caches as separate groups.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
            "SplitLargeCaches",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.adjust-dpc",
            Cpu,
            "Scheduler and timers",
            "DPC threshold",
            "Tunes when DPC work is rebalanced.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "AdjustDpcThreshold",
            800,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.deep-io-coalescing",
            Cpu,
            "Scheduler and timers",
            "Deep I/O coalescing",
            "Groups I/O completions in deep idle.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "DeepIoCoalescingEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.scheduler-assist",
            Cpu,
            "Scheduler and timers",
            "Scheduler assist override",
            "Overrides a scheduler hint flag.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "SchedulerAssistThreadFlagOverride",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.priority-dpc",
            Cpu,
            "Scheduler and timers",
            "Ideal DPC rate (priority key)",
            "Same idea, under PriorityControl.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\PriorityControl",
            "IdealDpcRate",
            800,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.passive-int",
            Cpu,
            "Scheduler and timers",
            "Passive interrupt priority",
            "Raises the priority of passive interrupt workers.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\I/O System",
            "PassiveIntRealTimeWorkerPriority",
            18,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.tick-power",
            Cpu,
            "Scheduler and timers",
            "Exit latency check",
            "Turns on the power exit latency check.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "ExitLatencyCheckEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.exit-latency",
            Cpu,
            "Scheduler and timers",
            "Exit latency",
            "Lowers the assumed idle exit latency.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "ExitLatency",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.unpark-count",
            Cpu,
            "Scheduler and timers",
            "Initial unpark count",
            "Starts Class 1 cores unparked.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "Class1InitialUnparkCount",
            100,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.energy-estimation",
            Cpu,
            "Scheduler and timers",
            "Energy estimation",
            "Stops Windows estimating power per app.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "EnergyEstimationEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.event-processor",
            Cpu,
            "Scheduler and timers",
            "Power event processor",
            "Stops the power event logger.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "EventProcessorEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "cpu.tagged-energy",
            Cpu,
            "Scheduler and timers",
            "Tagged energy logging",
            "Stops per-app energy logs.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "DisableTaggedEnergyLogging",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "TelemetryMaxApplication",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Power\EnergyEstimation\TaggedEnergy",
                "TelemetryMaxTagPerApplication",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-queue",
            Gaming,
            "Input",
            "Mouse data queue",
            "Sets the mouse driver's buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Mice with high polling rates can drop input.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters",
            "MouseDataQueueSize",
            20,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.keyboard-queue",
            Gaming,
            "Input",
            "Keyboard data queue",
            "Sets the keyboard driver's buffer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Fast typing can drop keys.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters",
            "KeyboardDataQueueSize",
            20,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-sensitivity",
            Gaming,
            "Input",
            "Mouse sensitivity",
            "Sets the Windows pointer speed to 6 of 11.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseSensitivity", "10")],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-trails",
            Gaming,
            "Input",
            "Mouse trails",
            "Turns off pointer trails.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseTrails", "0")],
    },
    SettingsTweak {
        meta: lib(
            "gaming.keyboard-response",
            Gaming,
            "Input",
            "Keyboard response",
            "Sets faster repeat in the accessibility layer.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "AutoRepeatDelay",
                "200",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "AutoRepeatRate",
                "6",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "BounceTime",
                "0",
            ),
            hkcu_str(
                r"Control Panel\Accessibility\Keyboard Response",
                "DelayBeforeAcceptance",
                "0",
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last BounceKey Setting",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Delay",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Repeat",
                0,
            ),
            hkcu(
                r"Control Panel\Accessibility\Keyboard Response",
                "Last Valid Wait",
                1000,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-keys",
            Gaming,
            "Input",
            "Mouse Keys shortcut",
            "Stops the Mouse Keys shortcut.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Accessibility\MouseKeys",
            "Flags",
            "38",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.touch-pointer",
            Gaming,
            "Input",
            "Absolute pointer",
            "Treats absolute pointers as absolute.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\mouhid\Parameters",
                "TreatAbsolutePointerAsAbsolute",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\mouhid\Parameters",
                "TreatAbsoluteAsRelative",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-bar-link",
            Gaming,
            "Game Bar and capture",
            "Game Bar links",
            "Stops ms-gamebar pop-ups after removing Game Bar.",
            ALCHEMY,
            Community,
        ),
        settings: &[
            hkcu_str(r"Software\Classes\ms-gamebar", "", "URL:ms-gamebar"),
            hkcu_str(r"Software\Classes\ms-gamebar", "URL Protocol", ""),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audio-capture",
            Gaming,
            "Game Bar and capture",
            "Game audio capture",
            "Stops capturing audio with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "AudioCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cursor-capture",
            Gaming,
            "Game Bar and capture",
            "Game cursor capture",
            "Stops capturing the cursor with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "CursorCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mic-capture",
            Gaming,
            "Game Bar and capture",
            "Game microphone capture",
            "Stops capturing the microphone with clips.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
            "MicrophoneCaptureEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-audio",
            Gaming,
            "Audio",
            "MMCSS audio priority",
            "Raises the audio task's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Priority",
                6,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Clock Rate",
                10000,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Scheduling Category",
                "High",
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Background Only",
                "False",
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
                "Affinity",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-pro-audio",
            Gaming,
            "Audio",
            "MMCSS pro audio",
            "Tunes the Pro Audio task.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Pro Audio",
                "Priority",
                1,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Pro Audio",
                "Scheduling Category",
                "High",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-display",
            Gaming,
            "Audio",
            "MMCSS display",
            "Tunes the display post-processing task.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\DisplayPostProcessing",
                "Priority",
                8,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\DisplayPostProcessing",
                "Scheduling Category",
                "High",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-games-extra",
            Gaming,
            "Audio",
            "MMCSS games extras",
            "Sets background and latency flags for games.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Affinity",
                0,
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Background Only",
                "False",
            ),
            hklm_str(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
                "Latency Sensitive",
                "True",
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-lazy",
            Gaming,
            "Audio",
            "MMCSS lazy mode",
            "Sets the MMCSS idle timeout.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            "LazyModeTimeout",
            10000,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.audiodg-priority",
            Gaming,
            "Audio",
            "Audio engine priority",
            "Raises Windows audio engine priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\audiodg.exe\PerfOptions",
                "CpuPriorityClass",
                3,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\audiodg.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.nahimic-none",
            Gaming,
            "Audio",
            "Realtek power management",
            "Keeps Realtek audio out of power saving.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Realtek\Audio\RtkNGUI64\PowerMgnt",
            "enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-be-priority",
            Gaming,
            "Per-game priority",
            "Fortnite BattlEye priority",
            "Starts the BattlEye build at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping_BE.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-eac-priority",
            Gaming,
            "Per-game priority",
            "Fortnite EAC priority",
            "Starts the EAC build at high priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FortniteClient-Win64-Shipping_EAC.exe\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.obs-priority",
            Gaming,
            "Per-game priority",
            "OBS above normal",
            "Starts OBS above normal priority.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\obs64.exe\PerfOptions",
            "CpuPriorityClass",
            6,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.obs32-priority",
            Gaming,
            "Per-game priority",
            "OBS 32-bit above normal",
            "Starts 32-bit OBS above normal.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\obs32.exe\PerfOptions",
            "CpuPriorityClass",
            6,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.dwm-priority",
            Gaming,
            "Per-game priority",
            "Desktop compositor priority",
            "Raises DWM's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "IoPriority",
                3,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\dwm.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.csrss-priority",
            Gaming,
            "Per-game priority",
            "Client server priority",
            "Raises CSRSS's priority.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "CpuPriorityClass",
                4,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "IoPriority",
                3,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cod-vsync",
            Gaming,
            "Game settings",
            "Modern Warfare V-Sync off",
            "Turns off V-Sync in the game's settings.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Activision\ModernWarfare",
            "VerticalSyncEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.cod-vram",
            Gaming,
            "Game settings",
            "Modern Warfare video memory",
            "Lowers its video memory scale.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Activision\ModernWarfare",
            "VideoMemoryScale",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-dx",
            Gaming,
            "Game settings",
            "Fortnite prefers D3D11",
            "Stops preferring D3D12.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
            "PreferD3D12",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.fortnite-low",
            Gaming,
            "Game settings",
            "Fortnite low quality",
            "Sets low quality in the identifiers key.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.AntiAliasingQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.EffectsQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.ShadowQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.TexturesQuality",
                0,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.ViewDistanceQuality",
                1,
            ),
            hkcu(
                r"Software\Epic Games\Unreal Engine\Identifiers\Fortnite",
                "sg.bSmoothFrameRate",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-overlay",
            Gaming,
            "Game settings",
            "Steam overlay",
            "Turns off the Steam in-game overlay.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "EnableOverlay", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-gpu-web",
            Gaming,
            "Game settings",
            "Steam web GPU acceleration",
            "Turns off GPU use in Steam's browser.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "GPUAccelWebViews", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-h264",
            Gaming,
            "Game settings",
            "Steam H.264 acceleration",
            "Turns off hardware video in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "H264HWAccel", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-smooth",
            Gaming,
            "Game settings",
            "Steam smooth scrolling",
            "Turns off smooth scrolling in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "SmoothScrollWebViews", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-dwrite",
            Gaming,
            "Game settings",
            "Steam DirectWrite",
            "Turns off DirectWrite in Steam.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "DWriteEnable", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-dpi",
            Gaming,
            "Game settings",
            "Steam DPI scaling",
            "Turns off DPI scaling in Steam.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "DPIScaling", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-silent",
            Gaming,
            "Game settings",
            "Steam silent start",
            "Starts Steam minimised.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "SilentStartup", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-updates",
            Gaming,
            "Game settings",
            "Steam auto update",
            "Stops Steam updating itself.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[hkcu(r"Software\Valve\Steam", "AutoUpdateEnabled", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-startup",
            Gaming,
            "Game settings",
            "Steam startup mode",
            "Opens Steam to the small window.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(r"Software\Valve\Steam", "StartupMode", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gaming.steam-tcp",
            Gaming,
            "Game settings",
            "Steam TCP settings",
            "Sets Steam's TCP values.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"Software\Valve\Steam", "TcpAckFrequency", 1),
            hkcu(r"Software\Valve\Steam", "TcpNoDelay", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.riot-updates",
            Gaming,
            "Game settings",
            "Riot auto update",
            "Stops Riot Client updating games at launch.",
            BATLEZ,
            Community,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Riot Games\Riot Client",
            "autoUpdateOnLaunch",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.game-dvr-user",
            Gaming,
            "Game Bar and capture",
            "Background recording",
            "Turns off Game DVR for your account.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"System\GameConfigStore", "GameDVR_Enabled", 0),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                "AppCaptureEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mmcss-sfio",
            Gaming,
            "Game settings",
            "Games I/O priority",
            "Raises the MMCSS I/O priority of games.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
            "SFIO Priority",
            "High",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.overlays",
            Gpu,
            "Windows graphics",
            "Hardware overlays",
            "Turns off driver overlays.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "DisableOverlays",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.miracast",
            Gpu,
            "Windows graphics",
            "Miracast support",
            "Stops Miracast.",
            BATLEZ,
            Observed,
        )
        .warn("Wireless displays stop working.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "PlatformSupportMiracast",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.preemption",
            Gpu,
            "Windows graphics",
            "GPU preemption",
            "Turns off preemption in the scheduler.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
            "EnablePreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.vsync-idle",
            Gpu,
            "Windows graphics",
            "V-Sync idle timeout",
            "Removes the V-Sync idle delay.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
            "VsyncIdleTimeout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.monitor-latency",
            Gpu,
            "Windows graphics",
            "Monitor latency tolerance",
            "Lowers how much delay Windows allows.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "MonitorLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "MonitorLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
                "MonitorLatencyTolerance",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.monitor-refresh-latency",
            Gpu,
            "Windows graphics",
            "Refresh latency tolerance",
            "Lowers allowed refresh delay.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
                "MonitorRefreshLatencyTolerance",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.tdr-async",
            Gpu,
            "Windows graphics",
            "TDR async reset",
            "Resets the GPU without blocking.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\DXGKrnl",
            "TdrResetFromTimeoutAsync",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-independent-flip",
            Gpu,
            "Windows graphics",
            "Independent flip",
            "Keeps independent flip on.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableIndependentFlip",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-advanced-flip",
            Gpu,
            "Windows graphics",
            "Advanced direct flip",
            "Turns off advanced direct flip.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableAdvancedDirectFlip",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-prediction",
            Gpu,
            "Windows graphics",
            "Interaction prediction",
            "Turns off input output prediction.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "InteractionOutputPredictionDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-hibernate",
            Gpu,
            "Windows graphics",
            "Thumbnail hibernation",
            "Stops saving taskbar thumbnails.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\DWM",
            "AlwaysHibernateThumbnails",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-locking",
            Gpu,
            "Windows graphics",
            "DWM memory locking",
            "Stops DWM locking memory.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableLockingMemory",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-shadows",
            Gpu,
            "Windows graphics",
            "DWM projected shadows",
            "Turns off projected shadows.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "DisableProjectedShadows",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "DisableProjectedShadowsRendering",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-hologram",
            Gpu,
            "Windows graphics",
            "Holographic compositor",
            "Turns off the holographic compositor.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableHologramCompositor",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-drawlist",
            Gpu,
            "Windows graphics",
            "Draw list caching",
            "Turns off draw list caching.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "DisableDrawListCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-effect-cache",
            Gpu,
            "Windows graphics",
            "Effect caching",
            "Keeps effect caching on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableEffectCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-megarects",
            Gpu,
            "Windows graphics",
            "Mega rects",
            "Keeps mega rects on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableMegaRects",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-cpu-clipping",
            Gpu,
            "Windows graphics",
            "CPU clipping",
            "Keeps CPU clipping on.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnableCpuClipping",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-parallel",
            Gpu,
            "Windows graphics",
            "Parallel mode policy",
            "Sets DWM's parallel mode.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "ParallelModePolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-linear",
            Gpu,
            "Windows graphics",
            "Linear resampling",
            "Resamples in linear space.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\DWM",
                "ResampleInLinearSpace",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\Windows\DWM", "ResampleModeOverride", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-wet-ink",
            Gpu,
            "Windows graphics",
            "Super wet ink",
            "Turns on super wet ink.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "SuperWetEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-primitive",
            Gpu,
            "Windows graphics",
            "Primitive reordering",
            "Turns off primitive reordering.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "EnablePrimitiveReordering",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-debug",
            Gpu,
            "Windows graphics",
            "DWM debug fail fast",
            "Turns off debug fail-fast.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(r"SOFTWARE\Microsoft\Windows\DWM", "DebugFailFast", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-shift",
            Gpu,
            "Windows graphics",
            "Animations shift key",
            "Stops Shift slowing animations.",
            ALCHEMY,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\DWM",
            "AnimationsShiftKey",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.wpf-hw",
            Gpu,
            "Windows graphics",
            "WPF hardware acceleration",
            "Runs WPF apps in software.",
            BATLEZ,
            Observed,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Microsoft\Avalon.Graphics",
            "DisableHWAcceleration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.colour-calibration",
            Gpu,
            "Windows graphics",
            "Calibration loader",
            "Stops colour calibration updates.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows NT\CurrentVersion\ICM\Calibration",
            "CalibrationUpdateFlag",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.gamma-range",
            Gpu,
            "Windows graphics",
            "GDI gamma range",
            "Widens the colour gamma range.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\ICM",
            "GdiIcmGammaRange",
            256,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.tiff-vk-debug",
            Gpu,
            "Windows graphics",
            "Vulkan loader debug",
            "Silences Vulkan loader logging.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_str("Environment", "VK_LOADER_DEBUG", "none")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-powermizer",
            Gpu,
            "NVIDIA",
            "PowerMizer",
            "Turns off PowerMizer.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PowerMizerEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-perf-level",
            Gpu,
            "NVIDIA",
            "Performance level source",
            "Sets the performance source.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PerfLevelSrc",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-power-saving",
            Gpu,
            "NVIDIA",
            "Driver power saving",
            "Turns off power saving mode.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnablePowerSavingMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-display-power",
            Gpu,
            "NVIDIA",
            "Display power saving",
            "Turns off display power saving.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\NVTweak",
            "DisplayPowerSaving",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-link-state",
            Gpu,
            "NVIDIA",
            "Max link state",
            "Forces full PCIe link speed.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "ForceMaxPerfLinkState",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-dynamic-pstate",
            Gpu,
            "NVIDIA",
            "Dynamic P-state switch",
            "Turns off the driver's dynamic state.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDynamicPState",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-low-latency",
            Gpu,
            "NVIDIA",
            "Low latency boost",
            "Turns on the driver's boost.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "LowLatencyBoost",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-msi",
            Gpu,
            "NVIDIA",
            "NVIDIA MSI",
            "Turns on message-signalled interrupts.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableMSI",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-dma",
            Gpu,
            "NVIDIA",
            "DMA acceleration",
            "Turns on DMA acceleration.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDMAAcceleration",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-async-queue",
            Gpu,
            "NVIDIA",
            "Async command queue",
            "Turns on the async queue.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableAsyncCmdQueue",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-p2p",
            Gpu,
            "NVIDIA",
            "Peer-to-peer transfers",
            "Turns on P2P transfers.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableP2PTransfers",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-instrumentation",
            Gpu,
            "NVIDIA",
            "Driver instrumentation",
            "Turns off driver instrumentation.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableDriverInstrumentation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-event-tracing",
            Gpu,
            "NVIDIA",
            "Kernel event tracing",
            "Turns off driver event tracing.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EnableKernelEventTracing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-event-driven",
            Gpu,
            "NVIDIA",
            "Event-driven mode",
            "Sets the driver's event mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "EventDrivenMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-polling",
            Gpu,
            "NVIDIA",
            "Completion polling",
            "Polls for completion.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "PollForCompletion",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-thread-policy",
            Gpu,
            "NVIDIA",
            "Thread policy",
            "Sets the driver's thread policy.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
            "ThreadPolicy",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-telemetry-driver",
            Gpu,
            "NVIDIA",
            "Driver telemetry",
            "Stops telemetry in the driver.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global",
                "EnableNvTelemetry",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\Startup",
                "SendTelemetryData",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableTelemetry",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "NvTelemetryContainer",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-profile-telemetry",
            Gpu,
            "NVIDIA",
            "Profile telemetry",
            "Stops profile telemetry.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvProfile",
            "TelemetryEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-ansel",
            Gpu,
            "NVIDIA",
            "Ansel",
            "Turns off Ansel.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\Ansel",
            "AnselAllowed",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-freestyle",
            Gpu,
            "NVIDIA",
            "Freestyle",
            "Turns off game filters.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\GeForceExperience",
            "EnableFreestyle",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rebar",
            Gpu,
            "NVIDIA",
            "Resizable BAR flag",
            "Sets the driver's Resizable BAR flag.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvDriver",
            "ResizableBar",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-camera",
            Gpu,
            "NVIDIA",
            "Nvfcamera",
            "Turns off the NVIDIA capture camera.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\NvfCamera",
            "Enable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-perf-sdk",
            Gpu,
            "NVIDIA",
            "Performance SDK",
            "Turns off the performance SDK.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\NVIDIA Corporation\Global\PerformanceSdk",
            "EnablePerfSdk",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-stereo",
            Gpu,
            "NVIDIA",
            "Stereo 3D",
            "Turns off 3D Vision.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\NVIDIA Corporation\Global\Stereo3D",
                "StereoEnable",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\Global\Stereo3D",
                "StereoDisplayMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-vsr",
            Gpu,
            "NVIDIA",
            "RTX Video Super Resolution",
            "Turns off video upscaling.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableRTXVideoSuperResolution",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableRTXVideoQualityEnhancement",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\NvVSR",
                "EnableSuperResolution",
                0,
            ),
            hklm(r"SOFTWARE\NVIDIA Corporation\NvVSR", "EnableVSR", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-gpudebugger",
            Gpu,
            "NVIDIA",
            "GPU debugger",
            "Turns off the GPU debugger hooks.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\NVIDIA Corporation\GPUDebugger", "DisableLoad", 1),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\GPUDebugger",
                "EnableInterface",
                0,
            ),
            hklm(
                r"SOFTWARE\NVIDIA Corporation\GPUDebugger",
                "EnableRemoteDebugger",
                0,
            ),
            hklm(r"SOFTWARE\NVIDIA Corporation\GPUDebugger", "LogMask", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rm-cache",
            Gpu,
            "NVIDIA",
            "RM registry caching",
            "Turns off RM registry caching.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "RmDisableRegistryCaching",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-l2-compression",
            Gpu,
            "NVIDIA",
            "L2 compression",
            "Turns off RM L2 compression.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
            "RMDisablePostL2Compression",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-per-core-dpc",
            Gpu,
            "NVIDIA",
            "Per-core DPC",
            "Runs NVIDIA DPCs on every core.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Power",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\NVAPI",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Global\NVTweak",
                "RmGpsPsEnablePerCpuCoreDpc",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-compute",
            Gpu,
            "NVIDIA",
            "Compute preemption",
            "Turns off compute preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "ComputePreemption",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption",
            Gpu,
            "NVIDIA",
            "Driver preemption",
            "Turns off driver preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "DisablePreemption",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnablePreemption",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-cuda",
            Gpu,
            "NVIDIA",
            "CUDA preemption",
            "Turns off CUDA preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "DisableCudaContextPreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-s3",
            Gpu,
            "NVIDIA",
            "Preemption on sleep",
            "Turns off preemption on sleep.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "DisablePreemptionOnS3S4",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-preemption-level",
            Gpu,
            "NVIDIA",
            "Preemption level",
            "Sets the GPU preemption level.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "GPUPreemptionLevel",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-context-isolation",
            Gpu,
            "NVIDIA",
            "Context isolation",
            "Turns off context isolation.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableContextIsolation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-reflex-driver",
            Gpu,
            "NVIDIA",
            "Driver low latency",
            "Turns on low latency mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableLowLatencyMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-async-compute",
            Gpu,
            "NVIDIA",
            "Async compute",
            "Turns on async compute.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableAsynchronousCompute",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-frame-reporting",
            Gpu,
            "NVIDIA",
            "Frame time reporting",
            "Turns off frame time reporting.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableFrameTimeReporting",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-debug-heap",
            Gpu,
            "NVIDIA",
            "GPU debug heap",
            "Turns off the debug heap.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableGPUDebugHeap",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-debug-layer",
            Gpu,
            "NVIDIA",
            "Driver debug layer",
            "Turns off the debug layer.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "EnableDebugLayer",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-logging",
            Gpu,
            "NVIDIA",
            "Driver logging",
            "Turns off driver logs.",
            BATLEZ,
            Community,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogErrorEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogErrorEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogEventEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogEventEntries",
                0,
            ),
            hklm(r"SYSTEM\CurrentControlSet\Services\nvlddmkm", "LogLevel", 0),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogPagingEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogPagingEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters",
                "LogWarningEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "LogWarningEntries",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "NVAPIDebugLevel",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\NVAPI",
                "NVAPIDebugLevel",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-hda-d3cold",
            Gpu,
            "NVIDIA",
            "HD Audio D3 cold",
            "Sets HD Audio low power state.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "EnableHDAudioD3Cold",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-contiguous",
            Gpu,
            "NVIDIA",
            "Contiguous system memory",
            "Prefers contiguous memory.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
            "PreferSystemMemoryContiguous",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-tdr",
            Gpu,
            "NVIDIA",
            "NVIDIA TDR values",
            "Sets the driver's own TDR values.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
                "TdrDelay",
                10,
            ),
            hklm(r"SYSTEM\CurrentControlSet\Services\nvlddmkm", "TdrLevel", 3),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-hdcp",
            Gpu,
            "NVIDIA",
            "HDCP key",
            "Clears the RM HDCP key flag.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .warn("Protected video may stop playing.")
        .admin(),
        settings: &[gpu("Nvidia", "RMHdcpKeyglobZero", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-ulps",
            Gpu,
            "AMD",
            "Ultra low power state",
            "Turns off ULPS.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .impact(Medium)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "EnableUlps", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-power-gating",
            Gpu,
            "AMD",
            "Power gating",
            "Turns off power gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "DisablePowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dmacopy",
            Gpu,
            "AMD",
            "DMA copy",
            "Turns off DMA copy.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableDMACopy", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-block-write",
            Gpu,
            "AMD",
            "Block write",
            "Turns off block write.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableBlockWrite", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-uvd-gating",
            Gpu,
            "AMD",
            "UVD power gating",
            "Turns off video decoder gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableUVDPowerGatingDynamic", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vce-gating",
            Gpu,
            "AMD",
            "VCE power gating",
            "Turns off video encoder gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableVCEPowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-drmdma-gating",
            Gpu,
            "AMD",
            "DRM DMA gating",
            "Turns off DRM DMA gating.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "DisableDrmdmaPowerGating", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-sclk-sleep",
            Gpu,
            "AMD",
            "Engine clock deep sleep",
            "Turns off deep sleep.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "PP_SclkDeepSleepDisable", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-thermal-throttle",
            Gpu,
            "AMD",
            "Thermal auto throttling",
            "Turns off auto throttling.",
            BATLEZ,
            Community,
        )
        .high()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .warn("The card relies on its own limits to avoid overheating.")
        .admin(),
        settings: &[gpu("Amd", "PP_ThermalAutoThrottlingEnable", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-gpu-down",
            Gpu,
            "AMD",
            "GPU power down",
            "Turns off GPU power down.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .desktop()
        .admin(),
        settings: &[gpu("Amd", "PP_GPUPowerDownEnabled", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-stutter",
            Gpu,
            "AMD",
            "Stutter mode",
            "Sets memory stutter mode.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "StutterMode", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-color-depth",
            Gpu,
            "AMD",
            "Auto colour depth",
            "Stops colour depth dropping.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "AutoColorDepthReduction_NA", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-rs-overlay",
            Gpu,
            "AMD",
            "Radeon overlay",
            "Turns off the Radeon overlay.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu_str("Amd", "AllowRSOverlay", "false")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-skins",
            Gpu,
            "AMD",
            "Radeon skins",
            "Turns off Radeon skins.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu_str("Amd", "AllowSkins", "false")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-snapshot",
            Gpu,
            "AMD",
            "Radeon snapshots",
            "Turns off Radeon snapshots.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu("Amd", "AllowSnapshot", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-subscription",
            Gpu,
            "AMD",
            "Radeon subscription",
            "Turns off Radeon subscription prompts.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[gpu("Amd", "AllowSubscription", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-cross-gpu",
            Gpu,
            "AMD",
            "Cross-GPU display",
            "Turns off cross-GPU display.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "KMD_EnableCrossGpuDisplaySupport", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-usage-tracking",
            Gpu,
            "AMD",
            "AMD usage tracking",
            "Stops AMD install tracking.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\AMD\Install", "UsageTracking", 0),
            hklm(r"SOFTWARE\WOW6432Node\AMD\Install", "UsageTracking", 0),
            hklm(r"SOFTWARE\AMD\Install", "AUEP", 0),
            hklm(r"SOFTWARE\WOW6432Node\AMD\Install", "AUEP", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-auto-update",
            Gpu,
            "AMD",
            "AMD auto update",
            "Stops AMD Software updating itself.",
            BATLEZ,
            Observed,
        )
        .hw(AmdGpu)
        .medium()
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\Install", "AutoUpdate", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dvr",
            Gpu,
            "AMD",
            "AMD recording",
            "Turns off AMD's recorder.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu),
        settings: &[
            hkcu(r"Software\AMD\DVR", "DvrEnabled", 0),
            hkcu(r"Software\AMD\DVR", "PerformanceMonitorOpacityWA", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-chill",
            Gpu,
            "AMD",
            "Radeon Chill",
            "Turns off Radeon Chill.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdwddmg",
            "ChillEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-rsr",
            Gpu,
            "AMD",
            "Radeon Super Resolution",
            "Turns off RSR.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableRSR",
                0,
            ),
            hklm(r"SOFTWARE\AMD\CN", "EnableRSR", 0),
            hklm(r"SOFTWARE\AMD\CN", "EnableRadeonSuperResolution", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-frtc",
            Gpu,
            "AMD",
            "Frame rate target",
            "Turns off Frame Rate Target Control.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdkmdag",
            "EnableFRTC",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-boost-cn",
            Gpu,
            "AMD",
            "Core Performance Boost",
            "Sets the CN boost flag.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "CorePerformanceBoost", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-power-scheme",
            Gpu,
            "AMD",
            "AMD power scheme",
            "Sets the CN power scheme.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "PowerScheme", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-fsr",
            Gpu,
            "AMD",
            "FidelityFX flag",
            "Turns off the CN FSR flag.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\AMD\CN",
            "EnableFidelityFXSuperResolution",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vrs",
            Gpu,
            "AMD",
            "Variable rate shading flag",
            "Turns off the CN VRS flag.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "EnableVariableRateShading", 0)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-apu-segment",
            Gpu,
            "AMD",
            "APU shared segment",
            "Sets the A+I shared segment.",
            ALCHEMY,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\services\amdkmdap",
                "KMD_APlusISharedMiniSegmentOptions",
                7,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\services\amdkmdap",
                "KMD_APlusISharedMiniSegmentSize",
                67108864,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-preemption",
            Gpu,
            "Intel",
            "Intel preemption",
            "Turns off Intel preemption.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx\Parameters",
            "EnablePreemption",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-msaa",
            Gpu,
            "Intel",
            "Intel MSAA",
            "Turns off Intel MSAA.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(r"SYSTEM\CurrentControlSet\Services\igfx", "MSAAEnable", 0),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\igfx",
                "PreRenderAALevel",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-raytracing",
            Gpu,
            "Intel",
            "Intel ray tracing",
            "Turns off Intel ray tracing.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "RayTracingEnable",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-scaling",
            Gpu,
            "Intel",
            "Intel dynamic scaling",
            "Turns on dynamic scaling.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "EnableDynamicScaling",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-sr",
            Gpu,
            "Intel",
            "Intel super resolution",
            "Turns on Intel super resolution.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "SuperResolutionEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.intel-xess",
            Gpu,
            "Intel",
            "Intel XeSS",
            "Turns on the XeSS driver.",
            BATLEZ,
            Community,
        )
        .hw(IntelGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\igfx",
            "XeSSDriverEnable",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-ltr",
            Gpu,
            "AMD",
            "Latency tolerance",
            "Sets the driver's PCIe latency tolerance values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "LTRMaxNoSnoopLatency", 1),
            gpu("Amd", "LTRNoSnoopL1Latency", 1),
            gpu("Amd", "LTRSnoopL0Latency", 1),
            gpu("Amd", "LTRSnoopL1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-bgm-ltr",
            Gpu,
            "AMD",
            "Background latency tolerance",
            "Sets the background PCIe latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "BGM_LTRMaxNoSnoopLatencyValue", 1),
            gpu("Amd", "BGM_LTRMaxSnoopLatencyValue", 1),
            gpu("Amd", "BGM_LTRNoSnoopL0Latency", 1),
            gpu("Amd", "BGM_LTRNoSnoopL1Latency", 1),
            gpu("Amd", "BGM_LTRSnoopL0Latency", 1),
            gpu("Amd", "BGM_LTRSnoopL1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dal-latency",
            Gpu,
            "AMD",
            "Display latency",
            "Sets the display engine's latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "DalNBLatencyForUnderFlow", 1),
            gpu("Amd", "DalUrgentLatencyNs", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-pp-latency",
            Gpu,
            "AMD",
            "PowerPlay latency",
            "Sets PowerPlay transition latency values.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            gpu("Amd", "PP_DGBMMMaxTransitionLatencyUvd", 1),
            gpu("Amd", "PP_DGBPMMaxTransitionLatencyGfx", 1),
            gpu("Amd", "PP_RTPMComputeF1Latency", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-compute-latency",
            Gpu,
            "AMD",
            "Compute latency",
            "Sets the kernel driver's compute latency.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "KMD_RpmComputeLatency", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-memclock-latency",
            Gpu,
            "AMD",
            "Memory clock switch",
            "Sets the memory clock switch latency.",
            BATLEZ,
            Community,
        )
        .medium()
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[gpu("Amd", "memClockSwitchLatency", 1)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-vsr",
            Gpu,
            "AMD",
            "Virtual Super Resolution",
            "Turns off Virtual Super Resolution.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "VSRSupportEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableVirtualSuperResolution",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dxr",
            Gpu,
            "AMD",
            "Ray tracing support",
            "Turns off ray tracing in the driver.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .warn("Games that need ray tracing won't start.")
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "DXRSupportEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EnableRayTracing",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-aa-flags",
            Gpu,
            "AMD",
            "Anti-aliasing flags",
            "Turns off driver anti-aliasing overrides.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "EQAAEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "MultiSampleAAEnable",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\amdkmdag",
                "OverrideAAEnable",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-dynamic-res",
            Gpu,
            "AMD",
            "Dynamic resolution",
            "Turns off the driver's dynamic resolution.",
            BATLEZ,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\amdkmdag",
            "AllowDynamicResolution",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-px-video",
            Gpu,
            "AMD",
            "Video on integrated",
            "Stops video being forced to the integrated GPU.",
            ALCHEMY,
            Community,
        )
        .hw(AmdGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\services\amdkmdap",
            "KMD_PXForceVideoPlaybackToIntegrated",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.amd-thermal-cn",
            Gpu,
            "AMD",
            "CN thermal limit",
            "Sets the CN thermal limit flag to 90.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(AmdGpu)
        .admin(),
        settings: &[hklm(r"SOFTWARE\AMD\CN", "ThermalLimit", 90)],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-gr535",
            Gpu,
            "NVIDIA",
            "GR535 flag",
            "Turns off the driver's GR535 feature flag.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
            "EnableGR535",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-rid-flags",
            Gpu,
            "NVIDIA",
            "RID feature flags",
            "Sets two driver RID feature flags.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableRID73779",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "EnableRID74361",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-param-flags",
            Gpu,
            "NVIDIA",
            "Driver parameter flags",
            "Turns off four driver parameter flags.",
            BATLEZ,
            Observed,
        )
        .medium()
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableDDC68",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3086",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3122",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\Parameters\FTS",
                "EnableGR3252",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.nv-scaling-flags",
            Gpu,
            "NVIDIA",
            "Scaling flags",
            "Turns off the driver's scaling flags.",
            BATLEZ,
            Observed,
        )
        .hw(NvidiaGpu)
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "Scaling",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\nvlddmkm\FTS",
                "ScalingMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.d3d-flip-no-vsync",
            Gpu,
            "Windows graphics",
            "Direct3D 9 flip",
            "Lets Direct3D 9 apps flip without waiting for vsync.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Direct3D", "FlipNoVsync", 1),
            hkcu(r"Software\Microsoft\Direct3D", "FlipNoVsync", 1),
            hklm(r"SOFTWARE\WOW6432Node\Microsoft\Direct3D", "FlipNoVsync", 1),
            hkcu(r"Software\WOW6432Node\Microsoft\Direct3D", "FlipNoVsync", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "gpu.ddraw-nonlocal",
            Gpu,
            "Windows graphics",
            "Non-local video memory",
            "Lets old DirectDraw apps use system video memory.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Microsoft\Direct3D", "UseNonLocalVidMem", 1),
            hkcu(r"Software\Microsoft\Direct3D", "UseNonLocalVidMem", 1),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Direct3D",
                "UseNonLocalVidMem",
                1,
            ),
            hkcu(
                r"Software\WOW6432Node\Microsoft\Direct3D",
                "UseNonLocalVidMem",
                1,
            ),
            hklm(r"SOFTWARE\Microsoft\DirectDraw", "UseNonLocalVidMem", 1),
            hkcu(r"Software\Microsoft\DirectDraw", "UseNonLocalVidMem", 1),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\DirectDraw",
                "UseNonLocalVidMem",
                1,
            ),
            hkcu(
                r"Software\WOW6432Node\Microsoft\DirectDraw",
                "UseNonLocalVidMem",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "memory.large-system-cache",
            Memory,
            "Paging and cache",
            "Large system cache",
            "Favours the file cache over apps.",
            BATLEZ,
            Community,
        )
        .medium()
        .warn("Games can have less memory available.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "LargeSystemCache",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.page-combining",
            Memory,
            "Paging and cache",
            "Memory page combining",
            "Stops Windows merging identical pages.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "DisablePageCombining",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.clear-pagefile",
            Memory,
            "Paging and cache",
            "Keep page file at shutdown",
            "Skips wiping the page file, for faster shutdown.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            "ClearPageFileAtShutdown",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.prefetcher",
            Memory,
            "Prefetch",
            "Prefetcher",
            "Stops Windows preloading app files.",
            BATLEZ,
            Observed,
        )
        .medium()
        .impact(Medium)
        .warn("Apps start slower, mostly on hard drives.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "EnablePrefetcher",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.superfetch",
            Memory,
            "Prefetch",
            "Superfetch parameter",
            "Stops memory preloading.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "EnableSuperfetch",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "memory.sf-tracing",
            Memory,
            "Prefetch",
            "Superfetch tracing",
            "Stops Superfetch tracing.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
            "SfTracingState",
            0,
        )],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\Ndu",
                "ImagePath",
            ),
            "memory.compat-ndu",
            Memory,
            "Prefetch",
            "Network Data Usage driver",
            "Stops the driver known for non-paged pool growth.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("The Data usage page stops counting.")
        .admin(),
        settings: &[hklm(r"SYSTEM\CurrentControlSet\Services\Ndu", "Start", 4)],
    },
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
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "AC",
            ),
            "power.nvme-noppme",
            Power,
            "Disk",
            "NVMe NOPPME",
            "Turns off non-operational power states on NVMe.",
            POWER_DOCS,
            Documented,
        )
        .desktop()
        .admin(),
        settings: &[
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "AC",
                1,
            ),
            power(
                r"0012ee47-9041-4b5d-9b77-535fba8b1442\fc7372b6-ab2d-43ee-8797-15e9841f2cca",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "AC",
            ),
            "power.usb-selective",
            Power,
            "USB and PCIe",
            "USB selective suspend (plan)",
            "Keeps USB devices powered on this plan.",
            POWER_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "AC",
                0,
            ),
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "AC",
            ),
            "power.usb-ioc",
            Power,
            "USB and PCIe",
            "USB IOC on all TDs",
            "Interrupts on every USB transfer.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .desktop()
        .admin(),
        settings: &[
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "AC",
                1,
            ),
            power(
                r"2a737441-1930-4402-8d77-b2bebba308a3\498c044a-201b-4631-a522-5c744ed4e678",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "AC",
            ),
            "power.away-mode",
            Power,
            "Sleep and buttons",
            "Away mode",
            "Allows media-style away mode.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "AC",
                1,
            ),
            power(
                r"238c9fa8-0aad-41ed-83f4-97be242c8f20\25dfa149-5dd1-4736-b5ab-e8a37b5b8187",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "AC",
            ),
            "power.standby-network",
            Power,
            "Sleep and buttons",
            "Network in standby",
            "Keeps the network on in standby.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "AC",
                1,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\f15576e8-98b7-4186-b944-eafa664402d9",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "AC",
            ),
            "power.device-idle",
            Power,
            "Sleep and buttons",
            "Device idle policy",
            "Favours performance over idle savings.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "AC",
                0,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\4faab71a-92e5-4726-b531-224559672d19",
                "DC",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "AC",
            ),
            "power.personality",
            Power,
            "Sleep and buttons",
            "Plan personality",
            "Marks the plan as high performance.",
            POWER_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "AC",
                1,
            ),
            power(
                r"fea3413e-7e05-4911-9a71-700331f1c294\245d8541-3943-4422-b025-13a784f679b7",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "AC",
            ),
            "power.brightness",
            Power,
            "Display and devices",
            "Full display brightness",
            "Uses full brightness on this plan.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "AC",
                100,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\aded5e82-b909-4619-9949-f5d71dac0bcb",
                "DC",
                100,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "AC",
            ),
            "power.dim-brightness",
            Power,
            "Display and devices",
            "No dimming",
            "Keeps full brightness when dimmed.",
            POWER_DOCS,
            Documented,
        )
        .laptop()
        .admin(),
        settings: &[
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "AC",
                100,
            ),
            power(
                r"7516b95f-f776-4464-8c53-06167f40cc99\f1fbfde2-a960-4165-9f88-50667911ce96",
                "DC",
                100,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "AC",
            ),
            "power.gpu-preference",
            Power,
            "Display and devices",
            "GPU preference policy",
            "Prefers the high-performance GPU.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "AC",
                2,
            ),
            power(
                r"5fb4938d-1ee8-4b0f-9a3c-5036b0ab995c\dd848b2a-8a5d-4451-9ae2-39cd41658f6c",
                "DC",
                2,
            ),
        ],
    },
    SettingsTweak {
        meta: guarded(
            RegTarget::new(
                Hive::PowerPlan,
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "AC",
            ),
            "power.timer-frequency",
            Power,
            "Display and devices",
            "JavaScript timer frequency",
            "Runs browser timers at full rate.",
            POWER_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            power(
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "AC",
                1,
            ),
            power(
                r"02f815b5-a5cf-4c84-bf20-649d1f75d3d8\4c793e7d-a264-42e1-87d3-7a0d2f523ccd",
                "DC",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "power.platform-aoac",
            Power,
            "System",
            "Modern Standby override",
            "Asks firmware for S3 sleep on supported PCs.",
            BATLEZ,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Sleep may stop working if the firmware has no S3.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "PlatformAoAcOverride",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.cs-enabled",
            Power,
            "System",
            "Connected standby",
            "Turns off connected standby.",
            BATLEZ,
            Community,
        )
        .medium()
        .reboot()
        .warn("Sleep may stop working if the firmware has no S3.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "CsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.custom-setup",
            Power,
            "System",
            "Plan customisation at setup",
            "Lets setup create default plans.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "CustomizeDuringSetup",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.platform-role",
            Power,
            "System",
            "Platform role",
            "Treats this PC as a desktop for power policy.",
            BATLEZ,
            Observed,
        )
        .medium()
        .desktop()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Power",
            "PlatformRole",
            1,
        )],
    },
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
    SettingsTweak {
        meta: lib(
            "storage.memoryusage",
            Storage,
            "File system",
            "NTFS memory usage",
            "Gives NTFS more paged pool for caching.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsMemoryUsage",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.mft-zone",
            Storage,
            "File system",
            "MFT zone",
            "Reserves more disk for the file table.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsMftZoneReservation",
            4,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.no-efs",
            Storage,
            "File system",
            "NTFS encryption",
            "Turns off the Encrypting File System.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .medium()
        .warn("EFS encrypted files can't be opened.")
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            "NtfsDisableEncryption",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.no-paging-encryption",
            Storage,
            "File system",
            "Page file encryption",
            "Keeps the page file unencrypted.",
            BATLEZ,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Policies",
            "NtfsEncryptPagingFile",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.boot-optimize",
            Storage,
            "File system",
            "Boot file optimisation",
            "Keeps boot files together on hard drives.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Dfrg\BootOptimizeFunction",
            "Enable",
            "y",
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.autochk-timeout",
            Storage,
            "File system",
            "Disk check countdown",
            "Shortens the boot-time disk check countdown.",
            TAIRIKU,
            Documented,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Control\Session Manager",
            "AutoChkTimeout",
            5,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.ahci-dipm",
            Storage,
            "Drives",
            "AHCI device power saving",
            "Stops SATA drives entering low power.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\storahci\Parameters",
            "EnableDipm",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.ahci-hipm",
            Storage,
            "Drives",
            "AHCI host power saving",
            "Stops the SATA link entering low power.",
            BATLEZ,
            Observed,
        )
        .impact(Medium)
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\storahci\Parameters",
            "EnableHipm",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-logging",
            Storage,
            "Drives",
            "NVMe driver logging",
            "Stops the NVMe driver's logs.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
            "EnableLogging",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-queue",
            Storage,
            "Drives",
            "NVMe queue depth",
            "Raises the NVMe I/O queue.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "IoQueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "IoQueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "QueueDepth",
                64,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "QueueDepth",
                64,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-requests",
            Storage,
            "Drives",
            "NVMe request limit",
            "Raises concurrent NVMe requests.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "NumberOfRequests",
                1500,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "NumberOfRequests",
                1500,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-arbitration",
            Storage,
            "Drives",
            "NVMe arbitration burst",
            "Raises commands per arbitration.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ArbitrationBurst",
                256,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ArbitrationBurst",
                256,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-queues",
            Storage,
            "Drives",
            "NVMe submission queues",
            "Sets the NVMe queue count.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "IoSubmissionQueueCount",
                3,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "IoSubmissionQueueCount",
                3,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-immediate",
            Storage,
            "Drives",
            "NVMe immediate data",
            "Sends small writes inline.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ImmediateData",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ImmediateData",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-hmb",
            Storage,
            "Drives",
            "NVMe host memory buffer",
            "Sets the memory the drive may borrow.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .warn("Drives without DRAM rely on this.")
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "HostMemoryBufferBytes",
                1500,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "HostMemoryBufferBytes",
                1500,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-flush",
            Storage,
            "Drives",
            "NVMe flush",
            "Forces flushes through.",
            ALCHEMY,
            Observed,
        )
        .medium()
        .reboot()
        .admin(),
        settings: &[
            hklm(
                r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters",
                "ForceFlush",
                1,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\StorNVMe\Parameters\Device",
                "ForceFlush",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "storage.nvme-timeout",
            Storage,
            "Drives",
            "NVMe I/O timeout",
            "Waits longer before resetting a drive.",
            BATLEZ,
            Observed,
        )
        .reboot()
        .admin(),
        settings: &[hklm(
            r"SYSTEM\CurrentControlSet\Services\stornvme\Parameters\Device",
            "IoTimeoutValue",
            255,
        )],
    },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Application Experience\AitAgent", "Enabled"), "tasks.ait-agent", Tasks, "Telemetry", "Application Impact Telemetry", "Collects app usage telemetry.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Application Experience\AitAgent")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Application Experience\MareBackup", "Enabled"), "tasks.mare-backup", Tasks, "Telemetry", "MareBackup", "Backs up compatibility data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Application Experience\MareBackup")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Application Experience\PcaPatchDbTask", "Enabled"), "tasks.pca-patch", Tasks, "Telemetry", "PCA Patch Database", "Updates compatibility fixes.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Application Experience\PcaPatchDbTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Application Experience\SdbinstMergeDbTask", "Enabled"), "tasks.sdbinst-merge", Tasks, "Telemetry", "Compatibility Database Merge", "Merges compatibility databases.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Application Experience\SdbinstMergeDbTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Application Experience\StartupAppTask", "Enabled"), "tasks.startup-app", Tasks, "Telemetry", "Startup App Report", "Reports apps that slow sign-in.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Application Experience\StartupAppTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator", "Enabled"), "tasks.ceip-consolidator", Tasks, "Telemetry", "CEIP Consolidator", "Uploads Customer Experience data.", BATLEZ, Observed).impact(Medium).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip", "Enabled"), "tasks.ceip-usb", Tasks, "Telemetry", "USB CEIP", "Collects USB usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask", "Enabled"), "tasks.ceip-kernel", Tasks, "Telemetry", "Kernel CEIP", "Collects kernel usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\BthSQM", "Enabled"), "tasks.ceip-bth", Tasks, "Telemetry", "Bluetooth CEIP", "Collects Bluetooth usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\BthSQM")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\Uploader", "Enabled"), "tasks.ceip-uploader", Tasks, "Telemetry", "CEIP Uploader", "Uploads usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\Uploader")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerCeipAssistant", "Enabled"), "tasks.ceip-server-assistant", Tasks, "Telemetry", "Server CEIP Assistant", "Collects server usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerCeipAssistant")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerRoleCollector", "Enabled"), "tasks.ceip-server-role", Tasks, "Telemetry", "Server Role Collector", "Collects server role data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerRoleCollector")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerRoleUsageCollector", "Enabled"), "tasks.ceip-server-usage", Tasks, "Telemetry", "Server Role Usage Collector", "Collects server role usage.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Customer Experience Improvement Program\Server\ServerRoleUsageCollector")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Device Information\Device User", "Enabled"), "tasks.device-info-user", Tasks, "Telemetry", "Device Information User", "Collects hardware data for the user.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Device Information\Device User")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Feedback\Siuf\DmClient", "Enabled"), "tasks.siuf-dmclient", Tasks, "Telemetry", "Feedback Client", "Sends feedback and diagnostics.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Feedback\Siuf\DmClient")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload", "Enabled"), "tasks.siuf-scenario", Tasks, "Telemetry", "Feedback Scenario Download", "Downloads feedback scenarios.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\PI\Sqm-Tasks", "Enabled"), "tasks.sqm", Tasks, "Telemetry", "Sqm Tasks", "Collects Windows usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\PI\Sqm-Tasks")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Sustainability\SustainabilityTelemetry", "Enabled"), "tasks.sustainability-telemetry", Tasks, "Telemetry", "Sustainability Telemetry", "Reports energy data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Sustainability\SustainabilityTelemetry")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Sustainability\PowerGridForecastTask", "Enabled"), "tasks.grid-forecast", Tasks, "Telemetry", "Power Grid Forecast", "Downloads carbon forecasts.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Sustainability\PowerGridForecastTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\NetTrace\GatherNetworkInfo", "Enabled"), "tasks.net-trace", Tasks, "Telemetry", "Gather Network Info", "Records network diagnostics.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\NetTrace\GatherNetworkInfo")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\PerformanceTrace\RequestTrace", "Enabled"), "tasks.request-trace", Tasks, "Telemetry", "Performance Trace Request", "Records performance traces on request.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\PerformanceTrace\RequestTrace")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Windows Error Reporting\QueueReporting", "Enabled"), "tasks.queue-reporting", Tasks, "Telemetry", "Error Report Queue", "Sends queued crash reports.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Windows Error Reporting\QueueReporting")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\ErrorDetails\EnableErrorDetailsUpdate", "Enabled"), "tasks.error-details", Tasks, "Telemetry", "Error Details Update", "Downloads error descriptions.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\ErrorDetails\EnableErrorDetailsUpdate")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures", "Enabled"), "tasks.flight-reconcile", Tasks, "Flighting and experiments", "Reconcile Features", "Applies feature rollouts.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing", "Enabled"), "tasks.flight-flush", Tasks, "Flighting and experiments", "Feature Usage Flushing", "Saves feature usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReceiver", "Enabled"), "tasks.flight-receiver", Tasks, "Flighting and experiments", "Feature Usage Receiver", "Receives feature usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReceiver")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting", "Enabled"), "tasks.flight-reporting", Tasks, "Flighting and experiments", "Feature Usage Reporting", "Reports feature usage data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Flighting\OneSettings\RefreshCache", "Enabled"), "tasks.onesettings", Tasks, "Flighting and experiments", "OneSettings Refresh", "Downloads remote configuration.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Flighting\OneSettings\RefreshCache")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner", "Enabled"), "tasks.troubleshooting-scanner", Tasks, "Diagnostics", "Troubleshooting Scanner", "Looks for problems to offer fixes for.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Diagnosis\Scheduled", "Enabled"), "tasks.scheduled-diagnosis", Tasks, "Diagnostics", "Scheduled Diagnosis", "Runs maintenance troubleshooters.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Diagnosis\Scheduled")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector", "Enabled"), "tasks.disk-diag-data", Tasks, "Diagnostics", "Disk Diagnostic Collector", "Sends disk health data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticResolver", "Enabled"), "tasks.disk-diag-resolver", Tasks, "Diagnostics", "Disk Diagnostic Resolver", "Warns about failing disks.", BATLEZ, Observed).medium().warn("Failing disk warnings stop appearing.").admin(), settings: &[task(r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticResolver")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DiskFootprint\Diagnostics", "Enabled"), "tasks.disk-footprint", Tasks, "Diagnostics", "Disk Footprint Diagnostics", "Measures disk use.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\DiskFootprint\Diagnostics")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem", "Enabled"), "tasks.power-efficiency", Tasks, "Diagnostics", "Power Efficiency Diagnostics", "Analyses power use.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\MemoryDiagnostic\ProcessMemoryDiagnosticEvents", "Enabled"), "tasks.memory-events", Tasks, "Diagnostics", "Memory Diagnostic Events", "Reports memory test results.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\MemoryDiagnostic\ProcessMemoryDiagnosticEvents")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\MemoryDiagnostic\RunFullMemoryDiagnostic", "Enabled"), "tasks.memory-full", Tasks, "Diagnostics", "Full Memory Diagnostic", "Runs scheduled memory tests.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\MemoryDiagnostic\RunFullMemoryDiagnostic")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WDI\ResolutionHost", "Enabled"), "tasks.resolution-host", Tasks, "Diagnostics", "Diagnostic Resolution Host", "Resolves problems found by diagnostics.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WDI\ResolutionHost")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Autochk\Proxy", "Enabled"), "tasks.autochk-proxy", Tasks, "Diagnostics", "Autochk Proxy", "Sends disk check results.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Autochk\Proxy")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Maintenance\WinSAT", "Enabled"), "tasks.winsat", Tasks, "Diagnostics", "WinSAT", "Benchmarks the PC.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Maintenance\WinSAT")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Maps\MapsToastTask", "Enabled"), "tasks.maps-toast", Tasks, "Maps and location", "Maps Toast", "Shows offline map notifications.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Maps\MapsToastTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Maps\MapsUpdateTask", "Enabled"), "tasks.maps-update", Tasks, "Maps and location", "Maps Update", "Updates offline maps.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Maps\MapsUpdateTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Location\Notifications", "Enabled"), "tasks.location-notify", Tasks, "Maps and location", "Location Notifications", "Shows location use prompts.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Location\Notifications")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Location\WindowsActionDialog", "Enabled"), "tasks.location-dialog", Tasks, "Maps and location", "Location Action Dialog", "Shows location action prompts.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Location\WindowsActionDialog")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\FamilySafetyMonitor", "Enabled"), "tasks.family-monitor", Tasks, "Family and account", "Family Safety Monitor", "Applies family safety settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\FamilySafetyMonitor")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\FamilySafetyRefreshTask", "Enabled"), "tasks.family-refresh", Tasks, "Family and account", "Family Safety Refresh", "Syncs family safety settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\FamilySafetyRefreshTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\FamilySafetyRefresh", "Enabled"), "tasks.family-refresh-legacy", Tasks, "Family and account", "Family Safety Refresh (legacy)", "Older name of the refresh task.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\FamilySafetyRefresh")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\FamilySafetyUpload", "Enabled"), "tasks.family-upload", Tasks, "Family and account", "Family Safety Upload", "Uploads family safety reports.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\FamilySafetyUpload")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\UpdateUserPictureTask", "Enabled"), "tasks.user-picture", Tasks, "Family and account", "Update User Picture", "Syncs your account picture.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\UpdateUserPictureTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\ThemesSyncedImageDownload", "Enabled"), "tasks.themes-sync", Tasks, "Family and account", "Synced Theme Images", "Downloads synced theme images.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\ThemesSyncedImageDownload")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\CloudExperienceHost\CreateObjectTask", "Enabled"), "tasks.cloud-host", Tasks, "Family and account", "Cloud Experience Host", "Finishes Microsoft account setup.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\CloudExperienceHost\CreateObjectTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\User Profile Service\HiveUploadTask", "Enabled"), "tasks.hive-upload", Tasks, "Family and account", "Profile Hive Upload", "Roams user profiles.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\User Profile Service\HiveUploadTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Workplace Join\Automatic-Device-Join", "Enabled"), "tasks.workplace-join", Tasks, "Family and account", "Automatic Device Join", "Joins work directories.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Workplace Join\Automatic-Device-Join")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\EnterpriseMgmt\MDMMaintenenceTask", "Enabled"), "tasks.mdm-maintenance", Tasks, "Family and account", "MDM Maintenance", "Maintains device management.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\EnterpriseMgmt\MDMMaintenenceTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Management\Provisioning\Cellular", "Enabled"), "tasks.provisioning-cellular", Tasks, "Family and account", "Cellular Provisioning", "Provisions mobile networks.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Management\Provisioning\Cellular")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Management\Provisioning\Logon", "Enabled"), "tasks.provisioning-logon", Tasks, "Family and account", "Logon Provisioning", "Applies provisioning at sign-in.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Management\Provisioning\Logon")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\License Manager\TempSignedLicenseExchange", "Enabled"), "tasks.temp-license", Tasks, "Family and account", "Temporary Licence Exchange", "Renews Store licences.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\License Manager\TempSignedLicenseExchange")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\InputSettingsRestoreDataAvailable", "Enabled"), "tasks.input-restore", Tasks, "Input sync", "Input Settings Restore", "Restores synced input settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\InputSettingsRestoreDataAvailable")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\LocalUserSyncDataAvailable", "Enabled"), "tasks.input-user", Tasks, "Input sync", "Input Local User Sync", "Syncs input settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\LocalUserSyncDataAvailable")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\MouseSyncDataAvailable", "Enabled"), "tasks.input-mouse", Tasks, "Input sync", "Mouse Settings Sync", "Syncs mouse settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\MouseSyncDataAvailable")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\PenSyncDataAvailable", "Enabled"), "tasks.input-pen", Tasks, "Input sync", "Pen Settings Sync", "Syncs pen settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\PenSyncDataAvailable")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\TouchpadSyncDataAvailable", "Enabled"), "tasks.input-touchpad", Tasks, "Input sync", "Touchpad Settings Sync", "Syncs touchpad settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\TouchpadSyncDataAvailable")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Input\syncpensettings", "Enabled"), "tasks.input-syncpen", Tasks, "Input sync", "Pen Sync", "Syncs pen data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Input\syncpensettings")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\International\Synchronize Language Settings", "Enabled"), "tasks.lang-sync", Tasks, "Input sync", "Language Settings Sync", "Syncs language settings.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\International\Synchronize Language Settings")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\LanguageComponentsInstaller\Installation", "Enabled"), "tasks.lang-install", Tasks, "Input sync", "Language Components Install", "Installs language features.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\LanguageComponentsInstaller\Installation")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources", "Enabled"), "tasks.lang-reconcile", Tasks, "Input sync", "Language Resources Reconcile", "Repairs language resources.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\LanguageComponentsInstaller\Uninstallation", "Enabled"), "tasks.lang-uninstall", Tasks, "Input sync", "Language Components Removal", "Removes language features.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\LanguageComponentsInstaller\Uninstallation")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\MUI\LPRemove", "Enabled"), "tasks.lp-remove", Tasks, "Input sync", "Language Pack Clean-up", "Removes unused language packs.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\MUI\LPRemove")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Speech\SpeechModelDownloadTask", "Enabled"), "tasks.speech-model", Tasks, "Input sync", "Speech Model Download", "Downloads speech models.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Speech\SpeechModelDownloadTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\SettingSync\NetworkStateChangeTask", "Enabled"), "tasks.settings-sync", Tasks, "Input sync", "Settings Sync Network Task", "Syncs settings when online.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\SettingSync\NetworkStateChangeTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WS\CrossDeviceResume", "Enabled"), "tasks.cross-device-resume", Tasks, "Input sync", "Cross Device Resume", "Resumes apps from your phone.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WS\CrossDeviceResume")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WS\CrossDeviceResumeTrigger", "Enabled"), "tasks.cross-device-trigger", Tasks, "Input sync", "Cross Device Resume Trigger", "Starts cross-device resume.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WS\CrossDeviceResumeTrigger")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Defrag\ScheduledDefrag", "Enabled"), "tasks.scheduled-defrag", Tasks, "Maintenance", "Scheduled Defrag", "Trims SSDs and defragments disks.", BATLEZ, Observed).medium().warn("SSDs stop getting scheduled TRIM unless you run it by hand.").admin(), settings: &[task(r"\Microsoft\Windows\Defrag\ScheduledDefrag")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DiskCleanup\SilentCleanup", "Enabled"), "tasks.silent-cleanup", Tasks, "Maintenance", "Silent Disk Clean-up", "Cleans disks when space runs low.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\DiskCleanup\SilentCleanup")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DiskFootprint\StorageSense", "Enabled"), "tasks.storage-sense", Tasks, "Maintenance", "Storage Sense", "Frees space on a schedule.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\DiskFootprint\StorageSense")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Registry\RegIdleBackup", "Enabled"), "tasks.reg-idle-backup", Tasks, "Maintenance", "Registry Idle Backup", "Backs up the registry.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\Registry\RegIdleBackup")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Servicing\StartComponentCleanup", "Enabled"), "tasks.component-cleanup", Tasks, "Maintenance", "Component Clean-up", "Removes old Windows components.", BATLEZ, Observed).medium().warn("The WinSxS folder grows over time.").admin(), settings: &[task(r"\Microsoft\Windows\Servicing\StartComponentCleanup")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Setup\SetupCleanupTask", "Enabled"), "tasks.setup-cleanup", Tasks, "Maintenance", "Setup Clean-up", "Clears files after upgrades.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Setup\SetupCleanupTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Setup\SnapshotCleanupTask", "Enabled"), "tasks.snapshot-cleanup", Tasks, "Maintenance", "Setup Snapshot Clean-up", "Clears setup snapshots.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Setup\SnapshotCleanupTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\RecoveryEnvironment\VerifyWinRE", "Enabled"), "tasks.verify-winre", Tasks, "Maintenance", "Verify Recovery Environment", "Checks Windows RE still works.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\RecoveryEnvironment\VerifyWinRE")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\StateRepository\MaintenanceTasks", "Enabled"), "tasks.state-repo", Tasks, "Maintenance", "State Repository Maintenance", "Maintains the app database.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\StateRepository\MaintenanceTasks")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Shell\IndexerAutomaticMaintenance", "Enabled"), "tasks.indexer-maint", Tasks, "Maintenance", "Indexer Maintenance", "Optimises the search index.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Shell\IndexerAutomaticMaintenance")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319", "Enabled"), "tasks.ngen", Tasks, "Maintenance", ".NET NGEN", "Precompiles .NET apps in idle time.", BATLEZ, Observed).medium().warn(".NET apps may start slower.").admin(), settings: &[task(r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 64", "Enabled"), "tasks.ngen-64", Tasks, "Maintenance", ".NET NGEN 64-bit", "Precompiles 64-bit .NET apps.", BATLEZ, Observed).medium().warn(".NET apps may start slower.").admin(), settings: &[task(r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 64")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 Critical", "Enabled"), "tasks.ngen-crit", Tasks, "Maintenance", ".NET NGEN Critical", "Precompiles important .NET code.", BATLEZ, Observed).medium().warn(".NET apps may start slower.").admin(), settings: &[task(r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 Critical")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 64 Critical", "Enabled"), "tasks.ngen-64-crit", Tasks, "Maintenance", ".NET NGEN 64-bit Critical", "Precompiles important 64-bit code.", BATLEZ, Observed).medium().warn(".NET apps may start slower.").admin(), settings: &[task(r"\Microsoft\Windows\.NET Framework\.NET Framework NGEN v4.0.30319 64 Critical")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WOF\WIM-Hash-Management", "Enabled"), "tasks.wof-hash", Tasks, "Maintenance", "WIM Hash Management", "Maintains compressed system files.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WOF\WIM-Hash-Management")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WOF\WIM-Hash-Validation", "Enabled"), "tasks.wof-validate", Tasks, "Maintenance", "WIM Hash Validation", "Validates compressed system files.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WOF\WIM-Hash-Validation")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Sysmain\ResPriStaticDbSync", "Enabled"), "tasks.res-pri", Tasks, "Maintenance", "Sysmain Resource Sync", "Syncs resource priorities.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Sysmain\ResPriStaticDbSync")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Sysmain\WsSwapAssessmentTask", "Enabled"), "tasks.ws-swap", Tasks, "Maintenance", "Sysmain Swap Assessment", "Measures memory swapping.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Sysmain\WsSwapAssessmentTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\SpacePort\SpaceAgentTask", "Enabled"), "tasks.space-agent", Tasks, "Maintenance", "Storage Spaces Agent", "Maintains Storage Spaces.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\SpacePort\SpaceAgentTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\SpacePort\SpaceManagerTask", "Enabled"), "tasks.space-manager", Tasks, "Maintenance", "Storage Spaces Manager", "Maintains Storage Spaces.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\SpacePort\SpaceManagerTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Storage Tiers Management\Storage Tiers Management Initialization", "Enabled"), "tasks.tiers-init", Tasks, "Maintenance", "Storage Tiers Init", "Prepares storage tiers.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Storage Tiers Management\Storage Tiers Management Initialization")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\FileHistory\File History (maintenance mode)", "Enabled"), "tasks.file-history", Tasks, "Maintenance", "File History Maintenance", "Runs File History.", BATLEZ, Observed).medium().warn("File History stops backing up.").admin(), settings: &[task(r"\Microsoft\Windows\FileHistory\File History (maintenance mode)")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\DUSM\dusmtask", "Enabled"), "tasks.dusm", Tasks, "Maintenance", "Data Usage Task", "Sends data usage.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\DUSM\dusmtask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\ApplicationData\DsSvcCleanup", "Enabled"), "tasks.dssvc-cleanup", Tasks, "Maintenance", "Data Sharing Clean-up", "Cleans shared data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\ApplicationData\DsSvcCleanup")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\ApplicationData\appuriverifierdaily", "Enabled"), "tasks.appuri", Tasks, "Maintenance", "App Link Verifier", "Verifies web links for apps.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\ApplicationData\appuriverifierdaily")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Printing\EduPrintProv", "Enabled"), "tasks.print-edu", Tasks, "Maintenance", "Education Printer Provisioning", "Sets up school printers.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Printing\EduPrintProv")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Printing\PrintJobCleanupTask", "Enabled"), "tasks.print-job-cleanup", Tasks, "Maintenance", "Print Job Clean-up", "Clears stale print jobs.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Printing\PrintJobCleanupTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Printing\PrinterCleanupTask", "Enabled"), "tasks.printer-cleanup", Tasks, "Maintenance", "Printer Clean-up", "Removes stale printers.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Printing\PrinterCleanupTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\PushToInstall\Registration", "Enabled"), "tasks.push-register", Tasks, "Maintenance", "PushToInstall Registration", "Registers for pushed apps.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\PushToInstall\Registration")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\InstallService\ScanForUpdates", "Enabled"), "tasks.scan-updates", Tasks, "Maintenance", "Store Update Scan", "Checks for Store app updates.", BATLEZ, Observed).medium().warn("Store apps stop updating on their own.").admin(), settings: &[task(r"\Microsoft\Windows\InstallService\ScanForUpdates")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\InstallService\ScanForUpdatesAsUser", "Enabled"), "tasks.scan-updates-user", Tasks, "Maintenance", "Store Update Scan as User", "Checks for your Store app updates.", BATLEZ, Observed).medium().warn("Store apps stop updating on their own.").admin(), settings: &[task(r"\Microsoft\Windows\InstallService\ScanForUpdatesAsUser")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\RetailDemo\CleanupOfflineContent", "Enabled"), "tasks.retail-cleanup", Tasks, "Maintenance", "Retail Demo Clean-up", "Removes store demo content.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\RetailDemo\CleanupOfflineContent")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask", "Enabled"), "tasks.remote-assist", Tasks, "Maintenance", "Remote Assistance", "Checks Remote Assistance policy.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\UPnP\UPnPHostConfig", "Enabled"), "tasks.upnp-config", Tasks, "Maintenance", "UPnP Host Config", "Configures UPnP.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\UPnP\UPnPHostConfig")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\XblGameSave\XblGameSaveTask", "Enabled"), "tasks.xbl-save", Tasks, "Maintenance", "Xbox Game Save", "Syncs Xbox saves.", BATLEZ, Observed).medium().warn("Xbox cloud saves stop syncing.").admin(), settings: &[task(r"\Microsoft\XblGameSave\XblGameSaveTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Time Synchronization\SynchronizeTime", "Enabled"), "tasks.time-sync", Tasks, "Network", "Scheduled Time Sync", "Keeps the clock right.", BATLEZ, Observed).medium().warn("The clock can drift.").admin(), settings: &[task(r"\Microsoft\Windows\Time Synchronization\SynchronizeTime")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Time Synchronization\ForceSynchronizeTime", "Enabled"), "tasks.time-force", Tasks, "Network", "Forced Time Sync", "Syncs the clock at startup.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\Time Synchronization\ForceSynchronizeTime")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Time Zone\SynchronizeTimeZone", "Enabled"), "tasks.time-zone", Tasks, "Network", "Time Zone Sync", "Updates time zone data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Time Zone\SynchronizeTimeZone")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Network Connectivity Status Indicator\NcsiIdentifyUserProxies", "Enabled"), "tasks.ncsi-proxy", Tasks, "Network", "NCSI Proxy Detection", "Finds proxies for connection checks.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Network Connectivity Status Indicator\NcsiIdentifyUserProxies")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\NlaSvc\WiFiTask", "Enabled"), "tasks.nla-wifi", Tasks, "Network", "NLA Wi-Fi Task", "Wi-Fi status for network location.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\NlaSvc\WiFiTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WCM\WiFiTask", "Enabled"), "tasks.wcm-wifi", Tasks, "Network", "WCM Wi-Fi Task", "Wi-Fi connection management.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Windows\WCM\WiFiTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Ras\MobilityManager", "Enabled"), "tasks.ras-mobility", Tasks, "Network", "VPN Mobility Manager", "Keeps VPNs alive across networks.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Ras\MobilityManager")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WlanSvc\CDSSync", "Enabled"), "tasks.wlan-cds", Tasks, "Network", "Wi-Fi CDS Sync", "Syncs Wi-Fi data.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WlanSvc\CDSSync")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WlanSvc\MoProfileManagement", "Enabled"), "tasks.wlan-profile", Tasks, "Network", "Wi-Fi Profile Management", "Manages mobile operator Wi-Fi.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WlanSvc\MoProfileManagement")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WwanSvc\NotificationTask", "Enabled"), "tasks.wwan-notify", Tasks, "Network", "Mobile Broadband Notifications", "Shows mobile network alerts.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WwanSvc\NotificationTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\WwanSvc\OobeDiscovery", "Enabled"), "tasks.wwan-oobe", Tasks, "Network", "Mobile Broadband Discovery", "Finds mobile networks at setup.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\WwanSvc\OobeDiscovery")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Work Folders\Work Folders Logon Synchronization", "Enabled"), "tasks.work-folders-logon", Tasks, "Network", "Work Folders Sign-in Sync", "Syncs work files at sign-in.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Work Folders\Work Folders Logon Synchronization")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\Work Folders\Work Folders Maintenance Work", "Enabled"), "tasks.work-folders", Tasks, "Network", "Work Folders Maintenance", "Maintains work files.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\Work Folders\Work Folders Maintenance Work")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\MicrosoftEdgeUpdateTaskMachineCore", "Enabled"), "tasks.edge-core", Tasks, "Third-party", "Edge Update Core", "Updates Edge.", BATLEZ, Observed).medium().warn("Edge stops updating, including security fixes.").admin(), settings: &[task(r"\MicrosoftEdgeUpdateTaskMachineCore")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\MicrosoftEdgeUpdateTaskMachineUA", "Enabled"), "tasks.edge-ua", Tasks, "Third-party", "Edge Update Hourly", "Checks hourly for Edge updates.", BATLEZ, Observed).medium().warn("Edge stops updating, including security fixes.").admin(), settings: &[task(r"\MicrosoftEdgeUpdateTaskMachineUA")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\GoogleUpdateTaskMachineCore", "Enabled"), "tasks.google-core", Tasks, "Third-party", "Google Update Core", "Updates Chrome.", BATLEZ, Observed).medium().warn("Chrome stops updating, including security fixes.").admin(), settings: &[task(r"\GoogleUpdateTaskMachineCore")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\GoogleUpdateTaskMachineUA", "Enabled"), "tasks.google-ua", Tasks, "Third-party", "Google Update Hourly", "Checks hourly for Chrome updates.", BATLEZ, Observed).medium().warn("Chrome stops updating, including security fixes.").admin(), settings: &[task(r"\GoogleUpdateTaskMachineUA")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\DropboxUpdateTaskMachineCore", "Enabled"), "tasks.dropbox-core", Tasks, "Third-party", "Dropbox Update Core", "Updates Dropbox.", BATLEZ, Observed).admin(), settings: &[task(r"\DropboxUpdateTaskMachineCore")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\DropboxUpdateTaskMachineUA", "Enabled"), "tasks.dropbox-ua", Tasks, "Third-party", "Dropbox Update Hourly", "Checks for Dropbox updates.", BATLEZ, Observed).admin(), settings: &[task(r"\DropboxUpdateTaskMachineUA")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Adobe Acrobat Update Task", "Enabled"), "tasks.adobe-update", Tasks, "Third-party", "Adobe Acrobat Update", "Updates Acrobat.", BATLEZ, Observed).medium().warn("Acrobat stops updating, including security fixes.").admin(), settings: &[task(r"\Adobe Acrobat Update Task")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Office\Office 15 Subscription Heartbeat", "Enabled"), "tasks.office-heartbeat", Tasks, "Third-party", "Office Subscription Heartbeat", "Checks your Office licence.", BATLEZ, Observed).medium().admin(), settings: &[task(r"\Microsoft\Office\Office 15 Subscription Heartbeat")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Office\OfficeTelemetryAgentFallBack", "Enabled"), "tasks.office-fallback", Tasks, "Third-party", "Office Telemetry Fallback", "Sends Office telemetry.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Office\OfficeTelemetryAgentFallBack")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Office\OfficeTelemetryAgentFallBack2016", "Enabled"), "tasks.office-fallback16", Tasks, "Third-party", "Office 2016 Telemetry Fallback", "Sends Office telemetry.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Office\OfficeTelemetryAgentFallBack2016")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Office\OfficeTelemetryAgentLogOn", "Enabled"), "tasks.office-logon", Tasks, "Third-party", "Office Telemetry at Sign-in", "Sends Office telemetry at sign-in.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Office\OfficeTelemetryAgentLogOn")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Office\OfficeTelemetryAgentLogOn2016", "Enabled"), "tasks.office-logon16", Tasks, "Third-party", "Office 2016 Telemetry at Sign-in", "Sends Office telemetry at sign-in.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Office\OfficeTelemetryAgentLogOn2016")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\Microsoft\Windows\OneDrive\OneDrive Per-Machine Standalone Update Task", "Enabled"), "tasks.onedrive-machine", Tasks, "Third-party", "OneDrive Machine Update", "Updates OneDrive for all users.", BATLEZ, Observed).admin(), settings: &[task(r"\Microsoft\Windows\OneDrive\OneDrive Per-Machine Standalone Update Task")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}", "Enabled"), "tasks.nv-mon", Tasks, "Third-party", "NVIDIA Telemetry Monitor", "Monitors NVIDIA telemetry.", BATLEZ, Observed).hw(NvidiaGpu).admin(), settings: &[task(r"\NvTmMon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}", "Enabled"), "tasks.nv-rep-logon", Tasks, "Third-party", "NVIDIA Telemetry at Sign-in", "Reports NVIDIA telemetry at sign-in.", BATLEZ, Observed).hw(NvidiaGpu).admin(), settings: &[task(r"\NvTmRepOnLogon_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}", "Enabled"), "tasks.nv-rep", Tasks, "Third-party", "NVIDIA Telemetry Report", "Reports NVIDIA telemetry.", BATLEZ, Observed).hw(NvidiaGpu).admin(), settings: &[task(r"\NvTmRep_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\AMDInstallLauncher", "Enabled"), "tasks.amd-launcher", Tasks, "Third-party", "AMD Install Launcher", "Runs AMD's installer at sign-in.", BATLEZ, Observed).hw(AmdGpu).admin(), settings: &[task(r"\AMDInstallLauncher")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\AMDLinkUpdate", "Enabled"), "tasks.amd-link", Tasks, "Third-party", "AMD Link Update", "Updates AMD Link.", BATLEZ, Observed).hw(AmdGpu).admin(), settings: &[task(r"\AMDLinkUpdate")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\AMDRyzenMasterSDKTask", "Enabled"), "tasks.amd-ryzen-master", Tasks, "Third-party", "Ryzen Master Task", "Starts Ryzen Master's SDK.", BATLEZ, Observed).hw(AmdCpu).admin(), settings: &[task(r"\AMDRyzenMasterSDKTask")] },
    SettingsTweak { meta: guarded(RegTarget::new(Hive::ScheduledTask, r"\ModifyLinkUpdate", "Enabled"), "tasks.amd-modify-link", Tasks, "Third-party", "AMD Modify Link Update", "Updates AMD Link settings.", BATLEZ, Observed).hw(AmdGpu).admin(), settings: &[task(r"\ModifyLinkUpdate")] },
    SettingsTweak {
        meta: lib(
            "windows.idle-wait",
            Windows,
            "Startup and shutdown",
            "Startup idle wait",
            "Stops waiting for idle before startup apps.",
            KUBAAM,
            Community,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            "WaitForIdleState",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hung-timeout",
            Windows,
            "Startup and shutdown",
            "Hung app timeout",
            "Treats apps as hung sooner.",
            TAIRIKU,
            Documented,
        ),
        settings: &[hkcu_str(r"Control Panel\Desktop", "HungAppTimeout", "2000")],
    },
    SettingsTweak {
        meta: lib(
            "windows.hooks-timeout",
            Windows,
            "Startup and shutdown",
            "Low-level hooks timeout",
            "Limits how long input hooks may take.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm_str(
            r"SYSTEM\CurrentControlSet\Control",
            "LowLevelHooksTimeout",
            "1000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.restart-apps",
            Windows,
            "Startup and shutdown",
            "Restart apps after sign-in",
            "Stops apps reopening.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "RestartApps",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.fast-user-switch",
            Windows,
            "Startup and shutdown",
            "Switch user on lock screen",
            "Hides the switch user option.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\UserSwitch",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sign-in-mode",
            Windows,
            "Startup and shutdown",
            "Boot to desktop",
            "Goes to the desktop after sign-in.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
            "SignInMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.smart-card-remove",
            Windows,
            "Startup and shutdown",
            "Smart card removal",
            "Locks the PC when a card is removed.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "ScRemoveOption",
            "2",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cached-logons",
            Windows,
            "Startup and shutdown",
            "Cached sign-ins",
            "Stops caching domain sign-ins.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Domain PCs can't sign in offline.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "CachedLogonsCount",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.visual-fx",
            Windows,
            "Visual effects",
            "Visual effects: best performance",
            "Uses the best performance preset.",
            BATLEZ,
            Community,
        )
        .impact(Medium),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
            "VisualFXSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.smooth-scroll",
            Windows,
            "Visual effects",
            "Smooth scrolling",
            "Turns off smooth scrolling.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(r"Control Panel\Desktop", "SmoothScroll", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.font-smoothing",
            Windows,
            "Visual effects",
            "Font smoothing",
            "Sets font smoothing to ClearType.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu_str(r"Control Panel\Desktop", "FontSmoothing", "2"),
            hkcu(r"Control Panel\Desktop", "FontSmoothingType", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.font-gamma",
            Windows,
            "Visual effects",
            "Font gamma",
            "Clears the gamma override.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(r"Control Panel\Desktop", "FontSmoothingGamma", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.foreground-lock",
            Windows,
            "Visual effects",
            "Foreground lock",
            "Lets apps take focus at once.",
            BATLEZ,
            Community,
        )
        .medium(),
        settings: &[hkcu(r"Control Panel\Desktop", "ForegroundLockTimeout", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.foreground-flash",
            Windows,
            "Visual effects",
            "Foreground flash",
            "Stops taskbar flashing.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "ForegroundFlashCount",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.window-arrange",
            Windows,
            "Visual effects",
            "Window arrangement",
            "Keeps Snap on.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "WindowArrangementActive",
            "1",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.lock-auto",
            Windows,
            "Visual effects",
            "Lock screen auto-lock",
            "Turns off the wallpaper auto-lock.",
            TAIRIKU,
            Community,
        ),
        settings: &[hkcu_str(
            r"Control Panel\Desktop",
            "LockScreenAutoLockActive",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dpi-96",
            Windows,
            "Visual effects",
            "Custom DPI",
            "Sets 100% scaling.",
            BATLEZ,
            Observed,
        )
        .medium()
        .sign_out(),
        settings: &[
            hkcu(r"Control Panel\Desktop", "LogPixels", 96),
            hkcu(r"Control Panel\Desktop", "Win8DpiScaling", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-watermark",
            Windows,
            "Visual effects",
            "Folder watermarks",
            "Turns off folder watermarks.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewWatermark",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-desktop",
            Windows,
            "Visual effects",
            "Peek at desktop",
            "Turns off Peek.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "DisablePreviewDesktop",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sound-scheme",
            Windows,
            "Visual effects",
            "No sounds",
            "Uses the No Sounds scheme.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu_str(r"AppEvents\Schemes", "", ".None")],
    },
    SettingsTweak {
        meta: lib(
            "windows.beep",
            Windows,
            "Visual effects",
            "Error beep",
            "Turns off the error beep.",
            KUBAAM,
            Community,
        ),
        settings: &[hkcu_str(r"Control Panel\Sound", "Beep", "no")],
    },
    SettingsTweak {
        meta: lib(
            "windows.color-filter",
            Windows,
            "Visual effects",
            "Colour filter",
            "Turns off colour filters.",
            BATLEZ,
            Observed,
        ),
        settings: &[
            hkcu(r"Software\Microsoft\ColorFiltering", "Active", 0),
            hkcu(r"Software\Microsoft\ColorFiltering", "HotkeyEnabled", 0),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.live-captions",
            Windows,
            "Visual effects",
            "Live captions at sign-in",
            "Stops Live captions starting.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\LiveCaptions",
            "StartLiveCaptionsOnLogin",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.voice-access",
            Windows,
            "Visual effects",
            "Voice access at sign-in",
            "Stops Voice access starting.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VoiceAccess",
                "StartVoiceAccessOnLogin",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VoiceAccess",
                "VoiceAccessState",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.desktop-process",
            Windows,
            "Explorer",
            "Desktop in separate process",
            "Keeps the desktop in its own process.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "DesktopProcess",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.map-drive-btn",
            Windows,
            "Explorer",
            "Map drive button",
            "Hides the map drive button.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "MapNetDrvBtn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.sharing-wizard",
            Windows,
            "Explorer",
            "Sharing wizard",
            "Turns off the sharing wizard.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SharingWizardOn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.compressed-color",
            Windows,
            "Explorer",
            "Colour for compressed files",
            "Turns off blue compressed names.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCompColor",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-quick-access",
            Windows,
            "Explorer",
            "Quick access",
            "Hides Quick access.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowQuickAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-pane",
            Windows,
            "Explorer",
            "Preview pane",
            "Hides the preview pane.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowPreviewPane",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.thumb-cache",
            Windows,
            "Explorer",
            "Thumbnail cache",
            "Stops caching thumbnails.",
            BATLEZ,
            Community,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbnailCache",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbnailCache",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.confirm-all",
            Windows,
            "Explorer",
            "Do this for all",
            "Clears the copy dialog default.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\OperationStatusManager",
            "ConfirmationCheckBoxDoForAll",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.autocomplete",
            Windows,
            "Explorer",
            "Autocomplete append",
            "Stops appending in the address bar.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu_str(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoComplete",
            "Append Completion",
            "No",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cached-icons",
            Windows,
            "Explorer",
            "Icon cache size",
            "Raises the icon cache.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "Max Cached Icons",
            "2000",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.previous-versions",
            Windows,
            "Explorer",
            "Previous Versions tab",
            "Hides the tab.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "NoPreviousVersionsPage",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.aic-anywhere",
            Windows,
            "Explorer",
            "Install apps from anywhere",
            "Allows apps from any source.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "AicEnabled",
            "Anywhere",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.no-autoplay",
            Windows,
            "Explorer",
            "AutoPlay for this user",
            "Turns off AutoPlay.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
            "DisableAutoplay",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.no-autorun",
            Windows,
            "Explorer",
            "AutoRun policy",
            "Stops programs running from media.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoAutorun",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.rename-wizards",
            Windows,
            "Explorer",
            "Web tasks",
            "Removes Publish to Web.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoPublishingWizard",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoWebServices",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.prints-wizard",
            Windows,
            "Explorer",
            "Online prints wizard",
            "Removes online printing.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoOnlinePrintsWizard",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.open-with-web",
            Windows,
            "Explorer",
            "Web search in Open With",
            "Stops searching the web for apps.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "NoInternetOpenWith",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.recent-docs",
            Windows,
            "Explorer",
            "Recent documents",
            "Stops tracking recent documents.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsHistory",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsHistory",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.recent-nethood",
            Windows,
            "Explorer",
            "Recent shares",
            "Stops adding shares to Network.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "Norecentdocsnethood",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.net-connect",
            Windows,
            "Explorer",
            "Connect menu",
            "Hides Connect in Start.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoNetConnectDisconnect",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoNetConnectDisconnect",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.active-desktop",
            Windows,
            "Explorer",
            "Active Desktop",
            "Turns off Active Desktop.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "ForceActiveDesktopOn",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoActiveDesktop",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoActiveDesktopChanges",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop",
                "NoAddingComponents",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop",
                "NoComponents",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.shell-security",
            Windows,
            "Explorer",
            "Shell extension security",
            "Only loads approved shell extensions.",
            TAIRIKU,
            Documented,
        )
        .medium()
        .warn("Unapproved context menu items stop loading.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            "EnforceShellExtensionSecurity",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.non-enum",
            Windows,
            "Explorer",
            "Hide Documents folder",
            "Hides shared documents.",
            KUBAAM,
            Community,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum",
            "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.file-zone",
            Windows,
            "Explorer",
            "Downloaded file zones",
            "Keeps the downloaded-file mark.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "SaveZoneInformation",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "SaveZoneInformation",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.scan-attachment",
            Windows,
            "Explorer",
            "Scan attachments",
            "Scans downloads with your antivirus.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "ScanWithAntiVirus",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Attachments",
                "ScanWithAntiVirus",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.chat",
            Windows,
            "Taskbar and Start",
            "Chat button",
            "Hides Chat.",
            BATLEZ,
            Observed,
        )
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarMn",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cortana-button",
            Windows,
            "Taskbar and Start",
            "Cortana button",
            "Hides Cortana.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCortanaButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.people",
            Windows,
            "Taskbar and Start",
            "People band",
            "Hides People.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People",
            "PeopleBand",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-size",
            Windows,
            "Taskbar and Start",
            "Lock taskbar size",
            "Locks the taskbar.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarSizeMove",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.balloon-tips",
            Windows,
            "Taskbar and Start",
            "Balloon tips",
            "Turns off balloon tips.",
            BATLEZ,
            Observed,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableBalloonTips",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableBalloonTips",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.pin-taskbar",
            Windows,
            "Taskbar and Start",
            "Taskbar pinning",
            "Blocks taskbar pins.",
            BATLEZ,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoPinningToTaskbar",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notification-center",
            Windows,
            "Taskbar and Start",
            "Notification centre",
            "Turns it off.",
            BATLEZ,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "DisableNotificationCenter",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.tablet-mode",
            Windows,
            "Taskbar and Start",
            "Tablet mode",
            "Stays in desktop mode.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                "TabletMode",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                "TabletModeBatteryThreshold",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-critical",
            Windows,
            "Taskbar and Start",
            "Critical toasts on lock",
            "Hides critical toasts when locked.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
            "NOC_GLOBAL_SETTING_ALLOW_CRITICAL_TOASTS",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-security",
            Windows,
            "Taskbar and Start",
            "Security notifications",
            "Hides Security and Maintenance toasts.",
            TAIRIKU,
            Observed,
        )
        .medium(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Windows.SystemToast.SecurityAndMaintenance",
            "Enabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-edge",
            Windows,
            "Taskbar and Start",
            "Edge notification ranking",
            "Stops Edge in Action Center.",
            TAIRIKU,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\MSEdge",
                "ShowInActionCenter",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\MSEdge",
                "Rank",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.notif-outlook",
            Windows,
            "Taskbar and Start",
            "Outlook notification ranking",
            "Stops Outlook in Action Center.",
            TAIRIKU,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Microsoft.OutlookForWindows_8wekyb3d8bbwe!Microsoft.OutlookforWindows",
                "ShowInActionCenter",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\Microsoft.OutlookForWindows_8wekyb3d8bbwe!Microsoft.OutlookforWindows",
                "Rank",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.app-assoc-toast",
            Windows,
            "Taskbar and Start",
            "Default app nags",
            "Stops Edge asking to be default.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.htm",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.html",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_http",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_https",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgeHTM_.pdf",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ApplicationAssociationToasts",
                "MSEdgePDF_.pdf",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.console-vt",
            Windows,
            "System",
            "Virtual terminal",
            "Turns on ANSI colours in the console.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu("Console", "VirtualTerminalLevel", 1)],
    },
    SettingsTweak {
        meta: lib(
            "windows.developer-sideload",
            Windows,
            "System",
            "Sideload apps",
            "Allows trusted app sideloading.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock",
            "AllowAllTrustedApps",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.developer-mode",
            Windows,
            "System",
            "Developer mode",
            "Turns on developer mode.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock",
            "AllowDevelopmentWithoutDevLicense",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.maintenance",
            Windows,
            "System",
            "Automatic maintenance",
            "Turns off automatic maintenance.",
            BATLEZ,
            Observed,
        )
        .medium()
        .impact(Medium)
        .warn("Disk trims and defrags stop unless you run them.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance",
            "MaintenanceDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.file-history",
            Windows,
            "System",
            "File History policy",
            "Turns off File History's schedule.",
            ALCHEMY,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\FileHistory",
            "ActiveHoursEnd",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.ts-enabled",
            Windows,
            "System",
            "Terminal Services flag",
            "Clears the Terminal Services flags.",
            TAIRIKU,
            Community,
        )
        .medium()
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSUserEnabled",
                0,
            ),
            hklm(
                r"SYSTEM\CurrentControlSet\Control\Terminal Server",
                "TSUserEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "TSAppCompat",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.dcom",
            Windows,
            "System",
            "DCOM",
            "Turns off DCOM.",
            TAIRIKU,
            Observed,
        )
        .medium()
        .warn("Some management tools and old apps stop working.")
        .admin(),
        settings: &[hklm_str(r"SOFTWARE\Microsoft\Ole", "EnableDCOM", "N")],
    },
    SettingsTweak {
        meta: lib(
            "windows.jit-debug",
            Windows,
            "System",
            "Script JIT debugger",
            "Turns off the script debugger.",
            TAIRIKU,
            Observed,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows Script\Settings",
            "JITDebug",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.aedebug",
            Windows,
            "System",
            "Automatic debugger",
            "Stops debuggers starting on crashes.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm_str(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\AeDebug",
            "Auto",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.smart-screen-note",
            Windows,
            "System",
            "Smart card provider",
            "Sets smart card sign-in rules.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowCertificatesWithNoEKU",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowIntegratedUnblock",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\SmartCardCredentialProvider",
                "AllowSignatureOnlyKeys",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.tcg-disabled",
            Windows,
            "System",
            "TCG security activation",
            "Stops activating self-encrypting drives.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\EnhancedStorageDevices",
            "TCGSecurityActivationDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.shutdown-tracker",
            Windows,
            "System",
            "Shutdown Event Tracker",
            "Removes the shutdown reason prompt.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Reliability",
                "ShutdownReasonOn",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Reliability",
                "ShutdownReasonUI",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.always-elevated",
            Windows,
            "System",
            "Always install elevated",
            "Makes sure this risky setting is off.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Installer",
            "AlwaysInstallElevated",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dep-explorer",
            Windows,
            "System",
            "DEP for Explorer",
            "Keeps Data Execution Prevention on Explorer.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoDataExecutionPrevention",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.heap-termination",
            Windows,
            "System",
            "Help pane DEP",
            "Keeps DEP on Help.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableHHDEP",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.credentials-delegation",
            Windows,
            "System",
            "Protected credentials",
            "Stops delegating protected credentials.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CredentialsDelegation",
            "AllowProtectedCreds",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.winrm-client",
            Windows,
            "System",
            "WinRM basic auth",
            "Turns off basic auth.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WinRM\Client",
                "AllowBasic",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows NT\Terminal Services",
                "AllowBasic",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.winrs",
            Windows,
            "System",
            "Remote shell",
            "Turns off remote shell.",
            TAIRIKU,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\WinRM\Service\WinRS",
            "AllowRemoteShellAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.ie-ssl3",
            Windows,
            "System",
            "SSL 3 fallback",
            "Blocks SSL 3 fallback.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\Internet Settings",
            "EnableSSL3Fallback",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.cert-errors",
            Windows,
            "System",
            "Certificate error override",
            "Stops ignoring certificate errors.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\Internet Settings",
            "PreventIgnoreCertErrors",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.zone-launch",
            Windows,
            "System",
            "Launch unsafe files prompt",
            "Prompts for programs from the Internet zone.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Zones\3",
                "1806",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Internet Settings\Zones\3",
                "1806",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.mi-layout",
            Windows,
            "System",
            "Optimal layout",
            "Stops auto layout tracking.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\OptimalLayout",
            "EnableAutoLayout",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hdr-gaming",
            Windows,
            "System",
            "Auto HDR",
            "Turns off Auto HDR.",
            BATLEZ,
            Community,
        ),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "EnableHDRForGaming",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "UseHDRForGaming",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.hdr-playback",
            Windows,
            "System",
            "HDR video playback",
            "Turns off HDR playback.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\VideoSettings",
                "EnableHDRForPlayback",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableHDRVideoAutoBrightness",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableHDRVideoAutoBrightness",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSDRtoHDRVideoExpansion",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\VideoManagement",
                "EnableSDRtoHDRVideoExpansion",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.colour-correction",
            Windows,
            "System",
            "Automatic colour correction",
            "Turns off colour correction.",
            BATLEZ,
            Community,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ColorManagement",
                "EnableColorCorrection",
                0,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ColorManagement",
                "EnableColorCorrection",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.media-frameserver",
            Windows,
            "System",
            "Media Foundation frame server",
            "Turns off frame server mode.",
            BATLEZ,
            Observed,
        )
        .medium()
        .warn("Some webcam apps may stop working.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Microsoft\Windows Media Foundation\Platform",
                "EnableFrameServerMode",
                0,
            ),
            hklm(
                r"SOFTWARE\WOW6432Node\Microsoft\Windows Media Foundation\Platform",
                "EnableFrameServerMode",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.account-picture",
            Windows,
            "System",
            "Account pictures",
            "Hides account pictures.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\AccountPicture",
            "AccountPictureDisabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.settings-account",
            Windows,
            "System",
            "Your account page",
            "Hides Your account in Settings.",
            BATLEZ,
            Observed,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowYourAccount",
            "value",
            0,
        )],
    },
    // Two settings the 0.5 audit took out of tweaks that stay: `windows.desktop-icons` wrote a shell
    // bag flag it never needed, and `windows.hub-mode` wrote a path missing `Microsoft`. A journal can
    // still hold them, and the broker restores only what a tweak declares.
    SettingsTweak {
        meta: lib(
            "windows.legacy-targets",
            Windows,
            "Retired settings",
            "Older settings",
            "Holds settings older releases wrote, so undo can still reach them.",
            TAIRIKU,
            Observed,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\Shell\Bags\1\Desktop",
                "FFlags",
                1075839525,
            ),
            hklm(r"SOFTWARE\Windows\CurrentVersion\Explorer", "HubMode", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "privacy.meet-now",
            Visuals,
            "Taskbar",
            "Meet Now",
            "Removes the Meet Now camera icon from the Windows 10 taskbar.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Meet Now was a shortcut for starting video calls. This hides it from the notification area. It only applies to Windows 10.")
        .sign_out(),
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
            "windows.start-pinned",
            Visuals,
            "Start menu",
            "Hide pinned apps in Start",
            "Hides the Pinned section of the new Start menu, leaving more room for All apps.",
            BATLEZ,
            Observed,
        )
        .medium()
        .builds(22000)
        .details("Same as the Pinned switch in Start settings on recent Windows 11 builds. Older versions ignore it. You reach apps from the All list or search, and your pins come back if you undo it."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Start",
            "ShowPinnedSection",
            0,
        )],
    },
];
