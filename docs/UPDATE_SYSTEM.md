# Update system

2VX Tweaks updates itself through the official Tauri 2 updater (`tauri-plugin-updater` 2.14,
`tauri-plugin-process` 2.4 for the restart). Updates are signed, fetched over HTTPS from the project's
GitHub releases, and never downloaded or installed without the user choosing to.

## How it works

```
Settings › Updates (Update Centre)                       GitHub
  check()  ── HTTPS GET ─────────────────────────────►  releases/download/updater-main/latest.json
           ◄── { version, notes, platforms.windows-x86_64 { url, signature } }
  [user reads the notes, clicks Download]
  download() ── HTTPS GET ────────────────────────────►  releases/download/v<version>/…_x64-setup.exe
           signature checked against the public key compiled into the app
  [user clicks Install and restart]
  install()  runs the NSIS installer (passive, UAC prompt) → 2VX closes → installer reopens it
```

- **Feed**: `https://github.com/<owner>/<repo>/releases/download/updater-main/latest.json`. `updater-main`
  is a rolling prerelease that only holds `latest.json`; each build is its own prerelease `v<version>`.
  GitHub's `releases/latest/download` URL is deliberately not used: it only sees non-prerelease
  releases, so it would never offer main builds (enforced by `tests/policy.rs`).
- **Signature**: Tauri's minisign-based scheme. The public key is in `tauri.conf.json`
  (`plugins.updater.pubkey`); the private key exists only on the owner's machine and as a GitHub secret.
  A download whose signature doesn't verify is rejected by the plugin before anything runs. Signature
  checking can't be turned off in the plugin, and the policy test refuses
  `dangerousInsecureTransportProtocol`.
- **Version rule**: the plugin's default comparator only accepts a version greater than the installed
  one, so a feed can't push an older build. The release workflow also refuses to publish a feed that is
  not newer than the one already published (`scripts/check-update-feed.mjs`).
- **Install mode**: `passive` (progress window, no questions). The NSIS installer is per-machine, so
  Windows asks for administrator permission once. On Windows `install()` exits the app; the installer
  starts the new version.

## Which builds can update

| Build                                  | Updates                                       | Why                                                                            |
| -------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------ |
| Development (`npm run app:dev`, debug) | No: "Updates aren't available for this build" | `get_update_support` returns unavailable for debug builds                      |
| Local release (`npm run app:build`)    | Yes, to the newest CI build                   | `tauri.conf.json` holds the feed URL. It can't publish: no updater artifacts   |
| CI main-channel build (`release.yml`)  | Yes                                           | The same feed, and only this build signs updater artifacts                     |
| Browser preview (`npm run dev`)        | Simulated                                     | `services/updater.ts` mock feeds, no network                                   |

`get_update_support` (Rust, `commands/app.rs`) reads the endpoint list from the app's own config, so the UI
always shows the real channel and URL.

Updating from a local build installs the CI build to `C:\Program Files\2VX Tweaks` and opens that copy. The local
exe stays where it is, unchanged. The local version stays below the feed because CI picks the next version from the
tags, not from the committed config.

Tested on 2026-11-04 in a local release build against the live feed: the check, download and signature check passed,
and the owner's click installed v0.2.2 over the old 0.1.0 copy in Program Files, which then reopened. The same route
then took it from v0.2.2 to v0.2.3, the first update from one CI build to the next.

## User-facing behaviour

- **Check for updates** (account menu) walks the same steps from one button: a click checks and, if
  there is a newer version, downloads it; the label then reads "Restart to update", and a second click installs
  and reopens the app. A check at startup never downloads: it changes the label to "Update available" and waits.
- **Update Centre** (Settings › Updates) shows one state at a time with one action:
  loading, unsupported (with the reason), not checked, checking, up to date (with last check time),
  available (with release notes), downloading (progress bar and bytes), ready (downloaded and verified),
  installing, restart required, and error (check / download / install, with Try again).
- **Release notes** are shown before the download and rendered as text, never HTML.
- **Automatic check**: once per app start, on by default, turned off in Settings › Updates. It only
  checks; it never downloads.
- **Home notice**: a small banner when an update is available or downloaded, dismissible per version,
  switchable in Settings › Notifications. The account card shows a dot.
- Nothing is downloaded or installed without a click. `use-updater.test.ts` asserts this.

## Code map

| File                                      | Role                                                                                                             |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `src-tauri/src/lib.rs`                    | Registers `tauri_plugin_updater` and `tauri_plugin_process`                                                      |
| `src-tauri/capabilities/default.json`     | Grants `updater:default` and `process:allow-restart` only                                                        |
| `src-tauri/src/commands/app.rs`           | `get_update_support`: available, reason, endpoint, channel                                                       |
| `src/services/updater.ts`                 | Wraps the plugins (desktop) or mock feeds (preview) behind `UpdaterService`                                      |
| `src/hooks/use-updater.ts`                | State machine (zustand) and `useUpdater()`                                                                       |
| `src/pages/settings/update-centre.tsx`    | Update Centre UI                                                                                                 |
| `src/components/common/update-notice.tsx` | Home banner                                                                                                      |
| `src-tauri/tests/policy.rs`               | Guards: pubkey present, no private key, no endpoint committed, HTTPS GitHub endpoint in CI, no `latest/download` |

## Testing

- `npm run test`: state machine (never installs unasked, retries, auto-check preference) and the
  release scripts (version, notes, feed validation).
- Browser preview mock feeds: open `http://localhost:1420/?update=<feed>#/settings` with `available`
  (default), `none`, `error`, `download-error` or `unconfigured`.
- **Not yet tested**: a real update between two signed CI builds. That needs the GitHub secrets, two
  published builds and a disposable Windows VM (never the only working development install). The steps
  are in `UPDATE_SETUP_GUIDE.md` › "First end-to-end test".

## Privacy

The update check is a normal HTTPS GET to github.com. It sends no identifiers,
hardware data or usage data. It is the only network request 2VX makes, and the user can turn it off.
This supersedes decision P0-8b ("no in-app updater") at the owner's request for 0.2; see
`docs/security.md`.
