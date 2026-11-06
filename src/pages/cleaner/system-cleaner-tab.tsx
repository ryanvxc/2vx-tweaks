import { useRef, useState, type ReactNode } from "react";
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
import { AnimatedNumber } from "@/components/common/animated-number";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Progress } from "@/components/ui/progress";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useStaggerIn } from "@/hooks/use-motion";
import { useCleanupScan } from "@/hooks/use-system";
import { burst } from "@/lib/dot-burst";
import {
  cleanableLocations,
  didClean,
  recommendedLocations,
  summariseCleanup,
  type CleanupSummary,
} from "@/lib/cleaner";
import type { Tone } from "@/lib/engine-text";
import { formatAgo, formatBytes } from "@/lib/format";
import { backend, toBackendError } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import type { CleanupLocation, CleanupPhase, CleanupStatus } from "@/types/system";

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
  if (l.status === "denied") return "Needs administrator";
  return `${l.status === "partial" ? "≥ " : ""}${formatBytes(l.bytes)}`;
}

type RowPhase = CleanupPhase | "waiting";

interface RowRun {
  phase: RowPhase;
  freedBytes: number;
  reason?: string;
}

const PHASE: Record<RowPhase, { tone: Tone; label: string }> = {
  waiting: { tone: "neutral", label: "Waiting" },
  started: { tone: "info", label: "Clearing" },
  clearing: { tone: "info", label: "Clearing" },
  finished: { tone: "success", label: "Cleared" },
  declined: { tone: "warning", label: "Declined" },
  failed: { tone: "destructive", label: "Failed" },
};

const FINAL: Record<CleanupStatus, CleanupPhase> = {
  cleared: "finished",
  elevationDeclined: "declined",
  failed: "failed",
};

const isDone = (p: RowPhase) => p !== "waiting" && p !== "started" && p !== "clearing";

const markCleaned = () => useUiStore.getState().set({ cleanerLastRun: Date.now() });

