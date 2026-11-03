import { describe, expect, it } from "vitest";
import { STATUSES } from "@/test/engine-fixtures";
import { preview } from "@/services/preview";
import { buildSystemReport } from "./system-report";

describe("buildSystemReport", () => {
  it("lists Windows, hardware and tweak states", () => {
    const report = buildSystemReport(
      preview.appInfo(),
      preview.snapshot(),
      STATUSES,
      new Date(Date.UTC(2026, 10, 3)),
    );
    expect(report).toContain("Build 26100.2605");
    expect(report).toContain("CPU: Preview CPU (8 cores)");
    expect(report).toContain(String.raw`Drive C:\ SSD NTFS`);
    expect(report).toContain("[x] privacy.applied-tweak (applied)");
    expect(report).toContain("[ ] privacy.user-tweak (notApplied)");
  });
});
