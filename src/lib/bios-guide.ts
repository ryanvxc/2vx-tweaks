import {
  Binary,
  BatteryCharging,
  Cable,
  CircuitBoard,
  Cpu,
  Gauge,
  Gpu,
  Grid2x2,
  KeyRound,
  Layers,
  MemoryStick,
  MonitorCheck,
  RotateCcw,
  ShieldAlert,
  type LucideIcon,
} from "lucide-react";
import type { BiosTab } from "@/app/navigation";
import type { Tone } from "@/lib/engine-text";
import { primaryGpu } from "@/lib/devices";
import type {
  DeviceReport,
  ExternalLink,
  FirmwareReport,
  HardwareDetails,
  SystemSnapshot,
} from "@/types/system";

export interface BiosFacts {
  firmware?: FirmwareReport;
  devices?: DeviceReport;
  hardware?: HardwareDetails;
  snapshot?: SystemSnapshot;
}

export interface BiosState {
  tone: Tone;
  label: string;
}

/** One thing to check. It only ever says what to look at: nothing in 2VX writes firmware settings. */
export interface BiosItem {
  id: string;
  tab: BiosTab;
  icon: LucideIcon;
  title: string;
  summary: string;
  /** What it is, and what to do about it. */
  detail: string;
  /** Names differ by board, so this says what to look for rather than a path. */
  lookFor: string | null;
  link?: (facts: BiosFacts) => ExternalLink | null;
  /** What Windows can see of it, when it can see anything. */
  state: (facts: BiosFacts) => BiosState | null;
  /** Left out on hardware it doesn't apply to. */
  applies?: (facts: BiosFacts) => boolean;
}

const nothing = () => null;

const isIntel = (f: BiosFacts) => /intel/i.test(f.snapshot?.cpu.vendor ?? "");

/** Intel's fix covers these desktop parts. Laptop chips end in other letters, such as H or U. */
const RAPTOR_LAKE_DESKTOP = /\bi[3579]-1[34]\d{3}(k|kf|ks|f|t|tf|s)?\b/i;

const yesNo = (value: boolean | null | undefined, on: string, off: string): BiosState | null =>
  value == null ? null : value ? { tone: "success", label: on } : { tone: "warning", label: off };

function memorySpeed({ hardware }: BiosFacts): BiosState | null {
  const modules = hardware?.memory.modules.filter((m) => m.speedMts && m.ratedSpeedMts) ?? [];
  if (modules.length === 0) return null;
  const running = Math.min(...modules.map((m) => m.speedMts ?? 0));
  const rated = Math.min(...modules.map((m) => m.ratedSpeedMts ?? 0));
  return running < rated
    ? { tone: "warning", label: `${running} of ${rated} MT/s` }
    : { tone: "success", label: `${running} MT/s` };
}

function memorySticks({ hardware }: BiosFacts): BiosState | null {
  const sticks = hardware?.memory.slotsUsed;
  if (!sticks) return null;
  return sticks === 1
    ? { tone: "warning", label: "1 stick" }
    : { tone: "neutral", label: `${sticks} sticks` };
}

function threads({ snapshot }: BiosFacts): BiosState | null {
  const cores = snapshot?.cpu.physicalCores;
  if (!snapshot || !cores) return null;
  return { tone: "neutral", label: `${cores} cores, ${snapshot.cpu.logicalCores} threads` };
}

