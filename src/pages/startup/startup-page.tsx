import { useState } from "react";
import { Copy, FolderOpen, KeyRound, RefreshCw, Rocket, SearchX } from "lucide-react";
import { STARTUP_TABS } from "@/app/navigation";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar } from "@/components/common/filter-bar";
import { StatusLabel } from "@/components/common/notice";
import { PageTabs } from "@/components/common/page-tabs";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useStaggerIn } from "@/hooks/use-motion";
import { useStartupItems } from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import { cn } from "@/lib/utils";
import type { StartupItem } from "@/types/system";

const SOURCE_LABEL: Record<StartupItem["source"], string> = {
  runKey: "Run key",
  runOnceKey: "Runs once",
  startupFolder: "Startup folder",
};

// Shared by the header and every row, so the columns line up.
const COLUMNS =
  "grid grid-cols-[13rem_minmax(0,1fr)_6.5rem_6rem_2.75rem_1.5rem] items-center gap-3";

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
        description={disabled > 0 ? `${disabled} disabled` : undefined}
        actions={
          <Button size="sm" variant="ghost" onClick={refetch} disabled={loading}>
            <RefreshCw className={loading ? "animate-spin" : undefined} /> Refresh
          </Button>
        }
      />
      <div className="flex flex-1 flex-col gap-3">
        <PageTabs
          label="Scope"
          tabs={tabs}
          value={tab}
          onChange={setTab}
          end={<FilterBar search={search} onSearch={setSearch} placeholder="Search startup" />}
        />
        <div className="flex flex-1 flex-col overflow-hidden panel">
          {error ? (
            <div className="p-4">
              <ErrorState message={error.message} onRetry={refetch} />
            </div>
          ) : loading && !data ? (
            <div className="p-4">
              <LoadingRows rows={5} />
            </div>
          ) : shown.length === 0 ? (
            <EmptyState
              icon={items.length === 0 ? Rocket : SearchX}
              title={items.length === 0 ? "Nothing starts with Windows" : "No entries match"}
              className="flex-1"
            />
          ) : (
            <>
              <div
                className={cn(
                  COLUMNS,
                  "h-9 shrink-0 border-b px-4 text-[12px] text-muted-foreground",
                )}
              >
                <span>Name</span>
                <span>Command</span>
                <span>Source</span>
                <span>Scope</span>
                <span>State</span>
                <DataSourceBadge source="live" />
              </div>
              <ul ref={list} className="divide-y">
                {shown.map((i) => (
                  <StartupRow key={i.id} item={i} />
                ))}
              </ul>
            </>
          )}
          {data && data.unreadable.length > 0 && (
            <p className="mt-auto border-t px-4 py-2 text-[12px] text-warning">
              Couldn't read: {data.unreadable.join("; ")}
            </p>
          )}
        </div>
      </div>
    </>
  );
}

function StartupRow({ item }: { item: StartupItem }) {
  const SourceIcon = item.source === "startupFolder" ? FolderOpen : KeyRound;
  const off = item.enabled === false;
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(item.command);
      toast.success("Command copied");
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };
  return (
    <li data-stagger className={cn(COLUMNS, "group h-10 px-4 transition-colors hover:bg-raised")}>
      <div className="flex min-w-0 items-center gap-2.5">
        <SourceIcon className="size-4 shrink-0 text-brand" aria-hidden />
        <span className="truncate text-[13px] font-medium" title={item.location}>
          {item.name}
        </span>
      </div>
      <code
        data-selectable
        className="truncate font-mono text-[11.5px] text-muted-foreground"
        title={item.command}
      >
        {item.command}
      </code>
      <span className="truncate text-[12.5px] text-muted-foreground">
        {SOURCE_LABEL[item.source]}
      </span>
      <span className="truncate text-[12.5px] text-muted-foreground">
        {item.scope === "allUsers" ? "All users" : "Your account"}
      </span>
      <StatusLabel
        tone={off ? "neutral" : "success"}
        title={off ? "Disabled in Task Manager" : "Starts with Windows"}
      >
        {off ? "Off" : "On"}
      </StatusLabel>
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
