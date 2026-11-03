const BYTE_UNITS = ["B", "KB", "MB", "GB", "TB"] as const;

/** Binary-unit byte formatting (1 KB = 1024 B), matching how Windows Explorer reports sizes. */
export function formatBytes(bytes: number | null | undefined, decimals = 1): string {
  if (bytes == null || !Number.isFinite(bytes) || bytes < 0) return "—";
  if (bytes < 1024) return `${Math.round(bytes)} B`;
  const exp = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), BYTE_UNITS.length - 1);
  const value = bytes / 1024 ** exp;
  return `${value.toFixed(value >= 100 ? 0 : decimals)} ${BYTE_UNITS[exp]}`;
}

export function formatRate(bytesPerSec: number | null | undefined): string {
  const b = formatBytes(bytesPerSec);
  return b === "—" ? b : `${b}/s`;
}

export function formatPercent(value: number | null | undefined, decimals = 0): string {
  if (value == null || !Number.isFinite(value)) return "—";
  return `${value.toFixed(decimals)}%`;
}

export function formatUptime(totalSecs: number | null | undefined): string {
  if (totalSecs == null || totalSecs < 0) return "—";
  const d = Math.floor(totalSecs / 86_400);
  const h = Math.floor((totalSecs % 86_400) / 3600);
  const m = Math.floor((totalSecs % 3600) / 60);
  if (d > 0) return `${d}d ${h}h`;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

export function formatFrequency(mhz: number | null | undefined): string {
  if (!mhz) return "—";
  return mhz >= 1000 ? `${(mhz / 1000).toFixed(2)} GHz` : `${mhz} MHz`;
}

export function percentOf(part: number, whole: number): number {
  if (!whole) return 0;
  return Math.min(100, Math.max(0, (part / whole) * 100));
}
