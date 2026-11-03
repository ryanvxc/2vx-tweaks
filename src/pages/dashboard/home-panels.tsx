import { Link } from "react-router";
import {
  CheckCircle2,
  ChevronRight,
  Cpu,
  Gamepad2,
  History,
  Rocket,
  SlidersHorizontal,
  Sparkles,
  X,
  type LucideIcon,
} from "lucide-react";
import { ImpactMeter } from "@/components/common/indicator";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, LoadingRows } from "@/components/common/states";
import { Skeleton } from "@/components/ui/skeleton";
import { useSnapshots, useTweakStatuses } from "@/hooks/use-system";
import { isBlocked } from "@/lib/engine-text";
import { activeProfile } from "@/lib/profiles";
import { tweakHref, tweakIcon } from "@/lib/tweak-presentation";
import { useUiStore } from "@/stores/ui-store";

/** Three headline numbers, each read from the engine. */
export function HomeStats() {
  const statuses = useTweakStatuses();
  const snapshots = useSnapshots();
  const applied = statuses.data?.filter((t) => t.state === "applied").length;
  const total = statuses.data?.length;
  const batches = snapshots.data?.filter((s) => s.kind === "apply").length;

  return (
    <div className="grid grid-cols-3 gap-3">
      <StatCard
        to="/tweaks"
        icon={SlidersHorizontal}
        label="Tweaks in effect"
        value={applied !== undefined ? `${applied} of ${total}` : undefined}
      />
      <StatCard
        to="/gaming"
        icon={Gamepad2}
        label="Gaming profile"
        value={statuses.data ? (activeProfile(statuses.data)?.name ?? "None") : undefined}
      />
      <StatCard
        to="/restore"
        icon={History}
        label="Changes you can review"
        value={snapshots.error ? "—" : batches !== undefined ? String(batches) : undefined}
        hint={snapshots.error ? "Journal unavailable" : undefined}
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
}: {
  to: string;
  icon: LucideIcon;
  label: string;
  value: string | undefined;
  hint?: string;
}) {
  return (
    <Link
      to={to}
      className="group flex items-center gap-3 rounded-xl border bg-card px-4 py-2 transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring"
    >
      <div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-brand/12 text-brand">
        <Icon className="size-[18px]" />
      </div>
      <div className="min-w-0 flex-1">
        <div className="text-[12.5px] text-muted-foreground">{label}</div>
        {value === undefined ? (
          <Skeleton className="mt-1 h-5 w-16" />
        ) : (
          <div className="truncate font-heading text-[18px] leading-tight font-bold tabular">
            {value}
          </div>
        )}
        {hint && <div className="truncate text-[11.5px] text-muted-foreground">{hint}</div>}
      </div>
      <ChevronRight className="size-4 text-muted-foreground/40 transition-transform group-hover:translate-x-0.5" />
    </Link>
  );
}

/** Real first steps through the app, one line, hidden with one click. */
export function Quickstart() {
  const snapshots = useSnapshots();
  const hide = useUiStore((s) => s.set);
  const anyApplied = (snapshots.data ?? []).some((s) => s.kind === "apply");
  const steps: { to: string; icon: LucideIcon; title: string; done?: boolean }[] = [
    { to: "/hardware", icon: Cpu, title: "See your hardware" },
    { to: "/tweaks", icon: SlidersHorizontal, title: "Apply a tweak", done: anyApplied },
    { to: "/startup", icon: Rocket, title: "Check startup" },
    { to: "/cleaner", icon: Sparkles, title: "See what caches use" },
    { to: "/restore", icon: History, title: "Know you can undo" },
  ];

  return (
    <nav
      aria-label="Quickstart"
      className="flex items-center gap-1 rounded-xl border bg-card py-1.5 pr-1.5 pl-4"
    >
      <span className="mr-2 text-card-title">Quickstart</span>
      <div className="flex min-w-0 flex-1 items-center gap-1">
        {steps.map((s) => {
          const Icon = s.done ? CheckCircle2 : s.icon;
          return (
            <Link
              key={s.to}
              to={s.to}
              className="flex h-7.5 min-w-0 items-center gap-1.5 rounded-md px-2 text-[12.5px] text-muted-foreground transition-colors outline-none hover:bg-accent hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring"
            >
              <Icon className={s.done ? "size-3.5 text-success" : "size-3.5 text-brand"} />
              <span className="truncate">{s.title}</span>
            </Link>
          );
        })}
      </div>
      <button
        type="button"
        aria-label="Hide Quickstart"
        onClick={() => hide({ showQuickstart: false })}
        className="flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground hover:bg-accent hover:text-foreground"
      >
        <X className="size-3.5" />
      </button>
    </nav>
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
      className="h-full"
      actions={
        open.length > 3 && (
          <Link to="/tweaks" className="text-[12px] font-medium text-brand hover:underline">
            All {open.length}
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
          className="py-2"
        />
      ) : (
        <ul className="-mx-2 flex flex-col">
          {open.slice(0, 3).map((t) => {
            const Icon = tweakIcon(t.meta);
            return (
              <li key={t.meta.id}>
                <Link
                  to={tweakHref(t.meta)}
                  className="flex items-center gap-3 rounded-md px-2 py-1.5 transition-colors hover:bg-accent/60"
                >
                  <Icon className="size-4 shrink-0 text-brand" />
                  <span
                    className="min-w-0 flex-1 truncate text-[13px] font-medium"
                    title={t.meta.name}
                  >
                    {t.meta.name}
                  </span>
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
