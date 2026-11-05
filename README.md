# 2VX Tweaks

A Windows 10/11 optimisation suite for gamers and enthusiasts, built with Tauri 2, Rust and React.
Every change is documented, backed up, verifiable and reversible.

## Download

**[Download the latest installer](https://github.com/ryanvxc/2vx-tweaks/releases/latest/download/2VX-Tweaks-Setup.exe)** for 64-bit Windows 10 and 11,
or pick a version on the [releases page](https://github.com/ryanvxc/2vx-tweaks/releases).

Windows SmartScreen may warn you, because the installer isn't code-signed yet: choose More info, then Run anyway. Once installed,
2VX Tweaks updates itself with Check for updates.

## Development

**Status: in development.** 320 tweaks with per-tweak undo, a Visuals page, Game Mode, a PC Cleaner and a Restore Centre.
See [project status](docs/PROJECT_STATUS.md).

```bash
npm install
npm run app:dev     # desktop app
npm run check:all   # lint, types, format, tests (TS + Rust)
```

Docs: [architecture](docs/architecture.md) · [writing tweaks](docs/optimisation-modules.md) ·
[Windows APIs](docs/windows-integration.md) · [security](docs/security.md) · [testing](docs/testing.md) ·
[build & release](docs/build-release.md)
