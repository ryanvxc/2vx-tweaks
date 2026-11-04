# Navigation structure

Defined in `src/app/navigation.ts`, which the sidebar, title bar and search all read. The sidebar and account menu use
Poppins; everything else uses Geist.

```
General
  Home             /
  Monitoring       /monitoring          ?tab=cpu | memory | gpu | network
  Restore Centre   /restore
  Resources        /tools

Tools
  2VX Game Mode    /gaming              ?view=configure (brand colour)

Optimisations
  Core             /tweaks?tab=core     (also /tweaks)
  Privacy          /tweaks?tab=privacy
  Apps             /tweaks?tab=apps
  Services         /tweaks?tab=services
  Hardware         /hardware
  PC Cleaner       /cleaner             ?tab=system | services | uninstall | autoruns

[Account card]     opens the account menu
```

Eleven rows in three groups. Rows and groups share out the sidebar's height, so the list ends a little above the account
card at any window size. A row stops growing at 64 px; on a taller window the spare height goes between the groups. The
rows for Core, Privacy, Apps and Services are the Tweaks page's tabs (`tabsInSidebar`), so they stay in step with the tabs
on the page, and Search still lists the page as Tweaks. Settings has no sidebar row: it's in the account menu, Search and the title of its
own page. Search is in the account menu, and Ctrl K opens it from anywhere. The title bar holds only minimise and close.

## Account menu

The card at the bottom of the sidebar shows the account's name and the Windows privilege level. Its menu holds the email
and version, Search, Settings, System detection (Hardware), Check for updates and Log out. The card shows a dot when an update
is waiting on the user, because the update row is hidden until the menu opens.

The account is an email saved on this PC (see `security.md`). With none saved, the menu offers Set up account instead of
Log out.

## Why this shape

- **Game Mode stands alone** in the brand colour: a person who wants games to run better shouldn't have to read a list.
  Everything else that changes the PC or reports on it is under Optimisations; service switches are a Tweaks tab, not
  their own page.
- **Tweaks has three rows**, not one: they're real sections and the most used part of the app. A row is always a real
  destination, never padding to fill the sidebar.
- **Monitoring and Hardware are separate**: one answers "right now", the other "what is this PC". The tiles on
  Monitoring are its selector, so it needs no second tab row.
- **Network, Power and Applications are not top-level**: nothing exists for them yet. An empty page would be navigation
  noise. The same goes for Support, BIOS and Debloat rows.
- **History became Restore Centre**: it says what the page is for.
- **Resources stays small** but stays in the sidebar: the report and Windows tools have no better home. Its URL is still
  `/tools`.

## Moving around

| Pattern | Use |
| --- | --- |
| Sidebar | Top-level places |
| Tabs (`PageTabs`) | Sections of one page; URL-backed |
| Search (Ctrl K) | Jump to any page, tab or tweak |
| Page title | In the title bar, where the sidebar's active item already says the same; no breadcrumb |
| Contextual links | Hardware → Monitoring, Home → primary locations |

## Search

Solves one real problem: 37 tweaks across five tabs. It searches pages, tabs and tweak names and descriptions.
Selecting a tweak opens its details in the right tab. Recent items and quick actions weren't added: with this few
pages they'd solve nothing.

## Collapsed sidebar

Icons only, 64 px wide, a tooltip per item. The account card reduces to its avatar, which keeps the update dot and opens
the same menu to the right. Settings › Appearance › Compact sidebar turns it on.

## Redirects

| Old | New |
| --- | --- |
| `/system`, `/system?tweak=…` | `/tweaks`, keeping the query |
| `/history` | `/restore` |
| `/settings#updates`, `/settings?tab=…` | `/settings` (one page) |
