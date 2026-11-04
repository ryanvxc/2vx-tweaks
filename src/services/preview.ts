import type { EngineStatus, TweakMeta, TweakStatus } from "@/types/engine";
import type {
  Account,
  AppInfo,
  BackendError,
  CleanupLocation,
  MetricsSample,
  StartupInventory,
  SystemSnapshot,
} from "@/types/system";

/**
 * Simulated backend used when the UI runs outside the desktop shell (browser dev
 * server, tests). Everything here is fake and the UI labels it as "Preview".
 * Never import this from components — go through `services/backend`.
 */

const GiB = 1024 ** 3;
const HISTORY = 300;

function appInfo(): AppInfo {
  return {
    name: "2VX Tweaks",
    version: "0.2.0",
    isElevated: false,
    logDir: "(preview — no log directory)",
    debugBuild: true,
  };
}

let previewAccount: Account | null = { email: "preview@example.com" };

const account = () => previewAccount;

function setAccount(email: string): Promise<Account> {
  if (!/^[^\s@]+@[^\s@.]+(\.[^\s@.]+)+$/.test(email.trim())) {
    const err: BackendError = { code: "invalid_input", message: "not an email address" };
    return Promise.reject(err);
  }
  previewAccount = { email: email.trim() };
  return Promise.resolve(previewAccount);
}

function clearAccount() {
  previewAccount = null;
}

function snapshot(): SystemSnapshot {
  return {
    hostname: "PREVIEW-PC",
    uptimeSecs: 3 * 86_400 + 4 * 3600,
    os: {
      productName: "Windows 11 Pro",
      displayVersion: "24H2",
      build: 26100,
      ubr: 2605,
      editionId: "Professional",
      isWindows11: true,
      architecture: "x86_64",
    },
    cpu: {
      brand: "Preview CPU (8 cores)",
      vendor: "Preview",
      physicalCores: 8,
      logicalCores: 16,
      frequencyMhz: 3600,
    },
    memory: { totalBytes: 32 * GiB, swapTotalBytes: 8 * GiB },
    gpus: [{ vendor: "Preview", model: "Preview GPU", dedicatedMemoryBytes: 12 * GiB }],
    disks: [
      {
        name: "System",
        mountPoint: "C:\\",
        fileSystem: "NTFS",
        kind: "ssd",
        totalBytes: 1000 * GiB,
        availableBytes: 412 * GiB,
        isRemovable: false,
      },
      {
        name: "Games",
        mountPoint: "D:\\",
        fileSystem: "NTFS",
        kind: "hdd",
        totalBytes: 2000 * GiB,
        availableBytes: 160 * GiB,
        isRemovable: false,
      },
    ],
    motherboard: { vendor: "Preview", name: "Preview Board" },
  };
}

// Smooth pseudo-random walk so preview charts look plausible without being mistaken for real data.
function wave(t: number, base: number, amp: number, period: number) {
  return Math.max(
    0,
    Math.min(100, base + amp * Math.sin(t / period) + amp * 0.4 * Math.sin(t / 3.7)),
  );
}

function sampleAt(timestampMs: number): MetricsSample {
  const t = timestampMs / 1000;
  const cores = Array.from({ length: 16 }, (_, i) => wave(t + i * 7, 22, 14, 9 + i));
  return {
    timestampMs,
    cpuPercent: wave(t, 24, 12, 11),
    perCorePercent: cores,
    memoryUsedBytes: (13.5 + Math.sin(t / 40)) * GiB,
    memoryTotalBytes: 32 * GiB,
    gpus: [
      {
        model: "Preview GPU",
        usagePercent: wave(t, 35, 20, 15),
        memoryTotalBytes: 12 * GiB,
      },
    ],
    networkRxBytesPerSec: Math.round(wave(t, 30, 25, 6) * 40_000),
    networkTxBytesPerSec: Math.round(wave(t, 10, 8, 5) * 12_000),
  };
}

