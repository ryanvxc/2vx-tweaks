import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { HEALTHY, SNAPSHOT, USER_TWEAK, change, outcome } from "@/test/engine-fixtures";
import { RestorePage } from "./restore-page";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  listTweaks: vi.fn(),
  listRetiredTweaks: vi.fn(),
  listSnapshots: vi.fn(),
  getSnapshotChanges: vi.fn(),
  revertSnapshot: vi.fn(),
  revertTweak: vi.fn(),
}));
const toast = vi.hoisted(() => ({ success: vi.fn(), warning: vi.fn(), error: vi.fn() }));

vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));
vi.mock("sonner", () => ({ toast }));

function renderPage() {
  render(
    <MemoryRouter>
      <TooltipProvider>
        <RestorePage />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue([]);
  mock.listTweaks.mockResolvedValue([USER_TWEAK]);
  mock.listRetiredTweaks.mockResolvedValue([]);
  mock.listSnapshots.mockResolvedValue([SNAPSHOT]);
  mock.getSnapshotChanges.mockResolvedValue([change()]);
});

describe("RestorePage", () => {
  it("shows an empty journal honestly", async () => {
    mock.listSnapshots.mockResolvedValue([]);
    renderPage();
    expect(await screen.findByText("No changes yet")).toBeInTheDocument();
  });

  it("names a removed tweak in old history and offers to undo what's still applied", async () => {
    const user = userEvent.setup();
    const gone = { id: "gaming.gone", name: "Gone tweak", risk: "low", undoable: true };
    mock.listRetiredTweaks.mockResolvedValue([gone]);
    mock.listSnapshots.mockResolvedValue([{ ...SNAPSHOT, tweakIds: [gone.id] }]);
    mock.revertTweak.mockResolvedValue(outcome());
    renderPage();

    const section = await screen.findByRole("region", { name: "Removed from 2VX" });
    expect(within(section).getByText("Gone tweak")).toBeInTheDocument();
    await user.click(within(section).getByRole("button", { name: "Undo" }));
    await user.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Undo" }));

    expect(mock.revertTweak).toHaveBeenCalledWith(gone.id);
    expect(mock.revertSnapshot).not.toHaveBeenCalled();
  });

  it("offers nothing when no removed tweak is still applied", async () => {
    mock.listRetiredTweaks.mockResolvedValue([
      { id: "gaming.gone", name: "Gone tweak", risk: "low", undoable: false },
    ]);
    mock.listSnapshots.mockResolvedValue([{ ...SNAPSHOT, tweakIds: ["gaming.gone"] }]);
    renderPage();

    expect(await screen.findByText("Gone tweak")).toBeInTheDocument();
    expect(screen.queryByRole("region", { name: "Removed from 2VX" })).not.toBeInTheDocument();
  });

  it("lists snapshots with tweak names and their recorded values", async () => {
    const user = userEvent.setup();
    renderPage();
    expect(await screen.findByText(USER_TWEAK.name)).toBeInTheDocument();
    // A finished change needs no label; only the exceptions get one.
    expect(screen.queryByText("Completed")).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Show changes in #7" }));
    expect(await screen.findByText(String.raw`HKCU\Software\Test\User`)).toBeInTheDocument();
    expect(screen.getByText("(not set) → 0 (REG_DWORD)")).toBeInTheDocument();
    expect(mock.getSnapshotChanges).toHaveBeenCalledWith(7);
  });

  it("undoes a batch after confirmation and refreshes the list", async () => {
    mock.revertSnapshot.mockResolvedValue(outcome({ snapshotId: 8 }));
    const user = userEvent.setup();
    renderPage();
    await user.click(await screen.findByRole("button", { name: "Undo" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(within(dialog).getByText(/left as is/)).toBeInTheDocument();
    const before = mock.listSnapshots.mock.calls.length;
    await user.click(within(dialog).getByRole("button", { name: "Undo" }));

    await vi.waitFor(() =>
      expect(toast.success).toHaveBeenCalledWith("Reverted", expect.anything()),
    );
    expect(mock.revertSnapshot).toHaveBeenCalledWith(7);
    expect(mock.listSnapshots.mock.calls.length).toBeGreaterThan(before);
  });

  it("warns when some values were left as is", async () => {
    mock.revertSnapshot.mockResolvedValue(
      outcome({ snapshotId: 8, status: "failed", changes: [change({ status: "conflict" })] }),
    );
    const user = userEvent.setup();
    renderPage();
    await user.click(await screen.findByRole("button", { name: "Undo" }));
    await user.click(
      within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Undo" }),
    );
    await vi.waitFor(() =>
      expect(toast.warning).toHaveBeenCalledWith("Partly reverted", expect.anything()),
    );
  });

  it("offers no undo for snapshots that have nothing left to revert", async () => {
    mock.listSnapshots.mockResolvedValue([
      { ...SNAPSHOT, status: "reverted" },
      { ...SNAPSHOT, id: 8, kind: "revert", reverts: 7 },
    ]);
    renderPage();
    expect(await screen.findByText(`Undid ${USER_TWEAK.name}`)).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Undo" })).not.toBeInTheDocument();
  });
});
