import { describe, expect, it } from "vitest";
import type { TweakMeta, TweakStatus } from "@/types/engine";
import { OFFERED_SOURCE, VISUALS_SOURCE } from "@/test/catalogue";
import { GPU_PROFILES, PRESET_TWEAKS, TWEAK_PRESETS, presetProgress } from "./tweak-presets";

const preset = (id: string) => TWEAK_PRESETS.find((p) => p.id === id)!;

function status(meta: Partial<TweakMeta>, rest: Partial<TweakStatus> = {}): TweakStatus {
  return {
    meta: {
      id: "privacy.a",
      category: "privacy",
      section: "Telemetry",
      risk: "low",
      warning: null,
      evidence: "documented",
      hardware: "any",
      ...meta,
    } as TweakMeta,
    compatibility: "compatible",
    state: "notApplied",
    settings: [],
    undoable: false,
    ...rest,
  };
}

describe("library presets", () => {
  const listedId = PRESET_TWEAKS.privacy[0];

  it("hold only low-risk tweaks that break nothing and are confirmed", () => {
    const privacy = preset("privacy");
    const meta = (over: Partial<TweakMeta>) => status({ id: listedId, ...over }).meta;
    expect(privacy.includes(meta({}))).toBe(true);
    expect(privacy.includes(meta({ risk: "moderate" }))).toBe(false);
    expect(privacy.includes(meta({ warning: "Breaks search." }))).toBe(false);
    expect(privacy.includes(meta({ evidence: "community" }))).toBe(false);
  });

  it("take only what they name, so a new tweak can't walk into one", () => {
    expect(preset("privacy").includes(status({ id: "privacy.brand-new" }).meta)).toBe(false);
    expect(preset("gaming").includes(status({ id: listedId }).meta)).toBe(false);
  });

  it("name only tweaks in the catalogue, none of them Visuals, each in one preset", () => {
    const ids = Object.values(PRESET_TWEAKS).flat();
    expect(ids.filter((id) => !OFFERED_SOURCE.includes(`"${id}"`))).toEqual([]);
    expect(ids.filter((id) => VISUALS_SOURCE.includes(`"${id}"`))).toEqual([]);
    expect(new Set(ids).size).toBe(ids.length);
  });

  it("never include a Visuals tweak, whatever else it matches", () => {
    const visual = (over: Partial<TweakMeta>) =>
      status({ id: listedId, category: "visuals", section: "Taskbar", ...over }).meta;
    expect(TWEAK_PRESETS.filter((p) => p.includes(visual({})))).toEqual([]);
    expect(GPU_PROFILES.filter((p) => p.includes(visual({ hardware: "nvidiaGpu" })))).toEqual([]);
  });

  it("select what's usable and not in effect yet", () => {
    const [first, second, third] = PRESET_TWEAKS.privacy;
    const statuses = [
      status({ id: first }),
      status({ id: second }, { state: "applied", undoable: true }),
      status({ id: third }, { compatibility: "wrongHardware" }),
    ];
    const progress = presetProgress(preset("privacy"), statuses);
    expect(progress.tweaks.map((t) => t.meta.id)).toEqual([first, second]);
    expect(progress.open.map((t) => t.meta.id)).toEqual([first]);
    expect(progress.undoable.map((t) => t.meta.id)).toEqual([second]);
  });
});
