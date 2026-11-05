import { createElement, useState } from "react";
import { Download, SearchX, Trash2 } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar } from "@/components/common/filter-bar";
import { CardGrid, CardSkeletons, OptimisationCard } from "@/components/common/optimisation-card";
import { EmptyState, ErrorState } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useStaggerIn } from "@/hooks/use-motion";
import { useRemovableApps } from "@/hooks/use-system";
import { appIcon } from "@/lib/app-icons";
import { backend, toBackendError } from "@/services/backend";
import type { RemovableApp } from "@/types/system";

type StateFilter = "installed" | "missing";

const SORTS: Record<string, (a: RemovableApp, b: RemovableApp) => number> = {
  name: (a, b) => a.name.localeCompare(b.name),
  nameDesc: (a, b) => b.name.localeCompare(a.name),
  installed: (a, b) => Number(b.installed) - Number(a.installed) || a.name.localeCompare(b.name),
};

function AppLogo({ app }: { app: RemovableApp }) {
  if (app.icon) return <img src={app.icon} alt="" className="size-6 shrink-0 object-contain" />;
  return createElement(appIcon(app.id), {
    className: "size-[22px] shrink-0 text-brand",
    strokeWidth: 1.75,
    "aria-hidden": true,
  });
}

export function UninstallTab() {
  const { data, error, loading, refetch } = useRemovableApps();
  const [search, setSearch] = useState("");
  const [state, setState] = useState<StateFilter[]>([]);
  const [sort, setSort] = useState("installed");
  const [busy, setBusy] = useState<string | null>(null);
  const [removing, setRemoving] = useState<RemovableApp | null>(null);
  const list = useStaggerIn<HTMLDivElement>(data ? "apps" : null);

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
            { value: "installed", label: "Installed first" },
            { value: "name", label: "Name, A to Z" },
            { value: "nameDesc", label: "Name, Z to A" },
          ],
        }}
        end={
          data && (
            <DataSourceBadge
              source="live"
              label={`${data.filter((a) => a.installed).length} of ${data.length} installed`}
            />
          )
        }
      />

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <CardSkeletons />
      ) : shown.length === 0 ? (
        <EmptyState icon={SearchX} title="No apps match" className="flex-1 panel" />
      ) : (
        <div ref={list}>
          <CardGrid>
            {shown.map((app) => (
              <OptimisationCard
                key={app.id}
                leading={<AppLogo app={app} />}
                title={app.name}
                description={app.description}
                footer={
                  app.installed ? (
                    <Button
                      className="w-full"
                      disabled={busy !== null}
                      onClick={() => setRemoving(app)}
                    >
                      <Trash2 /> {busy === app.id ? "Removing…" : "Uninstall"}
                    </Button>
                  ) : (
                    <Button
                      variant="outline"
                      className="w-full"
                      disabled={busy !== null}
                      onClick={() => void run(app, backend.reinstallApp, "is back")}
                    >
                      <Download /> {busy === app.id ? "Restoring…" : "Reinstall"}
                    </Button>
                  )
                }
              />
            ))}
          </CardGrid>
        </div>
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
