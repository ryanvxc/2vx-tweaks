# Design system

The 2VX visual language: a near-black UI with charcoal cards, violet icons and focus rings, and magenta
for primary actions and switched-on toggles. Dark only. All values live in
`src/styles/globals.css`; components use Tailwind tokens, never raw colours (the Windows caption-button
red is the one exception).

## Colour tokens (OKLCH)

Surfaces get lighter as they come forward: `background` < `sidebar` < `card` < `raised` < `popover`.

| Token | Value | Use |
| --- | --- | --- |
| `background` | `oklch(0.125 0.008 280)` | Window background |
| `sidebar` | `oklch(0.14 0.009 280)` | Sidebar |
| `card` | `oklch(0.16 0.01 280)` | Cards, settings groups |
| `raised` | `oklch(0.19 0.011 280)` | Hover and selected cards, icon tiles |
| `popover` | `oklch(0.2 0.012 280)` | Tooltips, menus, dialogs |
| `foreground` | `oklch(0.965 0.004 285)` | Body text |
| `muted-foreground` | `oklch(0.65 0.014 285)` | Secondary text (≥ 4.5:1 on `card`) |
| `brand` | `oklch(0.7 0.17 293)` | Icons, active navigation, focus ring |
| `primary` | `oklch(0.645 0.235 358)` | Primary buttons, switches that are on, selected cards, update notice |
| `border` | white at 7 % | Hairlines; `input` is 10 % |
| `success` / `warning` / `destructive` / `info` | green / amber / red / blue | **State only** |

Status colours mean something: success = applied/healthy, warning = needs attention or restart,
destructive = failed or dangerous, info = neutral facts (preview, scope). Never use them for decoration.
The impact meter also uses them: one green bar is a small effect, two amber a noticeable one, three red a
large one. The number of bars and the tooltip carry the same meaning without colour.
Chart series use `chart-1…5` (violet, blue, green, amber, magenta).

## Typography

Headings use Manrope Variable (`@fontsource-variable/manrope`, OFL 1.1) and body text uses Inter Variable
(`@fontsource-variable/inter`, OFL 1.1), with Segoe UI Variable as the fallback. Both are bundled.
Monospace: Cascadia Mono, then Consolas. Numbers that update use `tabular`.

| Utility | Size / weight | Use |
| --- | --- | --- |
| `text-page-title` | Manrope 24 px / 700 | One per page (`PageHeader`) |
| `text-section-title` | Manrope 15 px / 700 | Card and group headings |
| `text-card-title` | Manrope 14 px / 700 | Optimisation card titles, rows |
| `text-body` | 13 px / 1.45 | Descriptions |
| `text-caption` | 11.5 px | Metadata, hints, badges |

## Spacing, radius, layout

- Radius `--radius` 10 px; cards use `rounded-xl`, controls `rounded-md`.
- Page padding 28 px, card padding 16 px, grid gap 12 px.
- Sidebar 224 px (`--sidebar-width`), 64 px when compact. Content max width follows the window;
  card grids are 2 columns, 3 at `xl` (1280 px), 4 at `2xl` (1536 px).
- Minimum window 1080×700, default 1320×840 (`tauri.conf.json`). Layouts are checked at both sizes.
- Page anatomy: title; tabs; search, filter and sort on one row; content.
- Copy: a row is an icon, a title and at most one short line, and that line stays only if something could confuse.
  Never explain how the app works or why it's safe. Tweak card descriptions fit two lines (aim for under 70 characters);
  the detail goes in `details`.
- Home fits the default window with no scrolling.

## Motion

Motion answers something the user just did or shows that something is live. It never decorates, never loops except
the live dot and loading shimmer, and never makes anyone wait.

**One switch.** Settings › Reduce motion (`.reduce-motion` on `<html>`) is the only control. The Windows animation setting
is deliberately ignored: it's often off for speed on the PCs this app is for, and following it would remove all motion
for the people most likely to use 2VX, who then couldn't turn it back on from here. CSS motion is stopped by the
`.reduce-motion` rule; code-driven motion (`play` in `lib/motion.ts`, `AnimatedNumber`) reads the store. Test motion in
the real app, because the browser preview can't show this difference.

