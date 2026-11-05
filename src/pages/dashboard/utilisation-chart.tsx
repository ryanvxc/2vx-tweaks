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
import { formatBytes, formatPercent, formatRate, percentOf } from "@/lib/format";
import { RANGES, samplesIn, usagePoints, type Range, type UsagePoint } from "@/lib/usage-points";
import { cn } from "@/lib/utils";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

const VIEWS = [
  { value: "all", label: "All" },
  { value: "cpu", label: "CPU" },
  { value: "gpu", label: "GPU" },
  { value: "ram", label: "RAM" },
  { value: "network", label: "Network" },
] as const;

const RANGE_VIEWS = [
  { value: "1m", label: "1m" },
  { value: "5m", label: "5m" },
] as const;

type View = (typeof VIEWS)[number]["value"];
type Device = "cpu" | "gpu" | "ram";

interface Series {
  key: keyof UsagePoint;
  label: string;
  colour: string;
}

// Each device keeps its colour in every view, so a line means the same thing on All.
const SERIES: (Series & { key: Device })[] = [
  { key: "cpu", label: "CPU", colour: "var(--chart-1)" },
  { key: "gpu", label: "GPU", colour: "var(--chart-2)" },
  { key: "ram", label: "RAM", colour: "var(--chart-3)" },
];

// Download and upload share one scale in bytes, apart from the percentage lines.
const NETWORK: Series[] = [
  { key: "rx", label: "Download", colour: "var(--chart-2)" },
  { key: "tx", label: "Upload", colour: "var(--chart-5)" },
];

const TICKS: Record<Range, number[]> = {
  "1m": [-60, -45, -30, -15, 0],
  "5m": [-300, -240, -180, -120, -60, 0],
};

const tickLabel = (range: Range, v: number) =>
  range === "1m" ? `${Math.abs(v)} seconds` : v === 0 ? "now" : `${Math.abs(v) / 60} min`;

const agoLabel = (v: number) => {
  if (v === 0) return "now";
  const s = Math.abs(Math.round(v));
  return s < 60 ? `${s}s ago` : `${Math.floor(s / 60)}m ${s % 60}s ago`;
};

