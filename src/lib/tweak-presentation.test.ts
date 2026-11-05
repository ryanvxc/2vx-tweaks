import { describe, expect, it } from "vitest";
import { HANDWRITTEN, LIBRARY, VISUALS_SOURCE } from "@/test/catalogue";
import type { TweakCategory } from "@/types/engine";
import { DEFAULT_GAME_MODE_TWEAKS, GAME_MODE_TWEAKS, GAMING_PROFILES } from "./profiles";
import {
  capGroups,
  groupTweaks,
  searchRank,
  SECTION_ICON,
  TWEAK_ICON,
  TWEAK_SECTIONS,
  tweakHref,
  tweakPlacement,
  UNAVAILABLE_SECTION,
  VISUALS_SECTION_TAB,
  VISUALS_SETTINGS_PAGE,
} from "./tweak-presentation";
import { VISUAL_ICON } from "./visual-icons";

const CATALOGUE = [...HANDWRITTEN, ...LIBRARY];

const PLACED = Object.values(TWEAK_SECTIONS).flatMap((sections) => sections.flatMap((s) => s.ids));

describe("tweak presentation", () => {
  it("reads the real catalogue", () => {
    expect(CATALOGUE.length).toBeGreaterThanOrEqual(35);
  });

  it("gives every tweak its own icon", () => {
    // Network Priority's games share their section's icon.
    const iconned = HANDWRITTEN.filter((id) => !id.startsWith("network.qos-"));
    expect(iconned.filter((id) => !TWEAK_ICON[id])).toEqual([]);
    expect(new Set(Object.values(TWEAK_ICON)).size).toBe(Object.keys(TWEAK_ICON).length);
  });

  it("places every tweak once", () => {
    expect(HANDWRITTEN.filter((id) => !PLACED.includes(id))).toEqual([]);
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
      "/tweaks?tab=gaming&tweak=gaming.game-dvr-off",
    );
    expect(tweakHref({ id: "privacy.edge-shopping-off", category: "privacy" })).toBe(
      "/tweaks?tab=apps&tweak=privacy.edge-shopping-off",
    );
  });

  it("names a Windows page only for Visuals sections that exist", () => {
    expect(
      Object.keys(VISUALS_SETTINGS_PAGE).filter((section) => !VISUALS_SECTION_TAB[section]),
    ).toEqual([]);
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
      const tweaks = [tweak("gaming.game-dvr-off"), tweak("gaming.mmcss-games-priority")];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "gaming", catalogue: true }))).toEqual([
        ["Essentials", ["gaming.mmcss-games-priority", "gaming.game-dvr-off"]],
      ]);
    });

    it("drops sections with nothing in them", () => {
      const id = TWEAK_SECTIONS.privacy[1]!.ids[0]!;
      expect(
        ids(groupTweaks([tweak(id)], { ...options, tab: "privacy", catalogue: true })),
      ).toEqual([[TWEAK_SECTIONS.privacy[1]!.title, [id]]]);
    });

    it("never pads a section with a setting for one game", () => {
      const listed = (id: string, section: string) => ({
        ...tweak(id),
        meta: { ...tweak(id).meta, name: "Short", section },
      });
      const tweaks = [
        tweak("gaming.mmcss-games-priority"),
        listed("gaming.a", "Per-game priority"),
        listed("gaming.b", "Other"),
      ];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "gaming", catalogue: true }))).toEqual([
        ["Essentials", ["gaming.mmcss-games-priority", "gaming.b"]],
        ["Per-game priority", ["gaming.a"]],
      ]);
    });

    it("tops a section up to a full row of cards from the library", () => {
      const listed = (id: string, name: string, section: string) => ({
        ...tweak(id),
        meta: { ...tweak(id).meta, name, section },
      });
      const tweaks = [
        tweak("gaming.mmcss-games-priority"),
        listed("gaming.a", "A name far too long to fit on the card it would sit on", "Other"),
        listed("gaming.b", "Short", "Other"),
        listed("gaming.c", "Also short", "Other"),
      ];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "gaming", catalogue: true }))).toEqual([
        ["Essentials", ["gaming.mmcss-games-priority", "gaming.b", "gaming.c"]],
        ["Other", ["gaming.a"]],
      ]);
    });

    it("keeps the user's sort inside a section", () => {
      const tweaks = [tweak("gaming.game-dvr-off"), tweak("gaming.mmcss-games-priority")];
      expect(ids(groupTweaks(tweaks, { ...options, tab: "gaming", catalogue: false }))).toEqual([
        ["Essentials", ["gaming.game-dvr-off", "gaming.mmcss-games-priority"]],
      ]);
    });

    it("always puts what can't run last, even in a flat search", () => {
      const tweaks = [
        tweak("performance.last-access-off", true),
        tweak("performance.short-names-off"),
      ];
      const last = (tab: "core" | null) =>
        groupTweaks(tweaks, { ...options, tab, catalogue: false }).at(-1);
      expect(last("core")?.title).toBe("Unavailable");
      expect(last(null)?.title).toBe("Unavailable");
      expect(last(null)?.tweaks.map((t) => t.meta.id)).toEqual(["performance.last-access-off"]);
    });
  });

  describe("Visuals", () => {
    const visual = (id: string, section: string | null) => ({
      id,
      category: "visuals" as const,
      section,
    });

    it("lands each tweak on the tab its section belongs to", () => {
      const tabOf = (section: string) => tweakPlacement(visual("windows.a", section)).tab;
      expect(tabOf("Theme and colour")).toBe("appearance");
      expect(tabOf("Windows and desktop")).toBe("appearance");
      expect(tabOf("Start menu")).toBe("taskbar");
      expect(tabOf("Context menu")).toBe("explorer");
      expect(tabOf("Startup and shutdown")).toBe("signin");
      expect(tweakPlacement(visual("windows.a", "Taskbar")).route).toBe("/visuals");
    });

    it("keeps an id when it moves to Visuals, and links to the Visuals page", () => {
      expect(tweakHref(visual("windows.window-animations-off", "Effects and animation"))).toBe(
        "/visuals?tab=appearance&tweak=windows.window-animations-off",
      );
    });

    it("lists the sections of a tab in the written order, not alphabetically", () => {
      const tweaks = ["Open and save", "View", "Context menu", "Navigation pane"].map(
        (section) => ({
          meta: visual(`windows.${section.length}`, section),
        }),
      );
      const groups = groupTweaks(tweaks, {
        tab: "explorer",
        catalogue: true,
        unavailable: () => false,
      });
      expect(groups.map((g) => g.title)).toEqual([
        "View",
        "Navigation pane",
        "Context menu",
        "Open and save",
      ]);
    });

    it("gives every section an icon of its own", () => {
      const titles = Object.keys(VISUALS_SECTION_TAB);
      expect(titles.filter((t) => !SECTION_ICON[t])).toEqual([]);
      // "Startup and shutdown" is shared with the Windows section of the same name.
      const icons = titles.filter((t) => t !== "Startup and shutdown").map((t) => SECTION_ICON[t]);
      expect(new Set(icons).size).toBe(icons.length);
    });

    it("gives every Visuals card an icon of its own", () => {
      const ids = [...VISUALS_SOURCE.matchAll(/"([a-z]+\.[a-z0-9-]+)",\s*Visuals,/g)].map(
        (m) => m[1]!,
      );
      expect(ids.filter((id) => !VISUAL_ICON[id])).toEqual([]);
      expect(new Set(Object.values(VISUAL_ICON)).size).toBe(Object.keys(VISUAL_ICON).length);
    });

    it("has no Visuals tweak in Rust with a section the app can't place", () => {
      const found = [...VISUALS_SOURCE.matchAll(/"[a-z]+\.[a-z0-9-]+",\s*Visuals,\s*"([^"]+)"/g)];
      expect(found.length).toBeGreaterThan(50);
      expect(found.filter((m) => !(m[1]! in VISUALS_SECTION_TAB))).toEqual([]);
    });
  });

  it("still shows a tweak nobody placed yet", () => {
    expect(tweakPlacement({ id: "services.new-one", category: "services" })).toEqual({
      tab: "services",
      section: "More",
      route: "/cleaner",
      listed: false,
    });
  });
});

