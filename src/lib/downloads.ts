import {
  Activity,
  AudioWaveform,
  Eraser,
  Flame,
  Gpu,
  HardDrive,
  LineChart,
  ListTree,
  Rocket,
  Settings2,
  ShieldCheck,
  Thermometer,
  type LucideIcon,
} from "lucide-react";
import { driverLink, primaryGpu } from "@/lib/devices";
import type { ExternalLink, SystemSnapshot } from "@/types/system";

export interface Download {
  id: string;
  name: string;
  description: string;
  icon: LucideIcon;
  link: ExternalLink;
}

export interface DownloadGroup {
  title: string;
  items: Download[];
}

const has = (text: string | null | undefined, word: string) =>
  (text ?? "").toLowerCase().includes(word);

/** Drivers come from whoever made the part, so what's listed depends on this PC. */
function driverDownloads(snapshot: SystemSnapshot | undefined): Download[] {
  const gpu = primaryGpu(snapshot);
  const graphics = driverLink(gpu);
  const vendor = snapshot?.cpu.vendor;
  const chipset: ExternalLink | null = has(vendor, "intel")
    ? "intelDrivers"
    : has(vendor, "amd")
      ? "amdDrivers"
      : null;
  return [
    ...(graphics
      ? [
          {
            id: "gpu-driver",
            name: "Graphics driver",
            description: `The latest driver for ${gpu?.model ?? "your graphics card"}, from its maker.`,
            icon: Gpu,
            link: graphics,
          },
        ]
      : []),
    ...(chipset
      ? [
          {
            id: "chipset-driver",
            name: "Chipset driver",
            description: `The driver for your board's chipset, from ${has(vendor, "intel") ? "Intel" : "AMD"}.`,
            icon: Settings2,
            link: chipset,
          },
        ]
      : []),
    {
      id: "ddu",
      name: "Display Driver Uninstaller",
      description: "Fully removes a graphics driver, before a clean install.",
      icon: Eraser,
      link: "displayDriverUninstaller",
    },
    ...(driverLink(gpu) === "nvidiaDrivers"
      ? [
          {
            id: "nvcleanstall",
            name: "NVCleanstall",
            description: "Installs an NVIDIA driver with only the parts you choose.",
            icon: Rocket,
            link: "nvCleanstall" as const,
          },
          {
            id: "nvidia-profile-inspector",
            name: "NVIDIA Profile Inspector",
            description: "Reads and edits the driver's per-game settings.",
            icon: ListTree,
            link: "nvidiaProfileInspector" as const,
          },
        ]
      : []),
  ];
}

const MEASURE: Download[] = [
  {
    id: "hwinfo",
    name: "HWiNFO",
    description: "Every sensor in the PC, with temperatures, power and clock speeds.",
    icon: Thermometer,
    link: "hwinfo",
  },
  {
    id: "capframex",
    name: "CapFrameX",
    description: "Records frame times, so a change can be measured rather than guessed.",
    icon: LineChart,
    link: "capFrameX",
  },
  {
    id: "latencymon",
    name: "LatencyMon",
    description: "Finds the drivers behind audio dropouts and stutter.",
    icon: AudioWaveform,
    link: "latencyMon",
  },
  {
    id: "occt",
    name: "OCCT",
    description: "Stress tests for the CPU, memory and graphics card.",
    icon: Flame,
    link: "occt",
  },
  {
    id: "crystaldiskinfo",
    name: "CrystalDiskInfo",
    description: "Reads a drive's own health data and temperature.",
    icon: HardDrive,
    link: "crystalDiskInfo",
  },
];

const MICROSOFT: Download[] = [
  {
    id: "autoruns",
    name: "Autoruns",
    description: "Microsoft's full list of what starts with Windows.",
    icon: ShieldCheck,
    link: "autoruns",
  },
  {
    id: "process-explorer",
    name: "Process Explorer",
    description: "Microsoft's detailed view of everything that's running.",
    icon: Activity,
    link: "processExplorer",
  },
];

export const downloadGroups = (snapshot: SystemSnapshot | undefined): DownloadGroup[] => [
  { title: "Drivers", items: driverDownloads(snapshot) },
  { title: "Measure and test", items: MEASURE },
  { title: "From Microsoft", items: MICROSOFT },
];
