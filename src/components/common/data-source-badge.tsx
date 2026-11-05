import type { ReactNode } from "react";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { isDesktop } from "@/services/backend";
import { cn } from "@/lib/utils";

/**
 * Every data-bearing card declares where its numbers come from. This is a product
 * rule, not decoration: users must never mistake mock data for a measurement.
 */
export type DataSource = "live" | "derived" | "sample" | "planned";

const CONFIG: Record<DataSource | "preview", { label: string; tip: string; tone: string }> = {
  live: { label: "Live", tip: "Measured on this PC.", tone: "bg-success" },
  derived: {
    label: "Derived",
    tip: "Calculated from live data.",
    tone: "border border-muted-foreground/70",
  },
  sample: { label: "Sample data", tip: "Placeholder, not measured.", tone: "text-warning" },
  planned: { label: "Planned", tip: "Not built yet.", tone: "text-muted-foreground" },
  preview: { label: "Preview", tip: "Simulated in the browser.", tone: "text-muted-foreground" },
};

export function DataSourceBadge({
  source,
  pulse,
  label,
  spelled,
  className,
}: {
  source: DataSource;
  /** Words beside the dot, for a badge that stands on its own. */
  label?: ReactNode;
  /** Its own name beside the dot: a bare dot alone in a row looks like a stray pixel. */
  spelled?: boolean;
  /** Breathing dot, for the one reading being watched. Too many of these are just noise. */
  pulse?: boolean;
  className?: string;
}) {
  // Outside the desktop shell, "live" and "derived" values are simulated.
  const key = !isDesktop && (source === "live" || source === "derived") ? "preview" : source;
  const cfg = CONFIG[key];
  // Real numbers get a quiet dot. Anything that isn't real says so in words.
  const dot = key === "live" || key === "derived";
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          role="img"
          aria-label={cfg.label}
          className={cn(
            "inline-flex h-4 min-w-4 cursor-default items-center justify-center",
            (label || spelled) &&
              "gap-1.5 text-[13px] whitespace-nowrap text-muted-foreground tabular",
            !dot && "text-[11px] font-medium",
            !dot && cfg.tone,
            className,
          )}
        >
          {dot ? (
            <span
              className={cn(
                "size-1.5 rounded-full",
                cfg.tone,
                pulse && key === "live" && "animate-live",
              )}
            />
          ) : (
            cfg.label
          )}
          {dot && (label ?? (spelled && cfg.label))}
        </span>
      </TooltipTrigger>
      <TooltipContent side="top" className="max-w-60">
        {cfg.label}: {cfg.tip}
      </TooltipContent>
    </Tooltip>
  );
}