function metricsSince(sinceMs?: number): MetricsSample[] {
  const now = Math.floor(Date.now() / 1000) * 1000;
  const start = sinceMs === undefined ? now - (HISTORY - 1) * 1000 : sinceMs + 1000;
  const out: MetricsSample[] = [];
  for (let ts = Math.max(start, now - (HISTORY - 1) * 1000); ts <= now; ts += 1000) {
    out.push(sampleAt(ts));
  }
  return out;
}

// Invented tweaks so the card grid can be laid out in a browser. They are not the real
// catalogue (that lives in Rust) and their names say so. They borrow real IDs only so that
// tabs, sections and gaming profiles have something to place.
type Sample = Pick<TweakMeta, "category" | "permission" | "impact" | "addedIn" | "recommended"> & {
  id: string;
  slug: string;
  title: string;
  state: TweakStatus["state"];
  risk?: TweakMeta["risk"];
  restart?: TweakMeta["restart"];
  warning?: string;
};

const SAMPLES: Sample[] = [
  {
    id: "performance.foreground-boost",
    slug: "machine-wide",
    title: "machine-wide setting",
    category: "performance",
    permission: "administrator",
    impact: "medium",
    addedIn: "0.1",
    recommended: true,
    state: "notApplied",
  },
  {
    id: "privacy.app-launch-tracking-off",
    slug: "per-user",
    title: "per-user setting",
    category: "privacy",
    permission: "user",
    impact: "low",
    addedIn: "0.1",
    recommended: false,
    state: "applied",
    restart: "signOut",
  },
  {
    id: "privacy.advertising-id-off",
    slug: "new-privacy",
    title: "new privacy setting",
    category: "privacy",
    permission: "administrator",
    impact: "low",
    addedIn: "0.2",
    recommended: true,
    state: "notApplied",
  },
  {
    id: "windows.edge-sidebar-off",
    slug: "browser",
    title: "browser setting",
    category: "windows",
    permission: "administrator",
    impact: "low",
    addedIn: "0.2",
    recommended: false,
    state: "notApplied",
  },
  {
    id: "gaming.mmcss-games-priority",
    slug: "moderate",
    title: "moderate-risk setting",
    category: "gaming",
    permission: "user",
    impact: "high",
    addedIn: "0.2",
    recommended: false,
    state: "partial",
    risk: "moderate",
    warning: "Sample of a warning: names the features this can break.",
  },
  {
    id: "windows.storage-sense-off",
    slug: "outside",
    title: "value set outside 2VX",
    category: "windows",
    permission: "user",
    impact: "low",
    addedIn: "0.1",
    recommended: false,
    state: "applied",
  },
  {
    id: "services.print-spooler-off",
    slug: "service",
    title: "service switch",
    category: "services",
    permission: "administrator",
    impact: "medium",
    addedIn: "0.2",
    recommended: false,
    state: "notApplied",
    restart: "reboot",
    warning: "Sample of a warning: names the features this can break.",
  },
  {
    id: "gaming.game-dvr-off",
    slug: "gaming",
    title: "gaming setting",
    category: "gaming",
    permission: "administrator",
    impact: "medium",
    addedIn: "0.2",
    recommended: false,
    state: "notApplied",
  },
];

const SAMPLE_TWEAKS: TweakMeta[] = SAMPLES.map((x) => ({
  id: x.id,
  name: `Preview sample: ${x.title}`,
  description: "Stands in for a real tweak in browser preview. Applying is disabled here.",
  details: "Preview only. The desktop app lists the real, documented tweaks from its engine.",
  category: x.category,
  risk: x.risk ?? "low",
  permission: x.permission,
  supportedBuilds: { minBuild: 10240, maxBuild: null },
  device: "any",
  edition: "any",
  recommended: x.recommended,
  restart: x.restart ?? "none",
  impact: x.impact,
  warning: x.warning ?? null,
  addedIn: x.addedIn,
  references: ["https://learn.microsoft.com/"],
}));

function tweaks(): TweakMeta[] {
  return SAMPLE_TWEAKS;
}

