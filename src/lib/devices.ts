import type {
  DeviceEntry,
  DeviceReport,
  ExternalLink,
  GpuInfo,
  SystemSnapshot,
} from "@/types/system";

/** The GPU that runs games: the one with the most dedicated memory. */
export function primaryGpu(snapshot: SystemSnapshot | undefined): GpuInfo | undefined {
  return snapshot?.gpus.toSorted(
    (a, b) => (b.dedicatedMemoryBytes ?? 0) - (a.dedicatedMemoryBytes ?? 0),
  )[0];
}

export function driverOf(gpu: GpuInfo | undefined, devices: DeviceReport | undefined) {
  return devices?.gpus.find((d) => d.name === gpu?.model);
}

export type DeviceLine = DeviceEntry & { count: number };

/** Windows lists one entry per interface, so a single mouse can show up three times. */
export function collapseDevices(devices: DeviceEntry[]): DeviceLine[] {
  const lines = new Map<string, DeviceLine>();
  for (const device of devices) {
    const key = `${device.class}:${device.name}`;
    const line = lines.get(key);
    if (line) {
      line.count += 1;
      line.problem ||= device.problem;
    } else {
      lines.set(key, { ...device, count: 1 });
    }
  }
  return [...lines.values()];
}

export function deviceName(line: DeviceLine) {
  return line.count > 1 ? `${line.name} ×${line.count}` : line.name;
}

export function driverLink(gpu: GpuInfo | undefined): ExternalLink | null {
  const name = `${gpu?.vendor ?? ""} ${gpu?.model ?? ""}`.toLowerCase();
  if (name.includes("nvidia")) return "nvidiaDrivers";
  if (name.includes("amd") || name.includes("radeon")) return "amdDrivers";
  if (name.includes("intel")) return "intelDrivers";
  return null;
}
