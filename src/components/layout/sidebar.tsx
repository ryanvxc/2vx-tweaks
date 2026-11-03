import { NavLink } from "react-router";
import { PanelLeftClose, PanelLeftOpen, ShieldAlert, ShieldCheck } from "lucide-react";
import logo from "@/assets/logo.svg";
import { NAV_GROUPS, SETTINGS_ITEM, type NavItem } from "@/app/navigation";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useAppInfo } from "@/hooks/use-system";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import { UpdateButton } from "./update-button";

export function Sidebar() {
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  const toggle = useUiStore((s) => s.toggleSidebar);
  const { data: app } = useAppInfo();

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
          <span data-tauri-drag-region className="text-[15px] font-bold tracking-tight">
            2VX <span className="font-medium text-muted-foreground">Tweaks</span>
          </span>
        )}
      </div>

      <nav className="flex flex-1 flex-col gap-4 overflow-y-auto px-2.5 pt-2" aria-label="Main">
        {NAV_GROUPS.map((group) => (
          <div key={group.label} className="flex flex-col gap-px">
            {!collapsed && (
              <div className="mb-1 px-2.5 text-[11px] font-medium text-muted-foreground/70">
                {group.label}
              </div>
            )}
            {group.items.map((item) => (
              <SidebarLink key={item.to} item={item} collapsed={collapsed} />
            ))}
          </div>
        ))}
      </nav>

      <div className="flex flex-col gap-px border-t border-sidebar-border px-2.5 py-2.5">
        <UpdateButton collapsed={collapsed} />
        <SidebarLink item={SETTINGS_ITEM} collapsed={collapsed} />
        {app && !collapsed && (
          <div className="mt-1.5 flex items-center gap-2.5 rounded-lg bg-sidebar-accent/50 px-2.5 py-2">
            {app.isElevated ? (
              <ShieldAlert className="size-4 shrink-0 text-warning" />
            ) : (
              <ShieldCheck className="size-4 shrink-0 text-success" />
            )}
            <div className="min-w-0 flex-1 leading-tight">
              <div className="truncate text-[12px] font-medium text-foreground">
                {app.isElevated ? "Administrator" : "Standard user"}
              </div>
              <div className="text-[11px] text-muted-foreground tabular">v{app.version}</div>
            </div>
          </div>
        )}
        <button
          type="button"
          onClick={toggle}
          aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
          className={cn(
            "mt-1 flex h-8 items-center gap-3 rounded-md px-2.5 text-[12.5px] text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
            collapsed && "justify-center px-0",
          )}
        >
          {collapsed ? <PanelLeftOpen className="size-4" /> : <PanelLeftClose className="size-4" />}
          {!collapsed && <span>Collapse</span>}
        </button>
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
      className={({ isActive }) =>
        cn(
          "group relative flex h-8.5 items-center gap-2.5 rounded-md px-2.5 text-[13px] font-medium transition-colors",
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
