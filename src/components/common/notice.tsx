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
        "flex animate-rise items-start gap-3 rounded-lg border px-3.5 py-2.5",
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

const DOT: Record<Tone, string> = {
  success: "bg-success",
  warning: "bg-warning",
  destructive: "bg-destructive",
  info: "bg-info",
  neutral: "bg-muted-foreground/50",
};

/** A state in words, with a dot for its tone. Plain text, so it never reads as a button. */
export function StatusLabel({
  tone,
  children,
  className,
  title,
}: {
  tone: Tone;
  children: ReactNode;
  className?: string;
  title?: string;
}) {
  return (
    <span
      title={title}
      className={cn(
        "inline-flex shrink-0 items-center gap-1.5 text-[12px] whitespace-nowrap text-muted-foreground",
        className,
      )}
    >
      <span aria-hidden className={cn("size-1.5 shrink-0 rounded-full", DOT[tone])} />
      {children}
    </span>
  );
}
