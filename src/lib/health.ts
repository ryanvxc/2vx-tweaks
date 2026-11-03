import type { MetricsSample, SystemSnapshot } from "@/types/system";
import { formatBytes, formatUptime, percentOf } from "./format";

export type HealthStatus = "good" | "attention" | "warning";

export interface HealthCheck {
  id: string;
  label: string;
  status: HealthStatus;
  detail: string;
}

/**
 * Read-only health checks derived from measured data. Thresholds are deliberately
 * conservative: these are prompts to look, not alarms.
 */
export function evaluateHealth(snapshot: SystemSnapshot, latest?: MetricsSample): HealthCheck[] {
  const checks: HealthCheck[] = [];

  const systemDrive =
    snapshot.disks.find((d) => d.mountPoint.toUpperCase().startsWith("C:")) ?? snapshot.disks[0];
  if (systemDrive) {
    const freePct = percentOf(systemDrive.availableBytes, systemDrive.totalBytes);
    checks.push({
      id: "system-drive-space",
      label: "System drive space",
      status: freePct < 10 ? "warning" : freePct < 20 ? "attention" : "good",
      detail: `${formatBytes(systemDrive.availableBytes)} free of ${formatBytes(systemDrive.totalBytes)} (${freePct.toFixed(0)}%)`,
    });
  }

  if (latest && latest.memoryTotalBytes > 0) {
    const usedPct = percentOf(latest.memoryUsedBytes, latest.memoryTotalBytes);
    checks.push({
      id: "memory-pressure",
      label: "Memory pressure",
      status: usedPct > 90 ? "warning" : usedPct > 80 ? "attention" : "good",
      detail: `${usedPct.toFixed(0)}% of RAM in use`,
    });
  }

  if (snapshot.uptimeSecs != null) {
    const days = snapshot.uptimeSecs / 86_400;
    checks.push({
      id: "uptime",
      label: "Uptime",
      status: days > 14 ? "attention" : "good",
      detail:
        days > 14
          ? `Up ${formatUptime(snapshot.uptimeSecs)}. Restarting applies pending updates.`
          : `Up ${formatUptime(snapshot.uptimeSecs)}`,
    });
  }

  checks.push(
    snapshot.os.isWindows11
      ? {
          id: "os-support",
          label: "Windows version",
          status: "good",
          detail: `${snapshot.os.productName} ${snapshot.os.displayVersion ?? ""}`.trim(),
        }
      : {
          id: "os-support",
          label: "Windows version",
          status: "attention",
          detail: "Windows 10 reached end of support on 14 October 2025.",
        },
  );

  checks.push({
    id: "gpu-detected",
    label: "Graphics adapter",
    status: snapshot.gpus.length > 0 ? "good" : "attention",
    detail: snapshot.gpus[0]?.model ?? "No GPU reported by DXGI",
  });

  return checks;
}

export function summariseHealth(checks: HealthCheck[]) {
  const warnings = checks.filter((c) => c.status === "warning").length;
  const attention = checks.filter((c) => c.status === "attention").length;
  const status: HealthStatus = warnings ? "warning" : attention ? "attention" : "good";
  return { status, warnings, attention, total: checks.length };
}
