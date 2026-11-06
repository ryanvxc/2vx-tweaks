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
    useUiStore.setState({ showSplash: false, firstRunSeen: true });
  });

  it("opens behind the loading screen, unless it's turned off", async () => {
    vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    useUiStore.setState({ showSplash: true });
    render(<App />);
    expect(screen.getByRole("progressbar", { name: "Loading 2VX" })).toBeInTheDocument();

    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByRole("progressbar")).not.toBeInTheDocument());
    expect(screen.getByRole("heading", { name: /Welcome back/ })).toBeInTheDocument();
  });

  it("greets someone new once, and never promises what it can't do", async () => {
    const user = userEvent.setup();
    useUiStore.setState({ firstRunSeen: false });
    render(<App />);
    const dialog = await screen.findByRole("dialog", { name: "Before you start" });
    expect(within(dialog).getByText(/never changes your BIOS/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Not now" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(useUiStore.getState().firstRunSeen).toBe(true);
  });

  it("opens the BIOS guide, with no way to change anything", async () => {
    window.location.hash = "#/bios?tab=graphics";
    render(<App />);
    expect(await screen.findByRole("heading", { name: "BIOS" })).toBeInTheDocument();
    expect(await screen.findByText("Resizable BAR")).toBeInTheDocument();
    expect(screen.queryByRole("switch")).not.toBeInTheDocument();
  });

  it("splits the Restore Centre into changes, restore points and fixes", async () => {
    const user = userEvent.setup();
    window.location.hash = "#/restore";
    render(<App />);
    await user.click(await screen.findByRole("tab", { name: /Restore Points/ }));
    expect(await screen.findByText("Preview restore point")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Open System Restore" })).toBeInTheDocument();
    await user.click(screen.getByRole("tab", { name: /Fixes/ }));
    expect(await screen.findByText("Check Windows files")).toBeInTheDocument();
    expect(screen.getByText("Repair the Windows image")).toBeInTheDocument();
  });

  it("lists downloads on Resources", async () => {
    window.location.hash = "#/tools?tab=downloads";
    render(<App />);
    expect(await screen.findByText("Display Driver Uninstaller")).toBeInTheDocument();
    expect(screen.getByText("From Microsoft")).toBeInTheDocument();
  });

  it("renders Home with preview data clearly labelled", async () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: /Welcome back/ })).toBeInTheDocument();
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
    expect(await screen.findByRole("heading", { name: "2VX Game Mode" })).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "Start a Game" })).toBeInTheDocument();
    // The preview never claims an engine is connected.
    expect(screen.getByText("Browser preview")).toBeInTheDocument();
  });

  it("can only minimise or close the window", () => {
    render(<App />);
    expect(screen.getByRole("button", { name: "Minimise" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Close" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Maximise|Restore/ })).not.toBeInTheDocument();
  });

  it("gives Tweaks one sidebar row, with its tabs on the page", async () => {
    window.location.hash = "#/tweaks?tab=privacy";
    render(<App />);
    const nav = screen.getByRole("navigation", { name: "Main" });
    await waitFor(() =>
      expect(within(nav).getByRole("link", { name: "Tweaks" })).toHaveAttribute(
        "aria-current",
        "page",
      ),
    );
    expect(within(nav).queryByRole("link", { name: "Privacy" })).not.toBeInTheDocument();
    expect(await screen.findByRole("tab", { name: /Privacy/, selected: true })).toBeInTheDocument();
  });

  it("lists Visuals under Tweaks and above Hardware", () => {
    render(<App />);
    const nav = screen.getByRole("navigation", { name: "Main" });
    const names = within(nav)
      .getAllByRole("link")
      .map((link) => link.textContent);
    const from = names.indexOf("Tweaks");
    expect(names.slice(from, from + 6)).toEqual([
      "Tweaks",
      "Visuals",
      "Hardware",
      "PC Cleaner",
      "Network",
      "Advanced",
    ]);
  });

  it("opens Visuals with its tabs, as grids with no headings", async () => {
    window.location.hash = "#/visuals";
    render(<App />);
    expect(await screen.findByRole("heading", { name: "Visuals" })).toBeInTheDocument();
    expect(
      screen.getAllByRole("tab").map((tab) => tab.textContent?.replace(/\d+$/, "").trim()),
    ).toEqual(["Appearance", "Taskbar and Start", "File Explorer", "Sign-in"]);
    await screen.findByText("Preview sample: Visuals setting, Theme and colour");
    expect(screen.queryByRole("heading", { level: 2 })).not.toBeInTheDocument();
  });

  it("finds a Visuals tweak with Ctrl K and links to its tab", async () => {
    const user = userEvent.setup();
    render(<App />);
    await user.keyboard("{Control>}k{/Control}");
    const dialog = await screen.findByRole("dialog");
    await user.type(within(dialog).getByRole("textbox"), "Lock screen");
    await user.click(await within(dialog).findByRole("button", { name: /Lock screen.*Tweak/ }));
    await waitFor(() => expect(window.location.hash).toMatch(/^#\/visuals\?tab=signin&tweak=/));
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
