//! Visuals: settings whose main purpose is how Windows looks and feels. One entry per tweak; see
//! `library/mod.rs` for the builders. Their IDs keep the prefix they shipped with, because the
//! journal references them.

use super::*;

pub const TWEAKS: &[SettingsTweak] = &[
    SettingsTweak {
        meta: lib(
            "windows.transparency",
            Visuals,
            "Theme and colour",
            "Turn off transparency",
            "Turns off the see-through effect on the taskbar, Start and apps.",
            BATLEZ,
            Observed,
        )
        .details("Same as the Transparency effects switch in Settings. Menus get a solid colour, which can be easier to read and saves a very small amount of graphics work."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "EnableTransparency",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dark-apps",
            Visuals,
            "Theme and colour",
            "Dark mode for apps",
            "Apps that follow Windows switch to their dark theme.",
            TAIRIKU,
            Observed,
        )
        .details("Same as choosing Dark for default app mode in Colours settings. Settings, File Explorer and many other apps go dark. Apps with their own theme setting may need changing separately."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "AppsUseLightTheme",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.dark-system",
            Visuals,
            "Theme and colour",
            "Dark mode for Windows",
            "The taskbar, Start menu and system flyouts switch to dark colours.",
            TAIRIKU,
            Observed,
        )
        .details("Same as choosing Dark for default Windows mode in Colours settings. It covers the Windows shell, while apps have their own switch. Use both for a fully dark desktop."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "SystemUsesLightTheme",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.accent-taskbar",
            Visuals,
            "Theme and colour",
            "Accent colour on taskbar",
            "Colours the taskbar and Start with your accent colour.",
            TAIRIKU,
            Observed,
        )
        .details("The same as Show accent colour on Start and taskbar in Settings. It needs Windows set to dark mode. Undo puts them back to the plain theme colour."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            "ColorPrevalence",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.min-animate",
            Visuals,
            "Effects and animation",
            "No minimise animation",
            "Windows open and shrink at once when you minimise or maximise them.",
            BATLEZ,
            Observed,
        )
        .details("The same as Animate windows when minimizing and maximizing in Performance Options. It works on every edition of Windows, and only for your account.")
        .sign_out(),
        settings: &[hkcu_str(
            r"Control Panel\Desktop\WindowMetrics",
            "MinAnimate",
            "0",
        )],
    },
    SettingsTweak {
        meta: lib(
            "gaming.mouse-hover",
            Visuals,
            "Effects and animation",
            "Faster hover tips",
            "Tooltips and hover effects appear almost at once.",
            BATLEZ,
            Community,
        )
        .details("Cuts the hover delay from the usual 400 milliseconds to 10 across Windows. Things feel quicker, but tooltips can also pop up as you pass over them.")
        .sign_out(),
        settings: &[hkcu_str(r"Control Panel\Mouse", "MouseHoverTime", "10")],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-mode-animation",
            Visuals,
            "Effects and animation",
            "No fade on mode change",
            "Removes the fade when your screen changes resolution or refresh rate.",
            TAIRIKU,
            Observed,
        )
        .admin()
        .details("Windows 11 fades the display during mode changes, which some people find slow when games switch modes. This skips it. The value is undocumented and only recent Windows 11 builds have it. It applies to every account on this PC.")
        .sign_out()
        .builds(22631),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\Dwm",
            "ForceDisableModeChangeAnimation",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "gpu.dwm-peek",
            Visuals,
            "Effects and animation",
            "Turn off Peek",
            "Stops windows turning see-through when you hover over taskbar previews.",
            BATLEZ,
            Observed,
        )
        .details("Peek shows a window or the desktop full size while you hover over a preview. Turning it off removes that effect, and the thumbnails stay. It is the Enable Peek box in Performance Options.")
        .sign_out(),
        settings: &[hkcu(r"Software\Microsoft\Windows\DWM", "EnableAeroPeek", 0)],
    },
    SettingsTweak {
        meta: lib(
            "windows.menu-delay",
            Visuals,
            "Effects and animation",
            "Faster menus",
            "Menus and submenus open sooner when you hover over them.",
            BATLEZ,
            Observed,
        )
        .details("Cuts the pause before a menu opens from 400 ms to 50 ms, so Windows feels a bit quicker in use. It's purely a feel change, and menus may flick open as you move across them.")
        .sign_out(),
        settings: &[hkcu_str(r"Control Panel\Desktop", "MenuShowDelay", "50")],
    },
    SettingsTweak {
        meta: lib(
            "windows.drag-full",
            Visuals,
            "Effects and animation",
            "Drag windows as an outline",
            "Dragging a window moves an outline; the contents follow when you let go.",
            BATLEZ,
            Observed,
        )
        .details("An old option from Performance Options, useful on slow graphics or remote connections. On a modern PC the saving is tiny, so choose it only if you prefer the look.")
        .sign_out(),
        settings: &[hkcu_str(r"Control Panel\Desktop", "DragFullWindows", "0")],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-alpha",
            Visuals,
            "Effects and animation",
            "Plain selection box",
            "The drag-to-select box becomes a plain outline, not a see-through tint.",
            BATLEZ,
            Community,
        )
        .details("Same as Show translucent selection rectangle in Performance Options. It's a look preference; any speed difference is too small to see.")
        .sign_out(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewAlphaSelect",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.list-shadow",
            Visuals,
            "Effects and animation",
            "No icon label shadows",
            "Removes the soft shadow behind desktop icon names.",
            BATLEZ,
            Community,
        )
        .details("Same as Use drop shadows for icon labels on the desktop in Performance Options. Labels look flatter and may be harder to read on a bright wallpaper.")
        .sign_out(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ListviewShadow",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-animations",
            Visuals,
            "Effects and animation",
            "No taskbar animations",
            "Turns off the slide and fade effects on taskbar buttons.",
            BATLEZ,
            Observed,
        )
        .details("Part of the Performance Options list of effects. The taskbar feels a little more immediate, and the change is only in how it looks.")
        .sign_out(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAnimations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.desktop-icons",
            Visuals,
            "Windows and desktop",
            "Hide desktop icons",
            "Hides every icon on the desktop for a clean wallpaper.",
            TAIRIKU,
            Observed,
        )
        .details("Same as unticking Show desktop icons in the desktop's right-click menu. Your files stay in the Desktop folder, and you can reach them in File Explorer.")
        .sign_out(),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "HideIcons",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-groups",
            Visuals,
            "Windows and desktop",
            "No snap groups",
            "Stops snapped windows showing as a group in the taskbar and Alt+Tab.",
            TAIRIKU,
            Observed,
        )
        .details("Same as the snap groups switch in Multitasking settings on Windows 11. Each window is listed alone in taskbar previews, Task View and Alt+Tab. Snapping itself still works.")
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "EnableTaskGroups",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-flyout",
            Visuals,
            "Windows and desktop",
            "Hide the snap layouts flyout",
            "Stops the snap layouts panel appearing when you hover over Maximise.",
            BATLEZ,
            Community,
        )
        .details("Same as the switch in Multitasking settings on Windows 11. It avoids the panel appearing by accident. Dragging windows to the screen edges still works.")
        .builds(22000),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableSnapAssistFlyout",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "EnableSnapAssistFlyoutPreview",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-bar",
            Visuals,
            "Windows and desktop",
            "Hide the snap bar",
            "Stops the snap bar dropping down when you drag a window to the top.",
            BATLEZ,
            Observed,
        )
        .details("Same as the switch in Multitasking settings on Windows 11. Dragging a window up the screen no longer brings up the bar. Other ways of snapping still work.")
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "EnableSnapBar",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.snap-suggest",
            Visuals,
            "Windows and desktop",
            "No snap suggestions",
            "After you snap a window, Windows no longer suggests what to put beside it.",
            BATLEZ,
            Observed,
        )
        .details("Same as the Snap assist switch in Multitasking settings. The screen stays as you left it after snapping, and you fill the other half yourself."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "SnapAssist",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.alt-tab",
            Visuals,
            "Windows and desktop",
            "Hide Edge tabs in Alt+Tab",
            "Alt+Tab shows only open windows, without each Edge tab as its own entry.",
            TAIRIKU,
            Observed,
        )
        .details("Same as setting Alt+Tab to open windows only in Multitasking settings. Switching is quicker if you keep a lot of tabs open in Edge. Other browsers aren't affected.")
        .builds(19042),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "MultiTaskingAltTabFilter",
            3,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.touch-button",
            Visuals,
            "Taskbar",
            "Hide touch keyboard button",
            "Removes the touch keyboard button from the taskbar's notification area.",
            BATLEZ,
            Observed,
        )
        .details("Only useful on a PC that shows the button. The touch keyboard itself stays available if you need it.")
        .sign_out(),
        settings: &[hkcu(
            r"Software\Microsoft\TabletTip\1.7",
            "TipbandDesiredVisibility",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-left",
            Visuals,
            "Taskbar",
            "Align taskbar left",
            "Moves the Start button and taskbar icons to the left on Windows 11.",
            TAIRIKU,
            Observed,
        )
        .builds(22000)
        .details("Same as setting Taskbar alignment to Left in taskbar settings. It puts Start back in the corner, where it sits on Windows 10."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAl",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.task-view",
            Visuals,
            "Taskbar",
            "Hide the Task View button",
            "Removes the Task View button from the taskbar.",
            BATLEZ,
            Observed,
        )
        .details("Same as the switch in taskbar settings. It frees a little taskbar room. Win+Tab still opens Task View."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowTaskViewButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.copilot-button",
            Visuals,
            "Taskbar",
            "Hide the Copilot button",
            "Removes the Copilot button from the taskbar where Windows still shows it.",
            BATLEZ,
            Observed,
        )
        .details("Applies to Windows 10 and older Windows 11 builds that had a built-in Copilot button. Newer builds use a Copilot app instead, so nothing changes there. Copilot itself has a separate policy switch.")
        .builds(19045).until(22631),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCopilotButton",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.end-task",
            Visuals,
            "Taskbar",
            "End task in taskbar",
            "Adds End task to taskbar right-click menus, to close a frozen app quickly.",
            BATLEZ,
            Observed,
        )
        .details("Same as the End task switch in For developers settings on Windows 11. You can force-close an app without opening Task Manager. Unsaved work in that app is lost.")
        .builds(22621),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            "TaskbarEndTask",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.tray-all",
            Visuals,
            "Taskbar",
            "All tray icons",
            "Shows every notification area icon on Windows 10, instead of hiding some.",
            BATLEZ,
            Observed,
        )
        .details("Same as Always show all icons in the notification area on Windows 10. Nothing stays tucked behind the arrow. Windows 11 ignores it, so choose your icons in taskbar settings there.")
        .sign_out()
        .until(21999),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "EnableAutoTray",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.small-icons",
            Visuals,
            "Taskbar",
            "Small taskbar icons",
            "Makes the taskbar and its icons smaller on Windows 10.",
            BATLEZ,
            Observed,
        )
        .details("Same as Use small taskbar buttons in Windows 10 taskbar settings, which gives a slimmer taskbar and more room on screen. Windows 11 ignores this value.")
        .until(21999),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarSmallIcons",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-glom",
            Visuals,
            "Taskbar",
            "Never combine taskbar buttons",
            "Keeps each window as its own labelled taskbar button.",
            BATLEZ,
            Observed,
        )
        .details("Same as setting Combine taskbar buttons to Never. You can see every window at a glance with its title, which is useful with lots open. The taskbar fills up sooner."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarGlomLevel",
            2,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.taskbar-flash",
            Visuals,
            "Taskbar",
            "No taskbar flashing",
            "Stops taskbar buttons flashing when an app wants your attention.",
            TAIRIKU,
            Observed,
        )
        .details("Same as the Show flashing on taskbar apps switch on Windows 11. It helps when a background app would pull your eye away from a game. You may miss a message waiting for you.")
        .builds(22621),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarFlashing",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.start-recommend",
            Visuals,
            "Start menu",
            "No Start recommendations",
            "Stops Start suggesting tips, shortcuts and new apps in Recommended.",
            TAIRIKU,
            Observed,
        )
        .details("Same as the recommendations switch in Start settings on Windows 11. The section gets quieter, and recently added apps and files have their own switches.")
        .builds(22621),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_IrisRecommendations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.start-docs",
            Privacy,
            "Windows data",
            "No recent items in Start",
            "Stops Start, jump lists and File Explorer tracking the files you open.",
            BATLEZ,
            Observed,
        )
        .details("Same as the recently opened items switch in Start settings. Your recent files no longer show up for anyone glancing at the screen, but you lose quick access to them."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_TrackDocs",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.news-interests",
            Visuals,
            "Search and widgets",
            "Turn off the Widgets board",
            "Turns off the Widgets board and its news feed on Windows 11.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Goes further than hiding the button: the board won't open, and its news and content stop loading. You lose weather and other widgets too.")
        .sign_out(),
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
            "privacy.search-highlights",
            Visuals,
            "Search and widgets",
            "No search highlights",
            "Removes the daily pictures and trending topics from the search box.",
            POLICY_DOCS,
            Documented,
        )
        .admin()
        .details("Search highlights fill the search box and panel with content from Bing. This leaves a plain search box and a list of your recent items, and stops that content loading.")
        .sign_out(),
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
            "windows.widgets",
            Visuals,
            "Search and widgets",
            "Hide the Widgets button",
            "Removes the Widgets button and weather from the taskbar.",
            BATLEZ,
            Observed,
        )
        .builds(22000)
        .details("Same as the Widgets switch in taskbar settings on Windows 11. It tidies the taskbar and stops accidental pop-ups on hover. The widgets board itself stays unless you also turn off the feed."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarDa",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.search-box",
            Visuals,
            "Search and widgets",
            "Hide the search box",
            "Removes the search box and icon from the taskbar, freeing up room.",
            BATLEZ,
            Observed,
        )
        .details("Same as choosing Hide for Search in taskbar settings. Search still works: press the Windows key and start typing. It's just out of the way."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Search",
            "SearchboxTaskbarMode",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.icons-only",
            Visuals,
            "View",
            "Icons instead of thumbnails",
            "File Explorer shows plain icons rather than image and video thumbnails.",
            BATLEZ,
            Observed,
        )
        .details("Same as Always show icons, never thumbnails in Folder Options. It can help in huge media folders on slow drives. You lose the picture previews."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "IconsOnly",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-ext",
            Visuals,
            "View",
            "Show file extensions",
            "Shows endings like .exe and .txt on every file name.",
            BATLEZ,
            Observed,
        )
        .details("Same as File name extensions in File Explorer's View menu. You see what a file really is, which helps spot fakes such as invoice.pdf.exe. Take care when renaming."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "HideFileExt",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-hidden",
            Visuals,
            "View",
            "Show hidden files",
            "Shows files and folders marked as hidden, such as AppData.",
            BATLEZ,
            Observed,
        )
        .details("Same as Hidden items in File Explorer's View menu. Useful for finding game saves and config files. They appear faded, and the desktop may show a few extra files."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Hidden",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-system",
            Visuals,
            "View",
            "Show system files",
            "Shows Windows' protected system files in File Explorer.",
            BATLEZ,
            Observed,
        )
        .medium()
        .details("Same as unticking Hide protected operating system files in Folder Options. Useful for troubleshooting, not everyday use, as folders fill up with files that shouldn't be touched.")
        .warn("Deleting or moving these files can stop Windows starting."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSuperHidden",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.full-path",
            Visuals,
            "View",
            "Full path in title",
            "Shows a folder's full path in File Explorer's title, not only its name.",
            BATLEZ,
            Observed,
        )
        .details("Same as Display the full path in the title bar in Folder Options. It helps when several folders share a name, and makes no other change."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState",
            "FullPath",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.info-tip",
            Visuals,
            "View",
            "No hover tips in Explorer",
            "Stops the pop-up details when you hover over files and folders.",
            BATLEZ,
            Observed,
        )
        .details("Same as Show pop-up description for folder and desktop items in Folder Options. It keeps the screen clear while you move around File Explorer. File details are still in the Details pane."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowInfoTip",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.preview-handlers",
            Visuals,
            "View",
            "Turn off file previews",
            "The preview pane stops rendering the contents of selected files.",
            BATLEZ,
            Observed,
        )
        .details("Same as Show preview handlers in preview pane in Folder Options. It avoids slow previews of big files, and files aren't opened just by selecting them. You lose document previews."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowPreviewHandlers",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-frequent",
            Visuals,
            "View",
            "Hide frequent folders",
            "Stops File Explorer listing the folders you use most.",
            BATLEZ,
            Observed,
        )
        .details("Same as Show frequently used folders in Folder Options. Home shows only what you pin, which suits a shared PC. Pinned folders stay where they are."),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowFrequent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                "ShowFrequent",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-recent",
            Visuals,
            "View",
            "Hide recent files",
            "Stops File Explorer listing the files you opened recently.",
            BATLEZ,
            Observed,
        )
        .details("Same as Show recently used files in Folder Options. Recent files stop showing in Home, which helps on a shared screen. Your files aren't affected."),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowRecent",
                0,
            ),
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                "ShowRecent",
                0,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-cloud-recent",
            Visuals,
            "View",
            "Hide cloud files in Home",
            "Keeps cloud files out of File Explorer's Home and recent lists.",
            TAIRIKU,
            Observed,
        )
        .details("Same as the Show files from Office.com option in Folder Options. Online documents stop appearing beside local ones, which suits a shared screen. The files stay in your cloud storage.")
        .builds(22000),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowCloudFilesInQuickAccess",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.show-recommended",
            Visuals,
            "View",
            "Recommendations in Explorer",
            "Removes the Recommended files section from File Explorer's Home page.",
            BATLEZ,
            Observed,
        )
        .details("Same as the Show recommended section option in Folder Options, where your account offers it. It gives you a shorter Home page. PCs that never showed this section see no change.")
        .builds(22621),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer",
            "ShowRecommendations",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.enthusiast",
            Visuals,
            "View",
            "Detailed copy dialog",
            "Copy and move dialogs open showing speed, time left and file count.",
            TAIRIKU,
            Observed,
        )
        .details("Same as clicking More details in a transfer dialog, but it sticks. You see the speed graph and the items left without an extra click."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\OperationStatusManager",
            "EnthusiastMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.folder-type",
            Visuals,
            "View",
            "Generic folder views",
            "Stops Explorer guessing folder types, so big folders can open faster.",
            BATLEZ,
            Observed,
        )
        .details("Explorer normally inspects a folder's contents to pick a Pictures or Music layout, which can be slow in large Downloads folders. This uses one general layout everywhere. Folders you've set a view for keep it."),
        settings: &[hkcu_str(
            r"Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags\AllFolders\Shell",
            "FolderType",
            "NotSpecified",
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.compact-mode",
            Visuals,
            "View",
            "Compact File Explorer",
            "Tightens the spacing in File Explorer so more files fit on screen.",
            BATLEZ,
            Observed,
        )
        .builds(22000)
        .details("Same as Compact view in File Explorer's View menu. Rows and the navigation pane sit closer together, as they did before Windows 11. It makes touch use a little harder."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "UseCompactMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.nav-expand",
            Visuals,
            "Navigation pane",
            "Expand to open folder",
            "The navigation pane follows you and opens to the folder you're in.",
            TAIRIKU,
            Observed,
        )
        .details("The same as Expand to open folder in Folder Options. It helps you see where you are in a deep folder tree, at the cost of a longer side pane."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "NavPaneExpandToCurrentFolder",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.hub-mode",
            Visuals,
            "Navigation pane",
            "Hide Home in File Explorer",
            "Removes Home and Quick access from the top of the navigation pane.",
            BATLEZ,
            Observed,
        )
        .admin()
        .details("Gives a shorter navigation pane that starts at your drives and folders. If File Explorer opens on Home, set it to open on This PC. Pinned folders leave the pane too. It applies to every account on this PC.")
        .sign_out(),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
            "HubMode",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.home-namespace",
            Visuals,
            "Navigation pane",
            "Hide Gallery",
            "Removes Gallery from File Explorer's navigation pane.",
            TAIRIKU,
            Observed,
        )
        .details("Gallery only collects your photo folders, so hiding it shortens the side pane on Windows 11. Your photos aren't touched. A Windows update can bring the entry back.")
        .builds(22631),
        settings: &[hkcu(
            r"Software\Classes\CLSID\{e88865ea-0e1c-4e20-9aa6-edcd0212c87c}",
            "System.IsPinnedToNameSpaceTree",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.send-to-drives",
            Visuals,
            "Context menu",
            "No drives in Send to",
            "Removes drives from the Send to menu, leaving a shorter list.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Send to lists every drive, which gets long with card readers and USB sticks. This hides them from that submenu. You can still open drives in File Explorer as normal. It applies to every account on this PC.")
        .sign_out(),
        settings: &[
            hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoDrivesInSendToMenu",
                1,
            ),
            hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoDrivesInSendToMenu",
                1,
            ),
        ],
    },
    SettingsTweak {
        meta: lib(
            "windows.launch-pc",
            Visuals,
            "Open and save",
            "Open to This PC",
            "File Explorer opens on This PC, showing your drives, rather than Home.",
            BATLEZ,
            Observed,
        )
        .details("Same as the Open File Explorer to option in Folder Options. You land on your drives straight away, and recent files stay out of sight until you go to Home."),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "LaunchTo",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.open-with-store",
            Visuals,
            "Open and save",
            "No Store in Open with",
            "Removes the Store suggestion from Open with and unknown-file prompts.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Windows stops offering to look for an app in the Store when it can't open a file. You pick from apps you already have. The Store itself still works. It applies to every account on this PC."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
            "NoUseStoreOpenWith",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "privacy.cdm-lockscreen",
            Visuals,
            "Lock and sign-in",
            "Turn off lock screen Spotlight",
            "Stops the lock screen rotating Microsoft pictures and captions.",
            BATLEZ,
            Observed,
        )
        .details("Turns off Windows Spotlight images and the slideshow option on the lock screen, so Windows stops fetching new pictures for it. You may need to choose a picture in Settings."),
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
            "windows.lock-notifications",
            Visuals,
            "Lock and sign-in",
            "No lock screen notifications",
            "Stops notifications and reminders appearing on the lock screen.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Messages and calendar alerts stay private while the PC is locked. You still see them after signing in. It applies to every account on the PC."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableLockScreenAppNotifications",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.logon-animation",
            Visuals,
            "Lock and sign-in",
            "Sign-in screen blur off",
            "Shows your sign-in background sharp, without the frosted blur.",
            TAIRIKU,
            Documented,
        )
        .admin()
        .details("Windows normally blurs the picture behind the sign-in box. This keeps it clear, which suits a favourite lock screen image. It makes no other change. It applies to every account on this PC."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableAcrylicBackgroundOnLogon",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "windows.account-details",
            Visuals,
            "Lock and sign-in",
            "Hide email at sign-in",
            "Hides your email address on the sign-in screen.",
            BATLEZ,
            Documented,
        )
        .admin()
        .details("Stops your account's email address showing on the sign-in screen. Handy on a laptop used in public. Signing in works the same. It applies to every account on this PC."),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "BlockUserFromShowingAccountDetailsOnSignin",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "power.startup-sound",
            Visuals,
            "Startup and shutdown",
            "Turn off the startup sound",
            "Windows starts silently, without the chime at the sign-in screen.",
            TAIRIKU,
            Observed,
        )
        .admin()
        .details("Same as unticking Play Windows Startup sound in Sound settings. Nice for a PC that starts early or in a shared room. It only affects that one sound. It applies to every account on this PC.")
        .builds(22000),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\EditionOverrides",
            "UserSetting_DisableStartupSound",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.accent-title-bars",
            Visuals,
            "Theme and colour",
            "Accent colour on title bars",
            "Colours window title bars and borders with your accent colour.",
            COLOUR_DOCS,
            Observed,
        )
        .details("The same switch as Settings > Personalisation > Colours. Makes the active window easier to spot. Title bars go back to plain when you undo it.")
        .added("0.5"),
        settings: &[hkcu(r"Software\Microsoft\Windows\DWM", "ColorPrevalence", 1)],
    },
    SettingsTweak {
        meta: lib(
            "visuals.accent-from-wallpaper",
            Visuals,
            "Theme and colour",
            "Accent from wallpaper",
            "Windows picks your accent colour from the current wallpaper.",
            COLOUR_DOCS,
            Observed,
        )
        .details("The Automatic choice in Settings > Personalisation > Colours. The accent follows each new wallpaper, which replaces any colour you picked by hand.")
        .sign_out()
        .added("0.5"),
        settings: &[hkcu(r"Control Panel\Desktop", "AutoColorization", 1)],
    },
    SettingsTweak {
        meta: lib(
            "visuals.desktop-this-pc",
            Visuals,
            "Windows and desktop",
            "This PC on the desktop",
            "Adds the This PC icon to your desktop.",
            DESKTOP_DOCS,
            Observed,
        )
        .details("The same as ticking Computer in Desktop icon settings. Gives you a quick way into your drives from the desktop.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
            "{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.recycle-bin-off",
            Visuals,
            "Windows and desktop",
            "Hide the Recycle Bin icon",
            "Takes the Recycle Bin off your desktop for a cleaner look.",
            DESKTOP_DOCS,
            Observed,
        )
        .details("The same as unticking Recycle Bin in Desktop icon settings. Deleted files still go to the Recycle Bin, and you can still reach it from File Explorer.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
            "{645FF040-5081-101B-9F08-00AA002F954E}",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.shake-off",
            Visuals,
            "Windows and desktop",
            "Title bar shake off",
            "Shaking a window no longer minimises every other window.",
            &["https://winaero.com/enable-minimize-windows-with-title-bar-shake-in-windows-11-aero-shake"],
            Observed,
        )
        .details("Stops windows vanishing when you drag one around quickly. Windows 11 ships with this off, so it mainly matters on Windows 10.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "DisallowShaking",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.clock-seconds",
            Visuals,
            "Taskbar",
            "Seconds on the clock",
            "Shows seconds in the clock at the corner of the taskbar.",
            TASKBAR_DOCS,
            Observed,
        )
        .details("The same as Show seconds in system tray clock in Taskbar settings. Handy for timing things. The clock then updates every second, which Microsoft notes uses a little more power.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowSecondsInSystemClock",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.taskbar-all-displays",
            Visuals,
            "Taskbar",
            "Taskbar on all displays",
            "Shows the taskbar on every monitor, not just the main one.",
            TASKBAR_DOCS,
            Observed,
        )
        .details("The same as Show my taskbar on all displays in Taskbar settings. Only matters with more than one monitor.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "MMTaskbarEnabled",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.start-account-alerts",
            Visuals,
            "Start menu",
            "No account alerts in Start",
            "Removes the account reminders shown by your picture in Start.",
            START_DOCS,
            Observed,
        )
        .details("The same as turning off account-related notifications in Start settings. Hides prompts such as finishing your Microsoft account setup.")
        .builds(22621)
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Start_AccountNotifications",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.status-bar-off",
            Visuals,
            "View",
            "Hide the status bar",
            "Removes the strip at the bottom of File Explorer windows.",
            EXPLORER_DOCS,
            Observed,
        )
        .details("A Folder Options setting. Gives you one more row of files, but you lose the item count and the buttons that switch between details and thumbnails.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowStatusBar",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.item-checkboxes",
            Visuals,
            "View",
            "Item check boxes",
            "Adds a tick box to each file so you can select several without Ctrl.",
            EXPLORER_DOCS,
            Observed,
        )
        .details("The same as View > Show > Item check boxes in File Explorer. Most useful on a touch screen or when picking a few files from a long list.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "AutoCheckSelect",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.nav-all-folders",
            Visuals,
            "Navigation pane",
            "Show all folders",
            "Lists Control Panel, Recycle Bin and your user folder in the side pane.",
            EXPLORER_DOCS,
            Observed,
        )
        .details("A Folder Options setting. Turns the navigation pane into a full tree like older versions of Windows, at the cost of a longer list.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "NavPaneShowAllFolders",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.classic-context-menu",
            Visuals,
            "Context menu",
            "Full right-click menu",
            "Brings back the full right-click menu, without Show more options.",
            &["https://www.winhelponline.com/blog/get-classic-full-context-menu-windows-11/"],
            Community,
        )
        .details("Windows 11 hides many commands behind Show more options. This goes straight to the full menu. Microsoft doesn't document it, so an update could stop it working. You lose the short menu's icon row.")
        .builds(22000)
        .sign_out()
        .added("0.5"),
        settings: &[hkcu_str(
            r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
            "",
            "",
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.plain-sign-in",
            Visuals,
            "Lock and sign-in",
            "Plain sign-in background",
            "Shows a solid colour behind the sign-in box instead of a picture.",
            &["https://winaero.com/disable-logon-screen-background-image-in-windows-10-without-using-third-party-tools"],
            Observed,
        )
        .details("Removes the picture from the sign-in screen and uses your accent colour. The lock screen before it keeps its own picture. It applies to every account on this PC.")
        .admin()
        .sign_out()
        .added("0.5"),
        settings: &[hklm(
            r"SOFTWARE\Policies\Microsoft\Windows\System",
            "DisableLogonBackgroundImage",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.first-logon-animation-off",
            Visuals,
            "Startup and shutdown",
            "No first sign-in animation",
            "New accounts skip the Hi and Getting things ready screens.",
            &["https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-windowslogon"],
            Documented,
        )
        .details("The Show first sign-in animation policy. It only matters the first time an account signs in on this PC, such as a new user or a clean install, so an account already set up sees no change. It applies to every account on this PC.")
        .admin()
        .added("0.5"),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "EnableFirstLogonAnimation",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.verbose-status",
            Visuals,
            "Startup and shutdown",
            "Detailed startup messages",
            "Shows what Windows is doing during startup and shutdown.",
            &["https://learn.microsoft.com/en-us/troubleshoot/windows-server/performance/enable-verbose-startup-shutdown-logon-logoff-status-messages"],
            Documented,
        )
        .details("Replaces Please wait with messages such as which service is starting. Useful for finding out what's holding up a slow start or shutdown. It applies to every account on this PC.")
        .admin()
        .added("0.5"),
        settings: &[hklm(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            "VerboseStatus",
            1,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.taskbar-badges-off",
            Visuals,
            "Taskbar",
            "No badges on taskbar apps",
            "Removes unread counts and alert dots from taskbar icons.",
            TASKBAR_DOCS,
            Observed,
        )
        .details("The same as Show badges on taskbar apps in Taskbar settings. Keeps the taskbar calm, but you won't see at a glance that an app has something new.")
        .builds(22000)
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarBadges",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.onedrive-nav-off",
            Visuals,
            "Navigation pane",
            "Hide OneDrive",
            "Removes OneDrive from File Explorer's navigation pane.",
            &["https://pureinfotech.com/remove-onedrive-file-explorer-windows-11/"],
            Community,
        )
        .details("OneDrive keeps syncing and your files stay where they are. You just reach them through your user folder. A Windows update can bring the entry back.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Classes\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
            "System.IsPinnedToNameSpaceTree",
            0,
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.menu-give-access-off",
            Visuals,
            "Context menu",
            "Remove Give access to",
            "Takes Give access to off the right-click menu.",
            &["https://www.elevenforum.com/t/add-or-remove-give-access-to-context-menu-in-windows-11.11209/"],
            Community,
        )
        .details("A shorter menu if you never share folders over a network. Sharing still works from a folder's Properties.")
        .sign_out()
        .added("0.5"),
        settings: &[hkcu_str(
            r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
            "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}",
            "",
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.menu-cast-off",
            Visuals,
            "Context menu",
            "Remove Cast to Device",
            "Takes Cast to Device off the right-click menu.",
            &["https://www.howtogeek.com/720449/how-to-remove-cast-to-device-option-from-windows-10-context-menu/"],
            Community,
        )
        .details("A shorter menu if you never cast media to a TV or speaker. Casting from an app or from Settings isn't affected.")
        .sign_out()
        .added("0.5"),
        settings: &[hkcu_str(
            r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
            "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}",
            "",
        )],
    },
    SettingsTweak {
        meta: lib(
            "visuals.restart-apps",
            Visuals,
            "Lock and sign-in",
            "Reopen apps after sign-in",
            "Apps that support it reopen where you left off when you sign back in.",
            TAIRIKU,
            Observed,
        )
        .details("The same as the restartable apps switch in Sign-in options. Windows saves apps that support it when you sign out or restart, then opens them again. Apps that don't support it stay closed.")
        .added("0.5"),
        settings: &[hkcu(
            r"Software\Microsoft\Windows NT\CurrentVersion\Winlogon",
            "RestartApps",
            1,
        )],
    },
];
