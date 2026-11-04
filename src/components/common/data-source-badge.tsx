import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { isDesktop } from "@/services/backend";
import { cn } from "@/lib/utils";

/**
 * Every data-bearing card declares where its numbers come from. This is a product
 * rule, not decoration: users must never mistake mock data for a measurement.
 */
export type DataSource = "live" | "derived" | "sample" | "planned";

const CONFIG: Record<DataSource | "preview", { label: string; className: string; tip: string }> = {
  live: {
    label: "Live",
    className: "text-success border-success/25 bg-success/8",
    tip: "Measured on this PC.",
  },
  derived: {
    label: "Derived",
    className: "text-info border-info/25 bg-info/8",
    tip: "Calculated from live data.",
  },
  sample: {
    label: "Sample data",
    className: "text-warning border-warning/30 bg-warning/8",
    tip: "Placeholder, not measured.",
  },
  planned: {
    label: "Planned",
    className: "text-muted-foreground border-border bg-muted/50",
    tip: "Not built yet.",
  },
  preview: {
    label: "Preview",
    className: "text-info border-info/25 bg-info/8",
    tip: "Simulated in the browser.",
  },
};

export function DataSourceBadge({
  source,
  pulse,
  className,
}: {
  source: DataSource;
  /** Breathing dot, for the one reading being watched. Too many of these are just noise. */
  pulse?: boolean;
  className?: string;
}) {
  // Outside the desktop shell, "live" and "derived" values are simulated.
  const key = !isDesktop && (source === "live" || source === "derived") ? "preview" : source;
  const cfg = CONFIG[key];
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          className={cn(
            "inline-flex h-5 cursor-default items-center gap-1 rounded-full border px-2 text-[10.5px] font-medium tracking-wide",
            cfg.className,
            className,
          )}
        >
          {key === "live" && (
            <span className={cn("size-1.5 rounded-full bg-current", pulse && "animate-live")} />
          )}
          {cfg.label}
        </span>
      </TooltipTrigger>
      <TooltipContent side="top" className="max-w-60">
        {cfg.tip}
      </TooltipContent>
    </Tooltip>
  );
}
