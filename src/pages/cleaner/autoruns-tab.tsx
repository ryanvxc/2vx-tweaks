import { useState } from "react";
import { Copy, RefreshCw, Rocket, SearchX } from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar } from "@/components/common/filter-bar";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { useStaggerIn } from "@/hooks/use-motion";
import { useStartupItems } from "@/hooks/use-system";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import type { StartupItem, StartupOrigin, StartupScope } from "@/types/system";

const AUTORUNS_URL = "https://learn.microsoft.com/sysinternals/downloads/autoruns";

type StateFilter = "on" | "off";

const ORIGIN: Record<StartupOrigin, { label: string; className: string }> = {
  microsoft: { label: "Microsoft", className: "bg-muted text-muted-foreground" },
  thirdParty: { label: "Third Party", className: "bg-warning/12 text-warning" },
  unknown: { label: "Unknown", className: "bg-muted text-muted-foreground" },
};

const SOURCE: Record<StartupItem["source"], string> = {
  runKey: "Run key",
  runOnceKey: "Runs once",
  startupFolder: "Startup folder",
};

const SORTS: Record<string, (a: StartupItem, b: StartupItem) => number> = {
  name: (a, b) => a.name.localeCompare(b.name),
  nameDesc: (a, b) => b.name.localeCompare(a.name),
  type: (a, b) => a.origin.localeCompare(b.origin) || a.name.localeCompare(b.name),
};

const isOn = (item: StartupItem) => item.enabled !== false;

export function AutorunsTab() {
  const { data, error, loading, refetch } = useStartupItems();
  const [search, setSearch] = useState("");
  const [state, setState] = useState<StateFilter[]>([]);
  const [scope, setScope] = useState<StartupScope[]>([]);
  const [origin, setOrigin] = useState<StartupOrigin[]>([]);
  const [sort, setSort] = useState("name");
  const [busy, setBusy] = useState<string | null>(null);
  const list = useStaggerIn<HTMLUListElement>(data ? "autoruns" : null);

  const items = data?.items ?? [];
  const q = search.trim().toLowerCase();
  const shown = items
    .filter((i) => !q || `${i.name} ${i.command}`.toLowerCase().includes(q))
    .filter((i) => state.length === 0 || state.includes(isOn(i) ? "on" : "off"))
    .filter((i) => scope.length === 0 || scope.includes(i.scope))
    .filter((i) => origin.length === 0 || origin.includes(i.origin))
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

  const copyLink = async () => {
    try {
      await navigator.clipboard.writeText(AUTORUNS_URL);
      toast.success("Link copied");
    } catch {
      toast.error("Couldn't access the clipboard");
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
        ]}
        sort={{
          value: sort,
          onChange: setSort,
          options: [
            { value: "name", label: "Name, A to Z" },
            { value: "nameDesc", label: "Name, Z to A" },
            { value: "type", label: "Type" },
          ],
        }}
        end={
          <>
            <DataSourceBadge source="live" />
            <Button size="sm" variant="ghost" onClick={refetch} disabled={loading}>
              <RefreshCw className={loading ? "animate-spin" : undefined} /> Refresh
            </Button>
          </>
        }
      />

      <section className="flex items-center gap-4 panel px-4 py-3">
        <div className="min-w-0 flex-1">
          <h2 className="text-[13.5px] font-semibold">Autoruns</h2>
          <p data-selectable className="text-[12.5px] text-muted-foreground">
            What starts when you sign in. An entry you turn off leaves Task Manager's list until you
            turn it back on. For services, drivers and scheduled tasks, use Sysinternals Autoruns.
          </p>
        </div>
        <Button size="sm" variant="secondary" onClick={() => void copyLink()}>
          <Copy /> Copy link
        </Button>
      </section>

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
  const origin = ORIGIN[item.origin];
  const blocked = item.offInTaskManager
    ? "Turned off in Task Manager. Turn it back on there."
    : "2VX can't switch entries in the all-users Startup folder.";
  return (
    <li data-stagger className="flex flex-col gap-2 panel px-4 py-3">
      <div className="flex items-center justify-between gap-3">
        <h3
          className={cn(
            "truncate text-[13.5px] font-semibold",
            !isOn(item) && "text-muted-foreground",
          )}
        >
          {item.name}
        </h3>
        <span title={item.changeable ? undefined : blocked}>
          <Switch
            aria-label={`Start ${item.name} with Windows`}
            checked={isOn(item)}
            disabled={busy || !item.changeable}
            onCheckedChange={onToggle}
          />
        </span>
      </div>
      <dl className="grid grid-cols-[6rem_minmax(0,1fr)] items-center gap-x-3 gap-y-1 text-[12.5px]">
        <dt className="text-muted-foreground">Path</dt>
        <dd data-selectable className="truncate" title={item.command}>
          {item.path}
        </dd>
        <dt className="text-muted-foreground">Type</dt>
        <dd className="flex items-center gap-2">
          <span
            title={item.publisher ?? undefined}
            className={cn("rounded px-1.5 py-px text-[11.5px] font-medium", origin.className)}
          >
            {origin.label}
          </span>
          <span className="text-muted-foreground" title={item.location}>
            {SOURCE[item.source]} · {item.scope === "allUsers" ? "All users" : "Your account"}
          </span>
          {item.offInTaskManager && <span className="text-warning">Off in Task Manager</span>}
        </dd>
        <dt className="text-muted-foreground">Description</dt>
        <dd className="truncate text-muted-foreground">{item.description ?? "None given"}</dd>
      </dl>
    </li>
  );
}
