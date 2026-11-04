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
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Progress } from "@/components/ui/progress";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useSystemSnapshot } from "@/hooks/use-system";
import { formatBytes, formatFrequency, formatPercent, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { DiskInfo, SystemSnapshot } from "@/types/system";

/** The specification sheet: one row per part. Live usage lives on Monitoring; rows link there. */
export function HardwarePage() {
  const snapshot = useSystemSnapshot();
  const s = snapshot.data;

  return (
    <>
      <PageHeader title="Hardware" />
      {snapshot.error ? (
        <ErrorState
          title="Couldn't read hardware information"
          message={snapshot.error.message}
          onRetry={snapshot.refetch}
        />
      ) : !s ? (
        <LoadingRows rows={6} />
      ) : (
        <div className="flex flex-1 flex-col overflow-hidden panel">
          <div className="flex h-9 shrink-0 items-center gap-3 border-b px-4 text-[12px] text-muted-foreground">
            <span className="w-[12.875rem] shrink-0 xl:w-[16.375rem]">Part</span>
            <span className="flex-1">Details</span>
            <DataSourceBadge source="live" />
          </div>
          <ul className="flex flex-1 flex-col divide-y">
            <Part icon={Cpu} title={s.cpu.brand || "Unknown CPU"} live="cpu">
              <Facts
                items={[
                  ["Cores", s.cpu.physicalCores ?? "—"],
                  ["Threads", s.cpu.logicalCores],
                  ["Base clock", formatFrequency(s.cpu.frequencyMhz)],
                  ["Vendor", s.cpu.vendor || "—"],
                ]}
              />
            </Part>
            <Graphics snapshot={s} />
            <Part
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
            </Part>
            {s.motherboard && (s.motherboard.name || s.motherboard.vendor) && (
              <Part icon={CircuitBoard} title={s.motherboard.name ?? "Motherboard"}>
                <Facts items={[["Vendor", s.motherboard.vendor ?? "—"]]} />
              </Part>
            )}
            <Part icon={Monitor} title={windowsName(s)}>
              <Facts
                items={[
                  ["Build", `${s.os.build}${s.os.ubr != null ? `.${s.os.ubr}` : ""}`],
                  ["Edition", s.os.editionId ?? "—"],
                  ["Architecture", s.os.architecture],
                  ["PC name", s.hostname ?? "—"],
                ]}
              />
            </Part>
            {s.disks.length === 0 ? (
              <Part icon={HardDrive} title="Drives">
                <p className="text-[13px] text-muted-foreground">Windows didn't report a drive.</p>
              </Part>
            ) : (
              s.disks.map((d) => <Drive key={d.mountPoint} disk={d} />)
            )}
          </ul>
        </div>
      )}
    </>
  );
}

function windowsName(s: SystemSnapshot) {
  return `${s.os.productName}${s.os.displayVersion ? ` ${s.os.displayVersion}` : ""}`;
}

function Part({
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
    <li className="flex min-h-14 flex-1 items-center gap-3 px-4">
      <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
      <div data-selectable className="line-clamp-2 w-44 shrink-0 text-card-title xl:w-[14.5rem]">
        {title}
      </div>
      <div className="min-w-0 flex-1">{children}</div>
      {live ? (
        <Tooltip>
          <TooltipTrigger asChild>
            <Link
              to={`/monitoring?tab=${live}`}
              aria-label="Live usage"
              className="flex size-5 shrink-0 items-center justify-center rounded text-muted-foreground transition-colors hover:text-foreground"
            >
              <Activity className="size-3.5" />
            </Link>
          </TooltipTrigger>
          <TooltipContent>Live usage</TooltipContent>
        </Tooltip>
      ) : (
        <span className="size-5 shrink-0" />
      )}
    </li>
  );
}

function Fact({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="min-w-0">
      <dt className="text-[12px] text-muted-foreground">{label}</dt>
      <dd data-selectable className="truncate text-[13px] font-medium tabular">
        {children}
      </dd>
    </div>
  );
}

function Facts({ items }: { items: [string, ReactNode][] }) {
  return (
    <dl className="grid grid-cols-4 gap-3">
      {items.map(([label, value]) => (
        <Fact key={label} label={label}>
          {value}
        </Fact>
      ))}
    </dl>
  );
}

function Graphics({ snapshot }: { snapshot: SystemSnapshot }) {
  if (snapshot.gpus.length === 0)
    return (
      <Part icon={Gpu} title="Graphics">
        <p className="text-[13px] text-muted-foreground">Windows didn't report a GPU.</p>
      </Part>
    );
  return (
    <>
      {snapshot.gpus.map((g, i) => (
        <Part key={i} icon={Gpu} title={g.model ?? "Unknown GPU"} live="gpu">
          <Facts
            items={[
              [
                "Video memory",
                g.dedicatedMemoryBytes ? formatBytes(g.dedicatedMemoryBytes, 0) : "—",
              ],
              ["Vendor", g.vendor ?? "—"],
            ]}
          />
        </Part>
      ))}
    </>
  );
}

function Drive({ disk: d }: { disk: DiskInfo }) {
  const used = d.totalBytes - d.availableBytes;
  const pct = percentOf(used, d.totalBytes);
  const freePct = 100 - pct;
  const type = [
    d.kind !== "unknown" && d.kind.toUpperCase(),
    d.fileSystem,
    d.isRemovable && "removable",
  ]
    .filter(Boolean)
    .join(" · ");
  const label = d.name || (d.isRemovable ? "Removable disk" : "Local disk");
  return (
    <Part icon={HardDrive} title={`${d.mountPoint.replace(/\\$/, "")} ${label}`}>
      <div className="grid grid-cols-4 gap-3">
        <dl className="col-span-3 grid grid-cols-3 gap-3">
          <Fact label="Type">{type}</Fact>
          <Fact label="Size">{formatBytes(d.totalBytes)}</Fact>
          <Fact label="Free">{formatBytes(d.availableBytes)}</Fact>
        </dl>
        <div className="flex min-w-0 flex-col justify-between">
          <div className="text-[12px] text-muted-foreground tabular">{formatPercent(pct)} used</div>
          <Progress
            value={pct}
            indicatorClassName={cn(
              freePct < 10 ? "bg-destructive" : freePct < 20 ? "bg-warning" : "bg-brand",
            )}
            aria-label={`${d.mountPoint} used`}
          />
        </div>
      </div>
    </Part>
  );
}
