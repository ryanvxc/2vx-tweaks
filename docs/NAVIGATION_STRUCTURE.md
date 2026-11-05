# Navigation structure

Defined in `src/app/navigation.ts`, which the sidebar, title bar and search all read. The sidebar and account menu use
Poppins; everything else uses Geist.

```
General
  Home             /
  Restore Centre   /restore
  Resources        /tools

Tools
  2VX Game Mode    /gaming              ?view=configure (brand colour)

Optimisations
  Tweaks           /tweaks              ?tab=core | privacy | gaming | apps | tasks | power | presets
  Visuals          /visuals             ?tab=appearance | taskbar | explorer | signin
  Hardware         /hardware            ?tab=gpu | cpu | ram | peripherals | storage
  PC Cleaner       /cleaner             ?tab=system | services | uninstall | autoruns
  Network          /network             ?tab=tweaks | priority | adapters | bufferbloat
  Advanced         /advanced            ?tab=security | msi | devices | timer

[Account card]     opens the account menu
```

Eleven rows in three groups, named as EXM names them. The groups sit together at the top, 20 px apart, and any spare height
goes above the account card, never between groups. A row is 46 px, down to 34 px on a short screen. Settings is also in
the account menu, as it is in EXM. Search is in the account menu, and Ctrl K opens it from anywhere. The title bar holds
only minimise and close.

## Account menu

The card at the bottom of the sidebar shows the account's name and the Windows privilege level. Its menu holds the email
and version, Search, Settings, Hardware, Check for updates and Remove account. The card shows a dot when an update is
waiting on the user, because the update row is hidden until the menu opens.

The account is an email saved on this PC (see `security.md`). Remove account deletes that file and nothing else, after
a confirmation. With none saved, the menu offers Set up account instead.

## Why this shape

- **Game Mode stands alone** in the brand colour: a person who wants games to run better shouldn't have to read a list.
  Everything else that changes the PC or reports on it is under Optimisations.
- **One Tweaks page.** The Library was a second page with the same cards, so its tweaks moved to where their topic
  already was: Core, Privacy, Gaming, Apps and Power on Tweaks; how Windows looks (theme, taskbar, File Explorer, sign-in) on
  Visuals, placed by section; CPU, GPU, memory and storage on Hardware; network
  on Network; services on PC Cleaner. Search covers every tab of a page, so nothing needs a page of its own to be found.
- **Services have one home**, PC Cleaner, whether a service is switched by hand or through a preset.
- **Monitoring folded into Home.** Home already had the chart; CPU, GPU, RAM and Network views with a one or five minute
  range cover what the page did. `/monitoring` lands on Home.
- **A row is always a real destination**, never padding to fill the sidebar. That's why there are no Support, BIOS or
  Debloat rows. Settings holds General's fourth row, the one EXM gives to Support.
- **History became Restore Centre**: it says what the page is for.
- **Resources stays small** but stays in the sidebar: the report and Windows tools have no better home. Its URL is still
  `/tools`.

## Moving around

| Pattern | Use |
| --- | --- |
| Sidebar | Top-level places |
| Tabs (`PageTabs`) | Sections of one page; URL-backed |
| Search (Ctrl K) | Jump to any page, tab or tweak |
| Page title | In the page (`PageHero`), with the window controls alone in a thin bar above; no breadcrumb |
| Contextual links | Home → primary locations |

## Search

Solves one real problem: over a thousand tweaks across many tabs. It searches pages, tabs and tweak names and descriptions.
Selecting a tweak opens its details in the right tab. Recent items and quick actions weren't added: with this few
pages they'd solve nothing. Each tweak page also has its own search, which looks through every tab of that page and
matches the registry or service names a tweak touches.

## Collapsed sidebar

Icons only, 64 px wide, a tooltip per item. The account card reduces to its avatar, which keeps the update dot and opens
the same menu to the right. Settings › Appearance › Compact sidebar turns it on.

## Redirects

| Old | New |
| --- | --- |
| `/system`, `/system?tweak=…` | `/tweaks`, keeping the query |
| `/history` | `/restore` |
| `/monitoring` | `/` |
| `/library`, `/library?tweak=…` | `/tweaks`, keeping the query |
| `/startup` | `/cleaner?tab=autoruns` |
| `/settings#updates`, `/settings?tab=…` | `/settings` (one page) |
