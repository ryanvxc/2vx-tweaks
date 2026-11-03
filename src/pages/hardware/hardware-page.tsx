import { useState, type ReactNode } from "react";
import { Cpu, Gpu, HardDrive, MemoryStick, Network } from "lucide-react";
import { MetricChart } from "@/components/common/metric-chart";
import { PageTabs, type PageTab } from "@/components/common/page-tabs";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Progress } from "@/components/ui/progress";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { formatBytes, formatFrequency, formatPercent, formatRate, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { MetricsSample, SystemSnapshot } from "@/types/system";

type Tab = "cpu" | "gpu" | "memory" | "storage" | "network";

const TABS: PageTab<Tab>[] = [
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "storage", label: "Storage", icon: HardDrive },
  { value: "network", label: "Network", icon: Network },
];

export function HardwarePage() {
  const [tab, setTab] = useState<Tab>("cpu");
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const s = snapshot.data;
  const cpuChip = s?.cpu.brand;

  return (
    <>
      <PageHeader
        title="Hardware"
        description="Components detected on this PC and their live utilisation"
        actions={
          cpuChip && (
            <span className="flex h-7 items-center gap-1.5 rounded-full border bg-card px-3 text-[12px] text-muted-foreground">
              <Cpu className="size-3.5 text-brand" />
              <span data-selectable className="max-w-72 truncate">
                {cpuChip}
              </span>
            </span>
          )
        }
      />
      <div className="flex flex-col gap-4">
        <PageTabs label="Components" tabs={TABS} value={tab} onChange={setTab} />
        {snapshot.error ? (
          <ErrorState
            title="Couldn't read hardware information"
            message={snapshot.error.message}
            onRetry={snapshot.refetch}
          />
        ) : !s ? (
          <LoadingRows rows={6} />
        ) : (
          <>
            {metrics.error && metrics.samples.length === 0 && (
              <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
            )}
            {tab === "cpu" && <CpuTab snapshot={s} samples={metrics.samples} />}
            {tab === "gpu" && <GpuTab snapshot={s} samples={metrics.samples} />}
            {tab === "memory" && <MemoryTab snapshot={s} samples={metrics.samples} />}
            {tab === "storage" && <StorageTab snapshot={s} />}
            {tab === "network" && <NetworkTab samples={metrics.samples} />}
          </>
        )}
      </div>
    </>
  );
}

function rows(samples: MetricsSample[], pick: (s: MetricsSample) => Record<string, number | null>) {
  const now = samples.at(-1)?.timestampMs ?? 0;
  return samples.map((s) => ({ ago: Math.round((s.timestampMs - now) / 1000), ...pick(s) }));
}

const round = (v: number) => Math.round(v * 10) / 10;

function Facts({ items }: { items: [string, ReactNode][] }) {
  return (
    <dl className="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-3">
      {items.map(([label, value]) => (
        <div key={label} className="rounded-lg border bg-background/40 px-3 py-2.5">
          <dt className="text-[11.5px] text-muted-foreground">{label}</dt>
          <dd data-selectable className="mt-0.5 truncate text-[14px] font-semibold tabular">
            {value}
          </dd>
        </div>
      ))}
    </dl>
  );
}

function CpuTab({ snapshot, samples }: { snapshot: SystemSnapshot; samples: MetricsSample[] }) {
  const latest = samples.at(-1);
  const cores = latest?.perCorePercent ?? [];
  return (
    <div className="flex flex-col gap-4">
      <SectionCard title="Processor" description={snapshot.cpu.brand} source="live">
        <Facts
          items={[
            ["Usage now", latest ? formatPercent(latest.cpuPercent) : "—"],
            ["Cores", snapshot.cpu.physicalCores ?? "—"],
            ["Threads", snapshot.cpu.logicalCores],
            ["Base clock", formatFrequency(snapshot.cpu.frequencyMhz)],
            ["Vendor", snapshot.cpu.vendor || "—"],
          ]}
        />
      </SectionCard>
      <SectionCard title="Usage" description="Last 5 minutes" source="live">
        <MetricChart
          rows={rows(samples, (s) => ({ cpu: round(s.cpuPercent) }))}
          series={[{ key: "cpu", label: "CPU", color: "var(--chart-1)" }]}
        />
      </SectionCard>
      <SectionCard
        title="Per thread"
        description="Usage of each logical processor now"
        source="live"
      >
        {cores.length === 0 ? (
          <LoadingRows rows={2} />
        ) : (
          <div className="grid grid-cols-[repeat(auto-fill,minmax(64px,1fr))] gap-2">
            {cores.map((c, i) => (
              <div key={i} className="flex flex-col items-center gap-1 rounded-md border px-2 py-2">
                <div className="relative h-14 w-3 overflow-hidden rounded-full bg-muted">
                  <div
                    className="absolute inset-x-0 bottom-0 rounded-full bg-brand transition-[height] duration-500"
                    style={{ height: `${Math.min(100, c)}%` }}
                  />
                </div>
                <div className="text-[11px] font-medium tabular">{formatPercent(c)}</div>
                <div className="text-[10px] text-muted-foreground tabular">#{i}</div>
              </div>
            ))}
          </div>
        )}
      </SectionCard>
    </div>
  );
}

