import type { ReactNode } from "react";
import { Link } from "react-router";
import { Gamepad2, History, SlidersHorizontal, type LucideIcon } from "lucide-react";
import { AnimatedNumber } from "@/components/common/animated-number";
import { ImpactMeter } from "@/components/common/indicator";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { Skeleton } from "@/components/ui/skeleton";
import { useSnapshots, useTweakStatuses } from "@/hooks/use-system";
import { isBlocked } from "@/lib/engine-text";
import { activeProfile } from "@/lib/profiles";
import { tweakHref, tweakIcon } from "@/lib/tweak-presentation";

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
        value={
          applied !== undefined ? (
            <>
              <AnimatedNumber value={applied} /> of {total}
            </>
          ) : undefined
        }
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
        label="Changes"
        value={
          snapshots.error ? (
            "—"
          ) : batches !== undefined ? (
            <AnimatedNumber value={batches} />
          ) : undefined
        }
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
  value: ReactNode;
  hint?: string;
}) {
  return (
    <Link
      to={to}
      className="group flex items-center justify-between gap-3 panel px-4 py-2.5 transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring"
    >
      <div className="min-w-0">
        <div className="text-[12.5px] text-muted-foreground">{label}</div>
        {value === undefined ? (
          <Skeleton className="mt-1 h-6 w-20" />
        ) : (
          <div className="truncate text-[21px] leading-snug font-semibold tracking-tight tabular">
            {value}
          </div>
        )}
        {hint && <div className="truncate text-[11.5px] text-muted-foreground">{hint}</div>}
      </div>
      <Icon className="size-5 shrink-0 text-brand transition-transform duration-200 group-hover:scale-110" />
    </Link>
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
  // With nothing to suggest, the panels beside it take the room.
  if (data && open.length === 0) return null;

  return (
    <SectionCard
      title="Recommended"
      source="derived"
      actions={
        open.length > 3 && (
          <Link
            to="/tweaks"
            className="text-[12px] text-muted-foreground transition-colors hover:text-foreground"
          >
            All {open.length}
          </Link>
        )
      }
    >
      {loading && !data ? (
        <LoadingRows rows={3} />
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
