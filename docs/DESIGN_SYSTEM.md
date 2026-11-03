# Design system

The 0.2 visual language: a near-black, slightly violet charcoal UI with violet icons and focus rings,
and magenta for primary actions and switched-on toggles. Dark only. All values live in
`src/styles/globals.css`; components use Tailwind tokens, never raw colours (the Windows caption-button
red is the one exception).

## Colour tokens (OKLCH)

Surfaces get lighter as they come forward: `background` < `sidebar` < `card` < `raised` < `popover`.

| Token | Value | Use |
| --- | --- | --- |
| `background` | `oklch(0.135 0.006 285)` | Window background |
| `sidebar` | `oklch(0.155 0.007 285)` | Sidebar |
| `card` | `oklch(0.175 0.008 285)` | Cards, settings groups |
| `raised` | `oklch(0.205 0.009 285)` | Hover state of cards, icon tiles |
| `popover` | `oklch(0.215 0.01 285)` | Tooltips, menus, dialogs |
| `foreground` | `oklch(0.965 0.004 285)` | Body text |
| `muted-foreground` | `oklch(0.65 0.014 285)` | Secondary text (≥ 4.5:1 on `card`) |
| `brand` | `oklch(0.7 0.17 293)` | Icons, active navigation, focus ring, impact bars |
| `primary` | `oklch(0.64 0.225 352)` | Primary buttons, switches that are on, update notice |
| `border` | white at 7 % | Hairlines; `input` is 10 % |
| `success` / `warning` / `destructive` / `info` | green / amber / red / blue | **State only** |

Status colours mean something: success = applied/healthy, warning = needs attention or restart,
destructive = failed or dangerous, info = neutral facts (preview, scope). Never use them for decoration.
Chart series use `chart-1…5` (violet, blue, green, amber, magenta).

## Typography

Inter Variable (bundled from `@fontsource-variable/inter`, OFL 1.1) with Segoe UI Variable as the
fallback. Monospace: Cascadia Mono, then Consolas. Numbers that update use `tabular`.

| Utility | Size / weight | Use |
| --- | --- | --- |
| `text-page-title` | 22 px / 650 | One per page (`PageHeader`) |
| `text-section-title` | 15 px / 600 | Card and group headings |
| `text-card-title` | 13.5 px / 600 | Optimisation card titles, rows |
| `text-body` | 13 px / 1.45 | Descriptions |
| `text-caption` | 11.5 px | Metadata, hints, badges |

## Spacing, radius, layout

- Radius `--radius` 10 px; cards use `rounded-xl`, controls `rounded-md`.
- Page padding 24 px, card padding 16 px, grid gap 12–16 px.
- Sidebar 216 px (`--sidebar-width`), 56 px when compact. Content max width follows the window;
  card grids are 2 columns, 3 at `xl` (1280 px), 4 at `2xl` (1536 px).
- Minimum window 1080×700, default 1320×840 (`tauri.conf.json`). Layouts are checked at 1080 and 1320 wide.

## Motion

Transitions are 150 ms colour/opacity only. Everything is disabled by Settings › Reduce motion
(`.reduce-motion` on `<html>`) and by the Windows "Show animations" setting (`prefers-reduced-motion`).

## Components (`src/components/common/`)

| Component | Purpose |
| --- | --- |
| `OptimisationCard`, `CardGrid`, `NewBadge` | The card used by tweaks, gaming and cleaner: icon tile, title, description, indicators, one control |
| `Indicator`, `ImpactMeter` | Small focusable icons with tooltips (risk, admin, restart, undoable…) and the 3-bar impact estimate |
| `FilterBar` | Search box, filter chips (state, impact, risk, permission) and sort |
| `PageTabs` | Accessible tab list (`role="tablist"`) used by System Tweaks, Hardware, Startup |
| `SettingsGroup`, `SettingsRow` | Row layout for Settings: icon, title, description, control on the right |
| `MetricChart` | Recharts area chart for live metrics with a fixed time window |
| `UpdateNotice` | Dismissible Home banner when an update is available |
| `SectionCard`, `DataSourceBadge` | Card with a title and the live/derived/sample/planned badge |
| `ConfirmDialog` | Confirmation with `risk="high"` acknowledgement for dangerous actions |
| `StatusPill`, `Notice`, states | Small status labels, inline notices, loading/empty/error states |
| `PlannedFeatures` | Honest list of what a page will do but doesn't yet |

shadcn/ui primitives in `components/ui/` keep the shadcn API; only their colours and the tooltip style
were adapted.

## Icons

Lucide only (ISC). Brand logos are not drawn or downloaded: where a product would have a logo (Chrome,
Edge, NVIDIA) a neutral Lucide glyph is used. Icon sizes: 16 px inline, 18 px in card tiles, 20 px in
status lines.

## Accessibility

- Every interactive element is reachable by keyboard and has a visible `ring-brand` focus ring.
- Switches carry an action label ("Turn on X", "Undo X"); indicators have `aria-label`s matching their tooltips.
- Live regions (`aria-live="polite"`) announce updater and staging changes.
- Text contrast: foreground ≥ 12:1, muted ≥ 4.5:1 on cards.
