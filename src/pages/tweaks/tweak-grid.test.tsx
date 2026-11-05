import { render, screen, waitFor, within } from "@testing-library/react";
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
import { useUiStore } from "@/stores/ui-store";
import { TweakGrid } from "./tweak-grid";
import { TWEAKS_VIEW, VISUALS_VIEW } from "./tweak-views";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  getSystemSnapshot: vi.fn(),
  getAppInfo: vi.fn(),
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
  revertTweak: vi.fn(),
  listSnapshots: vi.fn(),
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

function renderGrid(initial = "/tweaks", browse = false) {
  render(
    <MemoryRouter initialEntries={[initial]}>
      <TooltipProvider>
        {browse ? (
          <TweakGrid view={TWEAKS_VIEW} />
        ) : (
          <TweakGrid categories={["privacy", "performance"]} />
        )}
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue(STATUSES);
  mock.getSystemSnapshot.mockResolvedValue({ uptimeSecs: null });
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

describe("TweakGrid on Visuals", () => {
  const visual = (id: string, name: string, section: string, restart: "none" | "signOut") => ({
    ...STATUSES[0]!,
    meta: { ...USER_TWEAK, id, name, category: "visuals" as const, section, restart },
  });
  const VISUALS = [
    visual("windows.v-theme", "Dark apps", "Theme and colour", "none"),
    visual("windows.v-align", "Left taskbar", "Taskbar", "signOut"),
  ];

  beforeEach(() => {
    mock.getTweakStatuses.mockResolvedValue([...STATUSES, ...VISUALS]);
  });

  const renderVisuals = () =>
    render(
      <MemoryRouter initialEntries={["/visuals"]}>
        <TooltipProvider>
          <TweakGrid view={VISUALS_VIEW} />
        </TooltipProvider>
      </MemoryRouter>,
    );

  it("lists only Visuals tweaks, on the tab their section belongs to", async () => {
    renderVisuals();
    expect(await screen.findByRole("switch", { name: "Turn on Dark apps" })).toBeInTheDocument();
    expect(screen.queryByRole("switch", { name: "Turn on Left taskbar" })).toBeNull();
    expect(screen.queryByRole("switch", { name: `Turn on ${USER_TWEAK.name}` })).toBeNull();
  });

  it("filters by Restart required, across every tab once searching", async () => {
    const user = userEvent.setup();
    renderVisuals();
    await user.type(await screen.findByRole("searchbox", { name: "Search" }), "a");
    await user.click(screen.getByRole("button", { name: /Filter/ }));
    expect(await screen.findByText("Restart required")).toBeInTheDocument();
    await user.click(screen.getByRole("menuitemcheckbox", { name: "Needs a restart or sign-out" }));
    await user.keyboard("{Escape}");
    expect(screen.getByRole("switch", { name: "Turn on Left taskbar" })).toBeInTheDocument();
    expect(screen.queryByRole("switch", { name: "Turn on Dark apps" })).toBeNull();
  });

  it("narrows a tab to one section with Category when nothing is searched", async () => {
    const peek = visual("windows.v-peek", "Peek previews", "Effects and animation", "none");
    mock.getTweakStatuses.mockResolvedValue([...VISUALS, peek]);
    const user = userEvent.setup();
    renderVisuals();
    await screen.findByRole("switch", { name: "Turn on Dark apps" });
    await user.click(screen.getByRole("button", { name: /Filter/ }));
    await user.click(
      await screen.findByRole("menuitemcheckbox", { name: "Effects and animation" }),
    );
    await user.keyboard("{Escape}");
    expect(screen.getByRole("switch", { name: "Turn on Peek previews" })).toBeInTheDocument();
    expect(screen.queryByRole("switch", { name: "Turn on Dark apps" })).toBeNull();
  });

  it("offers the page's tabs as Category, since every tweak shares one engine category", async () => {
    const user = userEvent.setup();
    renderVisuals();
    await user.type(await screen.findByRole("searchbox", { name: "Search" }), "a");
    await user.click(screen.getByRole("button", { name: /Filter/ }));
    await user.click(await screen.findByRole("menuitemcheckbox", { name: "Taskbar and Start" }));
    expect(screen.queryByRole("menuitemcheckbox", { name: "File Explorer" })).toBeNull();
    await user.keyboard("{Escape}");
    expect(screen.getByRole("switch", { name: "Turn on Left taskbar" })).toBeInTheDocument();
    expect(screen.queryByRole("switch", { name: "Turn on Dark apps" })).toBeNull();
  });
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
    expect(screen.queryByRole("switch", { name: `Turn on ${missing.meta.name}` })).toBeNull();
    expect(screen.getByText("Not installed")).toBeInTheDocument();
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
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(mock.applyPlan).toHaveBeenCalledOnce();
    expect(screen.queryByText(/selected/)).not.toBeInTheDocument();
  });

  it("marks the moment a tweak takes effect on its card", async () => {
    const animate = vi.fn();
    Element.prototype.animate = animate;
    useUiStore.setState({ reduceMotion: false });
    mock.applyPlan.mockResolvedValue(outcome());
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }));
    await user.click(screen.getByRole("button", { name: "Review 1 change" }));
    const dialog = await screen.findByRole("dialog");
    animate.mockClear();

    mock.getTweakStatuses.mockResolvedValue(
      STATUSES.map((t) => (t.meta.id === USER_TWEAK.id ? { ...t, state: "applied" as const } : t)),
    );
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    await waitFor(() => expect(animate).toHaveBeenCalledOnce());

    const [keyframes] = animate.mock.calls[0] as [Keyframe[]];
    expect(keyframes[0]).toHaveProperty("borderColor", "var(--success)");
    delete (Element.prototype as Partial<Element>).animate;
  });

  it("reminds about a restart after a clean apply", async () => {
    mock.applyPlan.mockResolvedValue(outcome({ restart: "reboot" }));
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }));
    await user.click(screen.getByRole("button", { name: "Review 1 change" }));
    const dialog = await screen.findByRole("dialog");
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    await waitFor(() => expect(toast.info).toHaveBeenCalledWith("Restart Windows to finish."));
  });

  it("keeps the review open when the apply didn't finish cleanly", async () => {
    mock.applyPlan.mockResolvedValue(outcome({ status: "rolledBack", error: "write failed" }));
    const user = userEvent.setup();
    renderGrid();
    await user.click(await screen.findByRole("switch", { name: `Turn on ${USER_TWEAK.name}` }));
    await user.click(screen.getByRole("button", { name: "Review 1 change" }));
    const dialog = await screen.findByRole("dialog");
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Not applied")).toBeInTheDocument();
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

  it("says a value is already set outside 2VX, with no switch that looks undoable", async () => {
    mock.getTweakStatuses.mockResolvedValue(
      STATUSES.map((s) => (s.meta.id === APPLIED_TWEAK.id ? { ...s, undoable: false } : s)),
    );
    renderGrid();
    expect(await screen.findByText("Already set")).toBeInTheDocument();
    expect(screen.queryByRole("switch", { name: `Undo ${APPLIED_TWEAK.name}` })).toBeNull();
  });

  it("never offers a setting it couldn't read", async () => {
    const unread = { ...STATUSES[0]!, state: "unknown" as const };
    mock.getTweakStatuses.mockResolvedValue([unread, ...STATUSES.slice(1)]);
    renderGrid();
    expect(await screen.findByLabelText("Couldn't read this setting")).toBeInTheDocument();
    expect(screen.getByRole("switch", { name: `Turn on ${unread.meta.name}` })).toBeDisabled();
  });

  it("marks a restart still to come and a change that failed", async () => {
    const reboot = { ...APPLIED_TWEAK, restart: "reboot" as const };
    mock.getTweakStatuses.mockResolvedValue(
      STATUSES.map((s) => (s.meta.id === APPLIED_TWEAK.id ? { ...s, meta: reboot } : s)),
    );
    mock.getSystemSnapshot.mockResolvedValue({ uptimeSecs: 60 });
    mock.listSnapshots.mockResolvedValue([
      {
        id: 2,
        kind: "apply",
        tweakIds: [APPLIED_TWEAK.id],
        reverts: null,
        status: "completed",
        error: null,
        appVersion: "0.4.0",
        createdAtMs: Date.now(),
        finishedAtMs: Date.now(),
        changeCount: 1,
      },
      {
        id: 1,
        kind: "apply",
        tweakIds: [STATUSES.find((s) => s.state !== "applied")!.meta.id],
        reverts: null,
        status: "rolledBack",
        error: "A policy put it back",
        appVersion: "0.4.0",
        createdAtMs: 1,
        finishedAtMs: 2,
        changeCount: 1,
      },
    ]);
    renderGrid();
    expect(await screen.findByLabelText("Restart to finish")).toBeInTheDocument();
    expect(await screen.findByLabelText("Last change failed")).toBeInTheDocument();
  });

  it("shows one topic at a time, and searches across all of them", async () => {
    const user = userEvent.setup();
    renderGrid("/tweaks?tab=privacy", true);
    expect(await screen.findByText(USER_TWEAK.name)).toBeInTheDocument();
    expect(screen.queryByText(ADMIN_TWEAK.name)).not.toBeInTheDocument();

    await user.type(screen.getByRole("searchbox", { name: "Search" }), "admin");
    expect(screen.queryByText(USER_TWEAK.name)).not.toBeInTheDocument();
    expect(screen.getByText(ADMIN_TWEAK.name)).toBeInTheDocument();

    await user.clear(screen.getByRole("searchbox", { name: "Search" }));
    await user.click(screen.getByRole("tab", { name: /Power/ }));
    expect(await screen.findByText("No tweaks match")).toBeInTheDocument();
  });

  it("puts tweaks that can't run on this PC in a last section, whatever the sort", async () => {
    const user = userEvent.setup();
    mock.getTweakStatuses.mockResolvedValue([
      { ...STATUSES[0]!, compatibility: "notInstalled" },
      ...STATUSES.slice(1),
    ]);
    renderGrid("/tweaks?tab=privacy", true);

    const last = async () => (await screen.findAllByRole("heading", { level: 2 })).at(-1);
    expect(await last()).toHaveTextContent("Unavailable");

    await user.click(screen.getByRole("button", { name: /Sort/ }));
    await user.click(await screen.findByRole("menuitemradio", { name: "Name" }));
    expect(await last()).toHaveTextContent("Unavailable");
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
