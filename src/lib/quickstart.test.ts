import { describe, expect, it } from "vitest";
import { STATUSES, USER_TWEAK } from "@/test/engine-fixtures";
import type { SnapshotRecord, TweakStatus } from "@/types/engine";
import {
  openRecommended,
  QUICKSTART_STEPS,
  savedBackups,
  type QuickstartFacts,
} from "./quickstart";

const FACTS: QuickstartFacts = {
  applied: false,
  recommendedLeft: 2,
  gameModeSet: false,
  opened: [],
};

const done = (facts: Partial<QuickstartFacts>) =>
  QUICKSTART_STEPS.filter((s) => s.done({ ...FACTS, ...facts })).map((s) => s.id);

const status = (patch: Partial<TweakStatus>, recommended = true): TweakStatus => ({
  ...STATUSES[0]!,
  meta: { ...USER_TWEAK, recommended },
  ...patch,
});

const snapshot = (patch: Partial<SnapshotRecord>) =>
  ({ id: 1, kind: "apply", status: "completed", ...patch }) as SnapshotRecord;

describe("Quickstart", () => {
  it("starts with nothing ticked", () => {
    expect(done({})).toEqual([]);
  });

  it("reads what it can from the PC", () => {
    expect(done({ applied: true })).toEqual(["tweak"]);
    expect(done({ recommendedLeft: 0 })).toEqual(["recommended"]);
    expect(done({ gameModeSet: true })).toEqual(["game-mode"]);
  });

  it("doesn't tick the recommended step before the tweaks are known", () => {
    expect(done({ recommendedLeft: null })).toEqual([]);
  });

  it("ticks the rest once they've been opened", () => {
    expect(done({ opened: ["startup", "backups"] })).toEqual(["startup", "backups"]);
  });

  it("ticks Resources once it has been opened", () => {
    expect(done({ opened: ["resources"] })).toEqual(["resources"]);
    expect(QUICKSTART_STEPS.find((s) => s.id === "resources")?.to).toBe("/tools");
  });

  it("has a unique id and a page for every step", () => {
    expect(new Set(QUICKSTART_STEPS.map((s) => s.id)).size).toBe(QUICKSTART_STEPS.length);
    expect(QUICKSTART_STEPS.every((s) => s.to.startsWith("/"))).toBe(true);
  });
});

describe("openRecommended", () => {
  it("leaves out what's applied, blocked or not recommended", () => {
    const open = status({ state: "notApplied", compatibility: "compatible" });
    const statuses = [
      open,
      status({ state: "applied", compatibility: "compatible" }),
      status({ state: "notApplied", compatibility: "unsupportedEdition" }),
      status({ state: "notApplied", compatibility: "compatible" }, false),
    ];
    expect(openRecommended(statuses)).toEqual([open]);
  });
});

describe("savedBackups", () => {
  it("counts only batches that can still be restored", () => {
    const kept = snapshot({ id: 1 });
    expect(
      savedBackups([
        kept,
        snapshot({ id: 2, status: "reverted" }),
        snapshot({ id: 3, status: "failed" }),
        snapshot({ id: 4, kind: "revert" }),
      ]),
    ).toEqual([kept]);
  });
});
