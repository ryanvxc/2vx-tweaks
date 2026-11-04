import { useState } from "react";
import { Copy, FolderOpen, KeyRound, RefreshCw, Rocket, SearchX } from "lucide-react";
import { STARTUP_TABS } from "@/app/navigation";
import { toast } from "sonner";
import { FilterBar } from "@/components/common/filter-bar";
import { Indicator } from "@/components/common/indicator";
import { PageTabs } from "@/components/common/page-tabs";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useStaggerIn } from "@/hooks/use-motion";
import { useStartupItems } from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import type { StartupItem } from "@/types/system";

const SOURCE_LABEL: Record<StartupItem["source"], string> = {
  runKey: "Run key",
  runOnceKey: "Runs once",
  startupFolder: "Startup folder",
};

export function StartupPage() {
  const { data, error, loading, refetch } = useStartupItems();
  const [tab, setTab] = useTabParam(STARTUP_TABS, "all");
  const [search, setSearch] = useState("");
  const items = data?.items ?? [];
  const q = search.trim().toLowerCase();
  const shown = items
    .filter((i) => tab === "all" || i.scope === tab)
    .filter((i) => !q || i.name.toLowerCase().includes(q) || i.command.toLowerCase().includes(q));
  const disabled = items.filter((i) => i.enabled === false).length;
  const list = useStaggerIn<HTMLUListElement>(data ? tab : null);

  const tabs = STARTUP_TABS.map((t) => ({
    ...t,
    count: t.value === "all" ? items.length : items.filter((i) => i.scope === t.value).length,
  }));

  return (
    <>
      <PageHeader
        title="Startup"
        actions={
          <Button size="sm" variant="outline" onClick={refetch} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : undefined} /> Refresh
          </Button>
        }
      />
      <div className="flex flex-col gap-4">
        <PageTabs label="Scope" tabs={tabs} value={tab} onChange={setTab} />
        <FilterBar
          search={search}
          onSearch={setSearch}
          placeholder="Search name or command"
          end={
            data && (
              <span className="text-[12px] text-muted-foreground tabular">{disabled} disabled</span>
            )
          }
        />
        <SectionCard title="Startup entries" source="live">
          {error ? (
            <ErrorState message={error.message} onRetry={refetch} />
          ) : loading && !data ? (
            <LoadingRows rows={5} />
          ) : shown.length === 0 ? (
            <EmptyState
              icon={items.length === 0 ? Rocket : SearchX}
              title={items.length === 0 ? "Nothing starts with Windows" : "No entries match"}
            />
          ) : (
            <ul ref={list} className="-mx-2 flex flex-col">
              {shown.map((i) => (
                <StartupRow key={i.id} item={i} />
              ))}
            </ul>
          )}
          {data && data.unreadable.length > 0 && (
            <p className="mt-3 text-[12px] text-warning">
              Couldn't read: {data.unreadable.join("; ")}
            </p>
          )}
        </SectionCard>
      </div>
    </>
  );
}

function StartupRow({ item }: { item: StartupItem }) {
  const SourceIcon = item.source === "startupFolder" ? FolderOpen : KeyRound;
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(item.command);
      toast.success("Command copied");
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };
  return (
    <li
      data-stagger
      className="group flex items-center gap-3 rounded-md px-2 py-2 transition-colors hover:bg-accent/40"
    >
      <SourceIcon className="size-4 shrink-0 text-brand" aria-hidden />
      <div className="min-w-0 flex-1">
        <div className="flex items-baseline gap-2">
          <span className="truncate text-[13px] font-medium" title={item.location}>
            {item.name}
          </span>
          <span className="shrink-0 text-[12px] text-muted-foreground">
            {SOURCE_LABEL[item.source]} · {item.scope === "allUsers" ? "All users" : "Your account"}
          </span>
        </div>
        <code
          data-selectable
          className="block truncate font-mono text-[11.5px] text-muted-foreground"
          title={item.command}
        >
          {item.command}
        </code>
      </div>
      {item.enabled === false ? (
        <Indicator icon={Rocket} tone="neutral" title="Disabled in Task Manager" />
      ) : (
        <Indicator icon={Rocket} tone="success" title="Starts with Windows" />
      )}
      <Button
        size="icon-xs"
        variant="ghost"
        aria-label={`Copy command for ${item.name}`}
        onClick={copy}
        className="opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
      >
        <Copy />
      </Button>
    </li>
  );
}
