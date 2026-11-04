# UI/UX redesign (0.2)

> Historical record of the first 0.2 redesign. The page structure described here was replaced by the
> makeover: see `INFORMATION_ARCHITECTURE.md`, `NAVIGATION_STRUCTURE.md` and `REDESIGN_ROADMAP.md`.
> System Tweaks is now Tweaks, History is now the Restore Centre, and "stage" is now "select".

The 0.2 redesign moves 2VX from a list-and-panel layout to a card-based optimisation app, in the visual
family of the EXM reference board (dark charcoal, violet icons, magenta toggles) without copying EXM's
code, assets, copy or layout pixel for pixel. Tokens and components are in `DESIGN_SYSTEM.md`.
Screenshots of the browser preview are in `docs/screenshots/redesign/`.

## Principles kept from 0.1

- Honesty first: every data card carries its `DataSourceBadge`; nothing presents sample data as measured.
- A switch never changes the system directly. Switching a tweak on **stages** it; changes are applied
  only from the review dialog, which lists every value.
- No fake pages. A page that can't do something yet says so in a "Planned" list instead of showing
  controls that don't work.

## Navigation

Sidebar groups: **Overview** (Home, Hardware), **Optimisations** (System Tweaks, Gaming, Startup,
PC Cleaner), **Tools** (History, Tools, Settings). The footer shows whether 2VX runs as administrator or a
standard user, and the version. A dot on Settings means an update is available. Compact mode shows icons
only. The old `/monitoring` route redirects to `/hardware`.

## Pages

| Page | What it shows | Data |
| --- | --- | --- |
| **Home** | Greeting with host and Windows build; update notice; stat tiles (optimisations in effect, changes recorded, health checks passed); live utilisation chart with CPU/GPU/Memory filter; Quickstart; live stats; recommended tweaks; hardware; health; recent changes | Live, derived; recommendations derived from detected tweak state (the old sample list is gone) |
| **System Tweaks** | Card grid with tabs (All / Privacy / Performance / Windows), search, filters (state, impact, risk, permission), sort, "Stage recommended"; each card has impact meter, indicators (risk, admin, restart, privacy, undoable, blocked, partial) and a switch; details dialog with values and references | Live from the engine |
| **Gaming** | Same grid for the gaming category (empty until a documented gaming value passes the rules) and the planned list | Live |
| **Hardware** | Tabs: CPU (per-thread load bars, history), GPU, Memory, Storage, Network; facts and live charts | Live |
| **Startup** | Every Run/RunOnce value (user and machine, 32- and 64-bit) and Startup-folder shortcut, with scope tabs, search, whether Task Manager has disabled it, and the command line to copy | Live, read-only |
| **PC Cleaner** | Size and file count of each allowlisted location (temp, shader caches, browser caches, error reports, Recycle Bin) with scan status | Live, read-only; cleaning is planned |
| **History** | Batches with exact old/new values and undo (unchanged from 0.1, restyled) | Live |
| **Tools** | Copyable system report; Windows tools with the command to open them yourself | Derived |
| **Settings** | Row-based sections: General, Appearance, Notifications, Updates (Update Centre), Behaviour, Security, About | Live |

## Interaction changes

- **Switch off = undo.** A tweak that 2VX applied can be switched off; after a confirmation 2VX restores
  exactly the values it recorded (`revert_tweak`, newest record per value, non-clobbering). A value set
  outside 2VX shows a locked switch with an explanation, because there is nothing recorded to restore.
- **Staging bar.** Staged tweaks collect in a sticky "Review N changes" bar that opens the change
  inspector.
- **Deep links.** `/system?tweak=<id>` opens a tweak's details; `/settings#updates` scrolls to the Update
  Centre. Pages scroll to the top when you navigate.
- **"New" badge** on tweaks added in the current minor version (`TweakMeta.added_in`).
- **Impact** is an estimate from the documentation, labelled as such; 2VX doesn't claim measured gains.
- **Reduce motion** in Settings is the one control; see `DESIGN_SYSTEM.md`.

## Visual QA

Checked in the browser preview with Playwright at 1320×840 (default window) and 1080×700 (minimum):
Home, System Tweaks, Settings, Update Centre downloading state, Hardware, PC Cleaner, Startup. No
overlapping text or horizontal scrolling was found at those sizes. Not checked: 1920-wide windows,
Windows display scaling above 100 %, and the real desktop shell (title bar, UAC) on a VM.

## Known gaps

- Light theme not built (Settings says so).
- Gaming has no tweaks yet; Startup and Cleaner are read-only.
- `tweak-list.tsx`, `dashboard/system-summary.tsx` and the old `pages/settings-page.tsx` are no longer
  used. They are kept until the owner confirms they can be deleted.
