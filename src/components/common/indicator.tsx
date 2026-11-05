import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import type { Impact } from "@/types/engine";

export type IndicatorTone = "success" | "warning" | "destructive" | "info" | "brand" | "neutral";

const TONE_TEXT: Record<IndicatorTone, string> = {
  success: "text-success",
  warning: "text-warning",
  destructive: "text-destructive",
  info: "text-info",
  brand: "text-brand",
  neutral: "text-muted-foreground",
};

const BOX = "size-7 border border-hairline bg-background/40";

/**
 * A small icon on a card that explains itself on hover or keyboard focus: a title, plus a
 * sentence when the title isn't enough. The label is also the accessible name, so the meaning never
 * depends on colour or hover alone.
 */
export function Indicator({
  icon: Icon,
  tone,
  title,
  children,
  boxed,
  className,
}: {
  icon: LucideIcon;
  tone: IndicatorTone;
  title: string;
  children?: ReactNode;
  /** In a small square, for the row at the bottom of a card. */
  boxed?: boolean;
  className?: string;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          tabIndex={0}
          aria-label={title}
          className={cn(
            "inline-flex size-6 cursor-default items-center justify-center rounded-md transition-colors outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring",
            TONE_TEXT[tone],
            boxed && BOX,
            className,
          )}
        >
          <Icon className="size-3.5" />
        </span>
      </TooltipTrigger>
      <IndicatorTip icon={Icon} tone={tone} title={title}>
        {children}
      </IndicatorTip>
    </Tooltip>
  );
}

function IndicatorTip({
  icon: Icon,
  tone,
  title,
  children,
}: {
  icon: LucideIcon;
  tone: IndicatorTone;
  title: string;
  children?: ReactNode;
}) {
  return (
    <TooltipContent side="top" className="max-w-64 px-3 py-2">
      <div className="flex items-center gap-1.5 font-semibold">
        <Icon className={cn("size-3.5", TONE_TEXT[tone])} />
        {title}
      </div>
      {children && <div className="mt-0.5 leading-snug text-muted-foreground">{children}</div>}
    </TooltipContent>
  );
}

const IMPACT: Record<Impact, { bars: number; color: string; title: string }> = {
  low: { bars: 1, color: "bg-success", title: "Small effect" },
  medium: { bars: 2, color: "bg-warning", title: "Noticeable effect" },
  high: { bars: 3, color: "bg-destructive", title: "Large effect" },
};

/** Three bars for the documented size of a tweak's effect. A rating, never a measurement. */
export function ImpactMeter({ impact }: { impact: Impact }) {
  const info = IMPACT[impact];
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          tabIndex={0}
          aria-label={info.title}
          className={cn(
            "group/impact inline-flex cursor-default items-end justify-center gap-[3px] rounded-md pb-[7px] transition-colors outline-none hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring",
            BOX,
          )}
        >
          {[1, 2, 3].map((n) => (
            <span
              key={n}
              className={cn(
                "w-[3px] origin-bottom rounded-full transition-transform duration-200 group-hover/impact:scale-y-115",
                n <= info.bars ? info.color : "bg-muted-foreground/25",
              )}
              style={{ height: 3 + n * 3 }}
            />
          ))}
        </span>
      </TooltipTrigger>
      <TooltipContent side="top">{info.title}</TooltipContent>
    </Tooltip>
  );
}
