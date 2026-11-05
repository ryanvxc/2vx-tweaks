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

  it("offers what is left to apply, at the tab that holds the first one", async () => {
    vi.mocked(backend.getTweakStatuses).mockResolvedValue([OPEN]);
    renderPage();
    expect(await screen.findByRole("link", { name: "1 recommended tweak" })).toHaveAttribute(
      "href",
      expect.stringMatching(/^\/tweaks\?tab=/),
    );
  });

  it("shows no action when nothing is left to apply", async () => {
    renderPage();
    await screen.findByRole("heading", { name: /Welcome back/ });
    expect(
      screen.queryByRole("link", { name: /^d+ recommended tweaks?$/ }),
    ).not.toBeInTheDocument();
    expect(screen.queryByRole("link", { name: "Open Game Mode" })).not.toBeInTheDocument();
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

  it("shows the health score on a card and opens the list from it", async () => {
    const user = userEvent.setup();
    renderPage();

    const card = await screen.findByRole("button", { name: /System health/ });
    expect(await within(card).findByText(/of \d+/)).toBeInTheDocument();
    expect(screen.queryByText(/Need help/)).not.toBeInTheDocument();
    await user.click(card);
    expect(await screen.findByRole("dialog")).toBeInTheDocument();
  });

  it("says how 2VX is running, and links to where updating stands", async () => {
    vi.spyOn(backend, "getAppInfo").mockResolvedValue({
      ...(await backend.getAppInfo()),
      isElevated: true,
    });
    renderPage();
    expect(await screen.findByText(/· Administrator$/)).toBeInTheDocument();
    expect(await screen.findByRole("link", { name: /Update Centre/ })).toHaveAttribute(
      "href",
      "/settings?tab=updates",
    );
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
