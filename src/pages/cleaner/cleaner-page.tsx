import {
  Compass,
  Cpu,
  FileWarning,
  Globe,
  HardDrive,
  Lock,
  RefreshCw,
  Sparkles,
  Trash2,
  type LucideIcon,
} from "lucide-react";
import { Indicator } from "@/components/common/indicator";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { PlannedFeatures } from "@/components/common/planned-features";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { ErrorState } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useCleanupScan } from "@/hooks/use-system";
import { formatBytes } from "@/lib/format";
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
  const found = (data ?? []).filter((l) => l.status !== "missing");
  const total = found.reduce((n, l) => n + l.bytes, 0);

  return (
    <>
      <PageHeader
        title="PC Cleaner"
        description="Measuring only: nothing is deleted."
        actions={
          <Button size="sm" variant="outline" onClick={refetch} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : undefined} /> Scan again
          </Button>
        }
      />
      <div className="flex flex-col gap-4">
        <div className="flex items-center gap-4 rounded-xl border bg-card px-5 py-4">
          <div className="flex size-11 items-center justify-center rounded-lg bg-brand/12 text-brand">
            <Sparkles className="size-5" />
          </div>
          <div className="min-w-0 flex-1">
            <div className="text-[12px] text-muted-foreground">
              Space used by the locations below
            </div>
            {loading && !data ? (
              <Skeleton className="mt-1 h-6 w-28" />
            ) : (
              <div className="text-[22px] leading-tight font-semibold tabular">
                {formatBytes(total)}
              </div>
            )}
          </div>
          <DataSourceBadge source="live" />
        </div>

        {error ? (
          <ErrorState message={error.message} onRetry={refetch} />
        ) : loading && !data ? (
          <CardGrid>
            {Array.from({ length: 6 }, (_, i) => (
              <Skeleton key={i} className="h-[132px] rounded-xl" />
            ))}
          </CardGrid>
        ) : (
          <CardGrid>
            {(data ?? []).map((l) => (
              <LocationCard key={l.id} location={l} />
            ))}
          </CardGrid>
        )}

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

function LocationCard({ location: l }: { location: CleanupLocation }) {
  const status =
    l.status === "missing" ? (
      <span className="text-[12px] text-muted-foreground">Not on this PC</span>
    ) : l.status === "denied" ? (
      <span className="text-[12px] text-muted-foreground">Needs administrator</span>
    ) : (
      <span className="text-right">
        <span className="block text-[15px] leading-tight font-semibold tabular">
          {l.status === "partial" ? "≥ " : ""}
          {formatBytes(l.bytes)}
        </span>
        <span className="text-[11px] text-muted-foreground tabular">
          {l.files.toLocaleString()} {l.id === "recycle-bin" ? "items" : "files"}
        </span>
      </span>
    );
  return (
    <OptimisationCard
      icon={ICONS[l.id] ?? HardDrive}
      title={l.label}
      description={l.description}
      muted={l.status === "missing"}
      indicators={
        <>
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
          <span
            data-selectable
            title={l.path}
            className="max-w-[16rem] truncate text-[11px] text-muted-foreground/70"
          >
            {l.path}
          </span>
        </>
      }
      control={status}
    />
  );
}
