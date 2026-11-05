//! Apps tweaks. One entry per tweak; see `library/mod.rs` for the builders.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "apps.edge-telemetry",
            Apps,
            "Edge",
            "No Edge diagnostic data",
            "Stops Edge sending Microsoft optional usage and diagnostic data.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge keeps sending only the required data Microsoft says it needs to keep the browser secure and up to date. Nothing changes in how Edge works. Edge will say it's managed by your organisation."),
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
            "No Edge personalisation",
            "Stops Edge sharing your browsing data to personalise ads and news.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge can send browsing history, favourites and usage data to Microsoft for personalisation. This turns that off, so what you see becomes less personal. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PersonalizationReportingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-experiments",
            Apps,
            "Edge",
            "No Edge experiments",
            "Stops Microsoft testing unfinished features and settings in your copy of Edge.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge normally downloads trial features and configuration from Microsoft. This blocks them so the browser behaves predictably. Fixes delivered through the same service can be missed too. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ExperimentationAndConfigurationServiceControl",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-rewards",
            Apps,
            "Edge",
            "No Edge rewards",
            "Removes Microsoft Rewards points, badges and offers from Edge.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Hides the Rewards button and the prompts to earn points. You can't earn Rewards through Edge afterwards, so leave this alone if you collect them. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "ShowMicrosoftRewards",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-promotional",
            Apps,
            "Edge",
            "No Edge promotional tabs",
            "Stops Edge opening full-page promotions for its features by itself.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge can open a tab to advertise its own features. This stops that. Nothing else in the browser changes. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "PromotionalTabsEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-searchbar",
            Apps,
            "Edge",
            "No Edge search bar",
            "Removes the Edge search bar from the desktop and from startup.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("The search bar is a separate floating box that sends searches through Edge. This turns it off and keeps it from loading at startup. Edge's own address bar is not affected. Edge will say it's managed by your organisation."),
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
            "apps.edge-top-sites",
            Apps,
            "Edge",
            "No Edge default top sites",
            "Removes the preset site tiles Microsoft adds to Edge's new tab page.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge fills the new tab page with tiles it picks itself, some of them sponsored. This takes those away and leaves the sites you use. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "NewTabPageHideDefaultTopSites",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-suggest",
            Apps,
            "Edge",
            "No Edge search suggestions",
            "Stops Edge sending your address bar typing to Microsoft as you type.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Normally what you type goes to your search engine to fetch suggestions. After this, only searches you actually run are sent. You keep history and favourites but lose search suggestions. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "SearchSuggestEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-nav-errors",
            Apps,
            "Edge",
            "No Edge similar-site suggestions",
            "Stops Edge sending failed web addresses to Microsoft to suggest similar sites.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("When a page can't be found, Edge can ask a Microsoft service for alternatives. This keeps mistyped addresses on your PC, and you see the normal error page. Edge will say it's managed by your organisation."),
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
            "apps.edge-autofill-card",
            Apps,
            "Edge",
            "No Edge card autofill",
            "Edge stops saving your payment cards and offering to fill them in at checkout.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Your card details stay out of the browser and out of your Microsoft account. You type them in or use a password manager instead. Edge will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "AutofillCreditCardEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.edge-3p-cookies",
            Apps,
            "Edge",
            "Block Edge third-party cookies",
            "Edge refuses cookies from outside sites, which cuts tracking across websites.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Edge keeps cookies for the site you are on and refuses those from other sites embedded in it. Edge will say it's managed by your organisation.")
        .warn("Some embedded sign-ins and payment windows can stop working."),
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
            "Strict Edge tracking prevention",
            "Sets Edge's tracking prevention to Strict, which blocks most trackers across sites.",
            EDGE_DOCS,
            Documented,
        )
        .admin()
        .details("Strict blocks most trackers and more ads than the default Balanced level. You can't change the level in Edge while this is on. Edge will say it's managed by your organisation.")
        .warn("Strict mode can break parts of some sites, such as embedded videos."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Edge",
            "TrackingPrevention",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-background",
            Apps,
            "Chrome",
            "No Chrome background mode",
            "Chrome closes fully with its last window instead of staying in the system tray.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Stops Chrome and its extensions running after you close it, which frees a little memory. Extensions that work in the background only run while Chrome is open. Chrome will say it's managed by your organisation."),
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
            "No Chrome metrics",
            "Stops Chrome sending Google usage statistics and crash reports.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Chrome can report how you use features and when it crashes. This turns that off and has no effect on how Chrome works. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "MetricsReportingEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-url-data",
            Apps,
            "Chrome",
            "No Chrome URL sharing",
            "Stops Chrome sending Google the addresses of the pages you visit.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("This is the setting Chrome calls making searches and browsing better. Turning it off has no effect on normal browsing. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "UrlKeyedAnonymizedDataCollectionEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-3p-cookies",
            Apps,
            "Chrome",
            "No Chrome third-party cookies",
            "Stops advertisers and other outside sites tracking you across websites with cookies.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Chrome keeps cookies for the site you are on and refuses those from other sites embedded in it. This cuts tracking across sites. Chrome will say it's managed by your organisation.")
        .warn("Some embedded sign-ins and payment windows can stop working."),
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
            "Clear Chrome cookies on close",
            "Chrome deletes every site's cookies when you close it, so tracking can't carry over.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .warn("Sites sign you out after closing Chrome.")
        .admin()
        .details("Sites can use cookies while Chrome is open, but they are wiped when it closes. This works well against long-term tracking, at the cost of signing in again each time. Chrome will say it's managed by your organisation."),
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
            "Block Chrome location requests",
            "Websites can no longer ask Chrome for your location, and the prompts disappear.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Every site is refused your location without asking. Store finders and map sites will need you to type a place instead. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultGeolocationSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-notifications",
            Apps,
            "Chrome",
            "Block Chrome notifications",
            "Websites can no longer ask to send you notifications, so those prompts disappear.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Blocks the pop-up asking to show notifications and stops all sites sending them. Web apps you rely on for alerts, such as web mail, can't notify you. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "DefaultNotificationsSetting",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-prediction",
            Apps,
            "Chrome",
            "No Chrome page preloading",
            "Chrome stops connecting to and loading pages it guesses you will open next.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Preloading speeds up some page loads but contacts sites you may never visit. Turning it off limits that contact and saves a little data. Pages can open slightly slower. Chrome will say it's managed by your organisation."),
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
            "No Chrome search suggestions",
            "Stops Chrome sending your address bar typing to Google as you type.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Normally what you type goes to your search engine to fetch suggestions. After this, only searches you actually run are sent. You keep history and bookmarks but lose search suggestions. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "SearchSuggestEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-spellcheck",
            Apps,
            "Chrome",
            "No Chrome spellcheck service",
            "Stops Chrome sending what you type to Google for enhanced spell checking.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Enhanced spell check sends text you type in the browser to Google. This turns it off. Chrome's basic spell check on your PC keeps working. Chrome will say it's managed by your organisation."),
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
            "No Chrome password manager",
            "Chrome stops offering to save passwords, for people with another manager.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .admin()
        .details("Removes the save-password prompts and stops Chrome storing new passwords. Worth it if you already use a separate password manager, otherwise leave it alone. Chrome will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Google\Chrome",
            "PasswordManagerEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-autofill-card",
            Apps,
            "Chrome",
            "No Chrome card autofill",
            "Chrome stops saving your payment cards and offering to fill them in at checkout.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Your card details stay out of the browser and out of your Google account. You type them in or use a password manager instead. Chrome will say it's managed by your organisation."),
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
            "No Chrome sign-in and sync",
            "Stops Chrome syncing your history, passwords and tabs to a Google account.",
            CHROME_DOCS,
            Documented,
        )
        .medium()
        .warn("Chrome sync stops working.")
        .admin()
        .details("Your browsing data stays on this PC instead of being copied to Google's servers. You lose sync between devices. You can still sign in to Google websites. Chrome will say it's managed by your organisation."),
        settings: &[
            hklm(r"SOFTWARE\Policies\Google\Chrome", "SigninAllowed", 0),
            hklm(r"SOFTWARE\Policies\Google\Chrome", "SyncDisabled", 1),
        ],
    },
    SettingsTweak {
        meta: lib(
            "apps.chrome-promos",
            Apps,
            "Chrome",
            "No Chrome promotional tabs",
            "Stops Chrome opening full-page promotions and welcome tabs by itself.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Chrome can open a tab to advertise its own features, sometimes after a Windows upgrade. This stops that. Nothing else in the browser changes. Chrome will say it's managed by your organisation."),
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
            "apps.chrome-sb-reporting",
            Apps,
            "Chrome",
            "Fewer Safe Browsing reports",
            "Sends Google fewer page and download samples. Safe Browsing stays on.",
            CHROME_DOCS,
            Documented,
        )
        .admin()
        .details("Extended reporting shares extra data about pages and downloads with Google. This opts out while keeping Safe Browsing itself. It's often already off, so you may see no change. Chrome will say it's managed by your organisation."),
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
            "apps.firefox-telemetry",
            Apps,
            "Firefox",
            "No Firefox telemetry",
            "Stops Firefox sending Mozilla usage, performance and technical data.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin()
        .details("Turns off Firefox's telemetry reporting. It has no effect on browsing. Firefox will say it's managed by your organisation."),
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
            "No Firefox studies",
            "Stops Mozilla running studies and trial features in your copy of Firefox.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin()
        .details("Studies install small experiments that test changes on real users. This opts you out, so Firefox only changes when it updates. Firefox will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableFirefoxStudies",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.firefox-default-agent",
            Apps,
            "Firefox",
            "No Firefox default browser agent",
            "Stops a daily background task that reports your default browser to Mozilla.",
            FIREFOX_DOCS,
            Documented,
        )
        .admin()
        .details("Firefox installs a scheduled task that checks which browser is your default and tells Mozilla. This stops it. Firefox itself works the same. Firefox will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Mozilla\Firefox",
            "DisableDefaultBrowserAgent",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.brave-background",
            Apps,
            "Brave, Opera and Vivaldi",
            "No Brave background mode",
            "Brave closes fully when you close its last window, freeing the memory it held.",
            BRAVE_DOCS,
            Documented,
        )
        .admin()
        .details("Stops Brave and its extensions carrying on in the background after you close it. Background extensions and notifications then only work while Brave is open. Brave will say it's managed by your organisation."),
        settings: &[hklm(
            r"SOFTWARE\Policies\BraveSoftware\Brave",
            "BackgroundModeEnabled",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.office-telemetry",
            Apps,
            "Office",
            "Less Office telemetry",
            "Reduces the usage and diagnostic data Office apps send to Microsoft.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Turns off Office's telemetry logging for installed versions. Office works the same. It doesn't cover the required service data Office sends to stay secure."),
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
            "apps.new-outlook-toggle",
            Apps,
            "Office",
            "Hide the New Outlook toggle",
            "Hides the Try the new Outlook switch, so classic Outlook stays as it is.",
            BATLEZ,
            Community,
        )
        .details("Only matters if you use classic Outlook from Office. The switch disappears from the top of the window and nothing else changes. You can still install the new Outlook separately."),
        settings: &[hkcu(
            r"Software\Microsoft\Office\16.0\Outlook\Options\General",
            "HideNewOutlookToggle",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "apps.onedrive-sync",
            Apps,
            "Other apps",
            "Turn off OneDrive",
            "Turns OneDrive off completely, for anyone who doesn't use it.",
            BATLEZ,
            Documented,
        )
        .medium()
        .warn("OneDrive stops syncing.")
        .admin()
        .details("Blocks the OneDrive app from running and syncing, and it stops appearing in File Explorer. Files already on this PC stay put, but files held only online can't be reached until you undo this."),
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
            "apps.onedrive-startup",
            Apps,
            "Other apps",
            "No OneDrive at startup",
            "OneDrive stops launching when you sign in. You can still open it yourself.",
            BATLEZ,
            Observed,
        )
        .details("Removes OneDrive from startup, so it doesn't load or sync in the background until you open it. Your files aren't touched. OneDrive can add itself back after an update.")
        .warn("Files stop syncing until you open OneDrive."),
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
            "No Spotify at startup",
            "Spotify stops launching when you sign in, so it only runs when you open it.",
            BATLEZ,
            Observed,
        )
        .details("Removes Spotify from Windows startup. This applies to the version from Spotify's website. If Spotify's own startup option is still on it can add itself back, so turn that off too."),
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
            "No Steam at startup",
            "Steam stops launching when you sign in, so it only runs when you open it.",
            BATLEZ,
            Observed,
        )
        .details("Removes Steam from Windows startup. Downloads and updates wait until you open Steam. If Steam's own startup option is still on it can add itself back, so turn that off too."),
        settings: &[hkcu_absent(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "Steam",
        )],
    },
];
