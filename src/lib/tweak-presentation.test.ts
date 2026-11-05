import { describe, expect, it } from "vitest";
import type { TweakCategory } from "@/types/engine";
import { DEFAULT_GAME_MODE_TWEAKS, GAME_MODE_TWEAKS, GAMING_PROFILES } from "./profiles";
import {
  groupTweaks,
  TWEAK_ICON,
  TWEAK_SECTIONS,
  tweakHref,
  tweakPlacement,
} from "./tweak-presentation";

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
    // Network Priority's game rows show a letter, not an icon.
    const iconned = CATALOGUE.filter((id) => !id.startsWith("network.qos-"));
    expect(iconned.filter((id) => !TWEAK_ICON[id])).toEqual([]);
    expect(new Set(Object.values(TWEAK_ICON)).size).toBe(Object.keys(TWEAK_ICON).length);
  });

  it("places every tweak once", () => {
    expect(CATALOGUE.filter((id) => !PLACED.includes(id))).toEqual([]);
    expect(new Set(PLACED).size).toBe(PLACED.length);
    expect(PLACED.filter((id) => !CATALOGUE.includes(id))).toEqual([]);
  });

  it("builds profiles only from real tweaks", () => {
    const ids = [...GAMING_PROFILES.flatMap((p) => p.tweakIds), ...GAME_MODE_TWEAKS];
    expect(ids.filter((id) => !CATALOGUE.includes(id))).toEqual([]);
  });

  it("offers Game Mode's defaults, and every profile, in Configure", () => {
    const listed = [...DEFAULT_GAME_MODE_TWEAKS, ...GAMING_PROFILES.flatMap((p) => p.tweakIds)];
    expect(listed.filter((id) => !GAME_MODE_TWEAKS.includes(id))).toEqual([]);
  });

  it("links a tweak to its primary location", () => {
    expect(tweakHref({ id: "gaming.game-dvr-off", category: "gaming" })).toBe(
      "/tweaks?tab=core&tweak=gaming.game-dvr-off",
    );
    expect(tweakHref({ id: "privacy.edge-shopping-off", category: "privacy" })).toBe(
      "/tweaks?tab=apps&tweak=privacy.edge-shopping-off",
    );
  });

  describe("groupTweaks", () => {
    const tweak = (id: string, blocked = false) => ({
      meta: { id, category: id.split(".")[0] as TweakCategory },
      blocked,
    });
    const options = { unavailable: (t: { blocked: boolean }) => t.blocked };
    const ids = (groups: ReturnType<typeof groupTweaks<ReturnType<typeof tweak>>>) =>
      groups.map((g) => [g.title, g.tweaks.map((t) => t.meta.id)]);

    it("follows the section's written order", () => {
      const tweaks = [
        tweak("windows.storage-sense-off"),
        tweak("performance.last-access-off"),
        tweak("gaming.game-dvr-off"),
      ];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "core", catalogue: true }))).toEqual([
        [
          "Core",
          ["performance.last-access-off", "gaming.game-dvr-off", "windows.storage-sense-off"],
        ],
      ]);
    });

    it("drops sections with nothing in them", () => {
      const id = TWEAK_SECTIONS.privacy[1]!.ids[0]!;
      expect(
        ids(groupTweaks([tweak(id)], { ...options, tab: "privacy", catalogue: true })),
      ).toEqual([[TWEAK_SECTIONS.privacy[1]!.title, [id]]]);
    });

    it("keeps the user's sort inside a section", () => {
      const tweaks = [tweak("windows.storage-sense-off"), tweak("performance.last-access-off")];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "core", catalogue: false }))).toEqual([
        ["Core", ["windows.storage-sense-off", "performance.last-access-off"]],
      ]);
    });

    it("always puts what can't run last, even in a flat search", () => {
      const tweaks = [tweak("gaming.game-dvr-off", true), tweak("windows.storage-sense-off")];
      const last = (tab: "core" | null) =>
        groupTweaks(tweaks, { ...options, tab, catalogue: false }).at(-1);
      expect(last("core")?.title).toBe("Unavailable");
      expect(last(null)?.title).toBe("Unavailable");
      expect(last(null)?.tweaks.map((t) => t.meta.id)).toEqual(["gaming.game-dvr-off"]);
    });
  });

  it("still shows a tweak nobody placed yet", () => {
    expect(tweakPlacement({ id: "services.new-one", category: "services" })).toEqual({
      tab: "services",
      section: "More",
      route: "/tweaks",
    });
  });
});