export function UtilisationChart({
  samples,
  snapshot,
}: {
  samples: MetricsSample[];
  snapshot: SystemSnapshot | undefined;
}) {
  const [view, setView] = useState<View>("all");
  const [range, setRange] = useState<Range>("1m");
  const data = usagePoints(samples, range);
  const network = view === "network";
  const shown: Series[] = network
    ? NETWORK
    : SERIES.filter((s) => view === "all" || s.key === view);

  return (
    <SectionCard
      title={
        view === "all" ? <SeriesKey now={data.at(-1)} /> : viewTitle(view, samples.at(-1), snapshot)
      }
      description={view === "all" ? undefined : viewStats(view, samplesIn(samples, range))}
      source="live"
      className="h-full"
      headerClassName="border-b items-center [&>[data-slot=card-title]]:row-span-2"
      actions={
        <>
          <Segmented label="Device" options={VIEWS} value={view} onChange={setView} />
          <Segmented label="Range" options={RANGE_VIEWS} value={range} onChange={setRange} />
        </>
      }
      contentClassName="relative min-h-[180px]"
    >
      {samples.length < 2 ? (
        <LoadingRows rows={5} />
      ) : (
        // Absolute so the graph takes whatever height the row gives it.
        // Keyed so choosing a device or range draws it in, while each new sample just updates.
        <div key={`${view}-${range}`} className="absolute inset-y-0 right-3 left-1 animate-reveal">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={data} margin={{ top: 8, right: 30, bottom: 2, left: 0 }}>
              <defs>
                {shown.map((s) => (
                  <linearGradient key={s.key} id={`util-${s.key}`} x1="0" y1="0" x2="0" y2="1">
                    {/* Several full fills would hide each other. */}
                    <stop
                      offset="0%"
                      stopColor={s.colour}
                      stopOpacity={shown.length > 1 ? 0.14 : 0.5}
                    />
                    <stop offset="100%" stopColor={s.colour} stopOpacity={0.02} />
                  </linearGradient>
                ))}
              </defs>
              <CartesianGrid stroke="var(--border)" strokeDasharray="3 6" vertical={false} />
              <XAxis
                dataKey="ago"
                type="number"
                domain={[-RANGES[range].seconds, 0]}
                ticks={TICKS[range]}
                interval={0}
                tickFormatter={(v: number) => tickLabel(range, v)}
                tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
                tickMargin={8}
              />
              <YAxis
                domain={network ? [0, "auto"] : [0, 100]}
                ticks={network ? undefined : [0, 20, 40, 60, 80, 100]}
                tickFormatter={(v: number) => (network ? formatRate(v) : `${v}%`)}
                tick={{ fill: "var(--muted-foreground)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
                width={network ? 76 : 48}
              />
              <Tooltip
                cursor={{ stroke: "var(--muted-foreground)", strokeOpacity: 0.3 }}
                contentStyle={{
                  background: "var(--popover)",
                  border: "1px solid var(--border)",
                  borderRadius: 8,
                  fontSize: 12,
                }}
                labelFormatter={(v) => agoLabel(Number(v))}
                formatter={(value, name) => [
                  value == null ? "—" : network ? formatRate(Number(value)) : `${value}%`,
                  name,
                ]}
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

/** A small radio group with a sliding highlight, used for the device and the range. */
function Segmented<T extends string>({
  label,
  options,
  value,
  onChange,
}: {
  label: string;
  options: readonly { value: T; label: string }[];
  value: T;
  onChange: (value: T) => void;
}) {
  const { container, indicator } = useSlidingIndicator<HTMLDivElement>('[aria-checked="true"]');
  return (
    <div
      ref={container}
      role="radiogroup"
      aria-label={label}
      className="relative isolate flex rounded-lg border bg-background/40 p-0.5"
    >
      <span
        ref={indicator}
        aria-hidden
        className="absolute top-0 left-0 -z-10 h-(--h) w-(--w) translate-x-(--x) translate-y-(--y) rounded-md bg-accent transition-[translate,width,height,opacity] duration-220"
      />
      {options.map((o) => (
        <button
          key={o.value}
          type="button"
          role="radio"
          aria-checked={value === o.value}
          onClick={() => onChange(o.value)}
          className={cn(
            "flex h-7 items-center rounded-md px-2 text-[13px] font-medium whitespace-nowrap transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring",
            value === o.value ? "text-foreground" : "text-muted-foreground hover:text-foreground",
          )}
        >
          {o.label}
        </button>
      ))}
    </div>
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
    <span className="flex items-center gap-3">
      {SERIES.map((s) => {
        const value = now?.[s.key];
        return (
          <span key={s.key} className="flex items-center gap-1.5">
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
  view: Exclude<View, "all">,
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
    case "network":
      return "All adapters";
  }
}

const average = (values: number[]) => values.reduce((a, b) => a + b, 0) / values.length;

/** The reading now, with the average and peak over the range on screen, in one line. */
function viewStats(view: Exclude<View, "all">, shown: MetricsSample[]) {
  const latest = shown.at(-1);
  if (!latest) return undefined;
  const spread = (values: number[], format: (v: number) => string, noun = "") =>
    `average ${noun}${format(average(values))} · peak ${noun}${format(Math.max(...values))}`;

  switch (view) {
    case "cpu":
      return `Now ${formatPercent(latest.cpuPercent)} · ${spread(
        shown.map((s) => s.cpuPercent),
        formatPercent,
      )}`;
    case "gpu": {
      const values = shown.flatMap((s) => s.gpus[0]?.usagePercent ?? []);
      const now = latest.gpus[0]?.usagePercent;
      return values.length === 0 || now == null
        ? "Windows gave no GPU reading"
        : `Now ${formatPercent(now)} · ${spread(values, formatPercent)}`;
    }
    case "ram": {
      const values = shown.map((s) => percentOf(s.memoryUsedBytes, s.memoryTotalBytes));
      return `Now ${formatBytes(latest.memoryUsedBytes)} (${formatPercent(values.at(-1))}) · ${spread(values, formatPercent)}`;
    }
    case "network":
      return `Down ${formatRate(latest.networkRxBytesPerSec)} · up ${formatRate(latest.networkTxBytesPerSec)} · ${spread(
        shown.map((s) => s.networkRxBytesPerSec),
        formatRate,
        "down ",
      )}`;
  }
}
