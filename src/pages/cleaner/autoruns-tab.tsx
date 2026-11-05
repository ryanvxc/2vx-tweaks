import { useState } from "react";
import {
  CalendarClock,
  FolderOpen,
  KeyRound,
  RefreshCw,
  Rocket,
  SearchX,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar } from "@/components/common/filter-bar";
import { StatusLabel } from "@/components/common/notice";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { useStaggerIn } from "@/hooks/use-motion";
import { useStartupItems } from "@/hooks/use-system";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import type { StartupItem, StartupOrigin, StartupScope, StartupSource } from "@/types/system";

type StateFilter = "on" | "off";

const ICON: Record<StartupSource, LucideIcon> = {
  runKey: KeyRound,
  runOnceKey: KeyRound,
  startupFolder: FolderOpen,
  task: CalendarClock,
};

const SORTS: Record<string, (a: StartupItem, b: StartupItem) => number> = {
  name: (a, b) => a.name.localeCompare(b.name),
  nameDesc: (a, b) => b.name.localeCompare(a.name),
  type: (a, b) => a.origin.localeCompare(b.origin) || a.name.localeCompare(b.name),
  source: (a, b) => a.sourceLabel.localeCompare(b.sourceLabel) || a.name.localeCompare(b.name),
};

const isOn = (item: StartupItem) => item.enabled !== false;

export function AutorunsTab() {
  const { data, error, loading, refetch } = useStartupItems();
  const [search, setSearch] = useState("");
  const [state, setState] = useState<StateFilter[]>([]);
  const [scope, setScope] = useState<StartupScope[]>([]);
  const [origin, setOrigin] = useState<StartupOrigin[]>([]);
  const [source, setSource] = useState<StartupSource[]>([]);
  const [sort, setSort] = useState("name");
  const [busy, setBusy] = useState<string | null>(null);
  const list = useStaggerIn<HTMLUListElement>(data ? "autoruns" : null);

  const items = data?.items ?? [];
  const q = search.trim().toLowerCase();
  const shown = items
    .filter((i) => !q || `${i.name} ${i.command} ${i.publisher ?? ""}`.toLowerCase().includes(q))
    .filter((i) => state.length === 0 || state.includes(isOn(i) ? "on" : "off"))
    .filter((i) => scope.length === 0 || scope.includes(i.scope))
    .filter((i) => origin.length === 0 || origin.includes(i.origin))
    .filter((i) => source.length === 0 || source.includes(i.source))
    .sort(SORTS[sort]);

  const toggle = async (item: StartupItem, on: boolean) => {
    setBusy(item.id);
    try {
      await backend.setStartupEnabled(item.id, on);
      refetch();
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      setBusy(null);
    }
  };

  return (
    <div className="flex flex-1 flex-col gap-3">
      <FilterBar
        search={search}
        onSearch={setSearch}
        placeholder="Search startup"
        filters={[
          {
            id: "state",
            label: "State",
            options: [
              { value: "on", label: "On" },
              { value: "off", label: "Off" },
            ],
            selected: state,
            onChange: (v) => setState(v as StateFilter[]),
          },
          {
            id: "scope",
            label: "Applies to",
            options: [
              { value: "currentUser", label: "Your account" },
              { value: "allUsers", label: "All users" },
            ],
            selected: scope,
            onChange: (v) => setScope(v as StartupScope[]),
          },
          {
            id: "origin",
            label: "Type",
            options: [
              { value: "thirdParty", label: "Third Party" },
              { value: "microsoft", label: "Microsoft" },
              { value: "unknown", label: "Unknown" },
            ],
            selected: origin,
            onChange: (v) => setOrigin(v as StartupOrigin[]),
          },
          {
            id: "source",
            label: "Source",
            options: [
              { value: "runKey", label: "Run key" },
              { value: "runOnceKey", label: "RunOnce key" },
              { value: "startupFolder", label: "Startup folder" },
              { value: "task", label: "Task" },
            ],
            selected: source,
            onChange: (v) => setSource(v as StartupSource[]),
          },
        ]}
        sort={{
          value: sort,
          onChange: setSort,
          options: [
            { value: "name", label: "Name, A to Z" },
            { value: "nameDesc", label: "Name, Z to A" },
            { value: "type", label: "Type" },
            { value: "source", label: "Source" },
          ],
        }}
        end={
          <>
            {data && (
              <DataSourceBadge
                source="live"
                label={`${items.filter(isOn).length} of ${items.length} on`}
              />
            )}
            <Button size="sm" variant="ghost" onClick={refetch} disabled={loading}>
              <RefreshCw className={loading ? "animate-spin" : undefined} /> Refresh
            </Button>
          </>
        }
      />

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={5} />
      ) : shown.length === 0 ? (
        <EmptyState
          icon={items.length === 0 ? Rocket : SearchX}
          title={items.length === 0 ? "Nothing starts with Windows" : "No entries match"}
          className="flex-1 panel"
        />
      ) : (
        <ul ref={list} className="flex flex-col gap-2">
          {shown.map((item) => (
            <AutorunCard
              key={item.id}
              item={item}
              busy={busy !== null}
              onToggle={(on) => void toggle(item, on)}
            />
          ))}
        </ul>
      )}
      {data && data.unreadable.length > 0 && (
        <p className="text-[12px] text-warning">Couldn't read: {data.unreadable.join("; ")}</p>
      )}
    </div>
  );
}

function AutorunCard({
  item,
  busy,
  onToggle,
}: {
  item: StartupItem;
  busy: boolean;
  onToggle: (on: boolean) => void;
}) {
  const Icon = ICON[item.source];
  const task = item.source === "task";
  const blocked = task
    ? "2VX only reads scheduled tasks. Change them in Task Scheduler."
    : item.offInTaskManager
      ? "Turned off in Task Manager. Turn it back on there."
      : "2VX can't switch entries in the all-users Startup folder.";
  const where = `${item.scope === "allUsers" ? "All users" : "Your account"}: ${item.location}`;
  return (
    <li
      data-stagger
      className="flex h-12 items-center gap-3 panel px-4 transition-colors hover:bg-raised"
    >
      <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
      <span
        data-selectable
        title={`${item.path}\n${item.description ?? ""}`.trim()}
        className={cn(
          "min-w-0 flex-1 truncate text-[13.5px] font-medium",
          !isOn(item) && "text-muted-foreground",
        )}
      >
        {item.name}
      </span>
      <span
        className="w-48 shrink-0 truncate text-[12.5px] text-muted-foreground"
        title={item.publisher ? "Reported by the program itself" : undefined}
      >
        {item.publisher ?? ""}
      </span>
      <span className="w-28 shrink-0 truncate text-[12.5px] text-muted-foreground" title={where}>
        {item.sourceLabel}
      </span>
      {/* A fixed slot, so the publisher and source columns line up from row to row. */}
      <div className="flex w-56 shrink-0 items-center justify-end gap-3">
        {item.offInTaskManager && (
          <StatusLabel tone="warning" title={blocked}>
            Off in Task Manager
          </StatusLabel>
        )}
        {!task && item.changeable ? (
          <Switch
            aria-label={`Start ${item.name} with Windows`}
            checked={isOn(item)}
            disabled={busy}
            onCheckedChange={onToggle}
          />
        ) : (
          !item.offInTaskManager && (
            <StatusLabel tone={isOn(item) ? "success" : "neutral"} title={blocked}>
              {isOn(item) ? "On, read-only" : "Off, read-only"}
            </StatusLabel>
          )
        )}
      </div>
    </li>
  );
}
