import { Link } from "react-router";
import {
  CheckCircle2,
  ChevronRight,
  Circle,
  Cpu,
  History,
  Rocket,
  ShieldCheck,
  SlidersHorizontal,
  Sparkles,
  type LucideIcon,
} from "lucide-react";
import { ImpactMeter } from "@/components/common/indicator";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, LoadingRows } from "@/components/common/states";
import { Skeleton } from "@/components/ui/skeleton";
import { useSnapshots, useTweakStatuses } from "@/hooks/use-system";
import { isBlocked } from "@/lib/engine-text";
import { tweakIcon } from "@/lib/tweak-presentation";
import { cn } from "@/lib/utils";
import type { HealthSummary } from "@/lib/health";

/** Three headline numbers, each read from the engine or derived from live checks. */
export function HomeStats({ health }: { health: HealthSummary | null }) {
  const statuses = useTweakStatuses();
  const snapshots = useSnapshots();
  const applied = statuses.data?.filter((t) => t.state === "applied").length;
  const total = statuses.data?.length;
  const batches = snapshots.data?.filter((s) => s.kind === "apply").length;

  return (
    <div className="grid grid-cols-3 gap-3">
      <StatCard
        to="/system"
        icon={SlidersHorizontal}
        label="Optimisations in effect"
        value={applied !== undefined ? `${applied} of ${total}` : undefined}
      />
      <StatCard
        to="/history"
        icon={History}
        label="Changes recorded"
        value={snapshots.error ? "—" : batches !== undefined ? String(batches) : undefined}
        hint={snapshots.error ? "Journal unavailable" : undefined}
      />
      <StatCard
        to="/hardware"
        icon={ShieldCheck}
        label="Health checks passed"
        value={
          health
            ? `${health.total - health.attention - health.warnings} of ${health.total}`
            : undefined
        }
        tone={health?.status === "good" ? "text-success" : health ? "text-warning" : undefined}
      />
    </div>
  );
}

function StatCard({
  to,
  icon: Icon,
  label,
  value,
  hint,
  tone,
}: {
  to: string;
  icon: LucideIcon;
  label: string;
  value: string | undefined;
  hint?: string;
  tone?: string;
}) {
  return (
    <Link
      to={to}
      className="group flex items-center gap-3.5 rounded-xl border bg-card px-4 py-3.5 transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring"
    >
      <div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-brand/12 text-brand">
        <Icon className="size-[18px]" />
      </div>
      <div className="min-w-0 flex-1">
        <div className="text-[12px] text-muted-foreground">{label}</div>
        {value === undefined ? (
          <Skeleton className="mt-1 h-5 w-16" />
        ) : (
          <div className={cn("text-[19px] leading-tight font-semibold tabular", tone)}>{value}</div>
        )}
        {hint && <div className="truncate text-[11.5px] text-muted-foreground">{hint}</div>}
      </div>
      <ChevronRight className="size-4 text-muted-foreground/40 transition-transform group-hover:translate-x-0.5" />
    </Link>
  );
}

/** Real first steps through the app, ticked off from detected state where possible. */
export function Quickstart() {
  const snapshots = useSnapshots();
  const anyApplied = (snapshots.data ?? []).some((s) => s.kind === "apply");
  const steps: { to: string; icon: LucideIcon; title: string; text?: string; done?: boolean }[] = [
    {
      to: "/hardware",
      icon: Cpu,
      title: "Look at your hardware",
    },
    {
      to: "/system",
      icon: SlidersHorizontal,
      title: "Apply a tweak",
      done: anyApplied,
    },
    {
      to: "/startup",
      icon: Rocket,
      title: "Check what starts with Windows",
    },
    {
      to: "/cleaner",
      icon: Sparkles,
      title: "See what caches use",
    },
    {
      to: "/history",
      icon: History,
      title: "Know you can undo",
      text: "Every change is journaled with its old value",
    },
  ];

  return (
    <SectionCard title="Quickstart" className="h-full">
      <ul className="-mx-2 flex flex-col">
        {steps.map((s) => (
          <li key={s.to}>
            <Link
              to={s.to}
              className="group flex items-center gap-3 rounded-md px-2 py-2 transition-colors outline-none hover:bg-accent/60 focus-visible:ring-2 focus-visible:ring-ring"
            >
              <s.icon className="size-4 shrink-0 text-brand" />
              <div className="min-w-0 flex-1">
                <div className="text-[13px] leading-snug font-medium">{s.title}</div>
                {s.text && (
                  <div className="truncate text-[11.5px] text-muted-foreground">{s.text}</div>
                )}
              </div>
              {s.done ? (
                <CheckCircle2 className="size-4 text-success" aria-label="Done" />
              ) : s.done === false ? (
                <Circle className="size-4 text-muted-foreground/40" aria-label="Not done yet" />
              ) : (
                <ChevronRight className="size-4 text-muted-foreground/40" />
              )}
            </Link>
          </li>
        ))}
      </ul>
    </SectionCard>
  );
}

/**
 * Recommended tweaks that are not in effect yet. Derived from the live catalogue state:
 * the engine marks a tweak recommended only when it is low risk and documented.
 */
export function RecommendedTweaks() {
  const { data, loading } = useTweakStatuses();
  const open = (data ?? []).filter(
    (t) => t.meta.recommended && t.state !== "applied" && !isBlocked(t.compatibility),
  );

  return (
    <SectionCard
      title="Recommended for this PC"
      source="derived"
      actions={
        open.length > 0 && (
          <Link to="/system" className="text-[12px] font-medium text-brand hover:underline">
            Review
          </Link>
        )
      }
    >
      {loading && !data ? (
        <LoadingRows rows={3} />
      ) : open.length === 0 ? (
        <EmptyState
          icon={CheckCircle2}
          title="Nothing to recommend"
          description="Every recommended tweak is already in effect on this PC."
          className="py-6"
        />
      ) : (
        <ul className="-mx-2 flex flex-col">
          {open.slice(0, 4).map((t) => {
            const Icon = tweakIcon(t.meta);
            return (
              <li key={t.meta.id}>
                <Link
                  to={`/system?tweak=${encodeURIComponent(t.meta.id)}`}
                  className="flex items-center gap-3 rounded-md px-2 py-2 transition-colors hover:bg-accent/60"
                >
                  <Icon className="size-4 shrink-0 text-brand" />
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-[13px] leading-snug font-medium">
                      {t.meta.name}
                    </div>
                    <div className="truncate text-[11.5px] text-muted-foreground">
                      {t.meta.description}
                    </div>
                  </div>
                  <ImpactMeter impact={t.meta.impact} />
                </Link>
              </li>
            );
          })}
        </ul>
      )}
    </SectionCard>
  );
}
