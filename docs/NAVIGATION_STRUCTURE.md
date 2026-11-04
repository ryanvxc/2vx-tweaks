# Navigation structure

Defined in `src/app/navigation.ts`, which the sidebar, title bar and search all read.

```
[Search… Ctrl K]

Overview
  Home             /
  Monitoring       /monitoring          ?tab=cpu | memory | gpu | network
  Hardware         /hardware

Optimise
  Tweaks           /tweaks              ?tab=windows | privacy | services
  Gaming           /gaming

Maintain
  Startup          /startup             ?tab=all | currentUser | allUsers
  PC Cleaner       /cleaner
  Restore Centre   /restore
  Tools            /tools

───────
Check for updates
Settings           /settings
[Privilege chip · version · collapse]
```

Nine top-level items plus Settings, in three groups named after what you're doing: looking, changing, looking after.

## Why this shape

- **Monitoring and Hardware are separate**: one answers "right now", the other "what is this PC". The tiles on
  Monitoring are its selector, so it needs no second tab row.
- **Tweaks and Gaming are the only optimise pages**: a person who wants games to run better goes to Gaming; anything
  else goes to Tweaks. Service switches are a Tweaks tab, not a page.
- **Network, Power and Applications are not top-level**: nothing exists for them yet. An empty page would be navigation
  noise.
- **History became Restore Centre**: it says what the page is for.
- **Tools stays small** but stays in the sidebar: the report and Windows tools have no better home.

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

Icons only, 64 px wide, a tooltip per item, the update dot stays, the search button stays. The privilege chip reduces to
its shield.

## Redirects

| Old | New |
| --- | --- |
| `/system`, `/system?tweak=…` | `/tweaks`, keeping the query |
| `/history` | `/restore` |
| `/settings#updates`, `/settings?tab=…` | `/settings` (one page) |
