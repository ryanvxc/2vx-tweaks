import { useNavigate } from "react-router";
import { History, Monitor, SlidersHorizontal } from "lucide-react";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { formatUptime } from "@/lib/format";
import type { SystemSnapshot } from "@/types/system";

// Only destinations that do something today; more join as their sections are built.
const QUICK_ACTIONS = [
  { label: "System tweaks", icon: SlidersHorizontal, to: "/system" },
  { label: "History", icon: History, to: "/history" },
];

export function SystemSummary({ snapshot }: { snapshot: SystemSnapshot | undefined }) {
  const navigate = useNavigate();

  return (
    <Card className="flex-row items-center gap-5 px-5 py-4">
      <div className="flex size-11 shrink-0 items-center justify-center rounded-lg bg-primary/12 text-primary">
        <Monitor className="size-5" />
      </div>

      <div className="min-w-0 flex-1">
        {snapshot ? (
          <>
            <div className="flex items-center gap-2.5">
              <span data-selectable className="truncate text-[15px] font-semibold">
                {snapshot.hostname ?? "This PC"}
              </span>
              <DataSourceBadge source="live" />
            </div>
            <div className="mt-0.5 flex flex-wrap gap-x-3 gap-y-0.5 text-[12.5px] text-muted-foreground">
              <span>
                {snapshot.os.productName}
                {snapshot.os.displayVersion ? ` ${snapshot.os.displayVersion}` : ""}
              </span>
              <Dot />
              <span className="tabular">
                Build {snapshot.os.build}
                {snapshot.os.ubr != null ? `.${snapshot.os.ubr}` : ""}
              </span>
              <Dot />
              <span className="tabular">Up {formatUptime(snapshot.uptimeSecs)}</span>
            </div>
          </>
        ) : (
          <div className="flex flex-col gap-2">
            <Skeleton className="h-4 w-40" />
            <Skeleton className="h-3.5 w-72" />
          </div>
        )}
      </div>

      <div className="flex shrink-0 items-center gap-2">
        {QUICK_ACTIONS.map(({ label, icon: Icon, to }) => (
          <Button key={to} variant="secondary" size="sm" onClick={() => navigate(to)}>
            <Icon /> {label}
          </Button>
        ))}
      </div>
    </Card>
  );
}

function Dot() {
  return <span className="text-muted-foreground/40">•</span>;
}
