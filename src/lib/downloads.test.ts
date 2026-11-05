import { describe, expect, it } from "vitest";
import { preview } from "@/services/preview";
import type { SystemSnapshot } from "@/types/system";
import { downloadGroups } from "./downloads";

const pc = (gpuVendor: string, gpuModel: string, cpuVendor: string): SystemSnapshot => {
  const base = preview.snapshot();
  return {
    ...base,
    cpu: { ...base.cpu, vendor: cpuVendor },
    gpus: [{ vendor: gpuVendor, model: gpuModel, dedicatedMemoryBytes: 8 }],
  };
};

const ids = (snapshot: SystemSnapshot | undefined) =>
  downloadGroups(snapshot).flatMap((g) => g.items.map((i) => i.id));
const linkOf = (snapshot: SystemSnapshot, id: string) =>
  downloadGroups(snapshot)
    .flatMap((g) => g.items)
    .find((i) => i.id === id)?.link;

describe("downloads", () => {
  it("sends each driver to the maker of the part", () => {
    const pcWithBoth = pc("NVIDIA", "RTX", "GenuineIntel");
    expect(linkOf(pcWithBoth, "gpu-driver")).toBe("nvidiaDrivers");
    expect(linkOf(pcWithBoth, "chipset-driver")).toBe("intelDrivers");
    expect(linkOf(pc("AMD", "Radeon", "AuthenticAMD"), "chipset-driver")).toBe("amdDrivers");
  });

  it("keeps NVIDIA's tools to NVIDIA cards", () => {
    expect(ids(pc("NVIDIA", "RTX", "GenuineIntel"))).toContain("nvcleanstall");
    expect(ids(pc("AMD", "Radeon", "AuthenticAMD"))).not.toContain("nvcleanstall");
  });

  it("leaves out drivers it can't place, rather than guessing", () => {
    expect(ids(undefined)).not.toContain("gpu-driver");
    expect(ids(undefined)).not.toContain("chipset-driver");
    expect(ids(undefined)).toContain("hwinfo");
  });
});