export const BIOS_GUIDE: BiosItem[] = [
  {
    id: "version",
    tab: "general",
    icon: CircuitBoard,
    title: "BIOS version",
    summary: "See what's installed, then compare it with your board maker's support page.",
    detail:
      "Board makers publish updates that add CPU support and fix bugs. Update if a release fixes something you need, and use only the file for your exact board. Keep the PC on steady power while it flashes, because a failed update can stop it booting.",
    lookFor: "Your board's model on its maker's support page",
    state: ({ firmware }) =>
      firmware?.version ? { tone: "neutral", label: firmware.version } : null,
  },
  {
    id: "uefi",
    tab: "general",
    icon: MonitorCheck,
    title: "UEFI boot",
    summary: "Windows 11 needs UEFI. Resizable BAR and Secure Boot do too.",
    detail:
      "With CSM (the old BIOS mode) off, the PC boots through UEFI. A PC installed in Legacy mode won't boot after switching until its disk is converted to GPT, so don't change this on its own.",
    lookFor: "CSM, or Boot Mode",
    state: ({ firmware }) =>
      firmware?.bootMode === "uefi"
        ? { tone: "success", label: "UEFI" }
        : firmware?.bootMode === "legacy"
          ? { tone: "warning", label: "Legacy" }
          : null,
  },
  {
    id: "secure-boot",
    tab: "general",
    icon: KeyRound,
    title: "Secure Boot",
    summary: "Leave on. Only trusted software can load while the PC starts.",
    detail:
      "Secure Boot blocks unsigned boot software, which is where the hardest malware to remove lives. Some games' anti-cheat requires it. Nothing in 2VX needs it off, and no tweak here will ever ask you to.",
    lookFor: "Secure Boot, usually under Boot or Security",
    link: () => "secureBootDocs",
    state: ({ devices }) => {
      const state = devices?.protections.find((p) => p.id === "secureBoot")?.state;
      return state === "on"
        ? { tone: "success", label: "On" }
        : state === "off"
          ? { tone: "warning", label: "Off" }
          : null;
    },
  },
  {
    id: "tpm",
    tab: "general",
    icon: Binary,
    title: "TPM",
    summary: "Windows 11 needs TPM 2.0. It's usually built into the CPU.",
    detail:
      "The TPM stores keys for BitLocker, Windows Hello and some anti-cheat. Most boards have it inside the CPU, under the name PTT on Intel and fTPM on AMD. If Windows finds none, check it isn't switched off.",
    lookFor: "PTT on Intel, fTPM on AMD",
    link: () => "tpmDocs",
    state: ({ firmware }) => {
      if (!firmware) return null;
      if (firmware.tpmVersion === null) return { tone: "warning", label: "Not found" };
      return {
        tone: firmware.tpmVersion >= 2 ? "success" : "warning",
        label: firmware.tpmVersion >= 2 ? "TPM 2.0" : "TPM 1.2",
      };
    },
  },
  {
    id: "reset",
    tab: "general",
    icon: RotateCcw,
    title: "How to reset the BIOS",
    summary: "The way back when a setting stops the PC starting.",
    detail:
      "Choose Load Optimized Defaults on the BIOS's exit screen. If the PC won't reach the BIOS, use the Clear CMOS button or jumper your board's manual describes, with the power off. Both lose any overclock and may turn XMP off.",
    lookFor: "Load Optimized Defaults, or Clear CMOS in the manual",
    state: nothing,
  },
  {
    id: "virtualization",
    tab: "cpu",
    icon: Layers,
    title: "CPU virtualisation",
    summary: "Needed by Memory integrity, WSL, Hyper-V and Android emulators.",
    detail:
      "Leave it on. Windows' Memory integrity protection depends on it, and turning it off doesn't help games on a modern CPU.",
    lookFor: "Intel VT-x, or AMD SVM",
    link: () => "memoryIntegrityDocs",
    state: ({ firmware }) => yesNo(firmware?.virtualization, "On", "Off"),
  },
  {
    id: "smt",
    tab: "cpu",
    icon: Cpu,
    title: "Hyper-Threading and SMT",
    summary: "Leave on, unless you're testing one game and can measure the difference.",
    detail:
      "Each core runs two threads where the CPU supports it. Some recent Intel CPUs don't have it, so matching core and thread counts aren't always a sign it's off.",
    lookFor: "Hyper-Threading on Intel, SMT on AMD",
    state: threads,
  },
  {
    id: "boost",
    tab: "cpu",
    icon: Gauge,
    title: "Turbo and Precision Boost",
    summary: "Leave on Auto. This is how a CPU reaches its advertised speed.",
    detail:
      "Boost needs no tuning to work. Manual limits and overclocks can make a PC unstable in ways that only show up weeks later, and may not be covered by your warranty.",
    lookFor: "Turbo Boost on Intel, Precision Boost on AMD",
    state: nothing,
  },
  {
    id: "intel-microcode",
    tab: "cpu",
    icon: ShieldAlert,
    title: "Intel 13th and 14th Gen fix",
    summary: "Install the latest BIOS. It carries Intel's fix for these CPUs.",
    detail:
      "Intel traced crashes on these desktop CPUs to voltage requests that age part of the chip, and fixed them through microcode delivered in BIOS updates. Those updates prevent the damage but can't reverse damage already done, so install one even if the PC seems fine.",
    lookFor: "A BIOS release that mentions microcode 0x12B or later",
    link: () => "intelInstabilityDocs",
    state: nothing,
    applies: (f) => isIntel(f) && RAPTOR_LAKE_DESKTOP.test(f.snapshot?.cpu.brand ?? ""),
  },
  {
    id: "memory-profile",
    tab: "memory",
    icon: MemoryStick,
    title: "XMP or EXPO",
    summary: "Memory starts below its rated speed until you turn its profile on.",
    detail:
      "The speed printed on a memory kit needs its profile switched on in the BIOS. This runs the memory beyond the CPU's standard speed, so if the PC turns unstable, go back to the slower profile or turn it off.",
    lookFor: "XMP on Intel, EXPO or DOCP on AMD",
    link: (f) => (isIntel(f) ? "xmpDocs" : "expoDocs"),
    state: memorySpeed,
  },
  {
    id: "memory-slots",
    tab: "memory",
    icon: Grid2x2,
    title: "Memory slots",
    summary: "Two sticks run in dual channel, in the slots your manual names.",
    detail:
      "Boards pair their slots, and often the pair isn't the first two. A single stick runs in single channel, which halves the bandwidth available to the CPU and to integrated graphics.",
    lookFor: "The memory table in your board's manual",
    state: memorySticks,
  },
  {
    id: "above-4g",
    tab: "graphics",
    icon: Cable,
    title: "Above 4G decoding",
    summary: "Lets the PC reach graphics memory above 4 GB. Resizable BAR needs it.",
    detail: "Turn this on along with Resizable BAR. It has no effect of its own on most PCs.",
    lookFor: "Above 4G Decoding",
    state: nothing,
  },
  {
    id: "resizable-bar",
    tab: "graphics",
    icon: Gpu,
    title: "Resizable BAR",
    summary: "Lets the CPU reach all of the graphics memory at once, not 256 MB at a time.",
    detail:
      "It needs a card that supports it, UEFI boot and Above 4G decoding. 2VX reads the size of the window Windows gave the card, so this shows what's really in effect, not just the BIOS setting. AMD calls it Smart Access Memory.",
    lookFor: "Re-Size BAR, or Smart Access Memory",
    link: ({ snapshot }) => {
      const name = `${primaryGpu(snapshot)?.vendor ?? ""}`.toLowerCase();
      return name.includes("nvidia")
        ? "resizableBarNvidia"
        : name.includes("amd")
          ? "resizableBarAmd"
          : null;
    },
    state: ({ firmware }) => yesNo(firmware?.resizableBar, "On", "Off"),
  },
  {
    id: "c-states",
    tab: "power",
    icon: BatteryCharging,
    title: "CPU C-states",
    summary: "Leave on Auto. They save power when the CPU is idle.",
    detail:
      "Turning these off keeps the CPU awake, which costs power and heat all day. Only try it to chase a measured latency problem, and put it back if nothing changes.",
    lookFor: "C-States, or Global C-state Control",
    state: nothing,
  },
  {
    id: "aspm",
    tab: "power",
    icon: Cable,
    title: "PCIe power saving",
    summary: "Leave on Auto here. Windows has its own switch, on Tweaks > Power.",
    detail:
      "ASPM lets PCIe links sleep when idle. The BIOS setting decides what's allowed, and Windows' power plan decides what's used, so 2VX's PCIe link power saving switch is the simpler one to try and undo.",
    lookFor: "ASPM, or PCIe power saving",
    state: nothing,
  },
];

/** The guide for this PC, one tab at a time. */
export const biosItems = (tab: BiosTab, facts: BiosFacts) =>
  BIOS_GUIDE.filter((item) => item.tab === tab && (item.applies?.(facts) ?? true));
