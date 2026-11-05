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

Pages use Geist Variable (`@fontsource-variable/geist`, OFL 1.1). The sidebar, account menu and Home use Poppins
(`@fontsource/poppins`, OFL 1.1, weights 400 to 600) through `font-nav`. Both are bundled, with Segoe UI Variable as the
fallback.
Monospace: Cascadia Mono, then Consolas. Numbers that update use `tabular`.

| Utility | Size / weight | Use |
| --- | --- | --- |
| `text-page-title` | 22 px / 600 | One per page, in the page (`PageHero`) |
| `text-section-title` | 14 px / 600 | Card and group headings |
| `text-card-title` | 13.5 px / 600 | Optimisation card titles, rows |
| `text-body` | 13 px / 1.45 | Descriptions |
| `text-caption` | 11.5 px | Metadata, hints, badges |

## Spacing, radius, layout

- Radius `--radius` 10 px; panels use `rounded-xl`, controls `rounded-md`.
- A surface is the `panel` utility: `--card` fill with a faint `--hairline` edge. Separation comes from the fill, not a
  visible outline, and panels are never nested.
- Page padding 20 px, panel padding 16 px, every gap 12 px.
- Sidebar 240 px (`--sidebar-width`), 64 px when compact, with a 72 px brand block. Its rows are 46 px, down to 34 px on a
  short screen, with the highlight capped at 40 px. Groups sit together 20 px apart, as in EXM, and spare height goes above
  the account card, never between groups. Content uses the full window width; `CardGrid` fits as many 19rem columns as
  there's room for.
- The window is one size, 1280×820 (`tauri.conf.json`), and can only be minimised or closed. On a screen too small for
  it, `preventOverflow` shrinks it to fit and pages scroll. Check layouts at 1280×820.
- Page anatomy, on every page: a thin title bar with the window controls, the title in the page (`PageHero`) with the
  page's one action on its right, full-width tabs, then search, filter and sort on one row, then content.
- The top-right action is only ever the recommended tweaks still open (`RecommendedAction`). With none left, the slot is
  empty: it isn't a place to advertise anything else.
- Pages fill the window: the page root is `flex flex-1 flex-col`, and something on it takes the spare height, usually a
  chart or one full-height table panel. Rows in such a panel may share the height (`flex-1`) so a short list still
  reaches the bottom.
- Lists are tables, not stacks of cards: one panel, an `h-9` header row, single-line rows, columns shared through a
  `COLUMNS` constant.
- Tweaks come in two shapes. A hand-placed tweak is an EXM card. A long list is folding sections: a 44 px bar (chevron,
  one violet icon, title, "N of M in effect") over `h-14` flat rows in one shared panel, split by hairlines. EXM uses
  separate 68 px row panels; flat rows are a deliberate departure, because 1,300 tweaks in those would be five screens
  of chrome per topic. Unavailable tweaks are always the last section, muted, with the reason where the switch would be.
- A card section is never a lone card in an empty row: `groupTweaks` tops it up to a full row from the tab's lists, and with
  one row of cards or fewer the first list starts open, so the page fills.
- A row's control sits in a fixed slot, so a word in one row doesn't push its indicators out of line with the next.
- A tweak has one home. Search looks through every tab of its page, and matches names, descriptions and the registry
  or service names a tweak touches.
- A preset, a service preset and a GPU profile are the same thing, a named list of tweaks, so all three are `PresetCard`.
  Its facts are counts of real tweaks. No scores, stars or ratings: nothing in 2VX measures them.
- No pill boxes. State is a dot and a word (`StatusLabel`); a label with nothing to report isn't shown at all. The one
  exception is `NewBadge`, whose box matches EXM's card.
- Copy: a row is an icon, a title and at most one short line, and that line stays only if something could confuse.
  Never explain how the app works or why it's safe. Tweak card descriptions fit two lines (aim for under 70 characters);
  the detail goes in `details`.
- Game Mode keeps its title in the page too (`PageHero`), and its layout follows EXM's: promises, then a panel holding
  the ring, the stats and Configure.
- Home is taller than the window, with Updates below the fold and its heading showing at the bottom of the default
  window. Live usage lives there: there is no separate monitoring page.
  Each Updates card has an original image in `src/assets/home/`, painted from seeded noise for 2VX. No third-party art.

## Motion

Motion answers something the user just did or shows that something is live. It never decorates, never loops except
the live dot, the newest point on Home's chart and the loading shimmer, and never makes anyone wait. Home's cards are the
one hover flourish: `DotField` rises and twinkles only while a card is hovered or focused, and draws nothing at rest.

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
| `OptimisationCard`, `CardGrid`, `NewBadge` | The fixed-height card used by Tweaks: icon, one-line title, two-line description, a dashed line, then boxed indicators and one control |
| `Indicator`, `ImpactMeter` | Bare icons with tooltips (high risk, hardware, community, warning, restart, failed, partial, unread) and the 3-bar impact rating. Each stands for a real state; the indicators shown must match what's true of the tweak |
| `FilterBar` | Search box, filter and sort menus on one row. The filter menu is two columns, so every group fits without its own scrollbar. The active count is plain text, not a chip |
| `PageTabs` | Accessible tab list (`role="tablist"`) on every page with tabs (`filled` is a full-width bar with a raised active tab and a white underline); `useTabParam` keeps the tab in the URL |
| `OptimisationRow` | The full-width row: an icon or letter tile, title, one line, indicators, one control. `h-[68px]` on its own, or `flat` (`h-14`, no panel of its own) inside a shared panel for long lists |
| `PresetCard` | A named list of tweaks: icon, title, state top-right, one line, counts, then the buttons along the bottom |
| `InfoPanel`, `ActionCard` | "How it works" panel with an optional link, and a card for something 2VX reads or sends you to: icon, title, status, rows, one action. Links only come from the fixed list in `links.rs` |
| `SettingsGroup`, `SettingsRow` | Rows under a heading for Settings: icon, title, optional line, control on the right. Groups sit inside a shared `panel` |
| `CommandPalette` (`components/layout/`) | Ctrl K search over pages, tabs and tweaks. It only navigates |
| `UpdateNotice` | Dismissible Home banner when an update is available |
| `DotField` | Canvas of small squares that rises from a card's bottom edge. The colour is the element's `color` |
| `SectionCard`, `DataSourceBadge` | Card with a title and the data source: solid dot `live`, hollow dot `derived`, plain text `sample`/`planned`/`Preview`. The name and meaning are in the tooltip and `aria-label` |
| `ConfirmDialog` | Confirmation with `risk="high"` acknowledgement for dangerous actions |
| `StatusLabel`, `Notice`, states | A dot and a word for state, inline notices, loading/empty/error states |

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
