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

  it("renders the dashboard with preview data clearly labelled", async () => {
    render(<App />);
    expect(screen.getByRole("heading", { name: "Dashboard" })).toBeInTheDocument();
    expect(await screen.findByText("PREVIEW-PC")).toBeInTheDocument();
    expect(screen.getByText(/browser preview/i)).toBeInTheDocument();
    // Sample content must carry the sample label.
    expect(screen.getByText("Sample data")).toBeInTheDocument();
  });

  it("navigates via the sidebar", async () => {
    const user = userEvent.setup();
    render(<App />);
    const nav = screen.getByRole("navigation", { name: "Main" });
    await user.click(within(nav).getByRole("link", { name: /gaming/i }));
    expect(await screen.findByRole("heading", { name: "Gaming" })).toBeInTheDocument();
    expect(await screen.findByText("No optimisations registered yet")).toBeInTheDocument();
  });
});