export function SystemCleanerTab() {
  const { data, error, loading, refetch } = useCleanupScan();
  const lastRun = useUiStore((s) => s.cleanerLastRun);
  const [picked, setPicked] = useState<string[]>([]);
  const [confirming, setConfirming] = useState(false);
  const [cleaning, setCleaning] = useState(false);
  const [run, setRun] = useState<Record<string, RowRun>>({});
  const [total, setTotal] = useState(0);
  const [summary, setSummary] = useState<CleanupSummary | null>(null);
  const list = useStaggerIn<HTMLUListElement>(data ? "scanned" : null);
  const start = useRef<HTMLButtonElement>(null);

  const rows = cleanableLocations(data ?? []);
  const selected = rows.filter((l) => picked.includes(l.id));
  const everything = rows.map((l) => l.id);
  const allPicked = rows.length > 0 && selected.length === rows.length;
  const names = Object.fromEntries(rows.map((l) => [l.id, l.label]));
  const runIds = Object.keys(run);
  const doneCount = runIds.filter((id) => isDone(run[id]!.phase)).length;

  const toggle = (id: string, on: boolean) =>
    setPicked((p) => (on ? [...p, id] : p.filter((x) => x !== id)));

  const scanAgain = () => {
    setRun({});
    setSummary(null);
    setTotal(0);
    refetch();
  };

  const clean = async () => {
    setConfirming(false);
    const ids = selected.map((l) => l.id);
    setSummary(null);
    setTotal(0);
    setRun(Object.fromEntries(ids.map((id) => [id, { phase: "waiting", freedBytes: 0 }])));
    setCleaning(true);
    try {
      const results = await backend.runCleanup(ids, (p) => {
        setTotal(p.totalFreedBytes);
        setRun((r) => ({ ...r, [p.id]: { phase: p.phase, freedBytes: p.freedBytes } }));
      });
      const done = summariseCleanup(results);
      setRun(
        Object.fromEntries(
          results.map((r) => [
            r.id,
            { phase: FINAL[r.status], freedBytes: r.freedBytes, reason: r.error ?? undefined },
          ]),
        ),
      );
      setSummary(done);
      setPicked([]);
      // A declined prompt changed nothing, so it isn't a run.
      if (didClean(done)) {
        markCleaned();
        refetch();
        burst(start.current, { tone: "success", count: 70, reach: 260 });
      }
    } catch (err) {
      setRun({});
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
          {data && (
            <DataSourceBadge
              source="live"
              label={`${rows.some((l) => l.status === "partial") ? "≥ " : ""}${formatBytes(
                rows.reduce((n, l) => n + (l.bytes ?? 0), 0),
              )} found`}
            />
          )}
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

      {cleaning && (
        <section className="flex items-center gap-4 panel px-4 py-3" aria-live="polite">
          <Progress
            className="flex-1"
            value={runIds.length > 0 ? (doneCount / runIds.length) * 100 : 0}
            aria-label="Locations done"
            indicatorClassName="animate-shimmer bg-linear-to-r from-primary via-brand to-primary bg-size-[200%_100%]"
          />
          <span className="shrink-0 text-[12.5px] text-muted-foreground tabular">
            {doneCount} of {runIds.length} done · {formatBytes(total)} freed
          </span>
        </section>
      )}
      {summary && !cleaning && (
        <CleanupReport summary={summary} names={names} onScanAgain={scanAgain} />
      )}

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={7} />
      ) : (
        <ul ref={list} className="flex flex-1 flex-col gap-2">
          {rows.map((l) => {
            const Icon = ICONS[l.id] ?? HardDrive;
            return (
              <li key={l.id} data-stagger className="flex max-h-[68px] min-h-12 flex-1">
                <label
                  title={l.description}
                  className="flex flex-1 cursor-pointer items-center gap-3 panel px-4 transition-colors hover:bg-raised"
                >
                  <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
                  <span className="min-w-0 flex-1 truncate text-[13.5px] font-medium">
                    {l.label}
                  </span>
                  {run[l.id] ? (
                    <RunState state={run[l.id]!} />
                  ) : (
                    <span className="shrink-0 text-[13px] text-muted-foreground tabular">
                      {sizeOf(l)}
                    </span>
                  )}
                  <Checkbox
                    aria-label={l.label}
                    disabled={cleaning}
                    checked={picked.includes(l.id)}
                    onCheckedChange={(on) => toggle(l.id, on === true)}
                  />
                </label>
              </li>
            );
          })}
        </ul>
      )}

      {/* Runs to the window's edge, so rows scrolling behind don't show under the button. */}
      <div className="sticky bottom-0 mt-auto -mb-5 bg-background pt-1 pb-5">
        <Button
          ref={start}
          className="h-12 w-full"
          disabled={selected.length === 0 || cleaning}
          onClick={() => setConfirming(true)}
        >
          {cleaning ? `Cleaning… ${formatBytes(total)} freed` : "Start cleaning"}
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

function RunState({ state }: { state: RowRun }) {
  const { tone, label } = PHASE[state.phase];
  const showSize = state.phase === "finished" || state.phase === "clearing";
  return (
    <span className="flex shrink-0 items-center gap-3 text-[13px] tabular">
      {showSize && <span className="text-muted-foreground">{formatBytes(state.freedBytes)}</span>}
      <StatusLabel key={state.phase} tone={tone} title={state.reason} className="animate-rise">
        {label}
      </StatusLabel>
    </span>
  );
}

/** What a run did: one headline, then a line for each location that didn't clear. */
function CleanupReport({
  summary,
  names,
  onScanAgain,
}: {
  summary: CleanupSummary;
  names: Record<string, string>;
  onScanAgain: () => void;
}) {
  const { freed, skipped, cleared, declined, failures } = summary;
  const onlyDeclined = cleared === 0 && declined.length > 0 && failures.length === 0;
  const headline: ReactNode = onlyDeclined ? (
    "Permission prompt declined, nothing was cleared"
  ) : cleared === 0 ? (
    "Nothing was cleared"
  ) : (
    <>
      Freed <AnimatedNumber value={freed} format={formatBytes} duration={900} />
    </>
  );
  const tone: Tone = onlyDeclined
    ? "warning"
    : cleared === 0
      ? "destructive"
      : failures.length > 0 || declined.length > 0
        ? "warning"
        : "success";
  const name = (id: string) => names[id] ?? id;
  return (
    <section className="flex flex-col gap-2 panel px-4 py-3" aria-live="polite">
      <div className="flex items-center justify-between gap-3">
        <div className="flex min-w-0 items-center gap-3">
          <StatusLabel tone={tone} className="text-[13.5px] font-medium text-foreground">
            {headline}
          </StatusLabel>
          {skipped > 0 && (
            <span className="truncate text-[12.5px] text-muted-foreground">
              {skipped.toLocaleString()} files in use
            </span>
          )}
        </div>
        <Button size="sm" variant="secondary" onClick={onScanAgain}>
          Scan again
        </Button>
      </div>
      {(declined.length > 0 || failures.length > 0) && (
        <ul className="flex flex-col gap-0.5 text-[12.5px] text-muted-foreground">
          {declined.length > 0 && (
            <li data-selectable className="truncate">
              Declined: {declined.map(name).join(", ")}
            </li>
          )}
          {failures.map((f) => (
            <li key={f.id} data-selectable className="truncate" title={f.reason}>
              {name(f.id)}: {f.reason}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
