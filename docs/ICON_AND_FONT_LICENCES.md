# Icon, font and asset licences

Everything visual that ships in 2VX Tweaks, where it comes from and under what licence. No premium or
paid assets are used, and no third-party brand logos are bundled.

| Asset | Source | Licence | Notes |
| --- | --- | --- | --- |
| Icons | [Lucide](https://lucide.dev) via `lucide-react` (npm) | ISC | Tree-shaken; only used icons are bundled |
| UI font | [Geist](https://github.com/vercel/geist-font) via `@fontsource-variable/geist` (npm) | SIL Open Font License 1.1 | Bundled into the app; nothing is fetched from Google Fonts or any CDN |
| Sidebar and account menu font | [Poppins](https://github.com/itfoundry/Poppins) via `@fontsource/poppins` (npm) | SIL Open Font License 1.1 | Bundled from the npm package; nothing is fetched from Google Fonts or any CDN |
| Monospace font | Cascadia Mono / Consolas | Windows system fonts | Not bundled; used if installed |
| 2VX logo (`src/assets/logo.svg`, `public/favicon.svg`) | Made for this project | Project's own | Violet→magenta gradient mark |
| App icons (`src-tauri/icons/`) | Generated from `logo.svg` with `npx tauri icon` | Project's own | Regenerate whenever the logo changes |
| UI primitives | [shadcn/ui](https://ui.shadcn.com) on [Radix](https://www.radix-ui.com) | MIT | Copied into `src/components/ui/` as shadcn intends |
| Charts | [Recharts](https://recharts.org) | MIT | |
| Toasts | [Sonner](https://sonner.emilkowal.ski) | MIT | |

## Rules

- Brand logos (Microsoft Edge, Chrome, NVIDIA, AMD, Windows) are trademarks. They are not downloaded or
  redrawn; neutral Lucide glyphs stand in for them.
- Nothing from EXM Tweaks (code, icons, images, fonts, copy) is used. `docs/research/` describes it only.
- A new asset needs a row in this table with its licence before it is merged.
- The Settings › About › Open-source licences row summarises this file.
