import type { ReactNode } from "react";
import { AlertTriangle, CheckCircle2, Info, OctagonAlert, type LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import type { Message, Tone } from "@/lib/engine-text";

const TONES: Record<Tone, { icon: LucideIcon; box: string; iconClass: string }> = {
  success: {
    icon: CheckCircle2,
    box: "border-success/25 bg-success/8",
    iconClass: "text-success",
  },
  warning: {
    icon: AlertTriangle,
    box: "border-warning/30 bg-warning/8",
    iconClass: "text-warning",
  },
  destructive: {
    icon: OctagonAlert,
    box: "border-destructive/25 bg-destructive/8",
    iconClass: "text-destructive",
  },
  info: { icon: Info, box: "border-info/25 bg-info/8", iconClass: "text-info" },
  neutral: { icon: Info, box: "border-border bg-muted/50", iconClass: "text-muted-foreground" },
};

/** An inline status message. The tone carries meaning (state), never decoration. */
export function Notice({
  tone,
  title,
  children,
  action,
  className,
}: {
  tone: Tone;
  title: string;
  children?: ReactNode;
  action?: ReactNode;
  className?: string;
}) {
  const t = TONES[tone];
  const Icon = t.icon;
  return (
    <div
      role={tone === "destructive" || tone === "warning" ? "alert" : "status"}
      className={cn(
        "flex animate-rise items-start gap-3 rounded-lg border p-3.5",
        t.box,
        className,
      )}
    >
      <Icon className={cn("mt-0.5 size-4 shrink-0", t.iconClass)} />
      <div className="min-w-0 flex-1">
        <div className="text-[13px] font-medium">{title}</div>
        {children && (
          <div
            data-selectable
            className="mt-0.5 text-[12.5px] leading-relaxed break-words text-muted-foreground"
          >
            {children}
          </div>
        )}
      </div>
      {action && <div className="shrink-0">{action}</div>}
    </div>
  );
}

export function MessageNotice({ tone, title, description }: Message) {
  return (
    <Notice tone={tone} title={title}>
      {description}
    </Notice>
  );
}

const PILL: Record<Tone, string> = {
  success: "text-success border-success/25 bg-success/8",
  warning: "text-warning border-warning/30 bg-warning/8",
  destructive: "text-destructive border-destructive/30 bg-destructive/10",
  info: "text-info border-info/25 bg-info/8",
  neutral: "text-muted-foreground border-border bg-muted/50",
};

/** Small state label, same shape as `RiskBadge`. */
export function StatusPill({
  tone,
  children,
  className,
}: {
  tone: Tone;
  children: ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex h-5 shrink-0 items-center rounded-full border px-2 text-[10.5px] font-medium tracking-wide whitespace-nowrap",
        PILL[tone],
        className,
      )}
    >
      {children}
    </span>
  );
}
