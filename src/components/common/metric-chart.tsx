import {
  Area,
  AreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { LoadingRows } from "./states";

export interface ChartSeries {
  key: string;
  label: string;
  color: string;
}

/**
 * The standard time-series graph: x is seconds before the newest sample, y is a percentage
 * (default) or any unit via `format`. Rows are `{ ago, [series.key]: number | null }`.
 */
export function MetricChart({
  rows,
  series,
  height = 220,
  max = 100,
  format = (v) => `${v}%`,
  windowSecs = 300,
}: {
  rows: Record<string, number | null>[];
  series: ChartSeries[];
  height?: number;
  /** `undefined` scales to the data. */
  max?: number;
  format?: (v: number) => string;
  windowSecs?: number;
}) {
  if (rows.length < 2) return <LoadingRows rows={4} />;
  const id = series.map((s) => s.key).join("-");
  return (
    <div style={{ height }} className="animate-reveal">
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart data={rows} margin={{ top: 6, right: 8, bottom: 0, left: 0 }}>
          <defs>
            {series.map((s) => (
              <linearGradient key={s.key} id={`mc-${id}-${s.key}`} x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={s.color} stopOpacity={0.25} />
                <stop offset="100%" stopColor={s.color} stopOpacity={0} />
              </linearGradient>
            ))}
          </defs>
          <CartesianGrid stroke="var(--border)" vertical={false} />
          <XAxis
            dataKey="ago"
            type="number"
            domain={[-(windowSecs - 1), 0]}
            tickFormatter={(v: number) => (v === 0 ? "now" : `${Math.round(-v / 60)}m`)}
            ticks={[-240, -180, -120, -60, 0].filter((t) => t > -windowSecs)}
            tick={{ fill: "var(--muted-foreground)", fontSize: 11 }}
            axisLine={false}
            tickLine={false}
          />
          <YAxis
            domain={[0, max ?? "auto"]}
            tickFormatter={format}
            tick={{ fill: "var(--muted-foreground)", fontSize: 11 }}
            axisLine={false}
            tickLine={false}
            width={max === 100 ? 40 : 64}
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
              value == null ? "—" : format(Number(value)),
              series.find((s) => s.key === name)?.label ?? String(name),
            ]}
          />
          {series.map((s) => (
            <Area
              key={s.key}
              type="monotone"
              dataKey={s.key}
              stroke={s.color}
              strokeWidth={1.5}
              fill={`url(#mc-${id}-${s.key})`}
              isAnimationActive={false}
              dot={false}
              connectNulls
            />
          ))}
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
