import { useState, type ReactNode } from "react";
import {
  Cpu,
  Gamepad2,
  Gauge,
  Gpu,
  Joystick,
  ListChecks,
  Loader2,
  MoonStar,
  Plus,
  Settings,
  Timer,
  Undo2,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useRunningPrograms } from "@/hooks/use-running-programs";
import { refreshEngineState, useEngineStatus, useSnapshots } from "@/hooks/use-system";
import { describeError, describeOutcome, isBlocked } from "@/lib/engine-text";
import { formatPercent, formatUptime } from "@/lib/format";
import { detectGame } from "@/lib/game-mode";
import { DEFAULT_GAME_MODE_TWEAKS } from "@/lib/profiles";
import { cn } from "@/lib/utils";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import type { TweakStatus } from "@/types/engine";
import { ChangeInspector } from "../tweaks/change-inspector";
import { AddGameDialog } from "./add-game-dialog";

const PROMISES: { icon: LucideIcon; colour: string; title: string; text: string }[] = [
  {
    icon: Gauge,
    colour: "text-chart-4",
    title: "Game first",
    text: "The active window gets three times the CPU time of background apps.",
  },
  {
    icon: MoonStar,
    colour: "text-chart-2",
    title: "Quiet background",
    text: "Edge stops running in the background while you play.",
  },
  {
    icon: ListChecks,
    colour: "text-chart-3",
    title: "Review first",
    text: "You see every value before it changes.",
  },
  {
    icon: Undo2,
    colour: "text-chart-1",
    title: "Full undo",
    text: "Turning it off puts back exactly what it changed.",
  },
];

type OrbState = "idle" | "ready" | "on" | "inEffect" | "blocked" | "busy";

const ORB: Record<OrbState, { label: string; hint: string }> = {
  idle: { label: "Start a Game", hint: "Game Mode can turn on once a game is running." },
  ready: { label: "Turn on", hint: "Review what Game Mode will change." },
  on: { label: "On", hint: "Stays on until you turn it off. Click to undo its changes." },
  inEffect: { label: "Already on", hint: "Everything you chose is already in effect." },
  blocked: { label: "Turn on", hint: "Choose something in Configure first." },
  busy: { label: "Working", hint: "Undoing Game Mode's changes." },
};

/** The ring and everything that says whether Game Mode can run. */
export function GameModeMain({
  statuses,
  onConfigure,
}: {
  statuses: TweakStatus[];
  onConfigure: () => void;
}) {
  const games = useUiStore((s) => s.games);
  const chosen = useUiStore((s) => s.gameModeTweaks) ?? DEFAULT_GAME_MODE_TWEAKS;
  const snapshotId = useUiStore((s) => s.gameModeSnapshot);
  const setUi = useUiStore((s) => s.set);
  const { data: snapshots } = useSnapshots();
  const engine = useEngineStatus();
  const programs = useRunningPrograms();
  const { latest } = useLiveMetrics();
  const [reviewing, setReviewing] = useState(false);
  const [adding, setAdding] = useState(false);
  const [busy, setBusy] = useState(false);

  const game = detectGame(programs, games);
  const usable = statuses.filter((t) => chosen.includes(t.meta.id) && !isBlocked(t.compatibility));
  const open = usable.filter((t) => t.state !== "applied");
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;
  // Undone from the Restore Centre counts as off. The journal says so; nothing else has to.
  const batch = snapshots?.find((s) => s.id === snapshotId);
  const on = snapshotId !== null && (!batch || batch.status === "completed");

  const orb: OrbState = busy
    ? "busy"
    : on
      ? "on"
      : !game
        ? "idle"
        : usable.length === 0 || changesDisabled
          ? "blocked"
          : open.length === 0
            ? "inEffect"
            : "ready";

  const turnOff = async () => {
    if (snapshotId === null) return;
    setBusy(true);
    try {
      const outcome = await backend.revertSnapshot(snapshotId);
      const { tone, title, description } = describeOutcome(outcome, "revert");
      setUi({ gameModeSnapshot: null });
      if (tone !== "success") toast.warning(title, { description });
      else if (description) toast.info(description);
    } catch (err) {
      const { title, description } = describeError(toBackendError(err));
      toast.error(title, { description });
    } finally {
      refreshEngineState();
      setBusy(false);
    }
  };

  return (
    <>
      <ul className="grid grid-cols-4 divide-x pt-8 pb-6">
        {PROMISES.map(({ icon: Icon, colour, title, text }) => (
          <li key={title} className="flex flex-col gap-2 px-5 first:pl-0 last:pr-0">
            <div className="flex items-center gap-2 text-[13.5px] font-medium">
              <Icon className={cn("size-[18px] shrink-0", colour)} aria-hidden />
              {title}
            </div>
            <p className="text-[13px] leading-relaxed text-muted-foreground">{text}</p>
          </li>
        ))}
      </ul>

      <section className="flex flex-1 flex-col gap-6 panel p-8">
        <div className="grid flex-1 grid-cols-[minmax(0,1.25fr)_minmax(0,1fr)] items-center gap-8">
          <Orb
            state={orb}
            onClick={() => {
              if (orb === "on") void turnOff();
              else setReviewing(true);
            }}
          />
          <div className="relative flex min-w-0 flex-col">
            <DataSourceBadge source="live" className="absolute -top-2 right-0" />
            <Stat icon={Gamepad2} label="Detected game">
              <span className="block max-w-64 truncate" title={game?.name}>
                {game?.name ?? "N/A"}
              </span>
            </Stat>
            <Stat icon={Timer} label="Running for">
              {game ? formatUptime(game.runningSeconds) : "N/A"}
            </Stat>
            <Stat icon={Cpu} label="CPU">
              {latest ? `${formatPercent(latest.cpuPercent)} usage` : "N/A"}
            </Stat>
            <Stat icon={Gpu} label="GPU">
              {latest?.gpus[0]?.usagePercent != null
                ? `${formatPercent(latest.gpus[0].usagePercent)} usage`
                : "N/A"}
            </Stat>
            <Stat icon={Joystick} label={games.length ? `${games.length} added` : "Add games"}>
              <Button size="sm" onClick={() => setAdding(true)}>
                <Plus /> Add games
              </Button>
            </Stat>
          </div>
        </div>

        <Button variant="secondary" className="h-12 w-full" onClick={onConfigure}>
          <Settings /> Configure Game Mode
        </Button>
      </section>

      <AddGameDialog open={adding} onOpenChange={setAdding} />
      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={open.map((t) => t.meta.id)}
        names={Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.name]))}
        warnings={Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.warning]))}
        onApplied={(id) => setUi({ gameModeSnapshot: id })}
      />
    </>
  );
}

