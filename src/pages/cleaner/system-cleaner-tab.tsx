import { useState } from "react";
import {
  Bug,
  ChevronDown,
  Clock,
  Compass,
  Cpu,
  FileClock,
  FileWarning,
  Gauge,
  Globe,
  HardDrive,
  History,
  PackageOpen,
  Trash2,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useStaggerIn } from "@/hooks/use-motion";
import { useCleanupScan } from "@/hooks/use-system";
import { cleanableLocations, recommendedLocations, summariseCleanup } from "@/lib/cleaner";
import { formatAgo, formatBytes } from "@/lib/format";
import { backend, toBackendError } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import type { CleanupLocation } from "@/types/system";

const ICONS: Record<string, LucideIcon> = {
  "user-temp": FileClock,
  "windows-temp": HardDrive,
  prefetch: Gauge,
  "directx-shader-cache": Cpu,
  "nvidia-shader-cache": Cpu,
  "amd-shader-cache": Cpu,
  "edge-cache": Globe,
  "chrome-cache": Compass,
  "brave-cache": Compass,
  "internet-cache": Globe,
  "package-caches": PackageOpen,
  "recent-files": History,
  "error-reports": FileWarning,
  "crash-dumps": Bug,
  "recycle-bin": Trash2,
};

function sizeOf(l: CleanupLocation): string {
  if (l.status === "denied") return "Needs admin";
  return `${l.status === "partial" ? "≥ " : ""}${formatBytes(l.bytes)}`;
}

const markCleaned = () => useUiStore.getState().set({ cleanerLastRun: Date.now() });

export function SystemCleanerTab() {
  const { data, error, loading, refetch } = useCleanupScan();
  const lastRun = useUiStore((s) => s.cleanerLastRun);
  const [picked, setPicked] = useState<string[]>([]);
  const [confirming, setConfirming] = useState(false);
  const [cleaning, setCleaning] = useState(false);
  const list = useStaggerIn<HTMLUListElement>(data ? "scanned" : null);

  const rows = cleanableLocations(data ?? []);
  const selected = rows.filter((l) => picked.includes(l.id));
  const everything = rows.map((l) => l.id);
  const allPicked = rows.length > 0 && selected.length === rows.length;

  const toggle = (id: string, on: boolean) =>
    setPicked((p) => (on ? [...p, id] : p.filter((x) => x !== id)));

  const clean = async () => {
    setConfirming(false);
    setCleaning(true);
    try {
      const { freed, skipped, failures } = summariseCleanup(
        await backend.cleanLocations(selected.map((l) => l.id)),
      );
      markCleaned();
      setPicked([]);
      toast.success(`Freed ${formatBytes(freed)}`, {
        description: skipped > 0 ? `${skipped.toLocaleString()} files were in use.` : undefined,
      });
      failures.forEach((f) => toast.error(f));
      refetch();
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      setCleaning(false);
    }
  };

  return (
    <div className="flex flex-1 flex-col gap-3">
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-3">
          <span className="flex h-8 items-center gap-2 panel rounded-lg px-3 text-[12.5px] text-muted-foreground">
            <Clock className="size-3.5" aria-hidden />
            {lastRun ? `Last ran ${formatAgo(lastRun)}` : "Not run yet"}
          </span>
          <DataSourceBadge source="live" />
        </div>
        <div className="flex">
          <Button
            size="sm"
            className="rounded-r-none"
            disabled={rows.length === 0}
            onClick={() => setPicked(allPicked ? [] : everything)}
          >
            {allPicked ? "Clear selection" : "Select all"}
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                size="icon-sm"
                className="rounded-l-none border-l border-primary-foreground/20"
                disabled={rows.length === 0}
                aria-label="Selection options"
              >
                <ChevronDown />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-44">
              <DropdownMenuItem onSelect={() => setPicked(recommendedLocations(rows))}>
                Recommended
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => setPicked(everything)}>Select all</DropdownMenuItem>
              <DropdownMenuItem onSelect={() => setPicked([])}>Clear selection</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={7} />
      ) : (
        <ul ref={list} className="flex flex-col gap-2">
          {rows.map((l) => {
            const Icon = ICONS[l.id] ?? HardDrive;
            return (
              <li key={l.id} data-stagger>
                <label
                  title={l.description}
                  className="flex h-12 cursor-pointer items-center gap-3 panel px-4 transition-colors hover:bg-raised"
                >
                  <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
                  <span className="min-w-0 flex-1 truncate text-[13.5px] font-medium">
                    {l.label}
                  </span>
                  <span className="shrink-0 text-[13px] text-muted-foreground tabular">
                    {sizeOf(l)}
                  </span>
                  <Checkbox
                    aria-label={l.label}
                    checked={picked.includes(l.id)}
                    onCheckedChange={(on) => toggle(l.id, on === true)}
                  />
                </label>
              </li>
            );
          })}
        </ul>
      )}

      <div className="sticky bottom-0 mt-auto bg-background pt-1">
        <Button
          className="h-12 w-full"
          disabled={selected.length === 0 || cleaning}
          onClick={() => setConfirming(true)}
        >
          {cleaning ? "Cleaning…" : "Start cleaning"}
        </Button>
      </div>

      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title="Delete these files?"
        description="This can't be undone."
        consequences={selected.map((l) => `${l.label}: ${sizeOf(l)}`)}
        risk="moderate"
        confirmLabel="Start cleaning"
        onConfirm={() => void clean()}
      />
    </div>
  );
}
