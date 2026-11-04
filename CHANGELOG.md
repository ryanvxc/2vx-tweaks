# Changelog

Main-channel builds are released automatically from `main` (see `docs/GITHUB_RELEASE_PIPELINE.md`); their
notes come from commit subjects. This file records the larger milestones.

## 0.2.0 (unreleased)

### Updates
- Check for updates now works in every release build, including ones built on your own PC.
- Slow connections no longer fail the download after 30 seconds.

### Tweaks
- Tweaks is rebuilt: a new header and tab bar, larger cards, and sections inside each tab.
- Tabs are now Core, Privacy, Apps and Services. Windows became Core, and the Microsoft Edge tweaks moved to Apps.
- Tweaks that can't run on this PC always sit in an Unavailable section at the bottom.

### PC Cleaner
- PC Cleaner now has four tabs: System Cleaner, Services, Uninstall and Autoruns.
- System Cleaner shows what each location holds. Tick what you want, or pick Recommended, then Start cleaning. Files are
  deleted for good, and temp folders keep anything from the last 24 hours.
- Services has two presets, Basic and Advanced, built from tweaks you can review and undo one at a time. Neither touches
  Windows Update, Defender or networking.
- Uninstall removes built-in Windows apps and brings them back while Windows keeps a copy.
- Autoruns replaces the Startup page. Every entry shows its path, publisher and description, and has a switch.
- Two new tweaks turn off the Telemetry and Geolocation services.

### Home
- Home is rebuilt: a greeting by account name, three cards that light up with rising dots on hover, a 60-second chart
  with CPU, GPU and RAM tabs, a Quickstart checklist, and Updates below the fold.
- Quickstart ticks what it can read from the PC. The rest tick once they've been opened.
- Patch Notes reads this file, so it works offline. System health opens in a dialog.
- The chart opens on All, with CPU, GPU and RAM on one graph. The row is shorter, so Updates shows sooner, and Quickstart
  scrolls.
- The Updates cards use original art painted for 2VX.

### Game Mode
- It's now called 2VX Game Mode, with its title at the top of the page, more room for the four promises and Configure
  right under the ring.
- Game Mode is one screen: a ring, the detected game, how long it has run and live CPU and GPU use. The ring turns on
  once a game is running.
- Games in Steam, Epic, GOG, Xbox, Ubisoft, EA and Riot folders are found by themselves. Add a Game covers the rest.
- Turning it on opens the usual review. Turning it off undoes that batch and nothing else, and it stays on until you do.
- Configure chooses what it turns on, with the three profiles as presets. Only changes that need no restart are on by
  default. Game DVR and MMCSS moved to Tweaks, with every other switch.

### Sidebar and account
- The sidebar is rebuilt in Poppins, with larger rows and three groups: General, Tools and Optimisations. Gaming is now
  Game Mode, in the brand colour, and Tools is now Resources.
- An account card at the bottom opens a menu with Settings, System detection, Check for updates and Log out. The account
  is an email saved on this PC: there's no server, password or sign-in, and it's never sent anywhere.
- The sidebar now fills its height down to the account card, at any window size. Windows, Privacy and Services have their
  own rows, and they follow the tabs on the Tweaks page.
- The window now has one size, 1280×820, and only minimise and close. A line separates the logo from the sidebar's rows.
- Search moved to the account menu. Ctrl K still opens it. Settings › Compact sidebar replaces the collapse button.
- The compact sidebar's links were losing their styling; they're centred and coloured properly now.

### Layout
- Pages fill the window: each page title moved into the title bar, filters
  moved up beside the tabs, and every page reaches the bottom of the window.
- Restore, Startup and PC Cleaner are single tables, Hardware is one specification sheet, Gaming shows all three
  profiles side by side, and Settings is one page instead of four tabs.
- Pill boxes are gone: state is a dot and a word, data sources are a dot, and buttons, indicators and rows lost their
  boxes. Cards have a filled surface instead of a visible outline.
- Tweaks has three tabs of similar size: Apps joined Windows, and the headings inside a tab are gone.
- Geist replaces Inter and Manrope.

### Motion and copy
- The app now responds to you: the sidebar, tabs, chart filter and selection outlines slide, pages and lists arrive,
  figures count up, buttons and switches react to a press, and a tweak's card pulses green the moment it takes effect.
