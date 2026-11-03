import type { ReactNode } from "react";
import { ArrowDown, ArrowUp, Cpu, Gpu, MemoryStick, Network, type LucideIcon } from "lucide-react";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Sparkline } from "@/components/common/sparkline";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { formatBytes, formatPercent, formatRate, percentOf } from "@/lib/format";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

const SPARK_POINTS = 60;

export function LiveStats({
  samples,
  snapshot,
}: {
  samples: MetricsSample[];
  snapshot: SystemSnapshot | undefined;
}) {
  const recent = samples.slice(-SPARK_POINTS);
  const latest = recent.at(-1);
  const gpu = latest?.gpus[0];
  const physical = snapshot?.cpu.physicalCores;

  return (
    <div className="grid grid-cols-4 gap-4">
      <StatTile
        icon={Cpu}
        label="CPU"
        value={latest ? formatPercent(latest.cpuPercent) : undefined}
        sub={
          snapshot ? `${physical ?? "?"} cores · ${snapshot.cpu.logicalCores} threads` : undefined
        }
        color="var(--chart-1)"
        series={recent.map((s) => s.cpuPercent)}
      />
      <StatTile
        icon={MemoryStick}
        label="Memory"
        value={
          latest
            ? formatPercent(percentOf(latest.memoryUsedBytes, latest.memoryTotalBytes))
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
        icon={Gpu}
        label="GPU"
        value={
          latest ? (gpu?.usagePercent != null ? formatPercent(gpu.usagePercent) : "—") : undefined
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
        icon={Network}
        label="Network"
        value={latest ? formatRate(latest.networkRxBytesPerSec) : undefined}
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
    </div>
  );
}

function StatTile({
  icon: Icon,
  label,
  value,
  valueIcon,
  sub,
  color,
  series,
  max = 100,
}: {
  icon: LucideIcon;
  label: string;
  value: string | undefined;
  valueIcon?: ReactNode;
  sub: ReactNode | undefined;
  color: string;
  series: number[];
  max?: number | "auto";
}) {
  return (
    <Card className="gap-0 overflow-hidden pt-4 pb-0">
      <div className="flex items-center justify-between px-4">
        <div className="flex items-center gap-2 text-[12.5px] font-medium text-muted-foreground">
          <Icon className="size-4" style={{ color }} />
          {label}
        </div>
        <DataSourceBadge source="live" />
      </div>
      <div className="mt-3 px-4">
        {value === undefined ? (
          <Skeleton className="h-7 w-20" />
        ) : (
          <div className="flex items-center gap-1.5">
            {valueIcon}
            <span className="text-2xl leading-none font-semibold tracking-tight tabular">
              {value}
            </span>
          </div>
        )}
        <div className="mt-1.5 h-4 truncate text-[12px] text-muted-foreground tabular">
          {sub ?? <Skeleton className="h-3 w-28" />}
        </div>
      </div>
      <div className="mt-2">
        <Sparkline data={series} color={color} max={max} height={40} />
      </div>
    </Card>
  );
}
