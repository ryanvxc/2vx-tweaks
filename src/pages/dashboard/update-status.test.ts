import { describe, expect, it } from "vitest";
import type { UpdaterState } from "@/hooks/use-updater";
import { updateStatus } from "./update-status";

const info = { version: "0.4.0" } as never;

describe("updateStatus", () => {
  it("names the version that is waiting", () => {
    expect(updateStatus({ kind: "available", info })).toBe("2VX 0.4.0 is available.");
    expect(updateStatus({ kind: "ready", info })).toBe("2VX 0.4.0 is ready to install.");
  });

  it("says plainly when there is nothing to do or nothing to check", () => {
    expect(updateStatus({ kind: "upToDate", checkedAt: 1 })).toBe("2VX is up to date.");
    expect(updateStatus({ kind: "unsupported", reason: "dev" })).toBe(
      "Updates aren't available in this build.",
    );
  });

  it("has a line for the other states", () => {
    for (const kind of ["loading", "idle", "checking"] as const) {
      expect(updateStatus({ kind } as UpdaterState)).toBe("Checking for updates.");
    }
    expect(updateStatus({ kind: "error", phase: "check", message: "x" })).toBe(
      "Couldn't check for updates.",
    );
  });
});
