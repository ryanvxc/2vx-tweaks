import { describe, expect, it } from "vitest";
import { STATUSES, USER_TWEAK } from "@/test/engine-fixtures";
import type { TweakStatus } from "@/types/engine";
import { activeProfile, GAMING_PROFILES, profileProgress } from "./profiles";

function tweak(id: string, patch: Partial<TweakStatus> = {}): TweakStatus {
  return { ...STATUSES[0]!, meta: { ...USER_TWEAK, id }, state: "notApplied", ...patch };
}

const on = (ids: string[]) => ids.map((id) => tweak(id, { state: "applied" }));
const [responsive, quiet, everything] = GAMING_PROFILES as [
  (typeof GAMING_PROFILES)[0],
  (typeof GAMING_PROFILES)[0],
  (typeof GAMING_PROFILES)[0],
];

describe("gaming profiles", () => {
  it("is active only when every tweak is in effect", () => {
    expect(activeProfile([])).toBeNull();
    expect(activeProfile(on(responsive.tweakIds.slice(0, 1)))).toBeNull();
    expect(activeProfile(on(responsive.tweakIds))?.id).toBe("responsive");
  });

  it("reports the fullest active profile", () => {
    expect(activeProfile(on(everything.tweakIds))?.id).toBe("everything");
    expect(activeProfile(on(quiet.tweakIds))?.id).toBe("quiet");
  });

  it("doesn't let a tweak this PC can't use hold a profile back", () => {
    const blocked = "privacy.background-apps-off";
    const statuses = [
      ...on(quiet.tweakIds.filter((id) => id !== blocked)),
      tweak(blocked, { compatibility: "unsupportedEdition" }),
    ];
    const progress = profileProgress(quiet, statuses);
    expect(progress.tweaks).toHaveLength(quiet.tweakIds.length);
    expect(progress.usable).toHaveLength(quiet.tweakIds.length - 1);
    expect(progress.active).toBe(true);
  });

  it("names the smaller profile when the larger one adds nothing on this PC", () => {
    const extra = everything.tweakIds.filter((id) => !quiet.tweakIds.includes(id));
    const statuses = [
      ...on(quiet.tweakIds),
      ...extra.map((id) => tweak(id, { compatibility: "unsupportedEdition" })),
    ];
    expect(activeProfile(statuses)?.id).toBe("quiet");
  });

  it("never activates a profile with nothing usable", () => {
    const statuses = responsive.tweakIds.map((id) =>
      tweak(id, { compatibility: "unsupportedEdition" }),
    );
    expect(profileProgress(responsive, statuses).active).toBe(false);
  });
});
