import type { ReactNode } from "react";
import type { MonitoringTab } from "@/app/navigation";
import { ArrowDown, ArrowUp, Cpu, Gpu, MemoryStick, Network, type LucideIcon } from "lucide-react";
import { AnimatedNumber } from "@/components/common/animated-number";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Sparkline } from "@/components/common/sparkline";
import { Skeleton } from "@/components/ui/skeleton";
import { useSlidingIndicator } from "@/hooks/use-motion";
import { formatBytes, formatPercent, formatRate, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

const SPARK_POINTS = 60;
// Shorter than the one-second sample, so each reading lands before the next arrives.
const TICK_MS = 450;

const tick = (value: number, format: (v: number) => string) => (
  <AnimatedNumber value={value} format={format} duration={TICK_MS} />
);

/** Four live numbers that also choose the graph below them, like a tab row with data. */
export function LiveStats({
  samples,
  snapshot,
  selected,
  onSelect,
}: {
  samples: MetricsSample[];
  snapshot: SystemSnapshot | undefined;
  selected: MonitoringTab;
  onSelect: (tab: MonitoringTab) => void;
}) {
  const recent = samples.slice(-SPARK_POINTS);
  const latest = recent.at(-1);
  const gpu = latest?.gpus[0];
  const physical = snapshot?.cpu.physicalCores;
  const { container: tiles, indicator: outline } =
    useSlidingIndicator<HTMLDivElement>('[aria-selected="true"]');

  return (
    <div ref={tiles} role="tablist" aria-label="Metric" className="relative grid grid-cols-4 gap-3">
      <StatTile
        selected={selected === "cpu"}
        onSelect={() => onSelect("cpu")}
        icon={Cpu}
        label="CPU"
        value={latest ? tick(latest.cpuPercent, formatPercent) : undefined}
        sub={
          snapshot ? `${physical ?? "?"} cores · ${snapshot.cpu.logicalCores} threads` : undefined
        }
        color="var(--chart-1)"
        series={recent.map((s) => s.cpuPercent)}
      />
      <StatTile
        selected={selected === "memory"}
        onSelect={() => onSelect("memory")}
        icon={MemoryStick}
        label="Memory"
        value={
          latest
            ? tick(percentOf(latest.memoryUsedBytes, latest.memoryTotalBytes), formatPercent)
            : undefined
        }
        sub={
          latest
            ? `${formatBytes(latest.memoryUsedBytes)} of ${formatBytes(latest.memoryTotalBytes)}`
            : undefined
        }
        color="var(--chart-2)"
        series={recent.map((s) => percentOf(s.memoryUsedBytes, s.memoryTotalBytes))}
      />
      <StatTile
        selected={selected === "gpu"}
        onSelect={() => onSelect("gpu")}
        icon={Gpu}
        label="GPU"
        value={
          latest
            ? gpu?.usagePercent != null
              ? tick(gpu.usagePercent, formatPercent)
              : "—"
            : undefined
        }
        sub={
          latest
            ? gpu
              ? `${formatBytes(gpu.memoryTotalBytes, 0)} VRAM`
              : "No GPU reported"
            : undefined
        }
        color="var(--chart-3)"
        series={recent.map((s) => s.gpus[0]?.usagePercent ?? 0)}
      />
      <StatTile
        selected={selected === "network"}
        onSelect={() => onSelect("network")}
        icon={Network}
        label="Network"
        value={latest ? tick(latest.networkRxBytesPerSec, formatRate) : undefined}
        valueIcon={<ArrowDown className="size-4 text-muted-foreground" />}
        sub={
          latest ? (
            <span className="inline-flex items-center gap-1">
              <ArrowUp className="size-3" /> {formatRate(latest.networkTxBytesPerSec)}
            </span>
          ) : undefined
        }
        color="var(--chart-4)"
        series={recent.map((s) => s.networkRxBytesPerSec)}
        max="auto"
      />
      <span
        ref={outline}
        aria-hidden
        className="pointer-events-none absolute top-0 left-0 h-(--h) w-(--w) translate-x-(--x) translate-y-(--y) rounded-xl border border-primary/50 transition-[translate,width,height,opacity] duration-220"
      />
    </div>
  );
}

function StatTile({
  selected,
  onSelect,
  icon: Icon,
  label,
  value,
  valueIcon,
  sub,
  color,
  series,
  max = 100,
}: {
  selected: boolean;
  onSelect: () => void;
  icon: LucideIcon;
  label: string;
  value: ReactNode;
  valueIcon?: ReactNode;
  sub: ReactNode | undefined;
  color: string;
  series: number[];
  max?: number | "auto";
}) {
  return (
    <button
      type="button"
      role="tab"
      aria-selected={selected}
      onClick={onSelect}
      className={cn(
        "flex flex-col overflow-hidden panel pt-3 text-left transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring",
        selected && "bg-raised",
      )}
    >
      <div className="flex w-full items-center justify-between px-3.5">
        <div className="flex items-center gap-2 text-[12.5px] font-medium text-muted-foreground">
          <Icon className="size-4" style={{ color }} />
          {label}
        </div>
        <DataSourceBadge source="live" pulse={selected} />
      </div>
      <div className="mt-1.5 flex w-full items-baseline justify-between gap-2 px-3.5">
        {value === undefined ? (
          <Skeleton className="h-6 w-16" />
        ) : (
          <div className="flex shrink-0 items-center gap-1">
            {valueIcon}
            <span className="text-[21px] leading-tight font-semibold tracking-tight tabular">
              {value}
            </span>
          </div>
        )}
        <div className="min-w-0 truncate text-[12px] text-muted-foreground tabular">
          {sub ?? <Skeleton className="h-3 w-20" />}
        </div>
      </div>
      <div className="mt-1 w-full">
        <Sparkline data={series} color={color} max={max} height={26} />
      </div>
    </button>
  );
}
