import { useCallback, useEffect, useState } from "react";
import { ChevronsDown, ChevronsUp, Clock, RotateCw, Timer, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { ErrorState } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { refreshTimer, useDeviceReport, useTimerHold } from "@/hooks/use-system";
import { backend, toBackendError } from "@/services/backend";
import type { SleepSample } from "@/types/system";
import { HintLine } from "@/components/common/hint-line";

const ms = (value: number | undefined) => (value == null ? "—" : value.toFixed(3));

function Reading({
  icon: Icon,
  label,
  value,
}: {
  icon: LucideIcon;
  label: string;
  value: number | undefined;
}) {
  return (
    <div data-stagger className="flex h-11 items-center gap-2.5 panel px-3.5 text-[13.5px]">
      <Icon className="size-4 text-brand" aria-hidden />
      <span className="text-muted-foreground">{label}</span>
      <span className="font-semibold tabular">{ms(value)}</span>
    </div>
  );
}

/** Lives and dies with this process, so there's nothing to back up or undo. */
function TimerHold() {
  const { data: held } = useTimerHold();
  const [busy, setBusy] = useState(false);

  const toggle = async (enabled: boolean) => {
    setBusy(true);
    try {
      await backend.setTimerHold(enabled);
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshTimer();
      setBusy(false);
    }
  };

  return (
    <CardGrid>
      <OptimisationCard
        icon={Timer}
        title="Hold the finest timer"
        description="Asks Windows for its finest timer while 2VX is open. It uses more power."
        indicators={<StatusLabel tone="neutral">This session only</StatusLabel>}
        control={
          <Switch
            aria-label="Hold the finest timer"
            checked={held === true}
            disabled={busy || held === undefined}
            onCheckedChange={(enabled) => void toggle(enabled)}
          />
        }
      />
    </CardGrid>
  );
}

export function TimerTab() {
  const { data, error, refetch } = useDeviceReport();
  const [samples, setSamples] = useState<SleepSample[] | null>(null);
  // The test runs when the tab opens, so it starts out running.
  const [running, setRunning] = useState(true);
  const timer = data?.timer ?? undefined;

  const measure = useCallback(
    () =>
      backend
        .measureSleep()
        .then(setSamples, (err: unknown) => toast.error(toBackendError(err).message))
        .finally(() => setRunning(false)),
    [],
  );
  useEffect(() => {
    void measure();
  }, [measure]);

  const again = () => {
    setRunning(true);
    void measure();
  };

  if (error) return <ErrorState message={error.message} onRetry={refetch} />;

  const average =
    samples && samples.reduce((sum, s) => sum + s.sleptMs, 0) / Math.max(samples.length, 1);

  return (
    <div className="flex flex-1 flex-col gap-3">
      <div className="flex items-center gap-2">
        <Reading icon={Clock} label="Current" value={timer?.currentMs} />
        <Reading icon={ChevronsUp} label="Max" value={timer?.coarsestMs} />
        <Reading icon={ChevronsDown} label="Min" value={timer?.finestMs} />
        <span className="ml-auto">
          <DataSourceBadge source="live" spelled />
        </span>
      </div>

      <TimerHold />

      <section data-stagger className="flex min-h-0 flex-1 flex-col overflow-hidden panel">
        <header className="flex h-14 shrink-0 items-center justify-between border-b px-5">
          <h2 className="text-[14.5px] font-semibold">Measure sleep</h2>
          <Button size="sm" variant="outline" disabled={running} onClick={again}>
            <RotateCw /> {running ? "Testing…" : "Test again"}
          </Button>
        </header>
        <div className="grid h-10 shrink-0 grid-cols-3 items-center border-b px-5 text-[12.5px] text-muted-foreground">
          <span>Resolution (ms)</span>
          <span className="text-center">Sleep (ms)</span>
          <span className="text-right">Delta (ms)</span>
        </div>
        {samples ? (
          <>
            <ul className="max-h-[260px] min-h-0 flex-1 divide-y overflow-y-auto">
              {samples.map((s, i) => (
                <li
                  key={i}
                  className="grid h-[26px] grid-cols-3 items-center px-5 text-[13px] tabular"
                >
                  <span>{ms(s.resolutionMs)}</span>
                  <span className="text-center">{ms(s.sleptMs)}</span>
                  <span className="text-right">{ms(s.deltaMs)}</span>
                </li>
              ))}
            </ul>
            <footer className="flex h-10 shrink-0 items-center justify-between border-t px-5 text-[13px]">
              <span className="text-muted-foreground">Average for a 1 ms sleep</span>
              <span className="font-semibold tabular">{ms(average ?? undefined)} ms</span>
            </footer>
          </>
        ) : (
          <p className="flex flex-1 items-center justify-center px-8 text-center text-[13px] text-muted-foreground">
            Sleeping 1 ms, 16 times
          </p>
        )}
      </section>

      <HintLine link={{ label: "Read Microsoft's note", to: "timerDocs" }}>
        Since Windows 10 2004 a timer request only counts for its own program, unless Global timer
        requests is on
      </HintLine>
    </div>
  );
}
