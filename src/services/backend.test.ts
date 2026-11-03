import { describe, expect, it } from "vitest";
import { backend, isDesktop } from "./backend";

// Tests run outside Tauri, so these exercise the browser-preview fallbacks.
describe("backend engine commands in preview", () => {
  it("runs outside the desktop shell", () => {
    expect(isDesktop).toBe(false);
  });

  it("serves labelled samples and no journal", async () => {
    const status = await backend.getEngineStatus();
    const statuses = await backend.getTweakStatuses();
    expect(status.journalAvailable).toBe(false);
    expect(status.tweakCount).toBe(statuses.length);
    // Preview samples must never pass for the real catalogue.
    for (const t of statuses) expect(t.meta.name).toMatch(/^Preview sample/);
    expect((await backend.listTweaks()).map((t) => t.id)).toEqual(statuses.map((t) => t.meta.id));
    await expect(backend.listSnapshots()).resolves.toEqual([]);
    await expect(backend.getSnapshotChanges(1)).resolves.toEqual([]);
  });

  it("refuses anything that would change the system instead of simulating it", async () => {
    const refused = { code: "preview", message: "Not available in browser preview" };
    await expect(backend.planTweaks(["gaming.game-mode"])).rejects.toEqual(refused);
    await expect(backend.applyPlan("0".repeat(32))).rejects.toEqual(refused);
    await expect(backend.revertSnapshot(1)).rejects.toEqual(refused);
  });
});
