//! Apps tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "apps.edge-telemetry",
            Apps,
            "Edge",
            "Edge diagnostic data",
            "Sends no optional data.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "DiagnosticData", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "MetricsReportingEnabled",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-personalization",
            Apps,
            "Edge",
            "Edge personalisation",
            "Stops ad and news personalisation.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PersonalizationReportingEnabled",
            0,
        )],
    },
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
            "apps.edge-experiments",
            Apps,
            "Edge",
            "Edge experiments",
            "Turns off experiments.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ExperimentationAndConfigurationServiceControl",
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
            "apps.edge-rewards",
            Apps,
            "Edge",
            "Edge rewards",
            "Hides Microsoft Rewards.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowMicrosoftRewards",
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
            "apps.edge-promotional",
            Apps,
            "Edge",
            "Edge promotional tabs",
            "Stops promotional tabs.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PromotionalTabsEnabled",
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
            "apps.edge-searchbar",
            Apps,
            "Edge",
            "Edge search bar",
            "Turns off the desktop search bar.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Microsoft\Edge", "SearchbarAllowed", 0),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "SearchbarIsEnabledOnStartup",
                0,
            ),
        ],
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
            "apps.edge-top-sites",
            Apps,
            "Edge",
            "Edge top sites",
            "Hides default top sites.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageHideDefaultTopSites",
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
            "apps.edge-suggest",
            Apps,
            "Edge",
            "Edge search suggestions",
            "Stops suggestions.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SearchSuggestEnabled",
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
            "apps.edge-nav-errors",
            Apps,
            "Edge",
            "Edge error pages",
            "Stops web lookups for typos.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "ResolveNavigationErrorsUseWebService",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Edge",
                "AlternateErrorPagesEnabled",
                0,
            ),
        ],
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
            "apps.edge-autofill-card",
            Apps,
            "Edge",
            "Edge card autofill",
            "Turns off card autofill.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AutofillCreditCardEnabled",
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
            "apps.edge-3p-cookies",
            Apps,
            "Edge",
            "Edge third-party cookies",
            "Blocks third-party cookies.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "BlockThirdPartyCookies",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-tracking",
            Apps,
            "Edge",
            "Edge tracking prevention",
            "Sets tracking prevention to strict.",
            EDGE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "TrackingPrevention",
            3,
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
            "apps.chrome-background",
            Apps,
            "Chrome",
            "Chrome background mode",
            "Stops Chrome running after close.",
            CHROME_DOCS,
            Documented,
        )
        .impact(Medium)
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "BackgroundModeEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-metrics",
            Apps,
            "Chrome",
            "Chrome metrics",
            "Stops usage reports.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "MetricsReportingEnabled",
            0,
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
            "apps.chrome-url-data",
            Apps,
            "Chrome",
            "Chrome URL data",
            "Stops anonymised URL data.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "UrlKeyedAnonymizedDataCollectionEnabled",
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
            "apps.chrome-3p-cookies",
            Apps,
            "Chrome",
            "Chrome third-party cookies",
            "Blocks third-party cookies.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "BlockThirdPartyCookies",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-cookies-session",
            Apps,
            "Chrome",
            "Chrome session cookies",
            "Keeps cookies for the session only.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .warn("Sites sign you out after closing Chrome.")
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultCookiesSetting",
            4,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-geolocation",
            Apps,
            "Chrome",
            "Chrome location",
            "Blocks location requests.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultGeolocationSetting",
            2,
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
            "apps.chrome-notifications",
            Apps,
            "Chrome",
            "Chrome notifications",
            "Blocks notification prompts.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultNotificationsSetting",
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
            "apps.chrome-prediction",
            Apps,
            "Chrome",
            "Chrome network prediction",
            "Stops preloading pages.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "NetworkPredictionOptions",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-suggest",
            Apps,
            "Chrome",
            "Chrome search suggestions",
            "Stops suggestions.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "SearchSuggestEnabled",
            0,
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
            "apps.chrome-spellcheck",
            Apps,
            "Chrome",
            "Chrome spellcheck service",
            "Stops online spellcheck.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "SpellCheckServiceEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-password-manager",
            Apps,
            "Chrome",
            "Chrome password manager",
            "Turns off saving passwords.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "PasswordManagerEnabled",
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
            "apps.chrome-autofill-card",
            Apps,
            "Chrome",
            "Chrome card autofill",
            "Turns off card autofill.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "AutofillCreditCardEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-sign-in",
            Apps,
            "Chrome",
            "Chrome sign-in",
            "Blocks signing in.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .warn("Chrome sync stops working.")
        .admin(),
        settings: &[
            hklm(r"SOFTWARE\Policies\Google\Chrome", "SigninAllowed", 0),
            hklm(r"SOFTWARE\Policies\Google\Chrome", "SyncDisabled", 1),
        ],
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
            "apps.chrome-promos",
            Apps,
            "Chrome",
            "Chrome promotional tabs",
            "Stops promotional tabs.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "PromotionalTabsEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "WelcomePageOnOSUpgradeEnabled",
                0,
            ),
        ],
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
            "apps.chrome-sb-reporting",
            Apps,
            "Chrome",
            "Chrome Safe Browsing reports",
            "Stops extended reports.",
            CHROME_DOCS,
            Documented,
        )
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "SafeBrowsingExtendedReportingEnabled",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "SubmitSafeBrowsingDownloadVerdicts",
                0,
            ),
            hklm(
                r"SOFTWARE\Policies\Google\Chrome",
                "SafeBrowsingAIDataCollectionEnabled",
                0,
            ),
        ],
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
            "apps.firefox-telemetry",
            Apps,
            "Firefox",
            "Firefox telemetry",
            "Turns off telemetry.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableTelemetry",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-studies",
            Apps,
            "Firefox",
            "Firefox studies",
            "Turns off studies.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableFirefoxStudies",
            1,
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
            "apps.firefox-default-agent",
            Apps,
            "Firefox",
            "Firefox default agent",
            "Turns off the browser agent.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableDefaultBrowserAgent",
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
            "apps.brave-background",
            Apps,
            "Brave, Opera and Vivaldi",
            "Brave background mode",
            "Stops Brave running after close.",
            BRAVE_DOCS,
            Documented,
        )
        .admin(),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "BackgroundModeEnabled",
            0,
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
            "apps.office-telemetry",
            Apps,
            "Office",
            "Office telemetry",
            "Turns off Office telemetry.",
            BATLEZ,
            Documented,
        )
        .admin(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Office\16.0\Common\ClientTelemetry",
                "DisableTelemetry",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Office\16.0\Common\ClientTelemetry",
                "DisableTelemetry",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Office\15.0\Common\ClientTelemetry",
                "DisableTelemetry",
                1,
            ),
            hkcu(
                r"Software\Microsoft\Office\Common\ClientTelemetry",
                "DisableTelemetry",
                1,
            ),
        ],
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
            "apps.new-outlook-toggle",
            Apps,
            "Office",
            "New Outlook toggle",
            "Hides the toggle.",
            BATLEZ,
            Community,
        ),
        settings: &[hkcu(
            r"Software\Microsoft\Office\16.0\Outlook\Options\General",
            "HideNewOutlookToggle",
            1,
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
            "apps.onedrive-sync",
            Apps,
            "Other apps",
            "OneDrive sync",
            "Turns off OneDrive file sync.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("OneDrive stops syncing.")
        .admin(),
        settings: &[
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
                "DisableFileSyncNGSC",
                1,
            ),
            hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
                "DisableFileSync",
                1,
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
            "apps.onedrive-startup",
            Apps,
            "Other apps",
            "OneDrive at startup",
            "Stops OneDrive starting.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_absent(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "OneDrive",
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.spotify-startup",
            Apps,
            "Other apps",
            "Spotify at startup",
            "Stops Spotify starting.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_absent(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "Spotify",
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.steam-startup",
            Apps,
            "Other apps",
            "Steam at startup",
            "Stops Steam starting.",
            BATLEZ,
            Observed,
        ),
        settings: &[hkcu_absent(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "Steam",
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
];
