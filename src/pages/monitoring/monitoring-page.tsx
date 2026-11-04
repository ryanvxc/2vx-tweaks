import { MONITORING_TABS, type MonitoringTab } from "@/app/navigation";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { MetricChart } from "@/components/common/metric-chart";
import { SectionCard } from "@/components/common/section-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import { formatBytes, formatPercent, formatRate, percentOf } from "@/lib/format";
import type { MetricsSample, SystemSnapshot } from "@/types/system";
import { LiveStats } from "./live-stats";

const round = (v: number) => Math.round(v * 10) / 10;

// The graph takes whatever height the page has left.
const FILL = { className: "flex-1", contentClassName: "relative min-h-[150px] flex-1" };

function rows(samples: MetricsSample[], pick: (s: MetricsSample) => Record<string, number | null>) {
  const now = samples.at(-1)?.timestampMs ?? 0;
  return samples.map((s) => ({ ago: Math.round((s.timestampMs - now) / 1000), ...pick(s) }));
}

/** Average and peak of a series, for the line above each graph. */
function summarise(values: number[], format: (v: number) => string) {
  if (values.length === 0) return "";
  const mean = values.reduce((a, b) => a + b, 0) / values.length;
  return `Average ${format(mean)} · peak ${format(Math.max(...values))}`;
}

export function MonitoringPage() {
  const [tab, setTab] = useTabParam(MONITORING_TABS, "cpu");
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const { samples } = metrics;

  return (
    <>
      <PageHeader title="Monitoring" />
      <div className="flex flex-1 flex-col gap-3">
        {metrics.error && samples.length === 0 && (
          <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
        )}
        <LiveStats samples={samples} snapshot={snapshot.data} selected={tab} onSelect={setTab} />
        <div key={tab} className="flex flex-1 animate-rise flex-col gap-3">
          <Graph tab={tab} samples={samples} snapshot={snapshot.data} />
          {tab === "cpu" && <PerThread samples={samples} />}
        </div>
      </div>
    </>
  );
}

function Graph({
  tab,
  samples,
  snapshot,
}: {
  tab: MonitoringTab;
  samples: MetricsSample[];
  snapshot: SystemSnapshot | undefined;
}) {
  switch (tab) {
    case "cpu":
      return (
        <SectionCard
          title={snapshot?.cpu.brand.trim() || "CPU"}
          description={summarise(
            samples.map((s) => s.cpuPercent),
            formatPercent,
          )}
          source="live"
          {...FILL}
        >
          <MetricChart
            rows={rows(samples, (s) => ({ cpu: round(s.cpuPercent) }))}
            series={[{ key: "cpu", label: "CPU", color: "var(--chart-1)" }]}
          />
        </SectionCard>
      );
    case "memory":
      return (
        <SectionCard
          title={
            samples.at(-1) ? `${formatBytes(samples.at(-1)!.memoryTotalBytes)} memory` : "Memory"
          }
          description={summarise(
            samples.map((s) => percentOf(s.memoryUsedBytes, s.memoryTotalBytes)),
            formatPercent,
          )}
          source="live"
          {...FILL}
        >
          <MetricChart
            rows={rows(samples, (s) => ({
              ram: round(percentOf(s.memoryUsedBytes, s.memoryTotalBytes)),
            }))}
            series={[{ key: "ram", label: "Memory", color: "var(--chart-2)" }]}
          />
        </SectionCard>
      );
    case "gpu": {
      const count = Math.max(snapshot?.gpus.length ?? 0, samples.at(-1)?.gpus.length ?? 0);
      if (count === 0)
        return (
          <SectionCard title="GPU" source="live">
            <p className="text-[13px] text-muted-foreground">Windows didn't report a GPU.</p>
          </SectionCard>
        );
      return (
        <>
          {Array.from({ length: count }, (_, i) => (
            <SectionCard
              key={i}
              title={snapshot?.gpus[i]?.model ?? samples.at(-1)?.gpus[i]?.model ?? "GPU"}
              description={summarise(
                samples.flatMap((s) => s.gpus[i]?.usagePercent ?? []),
                formatPercent,
              )}
              source="live"
              {...FILL}
            >
              <MetricChart
                rows={rows(samples, (s) => ({
                  gpu: s.gpus[i]?.usagePercent != null ? round(s.gpus[i]!.usagePercent!) : null,
                }))}
                series={[{ key: "gpu", label: "GPU", color: "var(--chart-3)" }]}
              />
            </SectionCard>
          ))}
        </>
      );
    }
    case "network":
      return (
        <SectionCard
          title="All adapters"
          description={`Download ${summarise(
            samples.map((s) => s.networkRxBytesPerSec),
            formatRate,
          )}`}
          source="live"
          {...FILL}
        >
          <MetricChart
            rows={rows(samples, (s) => ({
              rx: Math.round(s.networkRxBytesPerSec / 1024),
              tx: Math.round(s.networkTxBytesPerSec / 1024),
            }))}
            series={[
              { key: "rx", label: "Download", color: "var(--chart-2)" },
              { key: "tx", label: "Upload", color: "var(--chart-5)" },
            ]}
            max={undefined}
            format={(v) => formatRate(v * 1024)}
          />
        </SectionCard>
      );
  }
}

function PerThread({ samples }: { samples: MetricsSample[] }) {
  const cores = samples.at(-1)?.perCorePercent ?? [];
  return (
    <div className="flex items-center gap-4 panel px-4 py-2.5">
      <div className="flex shrink-0 items-center gap-1.5">
        <h2 className="text-section-title">Per thread</h2>
        <DataSourceBadge source="live" />
      </div>
      {cores.length === 0 ? (
        <div className="flex-1">
          <LoadingRows rows={1} />
        </div>
      ) : (
        <div className="grid flex-1 grid-cols-[repeat(auto-fit,minmax(1.75rem,1fr))] gap-1">
          {cores.map((c, i) => (
            <div key={i} className="flex flex-col items-center gap-1">
              <div className="h-8 w-full max-w-4 overflow-hidden rounded-[3px] bg-muted">
                <div
                  className="size-full bg-brand transition-[translate] duration-500"
                  style={{ translate: `0 ${100 - Math.min(100, c)}%` }}
                />
              </div>
              <div className="text-[10.5px] text-muted-foreground tabular">{Math.round(c)}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