function tweakStatuses(): TweakStatus[] {
  return SAMPLE_TWEAKS.map((meta, i) => {
    const sample = SAMPLES[i]!;
    const admin = meta.permission === "administrator";
    const applied = sample.state === "applied";
    const target = {
      hive: admin ? ("localMachine" as const) : ("currentUser" as const),
      key: String.raw`Software\2VX-Preview`,
      value: `Sample${i}`,
    };
    return {
      meta,
      compatibility: admin ? "needsElevation" : "compatible",
      state: sample.state,
      settings: [
        {
          target,
          targetLabel: `${admin ? "HKLM" : "HKCU"}\\${target.key}\\${target.value}`,
          current: applied ? { type: "dword", data: 0 } : null,
          currentLabel: applied ? "0 (REG_DWORD)" : "(not set)",
          desired: { type: "dword", data: 0 },
          desiredLabel: "0 (REG_DWORD)",
          matches: applied,
          error: null,
        },
      ],
      // The "outside 2VX" sample shows the toggle-locked case.
      undoable: applied && sample.slug !== "outside",
    };
  });
}

function engineStatus(): EngineStatus {
  return {
    tweakCount: SAMPLE_TWEAKS.length,
    journalAvailable: false,
    journalError: "No change journal in browser preview",
    interruptedSnapshots: 0,
  };
}

function startup(): StartupInventory {
  const user = String.raw`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`;
  const machine = String.raw`HKLM\Software\Microsoft\Windows\CurrentVersion\Run`;
  const folder = String.raw`C:\Users\Preview\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`;
  const item = (
    name: string,
    command: string,
    location: string,
    enabled: boolean | null,
    source: StartupInventory["items"][number]["source"] = "runKey",
  ): StartupInventory["items"][number] => ({
    id: `${location}\\${name}`,
    name: `Preview sample: ${name}`,
    command,
    source,
    scope: location.startsWith("HKLM") ? "allUsers" : "currentUser",
    location,
    enabled,
  });
  return {
    items: [
      item("Chat app", String.raw`"C:\Program Files\Chat\chat.exe" --minimized`, user, true),
      item("Cloud sync", String.raw`C:\Program Files\Sync\sync.exe /background`, user, false),
      item("Audio control panel", String.raw`C:\Windows\System32\audio.exe -s`, machine, null),
      item("Launcher", String.raw`C:\Games\Launcher\launcher.exe`, folder, true, "startupFolder"),
    ],
    unreadable: [],
  };
}

async function cleanup(): Promise<CleanupLocation[]> {
  await new Promise((r) => setTimeout(r, 600));
  const MiB = 1024 ** 2;
  const loc = (
    id: string,
    label: string,
    bytes: number,
    files: number,
    status: CleanupLocation["status"] = "measured",
  ): CleanupLocation => ({
    id,
    label,
    description: "Preview sample. The desktop app measures this location on your PC.",
    path: "(preview)",
    bytes,
    files,
    status,
  });
  return [
    loc("user-temp", "Your temporary files", 1840 * MiB, 5210),
    loc("windows-temp", "Windows temporary files", 0, 0, "denied"),
    loc("directx-shader-cache", "DirectX shader cache", 412 * MiB, 380),
    loc("nvidia-shader-cache", "NVIDIA shader cache", 0, 0, "missing"),
    loc("edge-cache", "Microsoft Edge cache", 655 * MiB, 9120),
    loc("recycle-bin", "Recycle Bin", 3.2 * 1024 * MiB, 214),
  ];
}

/** Anything that would change the system is refused outright, never simulated. */
function refuse(): Promise<never> {
  const err: BackendError = { code: "preview", message: "Not available in browser preview" };
  return Promise.reject(err);
}

export const preview = {
  appInfo,
  account,
  setAccount,
  clearAccount,
  snapshot,
  metricsSince,
  tweaks,
  tweakStatuses,
  engineStatus,
  startup,
  cleanup,
  refuse,
};
