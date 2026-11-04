import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useState } from "react";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { useSlidingIndicator } from "@/hooks/use-motion";
import { formatBytes } from "@/lib/format";
import { usagePoints, type UsagePoint } from "@/lib/usage-points";
import { cn } from "@/lib/utils";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

const VIEWS = [
  { value: "all", label: "All" },
  { value: "cpu", label: "CPU" },
  { value: "gpu", label: "GPU" },
  { value: "ram", label: "RAM" },
] as const;

type View = (typeof VIEWS)[number]["value"];
type Device = Exclude<View, "all">;

// Each device keeps its colour in every view, so a line means the same thing on All.
const SERIES: { key: Device; label: string; colour: string }[] = [
  { key: "cpu", label: "CPU", colour: "var(--chart-1)" },
  { key: "gpu", label: "GPU", colour: "var(--chart-2)" },
  { key: "ram", label: "RAM", colour: "var(--chart-3)" },
];

const WINDOW_SECONDS = 60;

export function UtilisationChart({
  samples,
  snapshot,
}: {
  samples: MetricsSample[];
  snapshot: SystemSnapshot | undefined;
}) {
  const [view, setView] = useState<View>("all");
  const { container: views, indicator: pill } =
    useSlidingIndicator<HTMLDivElement>('[aria-checked="true"]');
  const data = usagePoints(samples);
  const shown = SERIES.filter((s) => view === "all" || s.key === view);

  return (
    <SectionCard
      title={
        view === "all" ? <SeriesKey now={data.at(-1)} /> : viewTitle(view, samples.at(-1), snapshot)
      }
      source="live"
      className="h-full"
      headerClassName="border-b items-center [&>[data-slot=card-title]]:row-span-2"
      actions={
        <div
          ref={views}
          role="radiogroup"
          aria-label="Device"
          className="relative isolate flex rounded-lg border bg-background/40 p-0.5"
        >
          <span
            ref={pill}
            aria-hidden
            className="absolute top-0 left-0 -z-10 h-(--h) w-(--w) translate-x-(--x) translate-y-(--y) rounded-md bg-accent transition-[translate,width,height,opacity] duration-220"
          />
          {VIEWS.map((v) => (
            <button
              key={v.value}
              type="button"
              role="radio"
              aria-checked={view === v.value}
              onClick={() => setView(v.value)}
              className={cn(
                "flex h-7 items-center rounded-md px-3.5 text-[13px] font-medium transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring",
                view === v.value
                  ? "text-foreground"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              {v.label}
            </button>
          ))}
        </div>
      }
      contentClassName="relative min-h-[180px]"
    >
      {samples.length < 2 ? (
        <LoadingRows rows={5} />
      ) : (
        // Absolute so the graph takes whatever height the row gives it.
        // Keyed so choosing a device draws it in, while each new sample just updates.
        <div key={view} className="absolute inset-y-0 right-3 left-1 animate-reveal">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={data} margin={{ top: 8, right: 30, bottom: 2, left: 0 }}>
              <defs>
                {shown.map((s) => (
                  <linearGradient key={s.key} id={`util-${s.key}`} x1="0" y1="0" x2="0" y2="1">
                    {/* Three full fills would hide each other. */}
                    <stop
                      offset="0%"
                      stopColor={s.colour}
                      stopOpacity={view === "all" ? 0.14 : 0.5}
                    />
                    <stop offset="100%" stopColor={s.colour} stopOpacity={0.02} />
                  </linearGradient>
                ))}
              </defs>
              <CartesianGrid stroke="var(--border)" strokeDasharray="3 6" vertical={false} />
              <XAxis
                dataKey="ago"
                type="number"
                domain={[-WINDOW_SECONDS, 0]}
                ticks={[-60, -45, -30, -15, 0]}
                interval={0}
                tickFormatter={(v: number) => `${Math.abs(v)} seconds`}
                tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
                tickMargin={8}
              />
              <YAxis
                domain={[0, 100]}
                ticks={[0, 20, 40, 60, 80, 100]}
                tickFormatter={(v: number) => `${v}%`}
                tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
                width={48}
              />
              <Tooltip
                cursor={{ stroke: "var(--muted-foreground)", strokeOpacity: 0.3 }}
                contentStyle={{
                  background: "var(--popover)",
                  border: "1px solid var(--border)",
                  borderRadius: 8,
                  fontSize: 12,
                }}
                labelFormatter={(v) => (v === 0 ? "now" : `${-Number(v)}s ago`)}
                formatter={(value, name) => [value == null ? "—" : `${value}%`, name]}
              />
              {shown.map((s) => (
                <Area
                  key={s.key}
                  type="monotone"
                  dataKey={s.key}
                  name={s.label}
                  stroke={s.colour}
                  strokeWidth={2}
                  fill={`url(#util-${s.key})`}
                  isAnimationActive={false}
                  dot={(dot) =>
                    dot.index === data.length - 1 ? (
                      <NowDot key="now" {...dot} colour={s.colour} />
                    ) : (
                      <g key={dot.index} />
                    )
                  }
                  activeDot={{ r: 4, fill: s.colour, stroke: "none" }}
                  connectNulls
                />
              ))}
            </AreaChart>
          </ResponsiveContainer>
        </div>
      )}
    </SectionCard>
  );
}

/** Marks the newest reading, which is the only point on the line that's still moving. */
function NowDot({ cx, cy, colour }: { cx?: number; cy?: number; colour: string }) {
  if (cx == null || cy == null) return null;
  return (
    <g>
      <circle cx={cx} cy={cy} r={4} fill={colour} className="now-ping" />
      <circle cx={cx} cy={cy} r={4} fill={colour} />
    </g>
  );
}

/** With three lines on the graph, the title says which is which and where each one is now. */
function SeriesKey({ now }: { now: UsagePoint | undefined }) {
  return (
    <span className="flex items-center gap-5">
      {SERIES.map((s) => {
        const value = now?.[s.key];
        return (
          <span key={s.key} className="flex items-center gap-2">
            <span aria-hidden className="size-2 rounded-full" style={{ background: s.colour }} />
            {s.label}
            <span className="font-normal text-muted-foreground tabular">
              {value == null ? "—" : `${Math.round(value)}%`}
            </span>
          </span>
        );
      })}
    </span>
  );
}

/** Names the device being plotted: the chart follows the first GPU in the samples. */
function viewTitle(
  view: Device,
  latest: MetricsSample | undefined,
  snapshot: SystemSnapshot | undefined,
) {
  switch (view) {
    case "cpu":
      return snapshot?.cpu.brand.trim() || "CPU";
    case "gpu":
      return latest?.gpus[0]?.model || snapshot?.gpus[0]?.model || "GPU";
    case "ram":
      // Windows reports no memory module name through the APIs 2VX reads, so show the capacity.
      return latest ? `${formatBytes(latest.memoryTotalBytes)} system memory` : "Memory";
  }
}
