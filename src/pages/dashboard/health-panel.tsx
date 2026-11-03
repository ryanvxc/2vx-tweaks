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

export function HealthPanel({
  snapshot,
  latest,
}: {
  snapshot: SystemSnapshot | undefined;
  latest: MetricsSample | undefined;
}) {
  const checks = snapshot ? evaluateHealth(snapshot, latest) : [];
  const summary = summariseHealth(checks);

  return (
    <SectionCard title="System health" source="derived">
      {!snapshot ? (
        <LoadingRows rows={5} />
      ) : (
        <div className="flex flex-col gap-3">
          <div
            className={cn(
              "rounded-md border px-3 py-2 text-[12.5px] font-medium",
              summary.status === "good" && "border-success/20 bg-success/6 text-success",
              summary.status === "attention" && "border-warning/25 bg-warning/6 text-warning",
              summary.status === "warning" &&
                "border-destructive/25 bg-destructive/8 text-destructive",
            )}
          >
            {HEADLINE[summary.status]}
            <span className="font-normal text-muted-foreground">
              {" "}
              · {summary.total - summary.attention - summary.warnings} of {summary.total} checks
              passed
            </span>
          </div>
          <ul className="flex flex-col gap-2.5">
            {checks.map((c) => {
              const { icon: Icon, className } = ICON[c.status];
              return (
                <li key={c.id} className="flex items-start gap-2.5">
                  <Icon className={cn("mt-0.5 size-4 shrink-0", className)} />
                  <div className="min-w-0">
                    <div className="text-[13px] font-medium">{c.label}</div>
                    <div className="truncate text-[12px] text-muted-foreground tabular">
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
