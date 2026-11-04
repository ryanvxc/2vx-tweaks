import {
  Compass,
  Cpu,
  FileWarning,
  Globe,
  HardDrive,
  Lock,
  RefreshCw,
  Trash2,
  type LucideIcon,
} from "lucide-react";
import { AnimatedNumber } from "@/components/common/animated-number";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Indicator } from "@/components/common/indicator";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useStaggerIn } from "@/hooks/use-motion";
import { useCleanupScan } from "@/hooks/use-system";
import { formatBytes } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { CleanupLocation } from "@/types/system";

const ICONS: Record<string, LucideIcon> = {
  "user-temp": FileWarning,
  "windows-temp": HardDrive,
  "directx-shader-cache": Cpu,
  "nvidia-shader-cache": Cpu,
  "amd-shader-cache": Cpu,
  "edge-cache": Globe,
  "chrome-cache": Compass,
  "error-reports": FileWarning,
  "recycle-bin": Trash2,
};

export function CleanerPage() {
  const { data, error, loading, refetch } = useCleanupScan();
  const locations = [...(data ?? [])].sort(
    (a, b) => Number(a.status === "missing") - Number(b.status === "missing") || b.bytes - a.bytes,
  );
  const found = locations.filter((l) => l.status !== "missing");
  const total = found.reduce((n, l) => n + l.bytes, 0);
  const largest = locations[0]?.bytes ?? 0;
  const list = useStaggerIn<HTMLUListElement>(data ? "scanned" : null);

  return (
    <>
      <PageHeader
        title="PC Cleaner"
        actions={
          <Button size="sm" variant="ghost" onClick={refetch} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : undefined} /> Scan again
          </Button>
        }
      />
      <div className="flex flex-1 flex-col overflow-hidden panel">
        <div className="flex h-14 shrink-0 items-center gap-3 border-b px-4">
          {loading && !data ? (
            <Skeleton className="h-7 w-28" />
          ) : (
            <div className="text-[22px] leading-none font-semibold tracking-tight tabular">
              <AnimatedNumber value={total} format={formatBytes} duration={800} />
            </div>
          )}
          <div className="flex-1 self-center text-[12.5px] text-muted-foreground">
            in {found.length} locations · scan only
          </div>
          <DataSourceBadge source="live" />
        </div>

        {error ? (
          <div className="p-4">
            <ErrorState message={error.message} onRetry={refetch} />
          </div>
        ) : loading && !data ? (
          <div className="p-4">
            <LoadingRows rows={6} />
          </div>
        ) : (
          // Rows share the height, so a short list still reaches the bottom of the panel.
          <ul ref={list} className="flex flex-1 flex-col divide-y">
            {locations.map((l) => (
              <LocationRow key={l.id} location={l} largest={largest} />
            ))}
          </ul>
        )}
      </div>
    </>
  );
}

function LocationRow({ location: l, largest }: { location: CleanupLocation; largest: number }) {
  const Icon = ICONS[l.id] ?? HardDrive;
  const missing = l.status === "missing";
  const share = largest > 0 ? (l.bytes / largest) * 100 : 0;
  return (
    <li
      data-stagger
      className={cn("flex min-h-10 flex-1 items-center gap-3 px-4", missing && "opacity-50")}
    >
      <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
      <div
        className="w-56 min-w-0 shrink-0 truncate text-[13px] font-medium"
        title={`${l.description}\n${l.path}`}
      >
        {l.label}
      </div>
      <div className="h-1.5 min-w-0 flex-1 overflow-hidden rounded-full bg-muted" aria-hidden>
        <div
          className="h-full origin-left animate-grow-x rounded-full bg-brand transition-[width] duration-500"
          style={{ width: `${share}%` }}
        />
      </div>
      {l.status === "denied" && (
        <Indicator icon={Lock} tone="warning" title="Couldn't read this folder" />
      )}
      {l.status === "partial" && (
        <Indicator icon={FileWarning} tone="info" title="Very large">
          Scan stopped at 250,000 entries.
        </Indicator>
      )}
      {missing || l.status === "denied" ? (
        <span className="w-44 shrink-0 text-right text-[12.5px] text-muted-foreground">
          {missing ? "Not on this PC" : "Needs admin"}
        </span>
      ) : (
        <>
          <span className="w-24 shrink-0 text-right text-[12px] text-muted-foreground tabular">
            {l.files.toLocaleString()} {l.id === "recycle-bin" ? "items" : "files"}
          </span>
          <span className="w-[4.25rem] shrink-0 text-right text-[13px] font-semibold tabular">
            {l.status === "partial" ? "≥ " : ""}
            {formatBytes(l.bytes)}
          </span>
        </>
      )}
    </li>
  );
}
