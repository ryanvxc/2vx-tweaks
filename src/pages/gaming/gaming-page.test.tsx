import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { DEFAULT_GAME_MODE_TWEAKS } from "@/lib/profiles";
import { useUiStore } from "@/stores/ui-store";
import { change, HEALTHY, plan, STATUSES, USER_TWEAK } from "@/test/engine-fixtures";
import type { OperationOutcome, SnapshotRecord, TweakStatus } from "@/types/engine";
import { GamingPage } from "./gaming-page";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  getRunningPrograms: vi.fn(),
  getMetrics: vi.fn(),
  listSnapshots: vi.fn(),
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
  revertSnapshot: vi.fn(),
}));

const toast = vi.hoisted(() => ({
  info: vi.fn(),
  success: vi.fn(),
  warning: vi.fn(),
  error: vi.fn(),
}));
vi.mock("sonner", () => ({ toast }));
vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));

const GAME = { fileName: "game.exe", runningSeconds: 300, libraryGame: "Test Game" };

const done = (snapshotId: number): OperationOutcome => ({
  snapshotId,
  status: "completed",
  error: null,
  changes: [change({ snapshotId })],
  restart: "none",
});

const batch = (id: number, status: SnapshotRecord["status"]): SnapshotRecord => ({
  id,
  kind: "apply",
  tweakIds: [USER_TWEAK.id],
  reverts: null,
  status,
  error: null,
  appVersion: "0.2.0",
  createdAtMs: 0,
  finishedAtMs: 0,
  changeCount: 1,
});

function renderPage(initial = "/gaming") {
  render(
    <MemoryRouter initialEntries={[initial]}>
      <TooltipProvider>
        <GamingPage />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  useUiStore.setState({ games: [], gameModeTweaks: [USER_TWEAK.id], gameModeSnapshot: null });
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue(STATUSES);
  mock.getRunningPrograms.mockResolvedValue([]);
  mock.getMetrics.mockResolvedValue([]);
  mock.listSnapshots.mockResolvedValue([]);
  mock.planTweaks.mockImplementation((ids: string[]) => Promise.resolve(plan(ids)));
});

