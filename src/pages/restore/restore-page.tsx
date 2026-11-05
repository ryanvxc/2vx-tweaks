import { useEffect, useState } from "react";
import { Archive, ChevronDown, History, SlidersHorizontal, Undo2 } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { MessageNotice, StatusLabel } from "@/components/common/notice";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { RecommendedAction } from "@/components/common/recommended-action";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import { refreshEngineState, useRetiredTweaks, useSnapshots, useTweaks } from "@/hooks/use-system";
import {
  CHANGE_STATUS,
  SNAPSHOT_STATUS,
  canRevert,
  describeError,
  describeOutcome,
  formatTimestamp,
} from "@/lib/engine-text";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import type { ChangeRecord, RetiredTweak, RiskLevel, SnapshotRecord } from "@/types/engine";
import type { BackendError } from "@/types/system";

// Shared by the header and every row, so the columns line up.
const COLUMNS = "grid grid-cols-[minmax(0,1fr)_9.5rem_3.25rem_4.5rem_1.5rem] items-center gap-3";

export function RestorePage() {
  const { data, error, loading, refetch } = useSnapshots();
  const { data: tweaks } = useTweaks();
  const { data: retired } = useRetiredTweaks();
  const meta = Object.fromEntries([...(retired ?? []), ...(tweaks ?? [])].map((t) => [t.id, t]));
  const stillApplied = (retired ?? []).filter((t) => t.undoable);
  const names = (ids: string[]) => ids.map((id) => meta[id]?.name ?? id).join(", ") || "—";
  const riskOf = (ids: string[]): RiskLevel =>
    ids.some((id) => meta[id]?.risk === "high")
      ? "high"
      : ids.some((id) => meta[id]?.risk === "moderate")
        ? "moderate"
        : "low";

  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="Restore Centre"
        subtitle={summary(data) ?? "Every change 2VX has made"}
        action={<RecommendedAction />}
      />
      <div className="flex flex-1 flex-col gap-3">
        <EngineBanner showHistoryLink={false} />
        {stillApplied.length > 0 && (
          <section aria-label="Removed from 2VX" className="flex flex-col gap-2">
            <h2 className="text-[13px] font-semibold">Removed from 2VX, still on this PC</h2>
            <CardGrid>
              {stillApplied.map((t) => (
                <RetiredCard key={t.id} tweak={t} />
              ))}
            </CardGrid>
          </section>
        )}
        <div className="flex flex-1 flex-col overflow-hidden panel">
          {error ? (
            <div className="p-4">
              <ErrorState message={error.message} onRetry={refetch} />
            </div>
          ) : loading && !data ? (
            <div className="p-4">
              <LoadingRows />
            </div>
          ) : !data || data.length === 0 ? (
            <EmptyState icon={History} title="No changes yet" className="flex-1" />
          ) : (
            <>
              <div
                className={cn(
                  COLUMNS,
                  "h-9 shrink-0 border-b px-4 text-[12px] text-muted-foreground",
                )}
              >
                <span>Change</span>
                <span>Date</span>
                <span className="text-right">Values</span>
                <span />
                <DataSourceBadge source="live" />
              </div>
              <ul className="divide-y">
                {data.map((s) => (
                  <SnapshotRow
                    key={s.id}
                    snapshot={s}
                    title={names(s.tweakIds)}
                    risk={riskOf(s.tweakIds)}
                    tweakName={(id) => meta[id]?.name ?? id}
                  />
                ))}
              </ul>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

/** 2VX no longer offers this tweak, but what it changed stays until the user undoes it. */
function RetiredCard({ tweak }: { tweak: RetiredTweak }) {
  const [confirming, setConfirming] = useState(false);
  const [reverting, setReverting] = useState(false);

  const revert = () => {
    setConfirming(false);
    setReverting(true);
    backend
      .revertTweak(tweak.id)
      .then(
        (outcome) => {
          const m = describeOutcome(outcome, "revert");
          (m.tone === "success" ? toast.success : toast.warning)(m.title, {
            description: m.description,
          });
        },
        (err: unknown) => {
          const m = describeError(toBackendError(err));
          toast.error(m.title, { description: m.description });
        },
      )
      .finally(() => {
        setReverting(false);
        refreshEngineState();
      });
  };

  return (
    <>
      <OptimisationCard
        icon={Archive}
        title={tweak.name}
        description="This tweak was removed from 2VX. Your setting stays as it is until you undo it."
        control={
          <Button
            size="xs"
            variant="ghost"
            disabled={reverting}
            onClick={() => setConfirming(true)}
          >
            <Undo2 /> Undo
          </Button>
        }
      />
      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title={`Undo: ${tweak.name}`}
        description="Restores the recorded values."
        consequences={["Anything changed since is left as is"]}
        risk={tweak.risk}
        confirmLabel="Undo"
        onConfirm={revert}
      />
    </>
  );
}

function summary(snapshots: SnapshotRecord[] | undefined) {
  if (!snapshots || snapshots.length === 0) return undefined;
  const undoable = snapshots.filter((s) => canRevert(s.kind, s.status)).length;
  return `${snapshots.length} change${snapshots.length === 1 ? "" : "s"} · ${undoable} can be undone`;
}

function SnapshotRow({
  snapshot: s,
  title,
  risk,
  tweakName,
}: {
  snapshot: SnapshotRecord;
  title: string;
  risk: RiskLevel;
  tweakName: (id: string) => string;
}) {
  const [expanded, setExpanded] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [reverting, setReverting] = useState(false);
  const [changes, setChanges] = useState<ChangeRecord[] | null>(null);
  const [changesError, setChangesError] = useState<BackendError | null>(null);
  const status = SNAPSHOT_STATUS[s.status];
  const revertable = canRevert(s.kind, s.status);
  const Icon = s.kind === "apply" ? SlidersHorizontal : Undo2;

  // Reload the change list whenever the snapshot itself changes (e.g. after a revert).
  useEffect(() => {
    if (!expanded) return;
    let live = true;
    backend.getSnapshotChanges(s.id).then(
      (c) => {
        if (!live) return;
        setChanges(c);
        setChangesError(null);
      },
      (err: unknown) => {
        if (live) setChangesError(toBackendError(err));
      },
    );
    return () => {
      live = false;
    };
  }, [expanded, s.id, s.status]);

  const revert = () => {
    setConfirming(false);
    setReverting(true);
    backend
      .revertSnapshot(s.id)
      .then(
        (outcome) => {
          const m = describeOutcome(outcome, "revert");
          (m.tone === "success" ? toast.success : toast.warning)(m.title, {
            description: m.description,
          });
        },
        (err: unknown) => {
          const m = describeError(toBackendError(err));
          toast.error(m.title, { description: m.description });
        },
      )
      .finally(() => {
        setReverting(false);
        refreshEngineState();
      });
  };

  const detailsId = `snapshot-${s.id}`;
  return (
    <li>
      <div className={cn(COLUMNS, "min-h-11 px-4 transition-colors hover:bg-raised")}>
        <div className="flex min-w-0 items-center gap-2.5">
          <Icon className="size-4 shrink-0 text-muted-foreground" aria-hidden />
          <span className="truncate text-[13px] font-medium" title={title}>
            {s.kind === "apply" ? title : `Undid ${title}`}
          </span>
          {s.status !== "completed" && <StatusLabel tone={status.tone}>{status.label}</StatusLabel>}
        </div>
        <span className="text-[12.5px] text-muted-foreground tabular">
          {formatTimestamp(s.createdAtMs)}
        </span>
        {/* A button, so it looks like one: a quiet count that opens the row. */}
        <Button
          size="xs"
          variant="ghost"
          className="justify-self-end text-muted-foreground tabular"
          aria-expanded={expanded}
          aria-controls={detailsId}
          aria-label={`${s.changeCount} value${s.changeCount === 1 ? "" : "s"} in #${s.id}`}
          onClick={() => setExpanded((e) => !e)}
        >
          {s.changeCount}
        </Button>
        {revertable ? (
          <Button
            size="xs"
            variant="ghost"
            disabled={reverting}
            onClick={() => setConfirming(true)}
          >
            <Undo2 /> Undo
          </Button>
        ) : (
          <span />
        )}
        <Button
          size="icon"
          variant="ghost"
          className="size-6"
          aria-expanded={expanded}
          aria-controls={detailsId}
          aria-label={`${expanded ? "Hide" : "Show"} changes in #${s.id}`}
          onClick={() => setExpanded((e) => !e)}
        >
          <ChevronDown className={cn("transition-transform", expanded && "rotate-180")} />
        </Button>
      </div>

      {/* The list stays mounted after collapsing so it has something to close over. */}
      <div
        id={detailsId}
        inert={!expanded}
        className={cn(
          "grid transition-[grid-template-rows] duration-220",
          expanded ? "grid-rows-[1fr]" : "grid-rows-[0fr]",
        )}
      >
        <div className="min-h-0 overflow-hidden">
          {(expanded || changes) && (
            <div className="flex flex-col gap-2 px-4 pb-3 pl-[2.625rem]">
              {s.error && (
                <MessageNotice tone={status.tone} title={status.label} description={s.error} />
              )}
              {changesError ? (
                <ErrorState message={changesError.message} />
              ) : !changes ? (
                <LoadingRows rows={2} />
              ) : changes.length === 0 ? (
                <p className="text-[12.5px] text-muted-foreground">No values were changed.</p>
              ) : (
                <ul className="divide-y rounded-lg bg-background/50">
                  {changes.map((c) => (
                    <li key={c.id} className="flex items-start gap-3 px-3 py-2">
                      <div className="min-w-0 flex-1">
                        <div className="text-[12px] text-muted-foreground">
                          {tweakName(c.tweakId)}
                        </div>
                        <code data-selectable className="font-mono text-[11.5px] break-all">
                          {c.targetLabel}
                        </code>
                        <div className="text-[12px]">
                          {c.beforeLabel} → {c.afterLabel}
                        </div>
                        {c.error && <div className="text-[12px] text-warning">{c.error}</div>}
                      </div>
                      <StatusLabel tone={CHANGE_STATUS[c.status].tone}>
                        {CHANGE_STATUS[c.status].label}
                      </StatusLabel>
                    </li>
                  ))}
                </ul>
              )}
            </div>
          )}
        </div>
      </div>

      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title={`Undo: ${title}`}
        description="Restores the recorded values."
        consequences={[
          `Up to ${s.changeCount} value${s.changeCount === 1 ? "" : "s"}`,
          "Anything changed since is left as is",
        ]}
        risk={risk}
        confirmLabel="Undo"
        onConfirm={revert}
      />
    </li>
  );
}
