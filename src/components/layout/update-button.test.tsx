import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TooltipProvider } from "@/components/ui/tooltip";

const plugin = vi.hoisted(() => ({ support: vi.fn(), check: vi.fn(), restart: vi.fn() }));
const toast = vi.hoisted(() => ({ info: vi.fn(), success: vi.fn(), error: vi.fn() }));

vi.mock("@/services/updater", () => ({ updater: plugin }));
vi.mock("sonner", () => ({ toast }));

const { resetUpdater } = await import("@/hooks/use-updater");
const { useUiStore } = await import("@/stores/ui-store");
const { UpdateButton } = await import("./update-button");

const SUPPORTED = { available: true, reason: null, endpoint: "https://x/y.json", channel: "main" };

function renderButton() {
  render(<UpdateButton collapsed={false} />, { wrapper: TooltipProvider });
  return userEvent.setup();
}

const button = (name: string) => screen.findByRole("button", { name });

beforeEach(() => {
  resetUpdater();
  vi.clearAllMocks();
  useUiStore.setState({ autoCheckUpdates: false });
  plugin.support.mockResolvedValue(SUPPORTED);
});

describe("UpdateButton", () => {
  it("checks, downloads, then restarts to apply", async () => {
    const install = vi.fn(async () => {});
    plugin.check.mockResolvedValue({
      info: { version: "0.3.0", currentVersion: "0.2.0", date: null, notes: "" },
      download: async () => {},
      install,
    });
    const user = renderButton();

    await user.click(await button("Check for updates"));
    expect(install).not.toHaveBeenCalled();

    await user.click(await button("Restart to update"));
    expect(install).toHaveBeenCalledOnce();
  });

  it("says when there is nothing new", async () => {
    plugin.check.mockResolvedValue(null);
    const user = renderButton();
    await user.click(await button("Check for updates"));
    await waitFor(() => expect(toast.success).toHaveBeenCalledWith("2VX Tweaks is up to date"));
  });

  it("explains a build that can't update, and never checks", async () => {
    plugin.support.mockResolvedValue({ ...SUPPORTED, available: false, reason: "No channel." });
    const user = renderButton();
    await waitFor(async () => {
      await user.click(await button("Check for updates"));
      expect(toast.info).toHaveBeenCalledWith("No channel.");
    });
    expect(plugin.check).not.toHaveBeenCalled();
  });

  it("shows a failed check and offers it again", async () => {
    plugin.check.mockRejectedValue(new Error("offline"));
    const user = renderButton();
    await user.click(await button("Check for updates"));
    await waitFor(() => expect(toast.error).toHaveBeenCalledWith("offline"));
    expect(await button("Check for updates")).toBeEnabled();
  });
});
