import { NavLink } from "react-router";
import { PanelLeftClose, PanelLeftOpen, Search, ShieldAlert, ShieldCheck } from "lucide-react";
import logo from "@/assets/logo.svg";
import { NAV_GROUPS, SETTINGS_ITEM, type NavItem } from "@/app/navigation";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useAppInfo } from "@/hooks/use-system";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import { UpdateButton } from "./update-button";

export function Sidebar({ onSearch }: { onSearch: () => void }) {
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  const toggle = useUiStore((s) => s.toggleSidebar);
  const { data: app } = useAppInfo();
  const PrivilegeIcon = app?.isElevated ? ShieldAlert : ShieldCheck;
  const privilege = app?.isElevated ? "Administrator" : "Standard user";
  const privilegeTone = app?.isElevated ? "text-warning" : "text-success";

  const searchButton = (
    <button
      type="button"
      onClick={onSearch}
      aria-label="Search"
      className={cn(
        "flex h-8.5 w-full items-center gap-2.5 rounded-md border bg-card/60 px-2.5 text-[13px] text-muted-foreground transition-colors hover:bg-sidebar-accent/60 hover:text-foreground",
        collapsed && "justify-center border-transparent bg-transparent px-0",
      )}
    >
      <Search className="size-4 shrink-0" aria-hidden />
      {!collapsed && (
        <>
          <span className="flex-1 text-left">Search</span>
          <kbd className="font-sans text-[11px] text-muted-foreground/70">Ctrl K</kbd>
        </>
      )}
    </button>
  );

  return (
    <aside
      className={cn(
        "flex h-full shrink-0 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground transition-[width] duration-200",
        collapsed ? "w-16" : "w-(--sidebar-width)",
      )}
    >
      <div
        data-tauri-drag-region
        className={cn(
          "flex h-14 shrink-0 items-center gap-2.5",
          collapsed ? "justify-center" : "px-4",
        )}
      >
        <img src={logo} alt="" className="pointer-events-none size-7" draggable={false} />
        {!collapsed && (
          <span
            data-tauri-drag-region
            className="font-heading text-[16px] font-bold tracking-tight"
          >
            2VX <span className="font-medium text-muted-foreground">Tweaks</span>
          </span>
        )}
      </div>

      <div className="px-2.5 pb-1">
        {collapsed ? (
          <Tooltip>
            <TooltipTrigger asChild>{searchButton}</TooltipTrigger>
            <TooltipContent side="right">Search (Ctrl K)</TooltipContent>
          </Tooltip>
        ) : (
          searchButton
        )}
      </div>

      <nav className="flex flex-1 flex-col gap-3.5 overflow-y-auto px-2.5 pt-2.5" aria-label="Main">
        {NAV_GROUPS.map((group, i) => (
          <div key={group.label} className="flex flex-col gap-0.5">
            {!collapsed && (
              <div className="mb-0.5 px-2.5 text-[11.5px] font-medium text-muted-foreground/60">
                {group.label}
              </div>
            )}
            {collapsed && i > 0 && <div className="mx-3 mb-2 h-px bg-sidebar-border" />}
            {group.items.map((item) => (
              <SidebarLink key={item.to} item={item} collapsed={collapsed} />
            ))}
          </div>
        ))}
      </nav>

      <div className="flex flex-col gap-0.5 border-t border-sidebar-border px-2.5 py-2.5">
        <UpdateButton collapsed={collapsed} />
        <SidebarLink item={SETTINGS_ITEM} collapsed={collapsed} />
        <div
          className={cn(
            "mt-1.5 flex items-center gap-2 rounded-lg",
            collapsed ? "flex-col" : "bg-sidebar-accent/50 py-1.5 pr-1.5 pl-2.5",
          )}
        >
          {app && !collapsed && (
            <>
              <PrivilegeIcon className={cn("size-4 shrink-0", privilegeTone)} />
              <div className="min-w-0 flex-1 leading-tight">
                <div className="truncate text-[12px] font-medium text-foreground">{privilege}</div>
                <div className="text-[11px] text-muted-foreground tabular">v{app.version}</div>
              </div>
            </>
          )}
          {app && collapsed && (
            <Tooltip>
              <TooltipTrigger asChild>
                <span
                  tabIndex={0}
                  aria-label={privilege}
                  className="flex size-7 items-center justify-center"
                >
                  <PrivilegeIcon className={cn("size-4", privilegeTone)} />
                </span>
              </TooltipTrigger>
              <TooltipContent side="right">
                {privilege} · v{app.version}
              </TooltipContent>
            </Tooltip>
          )}
          <button
            type="button"
            onClick={toggle}
            aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
            className="flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
          >
            {collapsed ? (
              <PanelLeftOpen className="size-4" />
            ) : (
              <PanelLeftClose className="size-4" />
            )}
          </button>
        </div>
      </div>
    </aside>
  );
}

function SidebarLink({ item, collapsed }: { item: NavItem; collapsed: boolean }) {
  const Icon = item.icon;
  const link = (
    <NavLink
      to={item.to}
      end={item.to === "/"}
      aria-label={collapsed ? item.label : undefined}
      className={({ isActive }) =>
        cn(
          "group relative flex h-8.5 items-center gap-2.5 rounded-md px-2.5 text-[13.5px] font-medium transition-colors",
          "text-sidebar-foreground hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground",
          isActive && "bg-sidebar-accent text-sidebar-accent-foreground",
          collapsed && "justify-center px-0",
        )
      }
    >
      {({ isActive }) => (
        <>
          <Icon
            className={cn(
              "size-4 shrink-0",
              isActive ? "text-brand" : "text-muted-foreground group-hover:text-foreground",
            )}
          />
          {!collapsed && <span className="truncate">{item.label}</span>}
        </>
      )}
    </NavLink>
  );

  if (!collapsed) return link;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{link}</TooltipTrigger>
      <TooltipContent side="right">{item.label}</TooltipContent>
    </Tooltip>
  );
}
