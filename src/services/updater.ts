import { relaunch } from "@tauri-apps/plugin-process";
import { check, type DownloadEvent, type Update } from "@tauri-apps/plugin-updater";
import type { UpdateSupport } from "@/types/system";
import { backend, isDesktop } from "./backend";

/**
 * The only module that talks to the updater and process plugins. It never installs on its
 * own: `checkForUpdate` only reads the feed, and download/install run when the user asks.
 * Signature verification and the newer-version-only rule are enforced by the plugin in
 * Rust; nothing here can turn them off.
 *
 * Outside the desktop shell a scripted mock feed stands in, chosen with `?update=` in the
 * URL (see `MOCK_FEEDS`), so every Update Centre state can be reviewed without a network.
 */

export interface UpdateInfo {
  version: string;
  currentVersion: string;
  /** ISO date from the feed, when given. */
  date: string | null;
  /** Release notes (Markdown-ish plain text from the feed). */
  notes: string;
}

export interface DownloadProgress {
  downloaded: number;
  /** `null` when the server didn't send a length. */
  total: number | null;
}

export interface PendingUpdate {
  info: UpdateInfo;
  download: (onProgress: (p: DownloadProgress) => void) => Promise<void>;
  /** On Windows the installer closes the app itself; elsewhere call `restart` after. */
  install: () => Promise<void>;
}

export interface UpdaterService {
  support: () => Promise<UpdateSupport>;
  check: () => Promise<PendingUpdate | null>;
  restart: () => Promise<void>;
}

function fromPlugin(update: Update): PendingUpdate {
  return {
    info: {
      version: update.version,
      currentVersion: update.currentVersion,
      date: update.date ?? null,
      notes: update.body ?? "",
    },
    download: (onProgress) => {
      let downloaded = 0;
      let total: number | null = null;
      return update.download(
        (event: DownloadEvent) => {
          if (event.event === "Started") total = event.data.contentLength ?? null;
          if (event.event === "Progress") downloaded += event.data.chunkLength;
          onProgress({ downloaded, total });
        },
        // Without this the download inherits the check's short timeout and fails on slow lines.
        { timeout: DOWNLOAD_TIMEOUT_MS },
      );
    },
    install: () => update.install(),
  };
}

const CHECK_TIMEOUT_MS = 30_000;
const DOWNLOAD_TIMEOUT_MS = 10 * 60_000;

const desktop: UpdaterService = {
  support: backend.getUpdateSupport,
  check: async () => {
    const update = await check({ timeout: CHECK_TIMEOUT_MS });
    return update ? fromPlugin(update) : null;
  },
  restart: () => relaunch(),
};

// ---- Browser preview: scripted feeds, never a network request --------------------------

export const MOCK_FEEDS = ["none", "available", "error", "download-error", "unconfigured"] as const;
type MockFeed = (typeof MOCK_FEEDS)[number];

const MOCK_NOTES = `## What's new
- Redesigned interface: card grid for tweaks, new Home page and Settings
- Switch a tweak off to undo exactly what 2VX changed
- Hardware page with live per-core CPU, GPU, memory, storage and network
- Startup and PC Cleaner show what is on this PC (read-only)

## Fixes
- The Restore Centre shows per-tweak undo batches`;

const wait = (ms: number) => new Promise((r) => setTimeout(r, ms));

function mockFeed(): MockFeed {
  const value = new URLSearchParams(window.location.search).get("update");
  return MOCK_FEEDS.includes(value as MockFeed) ? (value as MockFeed) : "available";
}

const preview: UpdaterService = {
  support: async () =>
    mockFeed() === "unconfigured"
      ? {
          available: false,
          reason: "This build has no update channel. (Preview: ?update=unconfigured)",
          endpoint: null,
          channel: "main",
        }
      : {
          available: true,
          reason: null,
          endpoint: "https://github.com/OWNER/REPO/releases/download/updater-main/latest.json",
          channel: "main",
        },
  check: async () => {
    await wait(700);
    const feed = mockFeed();
    if (feed === "error") throw new Error("Couldn't reach the update server (preview)");
    if (feed === "none" || feed === "unconfigured") return null;
    return {
      info: {
        version: "0.3.0",
        currentVersion: "0.2.0",
        date: "2026-11-03T12:00:00Z",
        notes: MOCK_NOTES,
      },
      download: async (onProgress) => {
        const total = 9_400_000;
        for (let downloaded = 0; downloaded < total; downloaded += 940_000) {
          await wait(150);
          if (feed === "download-error" && downloaded > total / 2) {
            throw new Error("Signature check failed: the download was discarded (preview)");
          }
          onProgress({ downloaded, total });
        }
        onProgress({ downloaded: total, total });
      },
      install: async () => {
        await wait(400);
        throw new Error("Installing is not available in browser preview");
      },
    };
  },
  restart: async () => {
    throw new Error("Not available in browser preview");
  },
};

export const updater: UpdaterService = isDesktop ? desktop : preview;
