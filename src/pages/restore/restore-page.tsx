import { useEffect, useState } from "react";
import { ChevronDown, History, SlidersHorizontal, Undo2 } from "lucide-react";
import { pageDescription } from "@/app/navigation";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { EngineBanner } from "@/components/common/engine-banner";
import { MessageNotice, StatusPill } from "@/components/common/notice";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { refreshEngineState, useSnapshots, useTweaks } from "@/hooks/use-system";
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
import type { ChangeRecord, RiskLevel, SnapshotRecord } from "@/types/engine";
import type { BackendError } from "@/types/system";

export function RestorePage() {
  const { data, error, loading, refetch } = useSnapshots();
  const { data: tweaks } = useTweaks();
  const meta = Object.fromEntries((tweaks ?? []).map((t) => [t.id, t]));
  const names = (ids: string[]) => ids.map((id) => meta[id]?.name ?? id).join(", ") || "—";
  const riskOf = (ids: string[]): RiskLevel =>
    ids.some((id) => meta[id]?.risk === "high")
      ? "high"
      : ids.some((id) => meta[id]?.risk === "moderate")
        ? "moderate"
        : "low";

  return (
    <>
      <PageHeader title="Restore Centre" description={pageDescription("/restore")} />
      <div className="flex flex-col gap-3">
        <EngineBanner showHistoryLink={false} />
        {data && data.length > 0 && <Summary snapshots={data} />}
        <SectionCard title="Changes" source="live">
          {error ? (
            <ErrorState message={error.message} onRetry={refetch} />
          ) : loading && !data ? (
            <LoadingRows />
          ) : !data || data.length === 0 ? (
            <EmptyState
              icon={History}
              title="No changes yet"
              description="Every change 2VX makes is recorded here with its old value, so it can be undone."
            />
          ) : (
            <ul className="-mx-2 flex flex-col">
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
          )}
        </SectionCard>
      </div>
    </>
  );
}

function Summary({ snapshots }: { snapshots: SnapshotRecord[] }) {
  const undoable = snapshots.filter((s) => canRevert(s.kind, s.status)).length;
  const last = snapshots[0]!;
  return (
    <dl className="grid grid-cols-3 gap-3">
      <Figure label="Batches recorded" value={String(snapshots.length)} />
      <Figure label="Can be undone" value={String(undoable)} />
      <Figure label="Last change" value={formatTimestamp(last.createdAtMs)} />
    </dl>
  );
}

function Figure({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl border bg-card px-4 py-2.5">
      <dt className="text-[12.5px] text-muted-foreground">{label}</dt>
      <dd className="truncate font-heading text-[18px] leading-tight font-bold tabular">{value}</dd>
    </div>
  );
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
    <li className="rounded-md px-2 py-2.5">
      <div className="flex items-center gap-3">
        <div className="flex size-8 shrink-0 items-center justify-center rounded-md bg-muted">
          {s.kind === "apply" ? (
            <SlidersHorizontal className="size-4 text-muted-foreground" />
          ) : (
            <Undo2 className="size-4 text-muted-foreground" />
          )}
        </div>
        <div className="min-w-0 flex-1">
          <div className="truncate text-[13px] leading-snug font-medium">
            {s.kind === "apply" ? "Applied" : "Undid"}: {title}
          </div>
          <div className="text-[12px] leading-snug text-muted-foreground">
            {formatTimestamp(s.createdAtMs)} · {s.changeCount} value
            {s.changeCount === 1 ? "" : "s"}
            {s.reverts != null && ` · undoes #${s.reverts}`} · #{s.id}
          </div>
        </div>
        <StatusPill tone={status.tone}>{status.label}</StatusPill>
        {revertable && (
          <Button
            size="xs"
            variant="outline"
            disabled={reverting}
            onClick={() => setConfirming(true)}
          >
            <Undo2 /> Undo
          </Button>
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

      {expanded && (
        <div id={detailsId} className="mt-2.5 ml-11 flex flex-col gap-2">
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
            <ul className="divide-y rounded-md border">
              {changes.map((c) => (
                <li key={c.id} className="flex items-start gap-3 px-3 py-2">
                  <div className="min-w-0 flex-1">
                    <div className="text-[12px] text-muted-foreground">{tweakName(c.tweakId)}</div>
                    <code data-selectable className="font-mono text-[11.5px] break-all">
                      {c.targetLabel}
                    </code>
                    <div className="text-[12px]">
                      {c.beforeLabel} → {c.afterLabel}
                    </div>
                    {c.error && <div className="text-[12px] text-warning">{c.error}</div>}
                  </div>
                  <StatusPill tone={CHANGE_STATUS[c.status].tone}>
                    {CHANGE_STATUS[c.status].label}
                  </StatusPill>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}

      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title={`Undo: ${title}`}
        description="2VX puts back the values it recorded before this change."
        consequences={[
          `Restores up to ${s.changeCount} value${s.changeCount === 1 ? "" : "s"} to what they were before.`,
          "A value changed by something else since then is left as is and reported.",
          "Machine-wide values ask for administrator permission once.",
        ]}
        risk={risk}
        confirmLabel="Undo"
        onConfirm={revert}
      />
    </li>
  );
}
