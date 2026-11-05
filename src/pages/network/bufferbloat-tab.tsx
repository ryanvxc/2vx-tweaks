import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  CartesianGrid,
  Line,
  LineChart,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { Play, Square } from "lucide-react";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Notice } from "@/components/common/notice";
import { Button } from "@/components/ui/button";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { formatRate } from "@/lib/format";
import { backend } from "@/services/backend";
import type { PingTarget } from "@/types/system";
import {
  analyse,
  BASELINE_SECONDS,
  formatAdded,
  formatLoss,
  formatMs,
  LOAD_MIN_SECONDS,
  phaseAt,
  TEST_SECONDS,
  type Reading,
  type TargetResult,
  type Tick,
} from "./bufferbloat";
import { HintLine } from "@/components/common/hint-line";

type RunState = "idle" | "running" | "done" | "stopped";

const GATEWAY_COLOUR = "var(--chart-2)";
const INTERNET_COLOUR = "var(--chart-1)";
const X_TICKS = [0, 10, 20, 30, 40];

/**
 * Pings the router and the internet once a second, ten seconds idle and then thirty while the
 * user loads the connection somewhere else. The app generates no traffic of its own. Leaving the
 * tab unmounts this, which stops the timer.
 */
function useBufferbloatTest(bytesPerSec: number | null) {
  const [ticks, setTicks] = useState<Tick[]>([]);
  const [state, setState] = useState<RunState>("idle");
  const [second, setSecond] = useState(0);
  const [noGateway, setNoGateway] = useState(false);
  const rate = useRef(bytesPerSec);
  const timer = useRef<ReturnType<typeof setInterval> | null>(null);
  // Bumped on every stop, so a ping that returns late is dropped.
  const run = useRef(0);

  useEffect(() => {
    rate.current = bytesPerSec;
  }, [bytesPerSec]);

  const halt = useCallback(() => {
    if (timer.current) clearInterval(timer.current);
    timer.current = null;
    run.current += 1;
  }, []);

  useEffect(() => halt, [halt]);

  const start = useCallback(() => {
    halt();
    const id = run.current;
    // A ping slower than a second is skipped on the next tick: never two in flight per target.
    const inFlight = { gateway: false, internet: false };
    let hasGateway = true;
    let next = 0;
    setTicks([]);
    setNoGateway(false);
    setSecond(0);
    setState("running");

    const ask = async (target: PingTarget): Promise<Reading> => {
      if (inFlight[target]) return undefined;
      inFlight[target] = true;
      try {
        const result = await backend.pingTarget(target);
        if (target === "gateway" && result.address === "") {
          hasGateway = false;
          if (id === run.current) setNoGateway(true);
          return undefined;
        }
        return result.rttMs;
      } catch {
        return null;
      } finally {
        inFlight[target] = false;
      }
    };

    const tick = () => {
      const s = next++;
      if (phaseAt(s) === "done") {
        if (timer.current) clearInterval(timer.current);
        timer.current = null;
        setState("done");
        return;
      }
      setSecond(s);
      const bytes = rate.current;
      void Promise.all([hasGateway ? ask("gateway") : undefined, ask("internet")]).then(
        ([gateway, internet]) => {
          if (id !== run.current) return;
          setTicks((t) => [...t, { second: s, gateway, internet, bytesPerSec: bytes }]);
        },
      );
    };

    timer.current = setInterval(tick, 1000);
    tick();
  }, [halt]);

  const stop = useCallback(() => {
    halt();
    setState("stopped");
  }, [halt]);

  return { ticks, state, second, noGateway, start, stop };
}

