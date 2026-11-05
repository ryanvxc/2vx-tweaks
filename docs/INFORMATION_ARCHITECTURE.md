# Information architecture

Every feature has one primary location. Other pages may summarise it and link there; they don't rebuild it.

## Source of truth

| Information | Primary location | Elsewhere |
| --- | --- | --- |
| Live usage over one or five minutes | Home's chart | – |
| Specifications, drives, board, Windows edition | Hardware (the chip's menu) | – |
| Whether a tweak is in effect | Tweaks (read from the PC) | Home counts; Game Mode and its presets reference tweaks by ID |
| Past changes and undo | Restore Centre | Home: the newest one and the backup count; a tweak's switch can undo that tweak |
| System health | Home (a dialog from the System health card) | – |
| Update state and release notes | Settings › Updates | Account menu (fast path), dot on the account card, Home notice (alert) |
| Privilege level | Settings › General | Account card |
| App version | Settings › About | Account menu |

## What each page answers

| Page | Question | First screen | Behind a click |
| --- | --- | --- | --- |
| **Home** | Is my PC okay, and what is it doing? | Backups, health, tweaks in effect, live chart, Quickstart, Updates | Everything else |
| **Hardware** | What can I tune on each part? | Tabs per part, a device chip with the specifications, tweak cards | – |
| **Network** | What can I tune on my connection? | Tweaks, Network Priority, Adapter Tuner, Bufferbloat | – |
| **Advanced** | What's protecting this PC, and how does it run? | Security status, MSI mode, device settings, timer resolution | – |
| **Tweaks** | What can I change? | Tabs by topic, search, filters, sort; every tweak a card, in sections, with anything this PC can't run in a last Unavailable section | Details dialog with values, evidence and sources |
| **Game Mode** | Is a game running, and what should change for it? | Ring, detected game, live CPU and GPU | Configure (what it turns on), Add a Game |
| **PC Cleaner** | What can I clear, turn off or remove? | Four tabs: System Cleaner, Services, Uninstall, Autoruns | The review before each change |
| **Restore Centre** | What did 2VX change, and can I undo it? | One table of batches | Exact old and new values |
| **Resources** | How do I get a report or open a Windows tool? | Report (copy, export), Windows tools | – |
| **Settings** | How do I change the app? | Everything on one page | – |

## Progressive disclosure

1. A **card or row** shows icon, title, one short description, indicators and the switch.
2. A **tooltip** explains an indicator: what breaks, what needs a restart.
3. The **details dialog** says what it does and why, then effect, risk, permission, restart, undo, evidence and any
   hardware or Windows limit, every value and its sources.
4. The **review dialog** shows the dry-run plan, risks, restart needs and administrator prompt before anything changes.

## Tweak organisation

Engine categories describe behaviour. People look for a topic, so the UI groups tweaks in presentation code
(`src/lib/tweak-presentation.ts`), and the Rust catalogue stays unchanged. A test reads the Rust files and fails if a
tweak has no placement or icon, or a profile names a tweak that doesn't exist.

A category maps to one tab (`CATEGORY_TAB`), and each tweak's `section` names its list on that tab, so the 1,221 generated
tweaks place themselves. A short hand-picked set comes first (`TWEAK_SECTIONS`). Sorting by Name,
Expected effect, Status, Newest or Recently used only reorders within this; it never moves a tweak to another page. Full
placement is in `FEATURE_CATEGORISATION.md`.

## Profiles

A profile is a named list of tweak IDs in `src/lib/profiles.ts`. It adds no new system behaviour:

- It has no stored "active" flag. A profile is active when every tweak in it that this PC can use is in effect.
- Selecting tweaks in a profile opens the normal review dialog. Each tweak can be left out first.
- It says what each included tweak does and what it can break, and marks tweaks this PC can't use.
- It makes no FPS claim.

## URLs

Tabs live in the URL (`?tab=`), so the back button, search and deep links work. Old URLs redirect:
`/system` → `/tweaks`, `/history` → `/restore`. Settings has no tabs, so old `/settings?tab=…` links land on the page.
