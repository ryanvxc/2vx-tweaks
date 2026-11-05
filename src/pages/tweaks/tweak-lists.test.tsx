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
  tweak("privacy.a", "Commercial pipeline", "privacy", "Telemetry"),
  tweak("privacy.b", "Lock screen camera", "privacy", "App permissions"),
  tweak("windows.a", "Taskbar search box", "windows", "Taskbar and Start"),
  tweak("privacy.c", "Applied already", "privacy", "Telemetry", {
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
  it("folds a tab into sections with the first one open", async () => {
    const user = userEvent.setup();
    renderTweaks("/tweaks?tab=privacy");

    expect(await screen.findByText("Lock screen camera")).toBeInTheDocument();
    expect(screen.queryByText("Commercial pipeline")).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /Telemetry/ }));
    expect(screen.getByText("Commercial pipeline")).toBeInTheDocument();
  });

  it("counts what's in effect on each section", async () => {
    renderTweaks("/tweaks?tab=privacy");
    const section = await screen.findByRole("button", { name: /Telemetry/ });
    expect(within(section).getByText("1 of 2 in effect")).toBeInTheDocument();
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
