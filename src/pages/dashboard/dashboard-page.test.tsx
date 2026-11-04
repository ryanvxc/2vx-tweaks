import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { backend } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { SNAPSHOT, STATUSES, USER_TWEAK } from "@/test/engine-fixtures";
import type { TweakStatus } from "@/types/engine";
import { DashboardPage } from "./dashboard-page";

const OPEN: TweakStatus = {
  ...STATUSES[0]!,
  state: "notApplied",
  compatibility: "compatible",
  meta: { ...USER_TWEAK, recommended: true },
};

function renderPage() {
  render(
    <MemoryRouter>
      <TooltipProvider>
        <DashboardPage />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

beforeEach(() => {
  clearQueryCache();
  useUiStore.setState({
    reduceMotion: true,
    games: [],
    gameModeTweaks: null,
    gameModeSnapshot: null,
    quickstartOpened: [],
  });
  vi.spyOn(backend, "getTweakStatuses").mockResolvedValue([]);
  vi.spyOn(backend, "listSnapshots").mockResolvedValue([]);
});

afterEach(() => {
  vi.restoreAllMocks();
  useUiStore.setState({ reduceMotion: false });
});

describe("DashboardPage", () => {
  it("greets the account by name", async () => {
    vi.spyOn(backend, "getAccount").mockResolvedValue({ email: "someone@example.com" });
    renderPage();
    expect(
      await screen.findByRole("heading", { name: "Welcome back, someone!" }),
    ).toBeInTheDocument();
  });

  it("counts only the backups that can still be restored", async () => {
    vi.mocked(backend.listSnapshots).mockResolvedValue([
      SNAPSHOT,
      { ...SNAPSHOT, id: 2, status: "reverted" },
      { ...SNAPSHOT, id: 3, kind: "revert" },
    ]);
    renderPage();

    const card = await screen.findByRole("link", { name: /Backup saved/ });
    expect(card).toHaveAttribute("href", "/restore");
    expect(await within(card).findByText("1")).toBeInTheDocument();
  });

  it("offers what's left to apply, then Game Mode", async () => {
    vi.mocked(backend.getTweakStatuses).mockResolvedValue([OPEN]);
    renderPage();
    expect(await screen.findByRole("link", { name: "1 recommended tweak" })).toHaveAttribute(
      "href",
      "/tweaks",
    );
  });

  it("goes to Game Mode when nothing is left to apply", async () => {
    renderPage();
    expect(await screen.findByRole("link", { name: "Open Game Mode" })).toHaveAttribute(
      "href",
      "/gaming",
    );
  });

  it("ticks a Quickstart step that was opened, and keeps it", async () => {
    const user = userEvent.setup();
    renderPage();

    const step = await screen.findByRole("link", { name: /Check your startup apps/ });
    expect(within(step).getByTitle("Not done")).toBeInTheDocument();
    await user.click(step);
    expect(useUiStore.getState().quickstartOpened).toEqual(["startup"]);
    expect(within(step).getByTitle("Done")).toBeInTheDocument();

    await user.click(step);
    expect(useUiStore.getState().quickstartOpened).toEqual(["startup"]);
  });

  it("keeps health problems in the System health dialog, not at the top of the page", async () => {
    const user = userEvent.setup();
    const real = await backend.getSystemSnapshot();
    vi.spyOn(backend, "getSystemSnapshot").mockResolvedValue({
      ...real,
      disks: [{ ...real.disks[0]!, mountPoint: "C:\\", availableBytes: 1, totalBytes: 100 }],
    });
    renderPage();

    const health = await screen.findByRole("button", { name: /System health/ });
    expect(screen.queryByText("System drive space")).not.toBeInTheDocument();

    await user.click(health);
    expect(
      await within(await screen.findByRole("dialog")).findByText("System drive space"),
    ).toBeInTheDocument();
  });

  it("opens the patch notes from the changelog and the health checks", async () => {
    const user = userEvent.setup();
    renderPage();

    await user.click(await screen.findByRole("button", { name: /Patch Notes/ }));
    const notes = await screen.findByRole("dialog");
    expect(within(notes).getByRole("heading", { name: /^Patch Notes \d/ })).toBeInTheDocument();
    await user.keyboard("{Escape}");

    await user.click(await screen.findByRole("button", { name: /System health/ }));
    expect(
      await within(await screen.findByRole("dialog")).findByText("Uptime"),
    ).toBeInTheDocument();
  });
});
