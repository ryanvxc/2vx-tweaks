import { describe, expect, it } from "vitest";
import { OFFERED_SOURCE } from "@/test/catalogue";
import type { TweakMeta } from "@/types/engine";
import { KNOWN_FIRST, mostUsefulFirst } from "./tweak-order";

const LISTED = Object.values(KNOWN_FIRST).flat();

const meta = (id: string, patch: Partial<TweakMeta> = {}) =>
  ({
    id,
    name: id,
    risk: "low",
    impact: "low",
    evidence: "documented",
    recommended: false,
    ...patch,
  }) as TweakMeta;

const sorted = (...tweaks: TweakMeta[]) => tweaks.toSorted(mostUsefulFirst).map((t) => t.id);

describe("the order of a tab", () => {
  it("lists only real tweaks, each once", () => {
    // A macro builds adapter IDs, so the source only holds their endings.
    const written = (id: string) => `"${id.replace("network.adapter-", "")}"`;
    expect(LISTED.filter((id) => !OFFERED_SOURCE.includes(written(id)))).toEqual([]);
    expect(new Set(LISTED).size).toBe(LISTED.length);
  });

  it("opens each page on something people have heard of", () => {
    expect(KNOWN_FIRST.core?.[0]).toBe("windows.startup-delay");
    expect(KNOWN_FIRST.core?.indexOf("power.hibernate")).toBeLessThan(
      KNOWN_FIRST.core?.indexOf("power.button") ?? 0,
    );
    expect(KNOWN_FIRST.gaming?.[0]).toBe("gaming.game-dvr-off");
    expect(KNOWN_FIRST.privacy?.slice(0, 3)).toEqual([
      "privacy.diagnostic-data-minimum",
      "privacy.copilot-policy",
      "privacy.recall",
    ]);
  });

  it("keeps a niche setting below the known ones, whatever it's called", () => {
    expect(sorted(meta("windows.biometric-antispoof"), meta("windows.startup-delay"))).toEqual([
      "windows.startup-delay",
      "windows.biometric-antispoof",
    ]);
  });

  it("puts a listed tweak ahead of one that isn't", () => {
    expect(sorted(meta("windows.new"), meta("cpu.wpbt"))).toEqual(["cpu.wpbt", "windows.new"]);
  });

  it("leaves whatever can bite until last among the rest", () => {
    const safe = meta("a.safe", { name: "Zed" });
    const risky = meta("a.risky", { name: "Able", risk: "moderate", impact: "high" });
    expect(sorted(risky, safe)).toEqual(["a.safe", "a.risky"]);
  });

  it("orders the rest by what they do, then by name", () => {
    expect(
      sorted(
        meta("a.b", { name: "B" }),
        meta("a.a", { name: "A" }),
        meta("a.community", { name: "0", evidence: "community" }),
        meta("a.impact", { name: "Z", impact: "medium" }),
        meta("a.recommended", { name: "Y", recommended: true }),
      ),
    ).toEqual(["a.recommended", "a.impact", "a.a", "a.b", "a.community"]);
  });
});