- Settings › Reduce motion is the one control. The Windows animation setting no longer affects the app.
- Far less text: page descriptions, explanatory paragraphs and setting descriptions are gone, and tweak descriptions fit
  on their cards. Settings' Security tab, Home's Quickstart and the "planned" lists are removed.

### Loading screen
- 2VX now opens on a loading screen: dots form the logo as the real startup calls finish, with a fact or tip underneath.
  The cursor pushes the dots, clicking sends a pulse, Space shows another fact and Esc skips.
- It takes about 3 seconds and can't get stuck. Settings › Appearance turns it off, and Reduce motion replaces it with
  the static logo. The Windows animation setting doesn't affect it, because it's often off for speed.

### Makeover
- New structure, one primary location per feature: Overview (Home, Monitoring, Hardware), Optimise
  (Tweaks, Gaming), Maintain (Startup, PC Cleaner, Restore Centre, Tools). See
  `docs/INFORMATION_ARCHITECTURE.md`. `/system` and `/history` redirect, and old Settings links still work.
- Home is an overview that fits the default window: alerts, tweaks in effect, gaming profile, chart, health,
  recommendations, this PC, recent changes.
- Monitoring holds live data; its four tiles choose the graph. Hardware is now a specification sheet.
- Tweaks are grouped by topic: Windows, Privacy, Apps and Services, with sections and a search across
  all of them. Each tweak has its own icon, and review lists what a selection can break.
- Gaming profiles (Responsive, Quiet background, Everything) are named lists of existing tweaks: each shows
  what it changes, anything can be left out, and everything goes through the normal review. A profile counts
  as active only when its tweaks are detected in effect. No FPS claims.
- Settings has tabs. Restore Centre replaces History and opens with a summary. PC Cleaner shows a total and a
  ranked list.
- Ctrl K searches pages, tabs and tweaks. The title bar shows where you are.
- Darker surfaces, a stronger magenta and Manrope headings.

### Redesign
- New visual identity: charcoal surfaces, violet icons and focus, magenta primary actions and toggles;
  Inter typography scale; reduced-motion support. See `docs/DESIGN_SYSTEM.md`.
- Grouped sidebar with privilege and version footer; compact mode.
- Home: greeting, stat tiles, live utilisation chart with a series filter, Quickstart, recommended
  tweaks derived from detected state (the sample recommendations are gone), health and recent changes.
- System Tweaks and Gaming: card grid with category tabs, search, filters, sort, impact estimate,
  indicators, "Stage recommended", a review bar and a details dialog with values and references.
- Settings: row-based sections (General, Appearance, Notifications, Updates, Behaviour, Security, About).
- Sidebar: "Check for updates" checks, downloads and then offers "Restart to update".

### Features
- Switch a tweak off to undo exactly what 2VX changed (`revert_tweak`), after a confirmation.
- Four new documented tweaks: Edge shopping assistant, advertising ID, Edge sidebar, Edge
  recommendations.
- 27 more tweaks from a review of a 375+ line tweak pack: privacy, gaming, performance and one-at-a-time
  service switches. Only Microsoft-documented settings qualified; the rest, and why, is in
  `docs/optimisation-modules.md`.
- Cards show an impact rating, and a red warning only when a tweak can break a named feature.
- Tweaks can require an edition (Pro and above) or an installed service, and say so when they can't apply.
- Hardware page: per-thread CPU load, GPU, memory, storage and network tabs (replaces Monitoring).
- Startup page: read-only inventory of Run/RunOnce values and Startup-folder shortcuts, with Task
  Manager's disabled state.
- PC Cleaner: read-only scan of allowlisted temp, cache and error-report folders and the Recycle Bin.
- Tools: copyable system report and Windows tool commands.
- Signed in-app updates (Tauri updater) with an Update Centre, release notes before install, an optional
  automatic check and a Home notice. Nothing downloads or installs without a click.

### Build and release
- GitHub Actions: CI on pull requests and branches; main-channel release with monotonic versions,
  signed NSIS prerelease and a validated update feed. Publishing needs the signing secrets
  (`docs/UPDATE_SETUP_GUIDE.md`).
- App icons regenerated from the new logo.

## 0.1.0

- Stage 1 foundation (Tauri 2 + React shell, dashboard, live metrics) and stage 2 safety engine
  (journal, plan/apply/verify/rollback/revert, elevation broker, policy denylist).
- Stage 3 first optimisations: four documented tweaks, change inspector, History with undo.
- Per-machine NSIS installer into Program Files.
