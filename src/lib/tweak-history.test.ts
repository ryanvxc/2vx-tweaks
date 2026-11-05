import { describe, expect, it } from "vitest";
import type { SnapshotRecord } from "@/types/engine";
import { tweakHistory } from "./tweak-history";

const snapshot = (overrides: Partial<SnapshotRecord>): SnapshotRecord => ({
  id: 1,
  kind: "apply",
  tweakIds: ["a"],
  reverts: null,
  status: "completed",
  error: null,
  appVersion: "0.4.0",
  createdAtMs: 2_000,
  finishedAtMs: 2_100,
  changeCount: 1,
  ...overrides,
});

describe("tweakHistory", () => {
  it("takes the newest snapshot for a tweak", () => {
    const history = tweakHistory(
      [
        snapshot({ createdAtMs: 3_000 }),
        snapshot({ createdAtMs: 1_000, status: "failed", error: "denied" }),
      ],
      null,
    );
    expect(history.get("a")).toMatchObject({ lastUsedMs: 3_000, failed: null });
  });

  it("reports a failed or rolled-back change", () => {
    expect(
      tweakHistory([snapshot({ status: "rolledBack", error: "policy" })], null).get("a")?.failed,
    ).toBe("policy");
    expect(tweakHistory([snapshot({ status: "failed" })], null).get("a")?.failed).toBeTruthy();
  });

  it("knows a change made since this boot from one before it", () => {
    expect(tweakHistory([snapshot({})], 2, 3_000).get("a")?.sinceBoot).toBe(true);
    expect(tweakHistory([snapshot({})], 0, 3_000).get("a")?.sinceBoot).toBe(false);
    expect(tweakHistory([snapshot({ kind: "revert" })], 2, 3_000).get("a")?.sinceBoot).toBe(false);
    expect(tweakHistory([snapshot({})], null).get("a")?.sinceBoot).toBe(false);
  });
});