function GpuTab({ snapshot, samples }: { snapshot: SystemSnapshot; samples: MetricsSample[] }) {
  if (snapshot.gpus.length === 0)
    return (
      <EmptyState
        icon={Gpu}
        title="No GPU reported"
        description="Windows didn't report a graphics adapter."
        className="rounded-xl border bg-card"
      />
    );
  const latest = samples.at(-1);
  return (
    <div className="flex flex-col gap-4">
      {snapshot.gpus.map((g, i) => {
        const live = latest?.gpus[i];
        return (
          <SectionCard key={i} title={g.model ?? "Unknown GPU"} source="live">
            <Facts
              items={[
                ["Usage now", live?.usagePercent != null ? formatPercent(live.usagePercent) : "—"],
                [
                  "Dedicated memory",
                  g.dedicatedMemoryBytes ? formatBytes(g.dedicatedMemoryBytes, 0) : "—",
                ],
                ["Vendor", g.vendor ?? "—"],
              ]}
            />
            <div className="mt-4">
              <MetricChart
                rows={rows(samples, (s) => ({
                  gpu: s.gpus[i]?.usagePercent != null ? round(s.gpus[i]!.usagePercent!) : null,
                }))}
                series={[{ key: "gpu", label: "GPU", color: "var(--chart-3)" }]}
                height={180}
              />
            </div>
          </SectionCard>
        );
      })}
      <p className="px-1 text-[12px] text-muted-foreground">
        GPU usage is the sum over all engines as Windows reports it, capped at 100%. Used video
        memory isn't available system-wide yet, so it isn't shown.
      </p>
    </div>
  );
}

function MemoryTab({ snapshot, samples }: { snapshot: SystemSnapshot; samples: MetricsSample[] }) {
  const latest = samples.at(-1);
  return (
    <div className="flex flex-col gap-4">
      <SectionCard title="Memory" source="live">
        <Facts
          items={[
            ["In use", latest ? formatBytes(latest.memoryUsedBytes) : "—"],
            ["Installed", formatBytes(snapshot.memory.totalBytes)],
            [
              "Usage now",
              latest
                ? formatPercent(percentOf(latest.memoryUsedBytes, latest.memoryTotalBytes))
                : "—",
            ],
            ["Page file", formatBytes(snapshot.memory.swapTotalBytes)],
          ]}
        />
      </SectionCard>
      <SectionCard title="Usage" description="Last 5 minutes" source="live">
        <MetricChart
          rows={rows(samples, (s) => ({
            ram: round(percentOf(s.memoryUsedBytes, s.memoryTotalBytes)),
          }))}
          series={[{ key: "ram", label: "Memory", color: "var(--chart-2)" }]}
        />
      </SectionCard>
    </div>
  );
}

function StorageTab({ snapshot }: { snapshot: SystemSnapshot }) {
  if (snapshot.disks.length === 0)
    return (
      <EmptyState
        icon={HardDrive}
        title="No drives reported"
        className="rounded-xl border bg-card"
      />
    );
  return (
    <div className="grid grid-cols-2 gap-3">
      {snapshot.disks.map((d) => {
        const used = d.totalBytes - d.availableBytes;
        const pct = percentOf(used, d.totalBytes);
        const freePct = 100 - pct;
        return (
          <div key={d.mountPoint} className="rounded-xl border bg-card p-4">
            <div className="flex items-center gap-2.5">
              <HardDrive className="size-[18px] text-brand" />
              <div className="min-w-0 flex-1">
                <div className="truncate text-card-title">
                  {d.mountPoint} {d.name && <span className="text-muted-foreground">{d.name}</span>}
                </div>
                <div className="text-[12px] text-muted-foreground">
                  {d.kind === "unknown" ? "Drive" : d.kind.toUpperCase()} · {d.fileSystem}
                  {d.isRemovable ? " · removable" : ""}
                </div>
              </div>
              <span className="text-[13px] font-semibold tabular">{formatPercent(pct)}</span>
            </div>
            <Progress
              value={pct}
              className="mt-3"
              indicatorClassName={cn(
                freePct < 10 ? "bg-destructive" : freePct < 20 ? "bg-warning" : "bg-brand",
              )}
              aria-label={`${d.mountPoint} used`}
            />
            <div className="mt-2 flex justify-between text-[12px] text-muted-foreground tabular">
              <span>{formatBytes(used)} used</span>
              <span>
                {formatBytes(d.availableBytes)} free of {formatBytes(d.totalBytes)}
              </span>
            </div>
          </div>
        );
      })}
    </div>
  );
}

function NetworkTab({ samples }: { samples: MetricsSample[] }) {
  const latest = samples.at(-1);
  const peakRx = Math.max(0, ...samples.map((s) => s.networkRxBytesPerSec));
  const peakTx = Math.max(0, ...samples.map((s) => s.networkTxBytesPerSec));
  return (
    <div className="flex flex-col gap-4">
      <SectionCard title="Network" description="All adapters combined" source="live">
        <Facts
          items={[
            ["Download now", latest ? formatRate(latest.networkRxBytesPerSec) : "—"],
            ["Upload now", latest ? formatRate(latest.networkTxBytesPerSec) : "—"],
            ["Peak download (5 min)", formatRate(peakRx)],
            ["Peak upload (5 min)", formatRate(peakTx)],
          ]}
        />
      </SectionCard>
      <SectionCard title="Throughput" description="Last 5 minutes" source="live">
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
    </div>
  );
}
