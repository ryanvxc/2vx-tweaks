import { describe, expect, it } from "vitest";
import { GAMING_PROFILES } from "./profiles";
import { TWEAK_ICON, TWEAK_SECTIONS, tweakHref, tweakPlacement } from "./tweak-presentation";

// The Rust catalogue is the source of truth; this catches a tweak added there and not here.
const SOURCES = import.meta.glob<string>("/src-tauri/src/engine/tweaks/**/*.rs", {
  query: "?raw",
  import: "default",
  eager: true,
});
const CATALOGUE = Object.values(SOURCES).flatMap((source) =>
  [...source.matchAll(/^\s+id: "([a-z]+\.[a-z0-9-]+)",/gm)].map((m) => m[1]!),
);

const PLACED = Object.values(TWEAK_SECTIONS).flatMap((sections) => sections.flatMap((s) => s.ids));

describe("tweak presentation", () => {
  it("reads the real catalogue", () => {
    expect(CATALOGUE.length).toBeGreaterThanOrEqual(35);
  });

  it("gives every tweak its own icon", () => {
    expect(CATALOGUE.filter((id) => !TWEAK_ICON[id])).toEqual([]);
    expect(new Set(Object.values(TWEAK_ICON)).size).toBe(Object.keys(TWEAK_ICON).length);
  });

  it("places every tweak once, and gaming tweaks only on Gaming", () => {
    const pageTweaks = CATALOGUE.filter((id) => !id.startsWith("gaming."));
    expect(pageTweaks.filter((id) => !PLACED.includes(id))).toEqual([]);
    expect(new Set(PLACED).size).toBe(PLACED.length);
    expect(PLACED.filter((id) => !CATALOGUE.includes(id))).toEqual([]);
  });

  it("builds profiles only from real tweaks", () => {
    const ids = GAMING_PROFILES.flatMap((p) => p.tweakIds);
    expect(ids.filter((id) => !CATALOGUE.includes(id))).toEqual([]);
  });

  it("links a tweak to its primary location", () => {
    expect(tweakHref({ id: "gaming.game-dvr-off", category: "gaming" })).toBe(
      "/gaming?tweak=gaming.game-dvr-off",
    );
    expect(tweakHref({ id: "privacy.edge-shopping-off", category: "privacy" })).toBe(
      "/tweaks?tab=apps&tweak=privacy.edge-shopping-off",
    );
  });

  it("still shows a tweak nobody placed yet", () => {
    expect(tweakPlacement({ id: "services.new-one", category: "services" })).toEqual({
      tab: "services",
      section: "More",
    });
  });
});
