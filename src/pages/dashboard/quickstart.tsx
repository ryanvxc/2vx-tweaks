import { Link } from "react-router";
import { ListChecks } from "lucide-react";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { useSnapshots, useTweakStatuses } from "@/hooks/use-system";
import { openRecommended, QUICKSTART_STEPS, savedBackups } from "@/lib/quickstart";
import { useUiStore } from "@/stores/ui-store";

/** The first things to do here. Ticks come from the PC where they can, never from a guess. */
export function Quickstart() {
  const statuses = useTweakStatuses();
  const snapshots = useSnapshots();
  const games = useUiStore((s) => s.games);
  const gameModeTweaks = useUiStore((s) => s.gameModeTweaks);
  const gameModeSnapshot = useUiStore((s) => s.gameModeSnapshot);
  const opened = useUiStore((s) => s.quickstartOpened);
  const setUi = useUiStore((s) => s.set);

  const facts = {
    applied: savedBackups(snapshots.data ?? []).length > 0,
    recommendedLeft: statuses.data ? openRecommended(statuses.data).length : null,
    gameModeSet: games.length > 0 || gameModeTweaks !== null || gameModeSnapshot !== null,
    opened,
  };

  return (
    <section data-stagger className="flex min-h-0 flex-col gap-2">
      <div className="flex h-5 items-center gap-2">
        <ListChecks className="size-4 shrink-0" strokeWidth={1.75} aria-hidden />
        <h2 className="flex-1 text-section-title">Quickstart</h2>
        <DataSourceBadge source="derived" />
      </div>
      <ul className="min-h-0 flex-1 divide-y divide-dashed overflow-y-auto panel px-1.5">
        {QUICKSTART_STEPS.map((step, i) => {
          const done = step.done(facts);
          return (
            <li key={step.id}>
              <Link
                to={step.to}
                onClick={() => {
                  if (!opened.includes(step.id)) {
                    setUi({ quickstartOpened: [...opened, step.id] });
                  }
                }}
                className="my-1 flex items-center gap-3 rounded-lg px-2 py-1.5 transition-colors outline-none hover:bg-accent/60 focus-visible:ring-2 focus-visible:ring-ring"
              >
                <StepMark done={done} order={i} />
                <div className="min-w-0">
                  <div className="text-[13.5px] leading-snug font-medium">{step.title}</div>
                  <p className="text-[12px] leading-snug text-muted-foreground">{step.text}</p>
                </div>
              </Link>
            </li>
          );
        })}
      </ul>
    </section>
  );
}

/** Drawn by hand so a ticked step can trace its ring and then its check. */
function StepMark({ done, order }: { done: boolean; order: number }) {
  return (
    <svg viewBox="0 0 28 28" className="size-7 shrink-0" fill="none" strokeWidth={1.75}>
      <title>{done ? "Done" : "Not done"}</title>
      <circle cx={14} cy={14} r={12} className="stroke-border" />
      {done && (
        <g className="stroke-success" strokeLinecap="round" strokeLinejoin="round">
          <circle
            cx={14}
            cy={14}
            r={12}
            pathLength={1}
            transform="rotate(-90 14 14)"
            className="draw-in"
            style={{ animationDelay: `${order * 70}ms` }}
          />
          <path
            d="m9.5 14.5 3.2 3.2 6-6.4"
            pathLength={1}
            className="draw-in"
            style={{ animationDelay: `${order * 70 + 220}ms` }}
          />
        </g>
      )}
    </svg>
  );
}
