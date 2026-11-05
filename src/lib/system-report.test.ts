import { describe, expect, it } from "vitest";
import { STATUSES } from "@/test/engine-fixtures";
import { preview } from "@/services/preview";
import { buildSystemReport } from "./system-report";

describe("buildSystemReport", () => {
  it("lists Windows, hardware, health and the tweaks in effect", () => {
    const report = buildSystemReport(
      preview.appInfo(),
      preview.snapshot(),
      STATUSES,
      new Date(Date.UTC(2026, 10, 3)),
    );
    expect(report).toContain("Build 26100.2605");
    expect(report).toContain("CPU: Preview CPU (8 cores)");
    expect(report).toContain(String.raw`Drive C:\ SSD NTFS`);
    expect(report).toContain("[Health]");
    expect(report).toContain("[Tweaks applied: 1]");
    expect(report).toContain("privacy.applied-tweak");
    // Tweaks that aren't in effect stay out of the short report.
    expect(report).not.toContain("privacy.user-tweak");
  });

  it("adds every tweak with its state to the full list", () => {
    const report = buildSystemReport(
      preview.appInfo(),
      preview.snapshot(),
      STATUSES,
      new Date(),
      true,
    );
    expect(report).toContain("[x] privacy.applied-tweak (applied)");
    expect(report).toContain("[ ] privacy.user-tweak (notApplied)");
  });
});