function Orb({ state, onClick }: { state: OrbState; onClick: () => void }) {
  const { label, hint } = ORB[state];
  const inert = state !== "ready" && state !== "on";

  return (
    <div className="relative isolate flex items-center justify-center justify-self-center">
      <span aria-hidden data-orb={state} className="game-orb-glow" />
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            type="button"
            aria-disabled={inert}
            // Radix closes a tooltip on press, and here the tooltip is the answer to the press.
            onPointerDown={(e) => inert && e.preventDefault()}
            onClick={(e) => {
              e.preventDefault();
              if (!inert) onClick();
            }}
            className={cn(
              "flex size-52 items-center justify-center rounded-full border-2 bg-card text-[1.35rem] font-semibold transition-[scale,background-color,border-color,color] duration-200 outline-none focus-visible:ring-2 focus-visible:ring-ring",
              state === "idle" && "cursor-default border-success/70 text-muted-foreground/70",
              state === "ready" &&
                "border-success text-foreground hover:scale-[1.03] active:scale-[0.98]",
              state === "on" &&
                "border-success bg-success/10 text-success hover:scale-[1.03] active:scale-[0.98]",
              state === "inEffect" && "cursor-default border-success text-success",
              (state === "blocked" || state === "busy") &&
                "cursor-default border-border text-muted-foreground",
            )}
          >
            {state === "busy" ? (
              <Loader2 className="size-7 animate-spin" aria-label={label} />
            ) : (
              label
            )}
          </button>
        </TooltipTrigger>
        <TooltipContent
          side="right"
          sideOffset={10}
          className="flex max-w-60 items-center gap-2.5 px-3 py-2 text-[12.5px] leading-snug"
        >
          <Gamepad2 className="size-4 shrink-0 text-muted-foreground" aria-hidden />
          {hint}
        </TooltipContent>
      </Tooltip>
    </div>
  );
}

function Stat({
  icon: Icon,
  label,
  children,
}: {
  icon: LucideIcon;
  label: string;
  children: ReactNode;
}) {
  return (
    <div className="flex min-h-17 items-center gap-3">
      <Icon className="size-5 shrink-0 text-muted-foreground" strokeWidth={1.5} aria-hidden />
      <span className="min-w-0 flex-1 truncate text-[14px] text-muted-foreground">{label}</span>
      <div className="shrink-0 text-[14px] font-semibold tabular">{children}</div>
    </div>
  );
}
