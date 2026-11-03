import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it } from "vitest";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { App } from "./app";

// Runs outside Tauri, so the backend service serves preview data.
describe("App shell", () => {
  beforeEach(() => {
    window.location.hash = "#/";
    clearQueryCache();
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
      "#/settings#updates",
    );
  });

  it("navigates via the sidebar", async () => {
    const user = userEvent.setup();
    render(<App />);
    const nav = screen.getByRole("navigation", { name: "Main" });
    await user.click(within(nav).getByRole("link", { name: /gaming/i }));
    expect(await screen.findByRole("heading", { name: "Gaming" })).toBeInTheDocument();
    expect(await screen.findByText("No optimisations in this section yet")).toBeInTheDocument();
    // The preview never claims an engine is connected.
    expect(screen.getByText("Browser preview")).toBeInTheDocument();
  });
});
