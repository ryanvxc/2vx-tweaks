import type { TweakStatus } from "@/types/engine";
import type { AppInfo, SystemSnapshot } from "@/types/system";
import { HEALTH_HEADLINE, evaluateHealth, summariseHealth } from "./health";
import { formatBytes, formatFrequency, formatUptime } from "./format";

/**
 * Plain-text report for support requests. Only data the app already shows. It lists the tweaks
 * in effect; `full` adds every other one with its state.
 */
export function buildSystemReport(
  app: AppInfo,
  s: SystemSnapshot,
  tweaks: TweakStatus[],
  now = new Date(),
  full = false,
): string {
  const applied = tweaks.filter((t) => t.state === "applied");
  const checks = evaluateHealth(s);
  const health = summariseHealth(checks);
  const os = s.os;
  const lines = [
    `2VX Tweaks ${app.version}${app.debugBuild ? " (development build)" : ""} system report`,
    `Generated ${now.toISOString()}`,
    "",
    "[Windows]",
    `${os.productName}${os.displayVersion ? ` ${os.displayVersion}` : ""} (${os.editionId ?? "edition unknown"})`,
    `Build ${os.build}${os.ubr != null ? `.${os.ubr}` : ""}, ${os.architecture}`,
    `PC name: ${s.hostname ?? "unknown"}`,
    `Uptime: ${formatUptime(s.uptimeSecs)}`,
    `2VX running as: ${app.isElevated ? "administrator" : "standard user"}`,
    "",
    "[Hardware]",
    `CPU: ${s.cpu.brand} (${s.cpu.physicalCores ?? "?"} cores, ${s.cpu.logicalCores} threads, ${formatFrequency(s.cpu.frequencyMhz)})`,
    `Memory: ${formatBytes(s.memory.totalBytes)} (page file ${formatBytes(s.memory.swapTotalBytes)})`,
    ...s.gpus.map(
      (g, i) =>
        `GPU ${i + 1}: ${g.model ?? "unknown"}${g.dedicatedMemoryBytes ? ` (${formatBytes(g.dedicatedMemoryBytes, 0)})` : ""}`,
    ),
    ...(s.motherboard
      ? [`Board: ${s.motherboard.vendor ?? ""} ${s.motherboard.name ?? ""}`.trim()]
      : []),
    ...s.disks.map(
      (d) =>
        `Drive ${d.mountPoint} ${d.kind.toUpperCase()} ${d.fileSystem}: ${formatBytes(d.availableBytes)} free of ${formatBytes(d.totalBytes)}`,
    ),
    "",
    "[Health]",
    HEALTH_HEADLINE[health.status],
    ...checks
      .filter((c) => c.status !== "good")
      .map((c) => `${c.label}: ${c.detail} (${c.status})`),
    "",
    `[Tweaks applied: ${applied.length}]`,
    ...(applied.length === 0 ? ["(none)"] : applied.map((t) => t.meta.id)),
    ...(full
      ? [
          "",
          "[All tweaks]",
          ...tweaks.map(
            (t) => `${t.state === "applied" ? "[x]" : "[ ]"} ${t.meta.id} (${t.state})`,
          ),
        ]
      : []),
  ];
  return lines.join("\n");
}
