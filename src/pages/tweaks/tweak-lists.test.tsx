import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { HEALTHY, STATUSES, USER_TWEAK, plan } from "@/test/engine-fixtures";
import type { TweakCategory, TweakStatus } from "@/types/engine";
import { TweaksPage } from "./tweaks-page";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  getSystemSnapshot: vi.fn(),
  getAppInfo: vi.fn(),
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
  revertTweaks: vi.fn(),
  listSnapshots: vi.fn(),
}));

vi.mock("sonner", () => ({
  toast: { info: vi.fn(), success: vi.fn(), warning: vi.fn(), error: vi.fn() },
}));
vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));

function tweak(
  id: string,
  name: string,
  category: TweakCategory,
  section: string,
  overrides: Partial<TweakStatus> = {},
): TweakStatus {
  return {
    ...STATUSES[0]!,
    undoable: false,
    ...overrides,
    meta: { ...USER_TWEAK, id, name, category, section },
  };
}

const LISTED = [
  tweak("privacy.ceip", "Commercial pipeline", "privacy", "Telemetry"),
  tweak("privacy.tipc", "Lock screen camera", "privacy", "Speech"),
  tweak("windows.a", "Taskbar search box", "windows", "Taskbar and Start"),
  tweak("privacy.inventory", "Applied already", "privacy", "Telemetry", {
    state: "applied",
    undoable: true,
  }),
];

function renderTweaks(initial: string) {
  render(
    <MemoryRouter initialEntries={[initial]}>
      <TooltipProvider>
        <TweaksPage />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue(LISTED);
  mock.getSystemSnapshot.mockResolvedValue({ uptimeSecs: 60 });
  mock.getAppInfo.mockResolvedValue({
    name: "2VX Tweaks",
    version: "0.3.0",
    isElevated: false,
    logDir: "",
    debugBuild: false,
  });
  mock.planTweaks.mockImplementation((ids: string[]) => Promise.resolve(plan(ids)));
  mock.listSnapshots.mockResolvedValue([]);
});

describe("Long lists", () => {
  it("draws every section as cards, with nothing to open first", async () => {
    renderTweaks("/tweaks?tab=privacy");

    expect(await screen.findByText("Lock screen camera")).toBeInTheDocument();
    expect(screen.getByText("Commercial pipeline")).toBeInTheDocument();
    expect(document.querySelector("section [aria-expanded]")).toBeNull();
  });

  it("counts what's in effect on each section", async () => {
    renderTweaks("/tweaks?tab=privacy");
    const heading = await screen.findByRole("heading", { name: "Telemetry" });
    expect(within(heading.parentElement!).getByText("1 of 2 applied")).toBeInTheDocument();
  });

  it("searches the whole page from any tab, by name or by what a tweak changes", async () => {
    const user = userEvent.setup();
    renderTweaks("/tweaks?tab=core");
    expect(await screen.findByText("Taskbar search box")).toBeInTheDocument();

    await user.type(await screen.findByRole("searchbox"), "lock screen");
    expect(await screen.findByText("Lock screen camera")).toBeInTheDocument();
    expect(screen.queryByText("Taskbar search box")).not.toBeInTheDocument();
  });

  it("selects only what a preset still has to do, and leaves applying to the review", async () => {
    const user = userEvent.setup();
    renderTweaks("/tweaks?tab=presets");

    await user.click(await screen.findByRole("button", { name: "Select 2" }));
    expect(await screen.findByRole("button", { name: "Review 2 changes" })).toBeInTheDocument();
    expect(mock.applyPlan).not.toHaveBeenCalled();
  });

  it("asks before undoing a preset", async () => {
    const user = userEvent.setup();
    renderTweaks("/tweaks?tab=presets");

    await user.click(await screen.findByRole("button", { name: "Undo 1" }));
    expect(await screen.findByRole("alertdialog")).toBeInTheDocument();
    expect(mock.revertTweaks).not.toHaveBeenCalled();
  });
});
