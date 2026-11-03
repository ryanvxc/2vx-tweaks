# UX audit (0.2, before the makeover)

Audit of the 0.2 app, written before the makeover. Inputs: every page, layout and shared component under `src/`, the
35-tweak Rust catalogue, and the screenshots in the visual reference pack (EXM for presentation quality, 2VX for what
was weak). EXM is a reference for clarity and density only; none of its code, assets or copy is used.

## Main problems

1. **Hardware did two jobs.** Specifications and live graphs shared one page, so neither was quick to read. Usage,
   memory and network numbers repeated on Home.
2. **Home was a stack of everything.** Twelve blocks, three scroll-heights tall at the default window. CPU, memory and
   GPU usage appeared in both the chart and the tiles; hardware facts duplicated the Hardware page; health appeared as
   a tile and a panel.
3. **One flat grid of tweaks.** System Tweaks mixed 33 cards under engine categories. "Performance" held Edge settings,
   "Windows" held Edge settings, and "Privacy" held Edge settings, so Edge was in three places. Service switches (each
   of which turns a feature off) sat at the same level as harmless privacy values.
4. **Same icon on most cards.** 27 of the 35 tweaks fell back to their category icon.
5. **Cards were noisy.** Up to six indicators per card (impact, warning, privacy, admin, restart, undo), and a magenta
   border on anything switched on, so a page of applied tweaks read as an alarm.
6. **Gaming was almost empty.** Two cards and a planned list; nothing explained what to do first. Nothing combined
   tweaks into something a person could choose.
7. **Settings was one long scroll** with a scroll-spy, and a Notifications section holding a single row. Version
   appeared in Settings and the sidebar.
8. **"Planned for this section" took a whole card on four pages**, above-the-fold on some, describing things that
   don't exist.
9. **No way to jump to a tweak** without knowing its tab, and no shareable position inside a page (tabs lived in
   component state).
10. **PC Cleaner used large cards** for what is a short ranked list of sizes.
11. **Wording.** "Stage", "History" and "Derived" are engine words. A person looks for "select", "restore" and "undo".

## Feature inventory

Status: **Works** = real and functional; **Read-only** = real data, no action yet; **Sample** = preview only.

| Feature | What it does | Was at | Belongs in | Duplicates | Decision | Status |
| --- | --- | --- | --- | --- | --- | --- |
| Greeting and PC identity | Host, Windows version, build | Home header | Home | `SystemSummary` (unused) | Keep | Works |
| Update notice | Points to the Update Centre | Home | Home | Sidebar button | Keep, it's an alert | Works |
| Headline tiles | In effect, changes, health | Home | Home | Health panel | Replace health tile with active profile | Works |
| Utilisation chart | 5 min CPU/GPU/memory, names the device | Home | Home | Hardware tabs | Keep (you asked for the device name there) | Works |
| Live stat tiles | CPU, memory, GPU, network with sparklines | Home | **Monitoring** | Chart | Move; become Monitoring's selector | Works |
| Quickstart | First steps | Home | Home | – | Keep, smaller | Works |
| Recommended tweaks | Open recommended tweaks | Home | Home | – | Keep, links to primary location | Works |
| Hardware card | Full specification | Home | **Hardware** | Hardware page | Reduce to a four-line summary | Works |
| System health | Five derived checks | Home | Home | Headline tile | Keep, show problems first | Works |
| Recent changes | Latest journal batches | Home | Home | Restore Centre | Keep the four newest, link onward | Works |
| CPU/GPU/memory/network tabs | Facts plus live charts | Hardware | Split | Home | Facts to **Hardware**, charts to **Monitoring** | Works |
| Storage tab | Drive capacity | Hardware | **Hardware** | Home card | Keep on Hardware | Works |
| Per-thread load | Bars per logical CPU | Hardware | **Monitoring** | – | Move | Works |
| Tweak grid | Switches, details, undo | System Tweaks | **Tweaks** | – | Keep; reorganise, simplify cards | Works |
| Select recommended | Selects open recommended tweaks | System Tweaks | Tweaks | – | Keep | Works |
| Tweak details | Values, sources | Dialog | Same | – | Keep | Works |
| Change inspector | Dry-run plan, apply | Dialog | Same | – | Keep; add break warnings | Works |
| Tweak list (`TweakList`) | Old list view | Not routed | – | Tweak grid | Dead code; only its test uses it | Unused |
| Old Settings (`pages/settings-page.tsx`) | Old page | Not routed | – | Settings | Dead code | Unused |
| `SystemSummary` | Old Home header | Not routed | – | Home header | Dead code | Unused |
| Gaming | Two tweaks and a plan | Gaming | Gaming | – | Add profiles built from existing tweaks | Works |
| Startup | Every Run key and Startup shortcut | Startup | Startup | – | Keep; tidier rows | Read-only |
| PC Cleaner | Sizes of allowlisted folders | PC Cleaner | PC Cleaner | – | Ranked list with shares | Read-only |
| History | Journal, undo | History | **Restore Centre** | – | Rename; summary first | Works |
| System report | Copyable text | Tools | Tools | – | Keep | Works |
| Windows tools | Commands to copy | Tools | Tools | – | Keep | Works |
| Check for updates | Check, download, restart | Sidebar | Sidebar | Update Centre | Keep both: fast path and detail | Works |
| Update Centre | State, notes, actions | Settings | Settings › Updates | – | Keep | Works |
| Privilege chip | Admin or standard, version | Sidebar | Sidebar | Settings | Keep; merge with collapse | Works |
| Settings rows | Preferences | Settings | Settings | – | Tabs instead of scrolling | Works |
| Planned lists | What's not built | Four pages | Same | – | Collapsed by default | Sample |

## What the screenshots show

Worth learning from: one line of page purpose, secondary tabs under the header, search and filter in one row, short
fixed-height cards with a hairline above the control row, one or two small indicators, tooltips that say what breaks,
Settings as uniform rows, compact account menu.

Deliberately not followed: EXM's fixed-rating dots on power plans (2VX can't measure them), premium upsells, red
"Watch now" buttons, claims about FPS or latency, and anything that hides a Windows security feature.

## Not fixable by a redesign

These need backend work, not UI work, so the makeover labels them instead of faking them:

- PC Cleaner can't delete yet, and Startup can't disable anything.
- There are no power plans, network tools, services page, installed-apps page or benchmarks, because no backend exists
  for them. No navigation item is added for them.
- The app can't detect a running game, so there is no Game Mode dashboard like EXM's. Gaming shows profiles instead.
