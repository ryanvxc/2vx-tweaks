import type { Tone } from "@/lib/engine-text";
import type { RiskLevel } from "@/types/engine";
import { StatusLabel } from "./notice";

/** Moderate has no label: the engine still ranks it, but it gives the reader nothing to act on. */
const STYLES: Partial<Record<RiskLevel, { label: string; tone: Tone }>> = {
  low: { label: "Low risk", tone: "success" },
  high: { label: "High risk", tone: "destructive" },
};

export function RiskBadge({ risk, className }: { risk: RiskLevel; className?: string }) {
  const s = STYLES[risk];
  if (!s) return null;
  return (
    <StatusLabel tone={s.tone} className={className}>
      {s.label}
    </StatusLabel>
  );
}
