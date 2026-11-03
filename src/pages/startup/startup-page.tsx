import { useState } from "react";
import { Copy, FolderOpen, KeyRound, Layers, Rocket, SearchX, User, Users } from "lucide-react";
import { toast } from "sonner";
import { FilterBar } from "@/components/common/filter-bar";
import { Indicator } from "@/components/common/indicator";
import { StatusPill } from "@/components/common/notice";
import { PageTabs, type PageTab } from "@/components/common/page-tabs";
import { PlannedFeatures } from "@/components/common/planned-features";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useStartupItems } from "@/hooks/use-system";
import type { StartupItem, StartupScope } from "@/types/system";

const SOURCE_LABEL: Record<StartupItem["source"], string> = {
  runKey: "Run key",
  runOnceKey: "Runs once",
  startupFolder: "Startup folder",
};

export function StartupPage() {
  const { data, error, loading, refetch } = useStartupItems();
  const [tab, setTab] = useState<StartupScope | "all">("all");
  const [search, setSearch] = useState("");
  const items = data?.items ?? [];
  const q = search.trim().toLowerCase();
  const shown = items
    .filter((i) => tab === "all" || i.scope === tab)
    .filter((i) => !q || i.name.toLowerCase().includes(q) || i.command.toLowerCase().includes(q));
  const disabled = items.filter((i) => i.enabled === false).length;

  const tabs: PageTab<StartupScope | "all">[] = [
    { value: "all", label: "All", icon: Layers, count: items.length },
    {
      value: "currentUser",
      label: "Your account",
      icon: User,
      count: items.filter((i) => i.scope === "currentUser").length,
    },
    {
      value: "allUsers",
      label: "All users",
      icon: Users,
      count: items.filter((i) => i.scope === "allUsers").length,
    },
  ];

  return (
    <>
      <PageHeader
        title="Startup"
        description="Everything Windows starts when you sign in, read from the Run keys and Startup folders"
        actions={
          <Button size="sm" variant="outline" onClick={refetch} disabled={loading}>
            Refresh
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
              <span className="text-[12px] text-muted-foreground tabular">
                {items.length} entries · {disabled} disabled in Task Manager
              </span>
            )
          }
        />
        <SectionCard title="Startup entries" description="Read-only" source="live">
          {error ? (
            <ErrorState message={error.message} onRetry={refetch} />
          ) : loading && !data ? (
            <LoadingRows rows={5} />
          ) : shown.length === 0 ? (
            <EmptyState
              icon={items.length === 0 ? Rocket : SearchX}
              title={items.length === 0 ? "Nothing starts with Windows" : "No entries match"}
              description={
                items.length === 0
                  ? "No Run-key or Startup-folder entries were found."
                  : "Try another search."
              }
            />
          ) : (
            <ul className="-mx-2 flex flex-col">
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
        <PlannedFeatures
          features={[
            "Disable and re-enable entries with an undo, through the change journal",
            "Publisher and signature details for each entry",
            "Scheduled tasks and services that start at sign-in",
            "Startup impact where Windows reports it",
          ]}
        />
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
    <li className="group flex items-center gap-3 rounded-md px-2 py-2.5 hover:bg-accent/40">
      <SourceIcon className="size-4 shrink-0 text-brand" aria-hidden />
      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-2">
          <span className="truncate text-[13px] font-medium">{item.name}</span>
          <StatusPill tone="neutral">{SOURCE_LABEL[item.source]}</StatusPill>
          <StatusPill tone="neutral">
            {item.scope === "allUsers" ? "All users" : "Your account"}
          </StatusPill>
        </div>
        <code
          data-selectable
          className="block truncate font-mono text-[11.5px] text-muted-foreground"
          title={item.command}
        >
          {item.command}
        </code>
        <div className="truncate text-[11px] text-muted-foreground/70" title={item.location}>
          {item.location}
        </div>
      </div>
      {item.enabled === false ? (
        <Indicator icon={Rocket} tone="neutral" title="Disabled in Task Manager">
          Listed here but turned off in Task Manager's Startup tab, so Windows skips it.
        </Indicator>
      ) : (
        <Indicator icon={Rocket} tone="success" title="Starts with Windows">
          {item.enabled === true
            ? "Enabled in Task Manager's Startup tab."
            : "Task Manager has no record for it, so it runs."}
        </Indicator>
      )}
      <Button
        size="icon-xs"
        variant="ghost"
        aria-label={`Copy command for ${item.name}`}
        onClick={copy}
        className="opacity-0 group-hover:opacity-100 focus-visible:opacity-100"
      >
        <Copy />
      </Button>
    </li>
  );
}
