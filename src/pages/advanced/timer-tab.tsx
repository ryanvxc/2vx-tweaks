import { useState } from "react";
import { ChevronsDown, ChevronsUp, Clock, Play, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { ActionCard } from "@/components/common/action-card";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { ErrorState } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useDeviceReport } from "@/hooks/use-system";
import { backend, toBackendError } from "@/services/backend";
import type { SleepSample } from "@/types/system";

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

export function TimerTab() {
  const { data, error, refetch } = useDeviceReport();
  const [samples, setSamples] = useState<SleepSample[] | null>(null);
  const [running, setRunning] = useState(false);
  const timer = data?.timer ?? undefined;

  const run = async () => {
    setRunning(true);
    try {
      setSamples(await backend.measureSleep());
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      setRunning(false);
    }
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
          <DataSourceBadge source="live" />
        </span>
      </div>

      <div className="grid flex-1 grid-cols-[3fr_2fr] gap-3">
        <section data-stagger className="flex min-h-0 flex-col overflow-hidden panel">
          <header className="flex h-14 shrink-0 items-center justify-between border-b px-5">
            <h2 className="text-[14.5px] font-semibold">Measure sleep</h2>
            <Button size="sm" variant="outline" disabled={running} onClick={() => void run()}>
              <Play /> {running ? "Testing…" : "Start test"}
            </Button>
          </header>
          <div className="grid h-10 shrink-0 grid-cols-3 items-center border-b px-5 text-[12.5px] text-muted-foreground">
            <span>Resolution</span>
            <span className="text-center">Sleep</span>
            <span className="text-right">Delta</span>
          </div>
          {samples ? (
            <>
              <ul className="min-h-0 flex-1 divide-y overflow-y-auto">
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
              Sleeps for 1 ms 16 times and times each one, as a program that hasn't asked for a
              finer timer would.
            </p>
          )}
        </section>

        <ActionCard
          icon={Clock}
          title="Set timer resolution"
          description="Since Windows 10 version 2004, a timer request only counts for the program that made it. 2VX can't set one for your games, so there's nothing to set here. Games that need a finer timer ask for it themselves."
          action={{ label: "Read Microsoft's note", to: "timerDocs" }}
        />
      </div>
    </div>
  );
}
