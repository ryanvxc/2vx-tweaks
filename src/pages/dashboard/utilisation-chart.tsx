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
import { percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { MetricsSample } from "@/types/system";

const SERIES = [
  { key: "cpu", label: "CPU", color: "var(--chart-1)" },
  { key: "gpu", label: "GPU", color: "var(--chart-3)" },
  { key: "ram", label: "Memory", color: "var(--chart-2)" },
] as const;

type View = "all" | (typeof SERIES)[number]["key"];
const VIEWS: { value: View; label: string }[] = [
  { value: "all", label: "All" },
  { value: "cpu", label: "CPU" },
  { value: "gpu", label: "GPU" },
  { value: "ram", label: "Memory" },
];

export function UtilisationChart({ samples }: { samples: MetricsSample[] }) {
  const [view, setView] = useState<View>("all");
  const shown = SERIES.filter((s) => view === "all" || s.key === view);
  const now = samples.at(-1)?.timestampMs ?? 0;
  const data = samples.map((s) => ({
    ago: Math.round((s.timestampMs - now) / 1000),
    cpu: round(s.cpuPercent),
    gpu: s.gpus[0]?.usagePercent != null ? round(s.gpus[0].usagePercent) : null,
    ram: round(percentOf(s.memoryUsedBytes, s.memoryTotalBytes)),
  }));

  return (
    <SectionCard
      title="Utilisation"
      description="Last 5 minutes, sampled every second"
      source="live"
      actions={
        <div role="radiogroup" aria-label="Series" className="flex rounded-md border p-0.5">
          {VIEWS.map((v) => {
            const series = SERIES.find((x) => x.key === v.value);
            return (
              <button
                key={v.value}
                type="button"
                role="radio"
                aria-checked={view === v.value}
                onClick={() => setView(v.value)}
                className={cn(
                  "flex h-6 items-center gap-1.5 rounded px-2 text-[11.5px] font-medium transition-colors",
                  view === v.value
                    ? "bg-accent text-foreground"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                {series && (
                  <span className="size-1.5 rounded-full" style={{ background: series.color }} />
                )}
                {v.label}
              </button>
            );
          })}
        </div>
      }
      contentClassName="pl-2"
    >
      {samples.length < 2 ? (
        <div className="px-3">
          <LoadingRows rows={5} />
        </div>
      ) : (
        <div className="h-60">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={data} margin={{ top: 6, right: 8, bottom: 0, left: 0 }}>
              <defs>
                {SERIES.map((s) => (
                  <linearGradient key={s.key} id={`util-${s.key}`} x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor={s.color} stopOpacity={0.22} />
                    <stop offset="100%" stopColor={s.color} stopOpacity={0} />
                  </linearGradient>
                ))}
              </defs>
              <CartesianGrid stroke="var(--border)" vertical={false} />
              <XAxis
                dataKey="ago"
                type="number"
                domain={[-299, 0]}
                ticks={[-240, -180, -120, -60, 0]}
                tickFormatter={(v: number) => (v === 0 ? "now" : `${-v / 60}m`)}
                tick={{ fill: "var(--muted-foreground)", fontSize: 11 }}
                axisLine={false}
                tickLine={false}
              />
              <YAxis
                domain={[0, 100]}
                ticks={[0, 25, 50, 75, 100]}
                tickFormatter={(v: number) => `${v}%`}
                tick={{ fill: "var(--muted-foreground)", fontSize: 11 }}
                axisLine={false}
                tickLine={false}
                width={40}
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
                formatter={(value, name) => [
                  value == null ? "—" : `${value}%`,
                  SERIES.find((s) => s.key === name)?.label ?? String(name),
                ]}
              />
              {shown.map((s) => (
                <Area
                  key={s.key}
                  type="monotone"
                  dataKey={s.key}
                  stroke={s.color}
                  strokeWidth={1.5}
                  fill={`url(#util-${s.key})`}
                  isAnimationActive={false}
                  dot={false}
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

function round(v: number) {
  return Math.round(v * 10) / 10;
}
