import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { SettingsPage } from "./settings-page";

const auto = vi.hoisted(() => ({
  getAutoLaunch: vi.fn(),
  setAutoLaunch: vi.fn(),
}));

// The desktop path for this one row, and the preview's answers for the rest of the page.
vi.mock("@/services/backend", async (importOriginal) => {
  const original = await importOriginal<typeof import("@/services/backend")>();
  return { ...original, isDesktop: true, backend: { ...original.backend, ...auto } };
});

function renderPage() {
  render(
    <MemoryRouter>
      <TooltipProvider>
        <SettingsPage />
      </TooltipProvider>
    </MemoryRouter>,
  );
  return screen.findByRole("switch", { name: "Open at sign-in" });
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
});

describe("Open at sign-in", () => {
  it("says so when Task Manager has it off, and leaves that to Task Manager", async () => {
    auto.getAutoLaunch.mockResolvedValue("offInTaskManager");
    const row = await renderPage();

    expect(await screen.findByText("Off in Task Manager")).toBeInTheDocument();
    expect(row).toHaveAttribute("aria-checked", "false");
    expect(row).toBeDisabled();
    await userEvent.setup({ pointerEventsCheck: 0 }).click(row);
    expect(auto.setAutoLaunch).not.toHaveBeenCalled();
  });

  it("is on only when Windows will run it", async () => {
    auto.getAutoLaunch.mockResolvedValue("on");
    const row = await renderPage();

    await waitFor(() => expect(row).toHaveAttribute("aria-checked", "true"));
    expect(row).toBeEnabled();
    expect(screen.queryByText("Off in Task Manager")).not.toBeInTheDocument();
  });

  it("believes Windows, not the click, after turning it on", async () => {
    auto.getAutoLaunch.mockResolvedValueOnce("off");
    // A stale Task Manager entry holds back the value that was just written.
    auto.getAutoLaunch.mockResolvedValueOnce("offInTaskManager");
    auto.setAutoLaunch.mockResolvedValue(undefined);
    const row = await renderPage();
    await waitFor(() => expect(row).toBeEnabled());

    await userEvent.setup().click(row);

    expect(auto.setAutoLaunch).toHaveBeenCalledWith(true);
    expect(await screen.findByText("Off in Task Manager")).toBeInTheDocument();
    expect(row).toHaveAttribute("aria-checked", "false");
  });

  it("goes back to what it was when the change fails", async () => {
    auto.getAutoLaunch.mockResolvedValue("off");
    auto.setAutoLaunch.mockRejectedValue({ code: "system", message: "nope" });
    const row = await renderPage();
    await waitFor(() => expect(row).toBeEnabled());

    await userEvent.setup().click(row);

    await waitFor(() => expect(auto.setAutoLaunch).toHaveBeenCalled());
    await waitFor(() => expect(row).toHaveAttribute("aria-checked", "false"));
  });
});
