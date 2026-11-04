import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { useUiStore } from "@/stores/ui-store";
import { App } from "./app";

// Runs outside Tauri, so the backend service serves preview data.
describe("App shell", () => {
  beforeEach(() => {
    window.location.hash = "#/";
    clearQueryCache();
    useUiStore.setState({ showSplash: false });
  });

  it("opens behind the loading screen, unless it's turned off", async () => {
    vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    useUiStore.setState({ showSplash: true });
    render(<App />);
    expect(screen.getByRole("progressbar", { name: "Loading 2VX" })).toBeInTheDocument();

    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByRole("progressbar")).not.toBeInTheDocument());
    expect(screen.getByRole("heading", { name: "Welcome back" })).toBeInTheDocument();
  });

  it("renders Home with preview data clearly labelled", async () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: "Welcome back" })).toBeInTheDocument();
    expect(await screen.findByText(/PREVIEW-PC/)).toBeInTheDocument();
    expect(screen.getByText("Browser preview: simulated data")).toBeInTheDocument();
    // Live data in the browser is labelled as simulated.
    expect((await screen.findAllByText("Preview")).length).toBeGreaterThan(0);
  });

  it("shows the update notice from the mock feed and links to the Update Centre", async () => {
    render(<App />);
    expect(await screen.findByText("2VX Tweaks 0.3.0 is available")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "See what's new" })).toHaveAttribute(
      "href",
      "#/settings?tab=updates",
    );
  });

  it("navigates via the sidebar", async () => {
    const user = userEvent.setup();
    render(<App />);
    const nav = screen.getByRole("navigation", { name: "Main" });
    await user.click(within(nav).getByRole("link", { name: /game mode/i }));
    expect(await screen.findByRole("heading", { name: "Game Mode" })).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Responsive" })).toBeInTheDocument();
    // The preview never claims an engine is connected.
    expect(screen.getByText("Browser preview")).toBeInTheDocument();
  });

  it("keeps old addresses working", async () => {
    window.location.hash = "#/history";
    render(<App />);
    expect(await screen.findByRole("heading", { name: "Restore Centre" })).toBeInTheDocument();
    expect(window.location.hash).toBe("#/restore");
  });

  it("opens the Update Centre from the old section link", async () => {
    window.location.hash = "#/settings#updates";
    render(<App />);
    expect(await screen.findByRole("heading", { name: "Updates" })).toBeInTheDocument();
  });

  it("finds a page with Ctrl K and opens it", async () => {
    const user = userEvent.setup();
    render(<App />);
    await user.keyboard("{Control>}k{/Control}");
    const dialog = await screen.findByRole("dialog");
    await user.type(within(dialog).getByRole("textbox"), "cleaner{Enter}");
    expect(await screen.findByRole("heading", { name: "PC Cleaner" })).toBeInTheDocument();
  });
});
