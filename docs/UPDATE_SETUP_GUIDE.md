# Update setup guide

What the owner has to do, once, before installed copies of 2VX Tweaks can update themselves. Until these
steps are done the release workflow builds but publishes nothing, so there is no feed for installed copies to read.

## Already done in the repository

- An updater key pair was generated on the development machine with
  `npx tauri signer generate` (password protected):
  - private key: `%USERPROFILE%\.tauri\2vx-tweaks.key`
  - its password: `%USERPROFILE%\.tauri\2vx-tweaks.key.password`
  - public key: `%USERPROFILE%\.tauri\2vx-tweaks.key.pub`, also in `src-tauri/tauri.conf.json`
    (`plugins.updater.pubkey`)
- The private key and password are **outside the repository** and were never printed or committed.
  `tests/policy.rs` fails if a private key ever appears in `tauri.conf.json`.
- `.github/workflows/ci.yml` and `release.yml` are written (see `GITHUB_RELEASE_PIPELINE.md`).

**Back up the private key and its password** (a password manager or an offline drive). If they are lost,
installed copies can never be updated again: every user would have to reinstall a build with a new public
key by hand.

## 1. Create the GitHub repository and push

The repository must be **public** for installed copies to download release assets without a token.

```bash
gh repo create <owner>/2vx-tweaks --public --source . --remote origin
```

```bash
git push -u origin main
```

Pushing `main` runs the release workflow. Without the secrets it stops at "Not published" (expected).

## 2. Add the two secrets

GitHub → repository → Settings → Secrets and variables → Actions → New repository secret:

| Name | Value |
| --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | The whole contents of `%USERPROFILE%\.tauri\2vx-tweaks.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | The contents of `%USERPROFILE%\.tauri\2vx-tweaks.key.password` |

Paste them into GitHub's form yourself. Never paste them into a chat, an issue, a commit or a log. With the
GitHub CLI you can set them straight from the files, so the values never appear on screen:

```bash
gh secret set TAURI_SIGNING_PRIVATE_KEY < ~/.tauri/2vx-tweaks.key
```

```bash
gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD < ~/.tauri/2vx-tweaks.key.password
```

## 3. Allow the workflow to create releases

Settings → Actions → General → Workflow permissions: leave the default as **Read repository contents**.
`release.yml` asks for `contents: write` on its release job itself. If the organisation forbids that, the
publish step fails with a 403 and nothing is published.

## 4. Run the first release

Actions → "Release (main channel)" → Run workflow (or push a change to `main`). Expected result:

- a release `v0.2.0` (or the next patch after the latest tag), marked latest, with the NSIS setup, its
  `.sig`, `2VX-Tweaks-Setup.exe` and `latest.json`;
- a prerelease `updater-main` holding `latest.json`;
- the job summary lists the release notes and "Published update feed".

Copies built before the feed URL was committed never update. Install a current setup once to replace them.

## 5. First end-to-end test (on a VM, not the development PC)

Use a disposable Windows 10 or 11 VM with a snapshot, never the only working development install.

1. Install `v0.2.N` from step 4 on the VM. Settings › Updates shows "Up to date" and the feed URL.
2. Push a small change to `main` (or run the workflow again) to publish `v0.2.N+1`.
3. In the VM: Settings › Updates › Check now → "Version 0.2.N+1 is available" with its notes.
4. Download → progress → "downloaded and verified".
5. Install and restart → UAC prompt → installer → the app reopens on the new version.
6. Check History and the change journal survived (they live in `%LOCALAPPDATA%\com.twovx.tweaks`).
7. Negative checks: edit `latest.json` on a test fork to point at an unsigned file → the download is
   rejected with a signature error; set a lower version → "Up to date".

## 6. Optional: Windows code signing

Updater signatures protect the update path but don't stop SmartScreen warning on first download. For
that, sign the installer with an Authenticode certificate (EV or Azure Trusted Signing) through
`bundle.windows.signCommand` and add its credentials as further secrets. Not configured yet.

## Troubleshooting

| Symptom | Cause |
| --- | --- |
| Summary says "Not published" | `TAURI_SIGNING_PRIVATE_KEY` secret missing or empty |
| Build fails with "incorrect updater private key password" | Password secret doesn't match the key |
| "Validate the new update feed" fails: not newer | A tag or feed with that version already exists; bump `package.json` |
| Update Centre: "Couldn't check for updates" with 404 | `updater-main` release or its `latest.json` missing, or the repository is private |
| Update Centre: signature error | The build was signed with a different key than the installed app's public key |
