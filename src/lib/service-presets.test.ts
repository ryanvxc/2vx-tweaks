import { describe, expect, it } from "vitest";
import { HANDWRITTEN, LIBRARY } from "@/test/catalogue";
import { STATUSES, USER_TWEAK } from "@/test/engine-fixtures";
import type { TweakStatus } from "@/types/engine";
import { profileProgress } from "./profiles";
import { activePreset, SERVICE_PRESETS, undoableIn, type ServicePreset } from "./service-presets";

const [basic, advanced] = SERVICE_PRESETS as [ServicePreset, ServicePreset];

function tweak(id: string, patch: Partial<TweakStatus> = {}): TweakStatus {
  return { ...STATUSES[0]!, meta: { ...USER_TWEAK, id }, state: "notApplied", ...patch };
}

const on = (ids: string[]) => ids.map((id) => tweak(id, { state: "applied" }));

describe("service presets", () => {
  it("only name tweaks that exist in the Rust catalogue", () => {
    const catalogue = [...HANDWRITTEN, ...LIBRARY];
    expect(advanced.tweakIds.filter((id) => !catalogue.includes(id))).toEqual([]);
  });

  it("keep Advanced a strict superset of Basic", () => {
    expect(advanced.tweakIds).toEqual(expect.arrayContaining(basic.tweakIds));
    expect(advanced.tweakIds.length).toBeGreaterThan(basic.tweakIds.length);
  });

  it("show the fullest preset in effect, and only that one", () => {
    expect(activePreset([])).toBeNull();
    expect(activePreset(on(basic.tweakIds))?.id).toBe("basic");
    expect(activePreset(on(advanced.tweakIds))?.id).toBe("advanced");
  });

  it("offer undo only for what 2VX changed", () => {
    const [first, second] = basic.tweakIds as [string, string];
    const statuses = [
      tweak(first, { state: "applied", undoable: true }),
      tweak(second, { state: "applied" }),
    ];
    expect(undoableIn(profileProgress(basic, statuses))).toEqual([first]);
  });
});
