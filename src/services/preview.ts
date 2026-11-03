import type { AppInfo, MetricsSample, SystemSnapshot } from "@/types/system";

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
    version: "0.1.0",
    isElevated: false,
    logDir: "(preview — no log directory)",
    debugBuild: true,
  };
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

export const preview = { appInfo, snapshot, metricsSince };
