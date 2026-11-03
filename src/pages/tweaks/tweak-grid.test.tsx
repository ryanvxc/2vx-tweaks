import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import {
  ADMIN_TWEAK,
  APPLIED_TWEAK,
  HEALTHY,
  STATUSES,
  USER_TWEAK,
  outcome,
  plan,
} from "@/test/engine-fixtures";
import { TweakGrid } from "./tweak-grid";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  getAppInfo: vi.fn(),
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
  revertTweak: vi.fn(),
  listSnapshots: vi.fn(),
}));

vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));

function renderGrid(initial = "/tweaks", browse = false) {
  render(
    <MemoryRouter initialEntries={[initial]}>
      <TooltipProvider>
        <TweakGrid categories={["privacy", "performance"]} browse={browse} />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue(STATUSES);
  mock.getAppInfo.mockResolvedValue({
    name: "2VX Tweaks",
    version: "0.1.0",
    isElevated: false,
    logDir: "",
    debugBuild: false,
  });
  mock.planTweaks.mockImplementation((ids: string[]) => Promise.resolve(plan(ids)));
  mock.listSnapshots.mockResolvedValue([]);
});

describe("TweakGrid", () => {
  it("shows detected state on the switches", async () => {
    renderGrid();
    expect(
      await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }),
    ).not.toBeChecked();
    expect(screen.getByRole("switch", { name: `Undo ${APPLIED_TWEAK.name}` })).toBeChecked();
  });

  it("drops the New badge when most tweaks are new", async () => {
    // Fixtures are added in 0.1, so on 0.1.x all of them are new.
    renderGrid();
    await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` });
    expect(screen.queryByText("New")).not.toBeInTheDocument();
  });

  it("marks new tweaks when they are the minority", async () => {
    const [newest, ...rest] = STATUSES;
    const older = rest.map((t) => ({ ...t, meta: { ...t.meta, addedIn: "0.0" } }));
    mock.getTweakStatuses.mockResolvedValue([newest!, ...older]);
    renderGrid();
    expect(await screen.findAllByText("New")).toHaveLength(1);
  });

  it("flags tweaks that can break a feature and blocks ones that can't apply", async () => {
    const warned = { ...STATUSES[0]!, meta: { ...STATUSES[0]!.meta, warning: "Printing stops." } };
    const missing = { ...STATUSES[1]!, compatibility: "notInstalled" as const };
    mock.getTweakStatuses.mockResolvedValue([warned, missing, ...STATUSES.slice(2)]);
    renderGrid();
    expect(await screen.findAllByLabelText("Can break features")).toHaveLength(1);
    expect(screen.getByRole("switch", { name: `Turn on ${missing.meta.name}` })).toBeDisabled();
    expect(screen.getByRole("switch", { name: `Turn on ${warned.meta.name}` })).toBeEnabled();
  });

  it("selects a switch-on and applies only after review", async () => {
    mock.applyPlan.mockResolvedValue(outcome());
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }));
    expect(mock.planTweaks).not.toHaveBeenCalled();
    expect(screen.getByText(/selected/)).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Review 1 change" }));
    const dialog = await screen.findByRole("dialog");
    expect(mock.planTweaks).toHaveBeenCalledWith([USER_TWEAK.id]);
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Applied and verified")).toBeInTheDocument();
  });

  it("undoes an applied tweak after confirmation", async () => {
    mock.revertTweak.mockResolvedValue(outcome({ changes: [] }));
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Undo ${APPLIED_TWEAK.name}` }));
    const confirm = await screen.findByRole("alertdialog");
    expect(mock.revertTweak).not.toHaveBeenCalled();
    await user.click(within(confirm).getByRole("button", { name: "Undo" }));
    expect(mock.revertTweak).toHaveBeenCalledWith(APPLIED_TWEAK.id);
  });

  it("locks the switch of a value set outside 2VX", async () => {
    mock.getTweakStatuses.mockResolvedValue(
      STATUSES.map((s) => (s.meta.id === APPLIED_TWEAK.id ? { ...s, undoable: false } : s)),
    );
    renderGrid();
    expect(
      await screen.findByRole("switch", { name: `Undo ${APPLIED_TWEAK.name}` }),
    ).toBeDisabled();
  });

  it("shows one topic at a time, and searches across all of them", async () => {
    const user = userEvent.setup();
    renderGrid("/tweaks?tab=privacy", true);
    expect(await screen.findByText(USER_TWEAK.name)).toBeInTheDocument();
    expect(screen.queryByText(ADMIN_TWEAK.name)).not.toBeInTheDocument();

    await user.type(screen.getByRole("searchbox", { name: "Search tweaks" }), "admin");
    expect(screen.queryByText(USER_TWEAK.name)).not.toBeInTheDocument();
    expect(screen.getByText(ADMIN_TWEAK.name)).toBeInTheDocument();

    await user.clear(screen.getByRole("searchbox", { name: "Search tweaks" }));
    await user.click(screen.getByRole("tab", { name: /Services/ }));
    expect(screen.getByText("No tweaks match")).toBeInTheDocument();
  });

  it("names what a selected tweak can break in the review", async () => {
    const warned = { ...STATUSES[0]!, meta: { ...STATUSES[0]!.meta, warning: "Printing stops." } };
    mock.getTweakStatuses.mockResolvedValue([warned, ...STATUSES.slice(1)]);
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Turn on ${warned.meta.name}` }));
    await user.click(screen.getByRole("button", { name: "Review 1 change" }));
    const dialog = await screen.findByRole("dialog");
    expect(await within(dialog).findByText("Printing stops.")).toBeInTheDocument();
  });

  it("opens details from the URL", async () => {
    renderGrid(`/tweaks?tweak=${USER_TWEAK.id}`);
    const dialog = await screen.findByRole("dialog");
    expect(within(dialog).getByText("https://learn.microsoft.com/test")).toBeInTheDocument();
    expect(within(dialog).getByText("Now (not set)")).toBeInTheDocument();
  });

  it("disables changes while the journal is unavailable", async () => {
    mock.getEngineStatus.mockResolvedValue({ ...HEALTHY, journalAvailable: false });
    renderGrid();
    expect(
      await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }),
    ).toBeDisabled();
  });
});
