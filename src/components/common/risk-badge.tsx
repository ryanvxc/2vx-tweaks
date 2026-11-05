import type { Tone } from "@/lib/engine-text";
import type { RiskLevel } from "@/types/engine";
import { StatusLabel } from "./notice";

const STYLES: Record<RiskLevel, { label: string; tone: Tone }> = {
  low: { label: "Low risk", tone: "success" },
  moderate: { label: "Medium risk", tone: "warning" },
  high: { label: "High risk", tone: "destructive" },
};

export function RiskBadge({ risk, className }: { risk: RiskLevel; className?: string }) {
  const s = STYLES[risk];
  return (
    <StatusLabel tone={s.tone} className={className}>
      {s.label}
    </StatusLabel>
  );
}