describe("GamingPage", () => {
  it("does nothing until a game is running", async () => {
    const user = userEvent.setup();
    renderPage();

    const ring = await screen.findByRole("button", { name: "Start a Game" });
    expect(ring).toHaveAttribute("aria-disabled", "true");
    await user.click(ring);
    expect(mock.planTweaks).not.toHaveBeenCalled();
    expect(screen.getAllByText("N/A").length).toBeGreaterThanOrEqual(2);
  });

  it("names a detected game and how long it has run", async () => {
    mock.getRunningPrograms.mockResolvedValue([GAME]);
    renderPage();

    expect(await screen.findByText("Test Game")).toBeInTheDocument();
    expect(screen.getByText("5m")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Turn on" })).toBeInTheDocument();
  });

  it("turns on through the review, then undoes only its own batch", async () => {
    const user = userEvent.setup();
    mock.getRunningPrograms.mockResolvedValue([GAME]);
    mock.applyPlan.mockResolvedValue(done(7));
    mock.revertSnapshot.mockResolvedValue(done(8));
    renderPage();

    await user.click(await screen.findByRole("button", { name: "Turn on" }));
    await user.click(await screen.findByRole("button", { name: "Apply 1 change" }));
    await waitFor(() => expect(useUiStore.getState().gameModeSnapshot).toBe(7));
    expect(mock.planTweaks).toHaveBeenCalledWith([USER_TWEAK.id]);

    await user.click(await screen.findByRole("button", { name: "On" }));
    await waitFor(() => expect(useUiStore.getState().gameModeSnapshot).toBeNull());
    expect(mock.revertSnapshot).toHaveBeenCalledWith(7);
  });

  it("says when some values couldn't be put back", async () => {
    const user = userEvent.setup();
    useUiStore.setState({ gameModeSnapshot: 7 });
    mock.revertSnapshot.mockResolvedValue({ ...done(8), status: "failed" });
    renderPage();

    await user.click(await screen.findByRole("button", { name: "On" }));
    await waitFor(() =>
      expect(toast.warning).toHaveBeenCalledWith("Partly reverted", expect.anything()),
    );
  });

  it("stays on when the undo fails, so it can be tried again", async () => {
    const user = userEvent.setup();
    useUiStore.setState({ gameModeSnapshot: 7 });
    mock.revertSnapshot.mockRejectedValue({ code: "internal", message: "no" });
    renderPage();

    await user.click(await screen.findByRole("button", { name: "On" }));
    await waitFor(() => expect(toast.error).toHaveBeenCalled());
    expect(useUiStore.getState().gameModeSnapshot).toBe(7);
  });

  it("counts a batch undone from the Restore Centre as off", async () => {
    useUiStore.setState({ gameModeSnapshot: 7 });
    mock.listSnapshots.mockResolvedValue([batch(7, "reverted")]);
    mock.getRunningPrograms.mockResolvedValue([GAME]);
    renderPage();

    expect(await screen.findByRole("button", { name: "Turn on" })).toBeInTheDocument();
  });

  it("has nothing to apply when the chosen tweaks are already in effect", async () => {
    const applied: TweakStatus[] = STATUSES.map((t) => ({ ...t, state: "applied" }));
    mock.getTweakStatuses.mockResolvedValue(applied);
    mock.getRunningPrograms.mockResolvedValue([GAME]);
    renderPage();

    const ring = await screen.findByRole("button", { name: "Already on" });
    expect(ring).toHaveAttribute("aria-disabled", "true");
  });

  it("adds a game and detects it by file name", async () => {
    const user = userEvent.setup();
    mock.getRunningPrograms.mockResolvedValue([
      { fileName: "mine.exe", runningSeconds: 60, libraryGame: null },
    ]);
    renderPage();

    await user.click(await screen.findByRole("button", { name: "Add games" }));
    await user.type(await screen.findByRole("textbox", { name: "Name" }), "My Game");
    await user.type(screen.getByRole("textbox", { name: "Program file" }), "C:\\mine.exe{Enter}");
    expect(await screen.findByText("Enter a file name ending in .exe.")).toBeInTheDocument();

    await user.clear(screen.getByRole("textbox", { name: "Program file" }));
    await user.type(screen.getByRole("textbox", { name: "Program file" }), "mine.exe{Enter}");

    expect(await screen.findByText("My Game")).toBeInTheDocument();
    expect(screen.getByText("1 added")).toBeInTheDocument();
    expect(useUiStore.getState().games).toEqual([{ name: "My Game", exe: "mine.exe" }]);
  });

  it("saves the list in Configure and can go back to the default", async () => {
    const user = userEvent.setup();
    const dvr: TweakStatus = {
      ...STATUSES[0]!,
      meta: { ...USER_TWEAK, id: "gaming.game-dvr-off", name: "Game DVR" },
    };
    mock.getTweakStatuses.mockResolvedValue([dvr]);
    useUiStore.setState({ gameModeTweaks: null });
    renderPage("/gaming?view=configure");

    const save = await screen.findByRole("button", { name: "Save" });
    expect(save).toBeDisabled();
    await user.click(screen.getByRole("switch", { name: "Include Game DVR" }));
    await user.click(save);
    expect(useUiStore.getState().gameModeTweaks).toEqual([
      ...DEFAULT_GAME_MODE_TWEAKS,
      "gaming.game-dvr-off",
    ]);

    await user.click(await screen.findByRole("button", { name: "Configure Game Mode" }));
    await user.click(await screen.findByRole("button", { name: "Revert to Default" }));
    await user.click(screen.getByRole("button", { name: "Save" }));
    expect(useUiStore.getState().gameModeTweaks).toEqual(DEFAULT_GAME_MODE_TWEAKS);
  });
});
