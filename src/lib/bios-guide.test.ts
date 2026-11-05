import { describe, expect, it } from "vitest";
import { preview } from "@/services/preview";
import type { DeviceReport, FirmwareReport, HardwareDetails } from "@/types/system";
import { BIOS_GUIDE, biosItems, type BiosFacts } from "./bios-guide";

const item = (id: string) => {
  const found = BIOS_GUIDE.find((i) => i.id === id);
  if (!found) throw new Error(id);
  return found;
};

const firmware = (patch: Partial<FirmwareReport>): BiosFacts => ({
  firmware: { ...preview.firmwareReport(), ...patch },
});

const memory = (speedMts: number, ratedSpeedMts: number): BiosFacts => {
  const details = preview.hardwareDetails();
  const [first] = details.memory.modules;
  if (!first) throw new Error("the preview has memory");
  const module = { ...first, speedMts, ratedSpeedMts };
  const hardware: HardwareDetails = {
    ...details,
    memory: { ...details.memory, modules: [module] },
  };
  return { hardware };
};

describe("the BIOS guide", () => {
  it("never tells anyone to turn a protection off", () => {
    const words = BIOS_GUIDE.map((i) => `${i.title} ${i.summary} ${i.detail}`).join(" ");
    expect(words).not.toMatch(/disable secure boot|turn off secure boot|disable memory integrity/i);
    expect(words).not.toMatch(/\bfps\b/i);
    expect(item("secure-boot").summary).toMatch(/^Leave on/);
    expect(item("virtualization").detail).toMatch(/^Leave it on/);
  });

  it("gives every item its own id", () => {
    expect(new Set(BIOS_GUIDE.map((i) => i.id)).size).toBe(BIOS_GUIDE.length);
  });

  it("reads boot mode and TPM from the firmware report", () => {
    expect(item("uefi").state(firmware({ bootMode: "legacy" }))?.label).toBe("Legacy");
    expect(item("uefi").state({})).toBeNull();
    expect(item("tpm").state(firmware({ tpmVersion: 2 }))?.label).toBe("TPM 2.0");
    expect(item("tpm").state(firmware({ tpmVersion: null }))?.label).toBe("Not found");
  });

  it("says nothing about Resizable BAR when the window couldn't be read", () => {
    expect(item("resizable-bar").state(firmware({ resizableBar: null }))).toBeNull();
    expect(item("resizable-bar").state(firmware({ resizableBar: false }))?.label).toBe("Off");
  });

  it("flags memory below its rated speed", () => {
    expect(item("memory-profile").state(memory(4800, 6000))).toEqual({
      tone: "warning",
      label: "4800 of 6000 MT/s",
    });
    expect(item("memory-profile").state(memory(6000, 6000))?.tone).toBe("success");
  });

  it("reads Secure Boot from the same place as the Security tab", () => {
    const devices = (state: "on" | "off" | "unknown"): DeviceReport => ({
      ...preview.deviceReport(),
      protections: [{ id: "secureBoot", state }],
    });
    expect(item("secure-boot").state({ devices: devices("on") })?.tone).toBe("success");
    expect(item("secure-boot").state({ devices: devices("unknown") })).toBeNull();
  });

  it.each([
    ["13th Gen Intel(R) Core(TM) i7-13700K", true],
    ["Intel(R) Core(TM) i9-14900KS", true],
    ["13th Gen Intel(R) Core(TM) i7-13700H", false],
    ["Intel(R) Core(TM) Ultra 7 265K", false],
    ["Intel(R) Core(TM) i7-12700K", false],
  ])("shows Intel's fix only to the CPUs it covers: %s", (brand, shown) => {
    const base = preview.snapshot();
    const facts: BiosFacts = {
      snapshot: { ...base, cpu: { ...base.cpu, brand, vendor: "GenuineIntel" } },
    };
    expect(biosItems("cpu", facts).some((i) => i.id === "intel-microcode")).toBe(shown);
  });
});
