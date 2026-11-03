import type { ReactNode } from "react";
import { Link } from "react-router";
import {
  Activity,
  CircuitBoard,
  Cpu,
  Gpu,
  HardDrive,
  MemoryStick,
  Monitor,
  type LucideIcon,
} from "lucide-react";
import { pageDescription } from "@/app/navigation";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Progress } from "@/components/ui/progress";
import { useSystemSnapshot } from "@/hooks/use-system";
import { formatBytes, formatFrequency, formatPercent, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { DiskInfo, SystemSnapshot } from "@/types/system";

/** The specification sheet. Live usage lives on Monitoring; each card links there. */
export function HardwarePage() {
  const snapshot = useSystemSnapshot();
  const s = snapshot.data;

  return (
    <>
      <PageHeader title="Hardware" description={pageDescription("/hardware")} />
      {snapshot.error ? (
        <ErrorState
          title="Couldn't read hardware information"
          message={snapshot.error.message}
          onRetry={snapshot.refetch}
        />
      ) : !s ? (
        <LoadingRows rows={6} />
      ) : (
        <div className="flex flex-col gap-3">
          <div className="grid grid-cols-2 gap-3 [&>:last-child:nth-child(odd)]:col-span-2">
            <SpecCard icon={Cpu} title={s.cpu.brand || "Unknown CPU"} live="cpu">
              <Facts
                items={[
                  ["Cores", s.cpu.physicalCores ?? "—"],
                  ["Threads", s.cpu.logicalCores],
                  ["Base clock", formatFrequency(s.cpu.frequencyMhz)],
                  ["Vendor", s.cpu.vendor || "—"],
                ]}
              />
            </SpecCard>
            <Graphics snapshot={s} />
            <SpecCard
              icon={MemoryStick}
              title={`${formatBytes(s.memory.totalBytes, 0)} memory`}
              live="memory"
            >
              <Facts
                items={[
                  ["Installed", formatBytes(s.memory.totalBytes)],
                  ["Page file", formatBytes(s.memory.swapTotalBytes)],
                ]}
              />
            </SpecCard>
            <SpecCard icon={Monitor} title={windowsName(s)}>
              <Facts
                items={[
                  ["Build", `${s.os.build}${s.os.ubr != null ? `.${s.os.ubr}` : ""}`],
                  ["Edition", s.os.editionId ?? "—"],
                  ["Architecture", s.os.architecture],
                  ["PC name", s.hostname ?? "—"],
                ]}
              />
            </SpecCard>
            {s.motherboard && (s.motherboard.name || s.motherboard.vendor) && (
              <SpecCard icon={CircuitBoard} title={s.motherboard.name ?? "Motherboard"}>
                <Facts items={[["Vendor", s.motherboard.vendor ?? "—"]]} />
              </SpecCard>
            )}
          </div>

          <h2 className="mt-2 text-section-title">Drives</h2>
          {s.disks.length === 0 ? (
            <EmptyState
              icon={HardDrive}
              title="No drives reported"
              className="rounded-xl border bg-card"
            />
          ) : (
            <div className="grid grid-cols-2 gap-3">
              {s.disks.map((d) => (
                <DriveCard key={d.mountPoint} disk={d} />
              ))}
            </div>
          )}
        </div>
      )}
    </>
  );
}

function windowsName(s: SystemSnapshot) {
  return `${s.os.productName}${s.os.displayVersion ? ` ${s.os.displayVersion}` : ""}`;
}

function SpecCard({
  icon: Icon,
  title,
  live,
  children,
}: {
  icon: LucideIcon;
  title: string;
  /** Monitoring tab with this part's live usage. */
  live?: string;
  children: ReactNode;
}) {
  return (
    <div className="rounded-xl border bg-card px-4 py-3.5">
      <div className="mb-3 flex items-center gap-2.5">
        <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
        <h2 data-selectable className="min-w-0 flex-1 truncate text-card-title" title={title}>
          {title}
        </h2>
        {live && (
          <Link
            to={`/monitoring?tab=${live}`}
            className="flex shrink-0 items-center gap-1 text-[12px] font-medium text-brand hover:underline"
          >
            <Activity className="size-3.5" /> Live
          </Link>
        )}
        <DataSourceBadge source="live" />
      </div>
      {children}
    </div>
  );
}

function Facts({ items }: { items: [string, ReactNode][] }) {
  return (
    <dl className="grid grid-cols-4 gap-x-3 gap-y-2">
      {items.map(([label, value]) => (
        <div key={label} className="min-w-0">
          <dt className="text-[12px] text-muted-foreground">{label}</dt>
          <dd data-selectable className="truncate text-[13.5px] font-semibold tabular">
            {value}
          </dd>
        </div>
      ))}
    </dl>
  );
}

function Graphics({ snapshot }: { snapshot: SystemSnapshot }) {
  if (snapshot.gpus.length === 0)
    return (
      <SpecCard icon={Gpu} title="Graphics">
        <p className="text-[13px] text-muted-foreground">Windows didn't report a GPU.</p>
      </SpecCard>
    );
  return (
    <>
      {snapshot.gpus.map((g, i) => (
        <SpecCard key={i} icon={Gpu} title={g.model ?? "Unknown GPU"} live="gpu">
          <Facts
            items={[
              [
                "Dedicated memory",
                g.dedicatedMemoryBytes ? formatBytes(g.dedicatedMemoryBytes, 0) : "—",
              ],
              ["Vendor", g.vendor ?? "—"],
            ]}
          />
        </SpecCard>
      ))}
    </>
  );
}

function DriveCard({ disk: d }: { disk: DiskInfo }) {
  const used = d.totalBytes - d.availableBytes;
  const pct = percentOf(used, d.totalBytes);
  const freePct = 100 - pct;
  return (
    <div className="rounded-xl border bg-card px-4 py-3.5">
      <div className="flex items-center gap-2.5">
        <HardDrive className="size-[18px] shrink-0 text-brand" aria-hidden />
        <div className="min-w-0 flex-1 truncate text-card-title">
          {d.mountPoint.replace(/\\$/, "")}{" "}
          {d.name && <span className="font-normal text-muted-foreground">{d.name}</span>}
        </div>
        <span className="text-[12px] text-muted-foreground">
          {d.kind === "unknown" ? "Drive" : d.kind.toUpperCase()} · {d.fileSystem}
          {d.isRemovable ? " · removable" : ""}
        </span>
      </div>
      <Progress
        value={pct}
        className="mt-3"
        indicatorClassName={cn(
          freePct < 10 ? "bg-destructive" : freePct < 20 ? "bg-warning" : "bg-brand",
        )}
        aria-label={`${d.mountPoint} used`}
      />
      <div className="mt-2 flex justify-between text-[12.5px] text-muted-foreground tabular">
        <span>
          {formatBytes(used)} used ({formatPercent(pct)})
        </span>
        <span>
          {formatBytes(d.availableBytes)} free of {formatBytes(d.totalBytes)}
        </span>
      </div>
    </div>
  );
}
