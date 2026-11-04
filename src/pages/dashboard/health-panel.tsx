import { CheckCircle2, CircleAlert, TriangleAlert } from "lucide-react";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { evaluateHealth, summariseHealth, type HealthStatus } from "@/lib/health";
import { cn } from "@/lib/utils";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

const ICON = {
  good: { icon: CheckCircle2, className: "text-success" },
  attention: { icon: CircleAlert, className: "text-warning" },
  warning: { icon: TriangleAlert, className: "text-destructive" },
} satisfies Record<HealthStatus, unknown>;

const HEADLINE: Record<HealthStatus, string> = {
  good: "Everything looks healthy",
  attention: "A few things to look at",
  warning: "Needs attention",
};

// Worst first, so a problem is never below the fold of the panel.
const ORDER: Record<HealthStatus, number> = { warning: 0, attention: 1, good: 2 };

export function HealthPanel({
  snapshot,
  latest,
}: {
  snapshot: SystemSnapshot | undefined;
  latest: MetricsSample | undefined;
}) {
  const checks = snapshot ? evaluateHealth(snapshot, latest) : [];
  const summary = summariseHealth(checks);
  const sorted = [...checks].sort((a, b) => ORDER[a.status] - ORDER[b.status]);

  return (
    <SectionCard
      title="System health"
      source="derived"
      className="h-full"
      contentClassName="flex flex-col"
    >
      {!snapshot ? (
        <LoadingRows rows={5} />
      ) : (
        <div className="flex flex-1 flex-col gap-2.5">
          <div
            className={cn(
              "text-[13.5px] font-semibold",
              summary.status === "good" && "text-success",
              summary.status === "attention" && "text-warning",
              summary.status === "warning" && "text-destructive",
            )}
          >
            {HEADLINE[summary.status]}
            <span className="ml-1.5 font-normal text-muted-foreground tabular">
              {summary.total - summary.attention - summary.warnings} of {summary.total}
            </span>
          </div>
          <ul className="flex flex-1 flex-col justify-between gap-1.5">
            {sorted.map((c) => {
              const { icon: Icon, className } = ICON[c.status];
              return (
                <li key={c.id} className="flex items-start gap-2.5">
                  <Icon className={cn("mt-0.5 size-4 shrink-0", className)} />
                  <div className="min-w-0">
                    <div className="text-[13px] leading-tight font-medium">{c.label}</div>
                    <div
                      className="truncate text-[12px] text-muted-foreground tabular"
                      title={c.detail}
                    >
                      {c.detail}
                    </div>
                  </div>
                </li>
              );
            })}
          </ul>
        </div>
      )}
    </SectionCard>
  );
}
