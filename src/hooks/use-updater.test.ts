import { beforeEach, describe, expect, it, vi } from "vitest";

const plugin = vi.hoisted(() => ({
  support: vi.fn(),
  check: vi.fn(),
  restart: vi.fn(),
}));

vi.mock("@/services/updater", () => ({ updater: plugin }));

const { resetUpdater, useUpdaterStore } = await import("./use-updater");
const { useUiStore } = await import("@/stores/ui-store");

const INFO = { version: "0.3.0", currentVersion: "0.2.0", date: null, notes: "- Fixes" };
const SUPPORTED = {
  available: true,
  reason: null,
  endpoint: "https://github.com/o/r/x.json",
  channel: "main",
};

function pending(
  overrides: Partial<{ download: () => Promise<void>; install: () => Promise<void> }> = {},
) {
  return {
    info: INFO,
    download: vi.fn(
      async (onProgress: (p: { downloaded: number; total: number | null }) => void) => {
        onProgress({ downloaded: 50, total: 100 });
        onProgress({ downloaded: 100, total: 100 });
      },
    ),
    install: vi.fn(async () => {}),
    ...overrides,
  };
}

const state = () => useUpdaterStore.getState().state;

beforeEach(() => {
  resetUpdater();
  vi.clearAllMocks();
  useUiStore.setState({ autoCheckUpdates: true });
  plugin.support.mockResolvedValue(SUPPORTED);
});

describe("updater state machine", () => {
  it("is unsupported on builds without a channel and never checks", async () => {
    plugin.support.mockResolvedValue({ ...SUPPORTED, available: false, reason: "dev build" });
    await useUpdaterStore.getState().init();
    expect(state()).toEqual({ kind: "unsupported", reason: "dev build" });
    await useUpdaterStore.getState().check();
    expect(plugin.check).not.toHaveBeenCalled();
  });

  it("checks automatically only when the preference is on", async () => {
    useUiStore.setState({ autoCheckUpdates: false });
    await useUpdaterStore.getState().init();
    expect(state().kind).toBe("idle");
    expect(plugin.check).not.toHaveBeenCalled();
  });

  it("reports up to date", async () => {
    plugin.check.mockResolvedValue(null);
    await useUpdaterStore.getState().init();
    expect(state().kind).toBe("upToDate");
  });

  it("never downloads or installs without being asked", async () => {
    const p = pending();
    plugin.check.mockResolvedValue(p);
    await useUpdaterStore.getState().init();
    expect(state()).toEqual({ kind: "available", info: INFO });
    expect(p.download).not.toHaveBeenCalled();

    // Install is refused before the download finished.
    await useUpdaterStore.getState().install();
    expect(p.install).not.toHaveBeenCalled();

    await useUpdaterStore.getState().download();
    expect(state()).toEqual({ kind: "ready", info: INFO });
    expect(p.install).not.toHaveBeenCalled();

    await useUpdaterStore.getState().install();
    expect(p.install).toHaveBeenCalledOnce();
    expect(state().kind).toBe("restartRequired");
  });

  it("downloads straight after a check only when the button asks for it", async () => {
    const p = pending();
    plugin.check.mockResolvedValue(p);
    useUiStore.setState({ autoCheckUpdates: false });
    await useUpdaterStore.getState().init();

    await useUpdaterStore.getState().checkAndDownload();
    expect(state()).toEqual({ kind: "ready", info: INFO });
    expect(p.install).not.toHaveBeenCalled();

    plugin.check.mockResolvedValue(null);
    resetUpdater();
    await useUpdaterStore.getState().init();
    await useUpdaterStore.getState().checkAndDownload();
    expect(state().kind).toBe("upToDate");
  });

  it("offers a retry after a failed check, download or install", async () => {
    plugin.check.mockRejectedValueOnce(new Error("offline"));
    await useUpdaterStore.getState().init();
    expect(state()).toMatchObject({ kind: "error", phase: "check", message: "offline" });

    const p = pending({ download: vi.fn().mockRejectedValueOnce(new Error("bad signature")) });
    plugin.check.mockResolvedValue(p);
    await useUpdaterStore.getState().check();
    await useUpdaterStore.getState().download();
    expect(state()).toMatchObject({ kind: "error", phase: "download", message: "bad signature" });
    // Retry from the error state.
    p.download = vi.fn(async () => {});
    await useUpdaterStore.getState().download();
    expect(state().kind).toBe("ready");
  });
});
