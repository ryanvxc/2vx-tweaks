import { useEffect } from "react";
import { create } from "zustand";
import {
  updater,
  type DownloadProgress,
  type PendingUpdate,
  type UpdateInfo,
} from "@/services/updater";
import { logger } from "@/services/logger";
import { useUiStore } from "@/stores/ui-store";

/**
 * Update Centre state machine. Every step after "available" waits for the user: nothing
 * downloads or installs on its own. `checkAndDownload` is that click for the sidebar button.
 *
 *   unsupported                       (dev build or no channel)
 *   idle → checking → upToDate | available | error(check)
 *   available → downloading → ready | error(download)
 *   ready → installing → (Windows closes the app) | restartRequired | error(install)
 */
export type UpdaterState =
  | { kind: "loading" }
  | { kind: "unsupported"; reason: string }
  | { kind: "idle" }
  | { kind: "checking" }
  | { kind: "upToDate"; checkedAt: number }
  | { kind: "available"; info: UpdateInfo }
  | { kind: "downloading"; info: UpdateInfo; progress: DownloadProgress }
  | { kind: "ready"; info: UpdateInfo }
  | { kind: "installing"; info: UpdateInfo }
  | { kind: "restartRequired"; info: UpdateInfo }
  | { kind: "error"; phase: "check" | "download" | "install"; message: string; info?: UpdateInfo };

interface UpdaterStore {
  state: UpdaterState;
  endpoint: string | null;
  channel: string;
  lastCheckedAt: number | null;
  init: () => Promise<void>;
  check: () => Promise<void>;
  checkAndDownload: () => Promise<void>;
  download: () => Promise<void>;
  install: () => Promise<void>;
  restart: () => Promise<void>;
}

// The plugin's Update handle is not serialisable state; it lives beside the store.
let pending: PendingUpdate | null = null;
let initialised = false;

const message = (err: unknown) => (err instanceof Error ? err.message : String(err));

export const useUpdaterStore = create<UpdaterStore>()((set, get) => ({
  state: { kind: "loading" },
  endpoint: null,
  channel: "main",
  lastCheckedAt: null,

  init: async () => {
    if (initialised) return;
    initialised = true;
    try {
      const support = await updater.support();
      set({ endpoint: support.endpoint, channel: support.channel });
      if (!support.available) {
        set({
          state: { kind: "unsupported", reason: support.reason ?? "Updates are unavailable" },
        });
        return;
      }
      set({ state: { kind: "idle" } });
      if (useUiStore.getState().autoCheckUpdates) await get().check();
    } catch (err) {
      set({ state: { kind: "unsupported", reason: message(err) } });
    }
  },

  check: async () => {
    const kind = get().state.kind;
    if (kind === "unsupported" || kind === "checking" || kind === "downloading") return;
    set({ state: { kind: "checking" } });
    try {
      pending = await updater.check();
      const now = Date.now();
      set({
        lastCheckedAt: now,
        state: pending
          ? { kind: "available", info: pending.info }
          : { kind: "upToDate", checkedAt: now },
      });
      if (pending) logger.info(`update ${pending.info.version} available`, "updater");
    } catch (err) {
      logger.warn(`update check failed: ${message(err)}`, "updater");
      set({ state: { kind: "error", phase: "check", message: message(err) } });
    }
  },

  checkAndDownload: async () => {
    await get().check();
    if (get().state.kind === "available") await get().download();
  },

  download: async () => {
    const { state } = get();
    if (state.kind !== "available" && !(state.kind === "error" && state.phase === "download"))
      return;
    if (!pending) return;
    const info = pending.info;
    set({ state: { kind: "downloading", info, progress: { downloaded: 0, total: null } } });
    try {
      await pending.download((progress) => set({ state: { kind: "downloading", info, progress } }));
      set({ state: { kind: "ready", info } });
    } catch (err) {
      logger.warn(`update download failed: ${message(err)}`, "updater");
      set({ state: { kind: "error", phase: "download", message: message(err), info } });
    }
  },

  install: async () => {
    const { state } = get();
    if (state.kind !== "ready" && !(state.kind === "error" && state.phase === "install")) return;
    if (!pending) return;
    const info = pending.info;
    set({ state: { kind: "installing", info } });
    try {
      logger.info(`installing update ${info.version}`, "updater");
      await pending.install();
      // Reached only where the installer doesn't close the app (not Windows).
      set({ state: { kind: "restartRequired", info } });
    } catch (err) {
      logger.warn(`update install failed: ${message(err)}`, "updater");
      set({ state: { kind: "error", phase: "install", message: message(err), info } });
    }
  },

  restart: async () => {
    try {
      await updater.restart();
    } catch (err) {
      const info = "info" in get().state ? (get().state as { info?: UpdateInfo }).info : undefined;
      set({ state: { kind: "error", phase: "install", message: message(err), info } });
    }
  },
}));

/** Subscribes to the updater and starts it once per app session. */
export function useUpdater() {
  const store = useUpdaterStore();
  useEffect(() => {
    void useUpdaterStore.getState().init();
  }, []);
  return store;
}

/** For tests: forget the session's updater state. */
export function resetUpdater() {
  pending = null;
  initialised = false;
  useUpdaterStore.setState({ state: { kind: "loading" }, endpoint: null, lastCheckedAt: null });
}
