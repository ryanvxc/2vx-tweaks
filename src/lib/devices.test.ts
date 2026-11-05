import { describe, expect, it } from "vitest";
import type { DeviceEntry, DeviceReport, GpuInfo, SystemSnapshot } from "@/types/system";
import { collapseDevices, deviceName, driverLink, driverOf, primaryGpu } from "./devices";

const gpu = (model: string, vendor: string, memory: number | null): GpuInfo => ({
  model,
  vendor,
  dedicatedMemoryBytes: memory,
});

const device = (name: string, problem = false): DeviceEntry => ({
  name,
  class: "mouse",
  problem,
});

describe("collapseDevices", () => {
  it("folds identical names into one line with a count", () => {
    const lines = collapseDevices([device("HID mouse"), device("HID mouse"), device("Pad")]);
    expect(lines.map(deviceName)).toEqual(["HID mouse ×2", "Pad"]);
  });

  it("keeps a problem on any of them", () => {
    const [line] = collapseDevices([device("HID mouse"), device("HID mouse", true)]);
    expect(line?.problem).toBe(true);
  });

  it("keeps the same name apart across device types", () => {
    const lines = collapseDevices([device("Hub"), { ...device("Hub"), class: "usb" }]);
    expect(lines).toHaveLength(2);
  });
});

describe("primaryGpu", () => {
  it("picks the GPU with the most dedicated memory", () => {
    const snapshot = {
      gpus: [gpu("Intel UHD", "Intel", null), gpu("NVIDIA RTX", "NVIDIA", 8e9)],
    } as SystemSnapshot;
    expect(primaryGpu(snapshot)?.model).toBe("NVIDIA RTX");
  });

  it("is undefined before the snapshot arrives", () => {
    expect(primaryGpu(undefined)).toBeUndefined();
  });
});

describe("driverOf", () => {
  it("matches a driver by the GPU's name", () => {
    const devices = {
      gpus: [
        { name: "Other", version: "1", date: null },
        { name: "NVIDIA RTX", version: "617.14", date: null },
      ],
    } as DeviceReport;
    expect(driverOf(gpu("NVIDIA RTX", "NVIDIA", 1), devices)?.version).toBe("617.14");
  });
});

describe("driverLink", () => {
  it.each([
    ["NVIDIA GeForce RTX 3060", "NVIDIA", "nvidiaDrivers"],
    ["Radeon RX 7600", "Advanced Micro Devices", "amdDrivers"],
    ["AMD Radeon Graphics", "", "amdDrivers"],
    ["Intel Arc A750", "Intel", "intelDrivers"],
    ["Mystery GPU", "", null],
  ])("sends %s to its maker", (model, vendor, link) => {
    expect(driverLink(gpu(model, vendor, null))).toBe(link);
  });
});