export function BufferbloatTab() {
  const { latest } = useLiveMetrics();
  const bytesPerSec = latest ? latest.networkRxBytesPerSec + latest.networkTxBytesPerSec : null;
  const { ticks, state, second, noGateway, start, stop } = useBufferbloatTest(bytesPerSec);
  const running = state === "running";
  const result = useMemo(() => (state === "done" ? analyse(ticks) : null), [state, ticks]);
  const last = ticks.at(-1);
  const left = Math.max(TEST_SECONDS - second, 0);
  const phaseLeft = Math.max(BASELINE_SECONDS - second, 0);

  const status = running
    ? phaseAt(second) === "baseline"
      ? `Measuring idle latency, ${phaseLeft} s left. Keep other traffic off the connection`
      : `Run a speed test or a download now, ${left} s left`
    : state === "done"
      ? "Finished"
      : state === "stopped"
        ? "Stopped, no result"
        : `Pings your router and the internet once a second for ${TEST_SECONDS} seconds`;

  return (
    <div className="flex flex-1 flex-col gap-3">
      <section data-stagger className="flex h-16 items-center gap-4 panel px-5">
        <div className="min-w-0 flex-1">
          <h2 className="text-[14.5px] font-semibold">Latency under load</h2>
          <p data-selectable className="truncate text-[13px] text-muted-foreground">
            {status}
          </p>
        </div>
        <DataSourceBadge source="live" />
        {running ? (
          <Button variant="outline" onClick={stop}>
            <Square /> Stop
          </Button>
        ) : (
          <Button onClick={start}>
            <Play /> {state === "idle" ? "Start" : "Run again"}
          </Button>
        )}
      </section>

      {noGateway && (
        <Notice tone="info" title="No default gateway found">
          Only the internet is measured
        </Notice>
      )}

      <section
        data-stagger
        className="flex min-h-[220px] flex-1 flex-col overflow-hidden panel"
        aria-label="Round-trip time"
      >
        <header className="flex h-11 shrink-0 items-center gap-5 border-b px-5 text-[13px]">
          {!noGateway && <Legend colour={GATEWAY_COLOUR} label="Router" value={last?.gateway} />}
          <Legend colour={INTERNET_COLOUR} label="Internet" value={last?.internet} />
          <span className="ml-auto text-muted-foreground">
            Traffic{" "}
            <span className="font-medium text-foreground tabular">
              {bytesPerSec == null ? "—" : formatRate(bytesPerSec)}
            </span>
          </span>
        </header>
        <div className="relative min-h-0 flex-1">
          {ticks.length === 0 ? (
            <p className="absolute inset-0 flex items-center justify-center text-[13px] text-muted-foreground">
              {running ? "Waiting for the first reply" : "Start the test to chart round-trip time"}
            </p>
          ) : (
            <div className="absolute inset-y-0 right-3 left-1 pt-2">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={ticks} margin={{ top: 8, right: 24, bottom: 2, left: 0 }}>
                  <CartesianGrid stroke="var(--border)" strokeDasharray="3 6" vertical={false} />
                  <XAxis
                    dataKey="second"
                    type="number"
                    domain={[0, TEST_SECONDS]}
                    ticks={X_TICKS}
                    interval={0}
                    tickFormatter={(v: number) => `${v} s`}
                    tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                    axisLine={false}
                    tickLine={false}
                    tickMargin={8}
                  />
                  <YAxis
                    domain={[0, "auto"]}
                    tickFormatter={(v: number) => `${v} ms`}
                    tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                    axisLine={false}
                    tickLine={false}
                    width={56}
                  />
                  <ReferenceLine
                    x={BASELINE_SECONDS}
                    stroke="var(--muted-foreground)"
                    strokeOpacity={0.4}
                    strokeDasharray="4 4"
                    label={{
                      value: "Load",
                      position: "insideTopRight",
                      fill: "var(--muted-foreground)",
                      fontSize: 12,
                    }}
                  />
                  <Tooltip
                    cursor={{ stroke: "var(--muted-foreground)", strokeOpacity: 0.3 }}
                    contentStyle={{
                      background: "var(--popover)",
                      border: "1px solid var(--border)",
                      borderRadius: 8,
                      fontSize: 12,
                    }}
                    labelFormatter={(v) => `${v} s`}
                    formatter={(value, name) => [value == null ? "No reply" : `${value} ms`, name]}
                  />
                  {!noGateway && (
                    <Line
                      type="monotone"
                      dataKey="gateway"
                      name="Router"
                      stroke={GATEWAY_COLOUR}
                      strokeWidth={2}
                      dot={false}
                      isAnimationActive={false}
                    />
                  )}
                  <Line
                    type="monotone"
                    dataKey="internet"
                    name="Internet"
                    stroke={INTERNET_COLOUR}
                    strokeWidth={2}
                    dot={false}
                    isAnimationActive={false}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          )}
        </div>
      </section>

      {result &&
        (result.loadReached ? (
          <section data-stagger className="overflow-hidden panel" aria-label="Result">
            <div className={`${COLUMNS} h-9 border-b text-[12.5px] text-muted-foreground`}>
              <span>Target</span>
              <span className="text-right">Idle</span>
              <span className="text-right">Loaded</span>
              <span className="text-right">Added</span>
              <span className="text-right">Loss under load</span>
            </div>
            <ul className="divide-y">
              {!noGateway && <ResultRow label="Router" result={result.gateway} />}
              <ResultRow label="Internet" result={result.internet} />
            </ul>
            <p className="flex h-10 items-center border-t px-5 text-[12.5px] text-muted-foreground">
              Traffic peaked at {formatRate(result.peakBytesPerSec)}, above idle for{" "}
              {result.loadedSeconds} s
            </p>
          </section>
        ) : (
          <Notice tone="warning" title="The load phase didn't reach the connection">
            Traffic peaked at {formatRate(result.peakBytesPerSec)} and stayed above idle for{" "}
            {result.loadedSeconds} s, short of the {LOAD_MIN_SECONDS} needed. Run a larger download
            or speed test during the load phase and try again
          </Notice>
        ))}

      <HintLine link={{ label: "See what works", to: "bufferbloatFixes" }}>
        The queue sits on your router, so a Windows setting can't shorten it
      </HintLine>
    </div>
  );
}

const COLUMNS = "grid grid-cols-[1.4fr_1fr_1fr_1fr_1.2fr] items-center gap-4 px-5";

function ResultRow({ label, result }: { label: string; result: TargetResult | null }) {
  return (
    <li className={`${COLUMNS} h-11 text-[13.5px] tabular`}>
      <span className="font-medium">{label}</span>
      <span className="text-right">{formatMs(result?.idleMs)}</span>
      <span className="text-right">{formatMs(result?.loadedMs)}</span>
      <span className="text-right font-semibold">{formatAdded(result?.addedMs)}</span>
      <span className="text-right">{formatLoss(result?.loadedLossPercent)}</span>
    </li>
  );
}

function Legend({
  colour,
  label,
  value,
}: {
  colour: string;
  label: string;
  value: number | null | undefined;
}) {
  return (
    <span className="flex items-center gap-2">
      <span aria-hidden className="size-2 rounded-full" style={{ background: colour }} />
      {label}
      <span className="text-muted-foreground tabular">
        {value === undefined ? "—" : value === null ? "No reply" : formatMs(value)}
      </span>
    </span>
  );
}
