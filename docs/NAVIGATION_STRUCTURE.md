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
  Game Mode        /gaming              ?view=configure (brand colour)

Optimisations
  Tweaks           /tweaks              ?tab=windows | privacy | services
  Hardware         /hardware
  Startup          /startup             ?tab=all | currentUser | allUsers
  PC Cleaner       /cleaner

[Account card]     opens the account menu
```

Nine top-level items in three groups. Settings has no sidebar row: it's in the account menu, Search and the title of its
own page. Search is the magnifier in the title bar, and Ctrl K opens it from anywhere.

## Account menu

The card at the bottom of the sidebar shows the account's name and the Windows privilege level. Its menu holds the email
and version, Settings, System detection (Hardware), Check for updates and Log out. The card shows a dot when an update
is waiting on the user, because the update row is hidden until the menu opens.

The account is an email saved on this PC (see `security.md`). With none saved, the menu offers Set up account instead of
Log out.

## Why this shape

- **Game Mode stands alone** in the brand colour: a person who wants games to run better shouldn't have to read a list.
  Everything else that changes the PC or reports on it is under Optimisations; service switches are a Tweaks tab, not a
  page.
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

Solves one real problem: 35 tweaks across five tabs. It searches pages, tabs and tweak names and descriptions.
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