describe("long lists", () => {
  const meta = (section: string | null, id = "privacy.a") => ({
    id,
    category: "privacy" as const,
    section,
  });

  it("places a tweak with a section under its topic", () => {
    expect(tweakPlacement(meta("Telemetry"))).toEqual({
      tab: "privacy",
      section: "Telemetry",
      route: "/tweaks",
      listed: true,
    });
    expect(tweakPlacement({ id: "gpu.a", category: "gpu", section: "NVIDIA" }).route).toBe(
      "/hardware",
    );
    expect(tweakPlacement({ id: "network.a", category: "network", section: "TCP" }).route).toBe(
      "/network",
    );
    expect(tweakPlacement({ id: "services.a", category: "services", section: "VPN" }).route).toBe(
      "/cleaner",
    );
    expect(tweakPlacement({ id: "memory.a", category: "memory", section: "Prefetch" }).tab).toBe(
      "ram",
    );
  });

  it("links a tweak to its one location", () => {
    expect(tweakHref(meta("Telemetry"))).toBe("/tweaks?tab=privacy&tweak=privacy.a");
  });

  it("groups by the tweaks' own sections, unavailable last", () => {
    const tweaks = [
      { meta: meta("Speech", "privacy.c") },
      { meta: meta("Ads", "privacy.d") },
      { meta: meta("Speech", "privacy.e") },
    ];
    const groups = groupTweaks(tweaks, {
      tab: "privacy",
      catalogue: true,
      unavailable: (t) => t.meta.id === "privacy.e",
    });
    expect(groups.map((g) => g.title)).toEqual(["Ads", "Speech", UNAVAILABLE_SECTION]);
  });

  it("puts hand-placed sections first, and a library section of the same name joins its own", () => {
    const hand = TWEAK_SECTIONS.privacy[1]!;
    const groups = groupTweaks(
      [
        { meta: meta(hand.title, "privacy.x") },
        { meta: meta(hand.title, "privacy.y") },
        { meta: meta("Ads", "privacy.z") },
        { meta: { id: hand.ids[0]!, category: "privacy" as const, section: null } },
      ],
      { tab: "privacy", catalogue: true, unavailable: () => false },
    );
    expect(groups.map((g) => [g.title, g.tweaks.map((t) => t.meta.id)])).toEqual([
      [hand.title, [hand.ids[0], "privacy.x", "privacy.y"]],
      ["Ads", ["privacy.z"]],
    ]);
  });

  it("searches what a tweak changes as well as what it's called", () => {
    const m = { name: "Ads", description: "No ads", category: "privacy" as const };
    expect(searchRank(m, "disabledbypolicy", ["HKLM\\Foo\\DisabledByPolicy"])).toBe(5);
    expect(searchRank(m, "disabledbypolicy")).toBeNull();
  });

  it("caps a long tab across groups", () => {
    const groups = [
      { title: "A", tweaks: [1, 2] },
      { title: "B", tweaks: [3, 4] },
      { title: "C", tweaks: [5] },
    ];
    expect(capGroups(groups, 3)).toEqual([
      { title: "A", tweaks: [1, 2] },
      { title: "B", tweaks: [3] },
    ]);
    expect(capGroups(groups, Number.POSITIVE_INFINITY)).toEqual(groups);
  });

  it("ranks a search by where it matched", () => {
    const m = {
      name: "Game Bar tips",
      description: "Hides the overlay.",
      category: "gaming" as const,
      section: "Overlays",
    };
    expect(searchRank(m, "game")).toBe(0);
    expect(searchRank(m, "bar")).toBe(1);
    expect(searchRank(m, "ame")).toBe(2);
    expect(searchRank(m, "overlay.")).toBe(3);
    expect(searchRank(m, "overlays")).toBe(4);
    expect(searchRank(m, "gaming")).toBe(4);
    expect(searchRank(m, "printer")).toBeNull();
  });
});
