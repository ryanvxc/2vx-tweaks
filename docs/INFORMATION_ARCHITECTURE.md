# Information architecture

Every feature has one primary location. Other pages may summarise it and link there; they don't rebuild it.

## Source of truth

| Information | Primary location | Elsewhere |
| --- | --- | --- |
| Live usage, history, per-thread load | Monitoring | Home chart (the one exception, kept by request: it names the device) |
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
| **Home** | Is my PC okay, and what should I do next? | Backups, tweaks in effect, live chart, Quickstart, Patch Notes, newest change, health | Everything else |
| **Monitoring** | What is it doing right now? | Four live tiles that choose the graph | Per-thread load (CPU only) |
| **Hardware** | What can I tune on each part? | Tabs per part, a device chip with the specifications, tweak cards | – |
| **Network** | What can I tune on my connection? | Tweaks, Network Priority, Adapter Tuner, Bufferbloat | – |
| **Advanced** | What's protecting this PC, and how does it run? | Security status, MSI mode, device settings, timer resolution | – |
| **Tweaks** | What can I change? | Tabs, search, filters, cards in titled sections, with anything this PC can't run in a last Unavailable section | Details dialog with values and sources |
| **Library** | What else can I change? | Presets, a tab per category with collapsible sections, search across every tab | Details dialog with evidence, values and sources |
| **Game Mode** | Is a game running, and what should change for it? | Ring, detected game, live CPU and GPU | Configure (what it turns on), Add a Game |
| **PC Cleaner** | What can I clear, turn off or remove? | Four tabs: System Cleaner, Services, Uninstall, Autoruns | The review before each change |
| **Restore Centre** | What did 2VX change, and can I undo it? | One table of batches | Exact old and new values |
| **Resources** | How do I get a report or open a Windows tool? | Report, tools | – |
| **Settings** | How do I change the app? | Everything on one page | – |

## Progressive disclosure

1. A **card** shows icon, title, one short description, up to three indicators and the switch.
2. A **tooltip** explains an indicator: what breaks, what needs a restart.
3. The **details dialog** shows the explanation, every value and its documentation.
4. The **review dialog** shows the dry-run plan, risks, restart needs and administrator prompt before anything changes.
5. **Planned lists** are collapsed; they're honest but not what a page is for.

## Tweak organisation

Engine categories describe behaviour. People look for a topic, so the UI groups tweaks in presentation code
(`src/lib/tweak-presentation.ts`), and the Rust catalogue stays unchanged. A test reads the Rust files and fails if a
tweak has no placement or icon, or a profile names a tweak that doesn't exist.

The candidate groups CPU, GPU, RAM, Storage, Network, Power and Peripherals have 0 or 1 tweaks today, so they aren't
tabs. A tab must have enough content to justify itself; four tabs hold 33 tweaks today, and new ones can join when
tweaks exist for them. Full placement is in `FEATURE_CATEGORISATION.md`.

## Profiles

A profile is a named list of tweak IDs in `src/lib/profiles.ts`. It adds no new system behaviour:

- It has no stored "active" flag. A profile is active when every tweak in it that this PC can use is in effect.
- Selecting tweaks in a profile opens the normal review dialog. Each tweak can be left out first.
- It says what each included tweak does and what it can break, and marks tweaks this PC can't use.
- It makes no FPS claim.

## URLs

Tabs live in the URL (`?tab=`), so the back button, search and deep links work. Old URLs redirect:
`/system` → `/tweaks`, `/history` → `/restore`. Settings has no tabs, so old `/settings?tab=…` links land on the page.
