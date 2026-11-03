import { describe, expect, it } from "vitest";
import { outcome } from "@/test/engine-fixtures";
import { canRevert, describeError, describeOutcome } from "./engine-text";

describe("engine wording", () => {
  it("never calls a partial result a success", () => {
    expect(describeOutcome(outcome(), "apply").tone).toBe("success");
    expect(describeOutcome(outcome({ status: "rolledBack" }), "apply").tone).toBe("warning");
    expect(describeOutcome(outcome({ status: "failed" }), "apply").tone).toBe("destructive");
    expect(describeOutcome(outcome({ status: "failed" }), "revert").tone).toBe("warning");
    expect(describeOutcome(outcome({ snapshotId: null, changes: [] }), "apply").title).toBe(
      "Nothing to change",
    );
  });

  it("adds the restart requirement only when the batch completed", () => {
    expect(describeOutcome(outcome({ restart: "signOut" }), "apply").description).toMatch(
      /Sign out/,
    );
  });

  it("explains each refusal", () => {
    expect(describeError({ code: "preview", message: "" }).title).toBe(
      "Not available in browser preview",
    );
    expect(describeError({ code: "conflict", message: "x" }).description).toMatch(
      /Nothing was applied/,
    );
    expect(describeError({ code: "elevation_required", message: "x" }).description).toMatch(
      /Nothing was changed/,
    );
  });

  it("allows undo only for apply snapshots with changes possibly on the system", () => {
    expect(canRevert("apply", "completed")).toBe(true);
    expect(canRevert("apply", "interrupted")).toBe(true);
    expect(canRevert("apply", "failed")).toBe(true);
    expect(canRevert("apply", "rolledBack")).toBe(false);
    expect(canRevert("apply", "reverted")).toBe(false);
    expect(canRevert("revert", "completed")).toBe(false);
  });
});
