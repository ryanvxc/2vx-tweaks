import { useState } from "react";
import { Download, Package, SearchX, Trash2 } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar } from "@/components/common/filter-bar";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useStaggerIn } from "@/hooks/use-motion";
import { useRemovableApps } from "@/hooks/use-system";
import { backend, toBackendError } from "@/services/backend";
import type { RemovableApp } from "@/types/system";

type StateFilter = "installed" | "missing";

const SORTS: Record<string, (a: RemovableApp, b: RemovableApp) => number> = {
  name: (a, b) => a.name.localeCompare(b.name),
  nameDesc: (a, b) => b.name.localeCompare(a.name),
  installed: (a, b) => Number(b.installed) - Number(a.installed) || a.name.localeCompare(b.name),
};

export function UninstallTab() {
  const { data, error, loading, refetch } = useRemovableApps();
  const [search, setSearch] = useState("");
  const [state, setState] = useState<StateFilter[]>([]);
  const [sort, setSort] = useState("name");
  const [busy, setBusy] = useState<string | null>(null);
  const [removing, setRemoving] = useState<RemovableApp | null>(null);
  const list = useStaggerIn<HTMLUListElement>(data ? "apps" : null);

  const q = search.trim().toLowerCase();
  const shown = [...(data ?? [])]
    .filter((a) => !q || `${a.name} ${a.description}`.toLowerCase().includes(q))
    .filter((a) => state.length === 0 || state.includes(a.installed ? "installed" : "missing"))
    .sort(SORTS[sort]);

  const run = async (app: RemovableApp, action: (id: string) => Promise<void>, done: string) => {
    setBusy(app.id);
    try {
      await action(app.id);
      toast.success(`${app.name} ${done}`);
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
        placeholder="Search apps"
        filters={[
          {
            id: "state",
            label: "State",
            options: [
              { value: "installed", label: "Installed" },
              { value: "missing", label: "Not installed" },
            ],
            selected: state,
            onChange: (v) => setState(v as StateFilter[]),
          },
        ]}
        sort={{
          value: sort,
          onChange: setSort,
          options: [
            { value: "name", label: "Name, A to Z" },
            { value: "nameDesc", label: "Name, Z to A" },
            { value: "installed", label: "Installed first" },
          ],
        }}
        end={<DataSourceBadge source="live" />}
      />

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={7} />
      ) : shown.length === 0 ? (
        <EmptyState icon={SearchX} title="No apps match" className="flex-1 panel" />
      ) : (
        <ul ref={list} className="flex flex-col gap-2">
          {shown.map((app) => (
            <li
              key={app.id}
              data-stagger
              className="flex h-14 items-center gap-3.5 panel px-4 transition-colors hover:bg-raised"
            >
              {app.icon ? (
                <img src={app.icon} alt="" className="size-8 shrink-0 rounded-md object-contain" />
              ) : (
                <span className="flex size-8 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
                  <Package className="size-4" aria-hidden />
                </span>
              )}
              <span className="min-w-0 flex-1 truncate text-[13.5px] font-medium">{app.name}</span>
              <span className="truncate text-right text-[12.5px] text-muted-foreground">
                {app.description}
              </span>
              {app.installed ? (
                <Button
                  size="sm"
                  className="w-28 shrink-0"
                  disabled={busy !== null}
                  onClick={() => setRemoving(app)}
                >
                  <Trash2 /> {busy === app.id ? "Removing…" : "Uninstall"}
                </Button>
              ) : (
                <Button
                  size="sm"
                  variant="secondary"
                  className="w-28 shrink-0"
                  disabled={busy !== null}
                  onClick={() => void run(app, backend.reinstallApp, "is back")}
                >
                  <Download /> {busy === app.id ? "Restoring…" : "Reinstall"}
                </Button>
              )}
            </li>
          ))}
        </ul>
      )}

      <ConfirmDialog
        open={removing !== null}
        onOpenChange={(open) => !open && setRemoving(null)}
        title={`Uninstall ${removing?.name ?? ""}?`}
        description="It's removed for your account only."
        consequences={[
          "Its data for your account goes with it.",
          "Reinstall brings it back while Windows keeps a copy. After that, use Microsoft Store.",
        ]}
        risk="moderate"
        confirmLabel="Uninstall"
        onConfirm={() => {
          if (removing) void run(removing, backend.uninstallApp, "removed");
          setRemoving(null);
        }}
      />
    </div>
  );
}
