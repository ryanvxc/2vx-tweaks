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
import { pageDescription } from "@/app/navigation";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Indicator } from "@/components/common/indicator";
import { PlannedFeatures } from "@/components/common/planned-features";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
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

  return (
    <>
      <PageHeader
        title="PC Cleaner"
        description={pageDescription("/cleaner")}
        actions={
          <Button size="sm" variant="outline" onClick={refetch} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : undefined} /> Scan again
          </Button>
        }
      />
      <div className="flex flex-col gap-3">
        <div className="overflow-hidden rounded-xl border bg-card">
          <div className="flex items-center gap-4 border-b px-4 py-3">
            <div className="min-w-0 flex-1">
              {loading && !data ? (
                <Skeleton className="h-7 w-28" />
              ) : (
                <div className="font-heading text-[22px] leading-tight font-bold tabular">
                  {formatBytes(total)}
                </div>
              )}
              <div className="text-[12.5px] text-muted-foreground">
                in {found.length} locations. Measuring only: 2VX can't delete anything yet.
              </div>
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
            <ul className="divide-y">
              {locations.map((l) => (
                <LocationRow key={l.id} location={l} largest={largest} />
              ))}
            </ul>
          )}
        </div>

        <PlannedFeatures
          features={[
            "Review then clean: pick locations, see the exact list, confirm",
            "Skip files in use and anything changed in the last 24 hours",
            "Cleaning limited to this fixed list of locations, never arbitrary paths",
            "Empty the Recycle Bin through the Windows API",
          ]}
        />
      </div>
    </>
  );
}

function LocationRow({ location: l, largest }: { location: CleanupLocation; largest: number }) {
  const Icon = ICONS[l.id] ?? HardDrive;
  const missing = l.status === "missing";
  const share = largest > 0 ? (l.bytes / largest) * 100 : 0;
  return (
    <li className={cn("flex items-center gap-3.5 px-4 py-2.5", missing && "opacity-50")}>
      <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
      <div className="w-72 min-w-0 shrink-0">
        <div className="truncate text-[13px] font-medium" title={l.path}>
          {l.label}
        </div>
        <div className="truncate text-[12px] text-muted-foreground" title={l.description}>
          {l.description}
        </div>
      </div>
      <div className="h-1.5 min-w-0 flex-1 overflow-hidden rounded-full bg-muted" aria-hidden>
        <div className="h-full rounded-full bg-brand" style={{ width: `${share}%` }} />
      </div>
      {l.status === "denied" && (
        <Indicator icon={Lock} tone="warning" title="Couldn't read this folder">
          Windows only lets administrators read it. Run 2VX as administrator to measure it.
        </Indicator>
      )}
      {l.status === "partial" && (
        <Indicator icon={FileWarning} tone="info" title="Very large">
          The scan stopped after 250,000 entries, so the size shown is a lower bound.
        </Indicator>
      )}
      <div className="w-24 shrink-0 text-right">
        {missing ? (
          <span className="text-[12.5px] text-muted-foreground">Not on this PC</span>
        ) : l.status === "denied" ? (
          <span className="text-[12.5px] text-muted-foreground">Needs admin</span>
        ) : (
          <>
            <div className="text-[13.5px] leading-tight font-semibold tabular">
              {l.status === "partial" ? "≥ " : ""}
              {formatBytes(l.bytes)}
            </div>
            <div className="text-[11.5px] text-muted-foreground tabular">
              {l.files.toLocaleString()} {l.id === "recycle-bin" ? "items" : "files"}
            </div>
          </>
        )}
      </div>
    </li>
  );
}
