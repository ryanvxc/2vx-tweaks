# Security and safety

## Threat model

2VX modifies operating-system configuration. The two real risks are (1) the app damaging the user's system and
(2) the app's privileges being abused through its UI (e.g. injected script calling commands). Both are designed
against.

## Boundary: frontend ↔ backend

- The WebView has **no** shell, fs or opener plugins. It can only call app commands.
- Each command is declared in `src-tauri/build.rs` (`AppManifest::commands`), so it is **denied by default** until
  `capabilities/default.json` grants it. Adding a command without a capability entry makes it unreachable.
- Commands accept typed arguments only (enums, numbers, IDs validated against the registry). They never accept
  free-form paths, registry keys, process names or shell strings.
- CSP: `default-src 'self'`, scripts from `'self'` only, IPC origins allowed. `'unsafe-inline'` styles are
  required by Recharts and Radix positioning.
- `freezePrototype` is disabled because Recharts breaks with it (see `architecture.md`).

## Privilege

- The app runs unelevated by default and shows its privilege level in the title bar.
- Phase 2 decision pending: (a) relaunch elevated on demand, or (b) a small elevated helper with a fixed IPC
  vocabulary. (b) keeps the UI process unprivileged and is the current preference.
- Note: when launched from an elevated terminal, dev builds inherit elevation (the title bar shows "Administrator").

## Rules for system changes

1. Never disable Defender, Firewall, Windows Update, UAC, SmartScreen, VBS/HVCI, Secure Boot or similar protections.
2. Never bulk-disable services. Service changes are individual, explained and reversible.
3. Only documented settings, with references in `TweakMeta.references`.
4. Back up before every change. If any step fails, roll back the earlier steps, then report the error.
5. Deletion only inside a fixed allowlist of locations (temp, caches). Scan first, show results, delete on confirmation.
6. Boot configuration (`bcdedit`) and security settings need a `High` risk confirmation with explicit acknowledgement.
7. Never apply anything silently. Presets show the full list of changes before applying.
8. No performance claims without measurement on the user's machine.

## Logging and privacy

- Logs stay local (`%LOCALAPPDATA%\com.twovx.tweaks\logs`). No telemetry.
- Frontend log messages are stripped of control characters and capped at 2,000 chars, which prevents log injection.
