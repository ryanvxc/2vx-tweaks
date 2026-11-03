import { cn } from "@/lib/utils";
import type { RiskLevel } from "@/types/engine";

const STYLES: Record<RiskLevel, { label: string; className: string }> = {
  low: { label: "Low risk", className: "text-success border-success/25 bg-success/8" },
  moderate: { label: "Moderate risk", className: "text-warning border-warning/30 bg-warning/8" },
  high: {
    label: "High risk",
    className: "text-destructive border-destructive/30 bg-destructive/10",
  },
};

export function RiskBadge({ risk, className }: { risk: RiskLevel; className?: string }) {
  const s = STYLES[risk];
  return (
    <span
      className={cn(
        "inline-flex h-5 items-center rounded-full border px-2 text-[10.5px] font-medium tracking-wide",
        s.className,
        className,
      )}
    >
      {s.label}
    </span>
  );
}
