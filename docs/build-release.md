# Build and release

## Prerequisites (Windows)

- Node.js ≥ 20.19 (developed on 26.x) and npm
- Rust stable, MSVC target (`rustup default stable-x86_64-pc-windows-msvc`), with `rustfmt` and `clippy`
- Visual Studio Build Tools 2022 with "Desktop development with C++"
- WebView2 Runtime (preinstalled on Windows 11; the installer bootstraps it on Windows 10)

## Build

```bash
npm ci
npm run check:all
npm run app:build
```

Output goes to `src-tauri/target/release/bundle/{nsis,msi}/` (`npm run app:build -- --bundles nsis` builds only the
NSIS setup). The NSIS installer is per-machine (`bundle.windows.nsis.installMode`): it installs to
`C:\Program Files\2VX Tweaks` and asks for administrator permission once, at install time. The app itself still runs
unelevated. The release profile uses LTO, `opt-level = "s"`,
stripped symbols and `panic = "abort"`.

## Before the first public release (backlog P1-11)

- **Code signing**: an unsigned optimiser triggers SmartScreen, and some AV engines flag it. Configure
  `bundle.windows.certificateThumbprint` or a signing command (Azure Trusted Signing / EV certificate).
- **Updates**: signed in-app updates for CI builds; see `UPDATE_SYSTEM.md`, `GITHUB_RELEASE_PIPELINE.md` and
  `UPDATE_SETUP_GUIDE.md` (secrets to configure). Local `app:build` output has no update channel.
- **Version**: CI picks the next patch version from the latest `v*` tag. For a minor or major release, bump
  `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json` together; CI never lowers it.
- **Identifier**: `com.twovx.tweaks`. Changing it moves the app's data and log folders.
