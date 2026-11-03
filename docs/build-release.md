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

Output goes to `src-tauri/target/release/bundle/{nsis,msi}/`. The release profile uses LTO, `opt-level = "s"`,
stripped symbols and `panic = "abort"`.

## Before the first public release (Phase 5)

- **Code signing**: an unsigned optimiser triggers SmartScreen, and some AV engines flag it. Configure
  `bundle.windows.certificateThumbprint` or a signing command (Azure Trusted Signing / EV certificate).
- **Updater**: `tauri-plugin-updater` with signed update manifests.
- **Version**: bump `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json` together.
- **Identifier**: `com.twovx.tweaks`. Changing it moves the app's data and log folders.
