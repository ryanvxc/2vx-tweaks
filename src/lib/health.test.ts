import { describe, expect, it } from "vitest";
import { preview } from "@/services/preview";
import type { SystemSnapshot } from "@/types/system";
import { evaluateHealth, summariseHealth } from "./health";

const GiB = 1024 ** 3;

function snapshotWith(overrides: Partial<SystemSnapshot>): SystemSnapshot {
  return { ...preview.snapshot(), ...overrides };
}

describe("evaluateHealth", () => {
  it("flags a nearly full system drive", () => {
    const snap = snapshotWith({
      disks: [
        {
          name: "",
          mountPoint: "C:\\",
          fileSystem: "NTFS",
          kind: "ssd",
          totalBytes: 100 * GiB,
          availableBytes: 8 * GiB,
          isRemovable: false,
        },
      ],
    });
    const check = evaluateHealth(snap).find((c) => c.id === "system-drive-space");
    expect(check?.status).toBe("warning");
  });

  it("marks Windows 10 as needing attention", () => {
    const snap = snapshotWith({
      os: { ...preview.snapshot().os, build: 19045, isWindows11: false },
    });
    expect(evaluateHealth(snap).find((c) => c.id === "os-support")?.status).toBe("attention");
  });

  it("evaluates memory only when a sample is available", () => {
    const snap = preview.snapshot();
    expect(evaluateHealth(snap).some((c) => c.id === "memory-pressure")).toBe(false);
    const sample = preview.metricsSince().at(-1)!;
    const high = { ...sample, memoryUsedBytes: sample.memoryTotalBytes * 0.95 };
    expect(evaluateHealth(snap, high).find((c) => c.id === "memory-pressure")?.status).toBe(
      "warning",
    );
  });
});

describe("summariseHealth", () => {
  it("reports the worst status", () => {
    expect(
      summariseHealth([
        { id: "a", label: "", status: "good", detail: "" },
        { id: "b", label: "", status: "attention", detail: "" },
      ]),
    ).toMatchObject({ status: "attention", attention: 1, warnings: 0 });
  });
});
