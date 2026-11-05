import type { DeviceReport, ExternalLink, GpuInfo, SystemSnapshot } from "@/types/system";

/** The GPU that runs games: the one with the most dedicated memory. */
export function primaryGpu(snapshot: SystemSnapshot | undefined): GpuInfo | undefined {
  return snapshot?.gpus.toSorted(
    (a, b) => (b.dedicatedMemoryBytes ?? 0) - (a.dedicatedMemoryBytes ?? 0),
  )[0];
}

export function driverOf(gpu: GpuInfo | undefined, devices: DeviceReport | undefined) {
  return devices?.gpus.find((d) => d.name === gpu?.model);
}

export function driverLink(gpu: GpuInfo | undefined): ExternalLink | null {
  const name = `${gpu?.vendor ?? ""} ${gpu?.model ?? ""}`.toLowerCase();
  if (name.includes("nvidia")) return "nvidiaDrivers";
  if (name.includes("amd") || name.includes("radeon")) return "amdDrivers";
  if (name.includes("intel")) return "intelDrivers";
  return null;
}
