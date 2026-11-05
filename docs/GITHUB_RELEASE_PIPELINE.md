# GitHub release pipeline

Two workflows in `.github/workflows/`:

| Workflow | Trigger | Permissions | Does |
| --- | --- | --- | --- |
| `ci.yml` | Pull requests, pushes to branches other than `main`, manual, and called by `release.yml` | `contents: read` | Frontend checks on Ubuntu (typecheck, lint, Prettier, Vitest); backend on Windows (build frontend, `cargo fmt`, clippy `-D warnings`, `cargo test`) |
| `release.yml` | Push to `main` touching app paths, or manual | `contents: read`; the release job alone gets `contents: write` | Checks → version → notes → signed build → release → public installer → feed validation → feed publish |

## Release flow

1. **Path filter**: only pushes that change `src/`, `src-tauri/`, `public/`, `scripts/`, `index.html`,
   `package*.json`, `vite.config.ts`, `tsconfig*.json` or the workflows. Docs-only pushes don't release.
   A commit message containing `[skip release]` skips it too.
2. **Checks**: `ci.yml` runs as a reusable workflow; the release job `needs` it.
3. **Version** (`scripts/next-version.mjs`): the higher of `package.json`'s version and the latest
   `vX.Y.Z` tag with its patch bumped. So with tags up to `v0.2.4` and `package.json` at `0.2.0`, the next
   build is `0.2.5`; bumping `package.json` to `0.3.0` makes the next build `0.3.0`. It writes the
   version into `package.json`, `tauri.conf.json` and `Cargo.toml` in the runner only and refuses a
   version that isn't newer than the latest tag. Tags, not commits, record released versions, so CI never
   pushes to `main`.
4. **Notes** (`scripts/release-notes.mjs`): commit subjects since the previous tag (no merges, at most
   40), shown in the job summary, the GitHub release and the Update Centre.
5. **Signing key check**: if `TAURI_SIGNING_PRIVATE_KEY` is empty the job builds an unsigned installer,
   attaches it to the run as a 14-day artifact, writes "Not published" in the summary and stops there:
   no release, no tag, no feed. It never publishes an unsigned or fake feed.
6. **Build and release** (`tauri-apps/tauri-action@v1`): NSIS bundle only, signed updater artifacts,
   GitHub release `v<version>` on the pushed commit, with `latest.json` attached
   (`updaterJsonPreferNsis: true`). The feed URL is injected with `--config`; it isn't in the committed
   config.
7. **Publish the installer**: uploads the same setup as `2VX-Tweaks-Setup.exe`, adds the download link to
   the release body and marks the release as latest, so
   `releases/latest/download/2VX-Tweaks-Setup.exe` always means the newest build. The link goes on after
   the build because the action copies the release body into `latest.json`. The job fails here, before
   any feed goes out, if the release is a prerelease or has no installer. `tests/policy.rs` guards this.
8. **Validate** (`scripts/check-update-feed.mjs`): the new `latest.json` must have the expected version,
   a `windows-x86_64` entry, a signature on every platform, HTTPS URLs, and a version newer than the
   feed currently on `updater-main`.
9. **Publish the feed**: creates the `updater-main` prerelease if it doesn't exist and uploads `latest.json` with
   `--clobber`. Installed copies see the update on their next check.

`concurrency: release-main` with `cancel-in-progress: false` runs one release at a time and never
cancels one half-way.

## Security choices

- Default token permissions are read-only; only the release job can write, and only `contents`.
- `persist-credentials: false` on checkout: the token isn't left in `.git/config` for later steps.
- Secrets are passed as environment variables to the steps that need them and never echoed. Release notes
  (commit text) reach `run:` scripts only through environment variables, not `${{ }}` interpolation,
  so a commit subject can't inject shell.
- Actions are pinned to major versions from their official publishers (`actions/*`, `tauri-apps/*`,
  `dtolnay/rust-toolchain`, `Swatinem/rust-cache`). Pinning to commit SHAs is stricter; do it once the
  repository exists (Dependabot can keep SHAs current).
- The workflow can't publish a downgrade: the version script and the feed check both refuse it.

## What is not done by the pipeline

- **Authenticode code signing** of the installer and exe. Without it Windows SmartScreen warns on
  download. Add a certificate (EV, or Azure Trusted Signing) and `bundle.windows.signCommand` when one
  is available; see `build-release.md`.
- **Stable channel**: only the main channel exists. A stable channel would use a second
  rolling tag (for example `updater-stable`) and a second build of the app pointing at it.
- **Repository visibility**: release downloads must be public for installed copies to fetch them. A
  private repository needs a different feed host.

## Current status

The workflows are written and the policy test checks the feed URL rules, but they have **never run**:
this repository has no GitHub remote yet and the signing secrets are not configured. Until both exist,
nothing is published. `UPDATE_SETUP_GUIDE.md` lists the steps.
