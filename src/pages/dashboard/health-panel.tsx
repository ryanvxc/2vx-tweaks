import { CheckCircle2, CircleAlert, TriangleAlert } from "lucide-react";
import { LoadingRows } from "@/components/common/states";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import {
  HEALTH_HEADLINE,
  summariseHealth,
  type HealthCheck,
  type HealthStatus,
} from "@/lib/health";
import { cn } from "@/lib/utils";

const ICON = {
  good: { icon: CheckCircle2, className: "text-success" },
  attention: { icon: CircleAlert, className: "text-warning" },
  warning: { icon: TriangleAlert, className: "text-destructive" },
} satisfies Record<HealthStatus, unknown>;

// Worst first, so a problem is never below the fold of the panel.
const ORDER: Record<HealthStatus, number> = { warning: 0, attention: 1, good: 2 };

export function HealthChecks({ checks }: { checks: HealthCheck[] }) {
  if (checks.length === 0) return <LoadingRows rows={5} />;
  const summary = summariseHealth(checks);
  const sorted = [...checks].sort((a, b) => ORDER[a.status] - ORDER[b.status]);

  return (
    <div className="flex flex-col gap-3">
      <div
        className={cn(
          "text-[13.5px] font-semibold",
          summary.status === "good" && "text-success",
          summary.status === "attention" && "text-warning",
          summary.status === "warning" && "text-destructive",
        )}
      >
        {HEALTH_HEADLINE[summary.status]}
        <span className="ml-1.5 font-normal text-muted-foreground tabular">
          {summary.total - summary.attention - summary.warnings} of {summary.total}
        </span>
      </div>
      <ul className="flex flex-col gap-2.5">
        {sorted.map((c) => {
          const { icon: Icon, className } = ICON[c.status];
          return (
            <li key={c.id} className="flex items-start gap-2.5">
              <Icon className={cn("mt-0.5 size-4 shrink-0", className)} />
              <div className="min-w-0">
                <div className="text-[13px] leading-tight font-medium">{c.label}</div>
                <div className="text-[12px] text-muted-foreground tabular">{c.detail}</div>
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

/** The full list behind the Home card, which only has room for the score. */
export function HealthDialog({
  open,
  onOpenChange,
  checks,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  checks: HealthCheck[];
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[calc(100vh-4rem)] flex-col sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>System health</DialogTitle>
        </DialogHeader>
        <HealthChecks checks={checks} />
      </DialogContent>
    </Dialog>
  );
}