| Moment | Time | How |
|---|---|---|
| Press (button, switch thumb) | 120 ms | `active:scale-[0.97]`, thumb stretches |
| Tooltip | 120 ms in, 75 out | fade with a small slide |
| Dropdown | 140 ms in, 100 out | `ease-arrive` |
| Dialog | 200 ms in, 130 out | rises slightly; exits faster than it enters |
| Command palette | 100 ms in, 75 out | no slide, because it's opened from the keyboard all day |
| Page, tab content, notices | 220 ms | `animate-rise` |
| Sidebar pill, tab underline, chart filter, selection outline | ~200 ms | one shared element slides (`useSlidingIndicator`) |
| List entry | 240 ms | `useStaggerIn`: 24 ms apart, capped at 6 steps and 24 items, once per tab, never while filtering |
| Figures | 450–600 ms | `AnimatedNumber` counts up once, then ticks |
| Apply succeeded | 1.1 s | the card's border and background pulse `--success` |

Keep routine UI under 300 ms and ease out (`--ease-arrive`, the default timing function). Use the `--animate-*` tokens in
`globals.css`. Tailwind 4's `translate-*` and `scale-*` use the CSS `translate` and `scale` properties, so transitions
must name those, not `transform`. Don't add a motion library.

The loading screen follows Settings › Reduce motion too, which gives a static logo for at least 1.2 s. It's brief,
never flashes, and can be skipped or turned off. Otherwise a canvas of about 1,800 dots forms the logo as the real
startup calls settle (`components/layout/splash-screen.tsx`, physics in `lib/splash-field.ts`). The bar and the stage
text can run behind those calls but never ahead of them, a failed call counts as settled because its page reports the
error, and the screen gives up on a stuck call after 8 s. It is on top of the app, which loads underneath and stays
`inert` until it goes. The cursor pushes dots, a click sends a pulse, Space or → changes the fact and Esc or Enter
skips. Moving the cursor holds it for up to 7 s. Settings › Appearance turns it off. Canvas colours are mixed from
`--brand`, `--primary` and `--foreground`, and facts in `lib/splash-facts.ts` must be plainly true.

## Components (`src/components/common/`)

| Component | Purpose |
| --- | --- |
| `OptimisationCard`, `CardGrid`, `NewBadge` | The card used by Tweaks and Gaming: icon, title, description, a dashed hairline, then indicators and one control |
| `Indicator`, `ImpactMeter` | 28 px bordered tiles with tooltips (warning, restart, blocked, partial) and the 3-bar impact estimate |
| `FilterBar` | Search box, filter and sort menus on one row |
| `PageTabs` | Accessible tab list (`role="tablist"`) used by Tweaks, Startup and Settings; `useTabParam` keeps the tab in the URL |
| `SettingsGroup`, `SettingsRow` | Row layout for Settings: icon, title, description, control on the right |
| `MetricChart` | Recharts area chart for live metrics with a fixed time window |
| `CommandPalette` (`components/layout/`) | Ctrl K search over pages, tabs and tweaks. It only navigates |
| `UpdateNotice` | Dismissible Home banner when an update is available |
| `SectionCard`, `DataSourceBadge` | Card with a title and the live/derived/sample/planned badge |
| `ConfirmDialog` | Confirmation with `risk="high"` acknowledgement for dangerous actions |
| `StatusPill`, `Notice`, states | Small status labels, inline notices, loading/empty/error states |

shadcn/ui primitives in `components/ui/` keep the shadcn API; only their colours and the tooltip style
were adapted.

## Icons

Lucide only (ISC). Brand logos are not drawn or downloaded: where a product would have a logo (Chrome,
Edge, NVIDIA) a neutral Lucide glyph is used. Icon sizes: 16 px inline, 18 px in card tiles, 20 px in
status lines.

## Accessibility

- Every interactive element is reachable by keyboard and has a visible `ring-brand` focus ring.
- Switches carry an action label ("Turn on X", "Undo X"); indicators have `aria-label`s matching their tooltips.
- Live regions (`aria-live="polite"`) announce updater and selection changes.
- Text contrast: foreground ≥ 12:1, muted ≥ 4.5:1 on cards.
- The Monitoring tiles and the Gaming profile cards are tabs and radios with a selected state, so the keyboard
  and screen readers see them as choices.
