import { describe, expect, it } from "vitest";
import type { TweakMeta, TweakStatus } from "@/types/engine";
import { LIBRARY_PRESETS, presetProgress } from "./library-presets";

const preset = (id: string) => LIBRARY_PRESETS.find((p) => p.id === id)!;

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
  it("hold only low-risk tweaks that break nothing and are confirmed", () => {
    const privacy = preset("privacy");
    expect(privacy.includes(status({}).meta)).toBe(true);
    expect(privacy.includes(status({ risk: "moderate" }).meta)).toBe(false);
    expect(privacy.includes(status({ warning: "Breaks search." }).meta)).toBe(false);
    expect(privacy.includes(status({ evidence: "community" }).meta)).toBe(false);
  });

  it("leave out the shipped catalogue", () => {
    expect(preset("privacy").includes(status({ section: null }).meta)).toBe(false);
  });

  it("match a category, or the hardware scope for This hardware", () => {
    expect(preset("gaming").includes(status({ category: "privacy" }).meta)).toBe(false);
    const gpu = status({ category: "gpu", hardware: "nvidiaGpu" }).meta;
    expect(preset("hardware").includes(gpu)).toBe(true);
    expect(preset("hardware").includes(status({ category: "gpu" }).meta)).toBe(false);
  });

  it("select what's usable and not in effect yet", () => {
    const statuses = [
      status({ id: "privacy.a" }),
      status({ id: "privacy.b" }, { state: "applied", undoable: true }),
      status({ id: "privacy.c" }, { compatibility: "wrongHardware" }),
    ];
    const progress = presetProgress(preset("privacy"), statuses);
    expect(progress.tweaks.map((t) => t.meta.id)).toEqual(["privacy.a", "privacy.b"]);
    expect(progress.open.map((t) => t.meta.id)).toEqual(["privacy.a"]);
    expect(progress.undoable.map((t) => t.meta.id)).toEqual(["privacy.b"]);
  });
});
