import { Link } from "react-router";
import { History } from "lucide-react";
import { StatusPill } from "@/components/common/notice";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useSnapshots, useTweaks } from "@/hooks/use-system";
import { SNAPSHOT_STATUS, formatTimestamp } from "@/lib/engine-text";

export function RecentActivity() {
  const { data, error, loading, refetch } = useSnapshots();
  const { data: tweaks } = useTweaks();
  const names = Object.fromEntries((tweaks ?? []).map((t) => [t.id, t.name]));
  const recent = (data ?? []).slice(0, 4);

  return (
    <SectionCard
      title="Recent changes"
      description="Changes made by 2VX"
      source="live"
      actions={
        recent.length > 0 && (
          <Button asChild size="xs" variant="ghost">
            <Link to="/history">View all</Link>
          </Button>
        )
      }
    >
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={2} />
      ) : recent.length === 0 ? (
        <EmptyState
          icon={History}
          title="No changes yet"
          description="Every change 2VX makes is listed here and in History, with an undo."
          className="py-6"
        />
      ) : (
        <ul className="-mx-2 flex flex-col">
          {recent.map((s) => {
            const status = SNAPSHOT_STATUS[s.status];
            return (
              <li key={s.id} className="flex items-center gap-3 rounded-md px-2 py-2">
                <div className="min-w-0 flex-1">
                  <div className="truncate text-[13px] leading-snug font-medium">
                    {s.kind === "apply" ? "Applied" : "Undid"}:{" "}
                    {s.tweakIds.map((id) => names[id] ?? id).join(", ")}
                  </div>
                  <div className="text-[12px] text-muted-foreground">
                    {formatTimestamp(s.createdAtMs)}
                  </div>
                </div>
                <StatusPill tone={status.tone}>{status.label}</StatusPill>
              </li>
            );
          })}
        </ul>
      )}
    </SectionCard>
  );
}
