import { NavLink, useLocation, useMatch } from "react-router";
import logo from "@/assets/logo.svg";
import { NAV_GROUPS, type NavItem } from "@/app/navigation";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useSlidingIndicator } from "@/hooks/use-motion";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import { AccountMenu } from "./account-menu";

export function Sidebar() {
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  // Only the links read the route on their own, and the pill has to move when it changes.
  useLocation();
  const { container: navBox, indicator: navPill } =
    useSlidingIndicator<HTMLElement>('a[aria-current="page"]');

  return (
    <aside
      className={cn(
        "flex h-full shrink-0 flex-col border-r border-sidebar-border bg-sidebar font-nav text-sidebar-foreground transition-[width] duration-220",
        collapsed ? "w-16" : "w-(--sidebar-width)",
      )}
    >
      <div
        data-tauri-drag-region
        className={cn(
          "flex h-(--titlebar-height) shrink-0 items-center gap-2.5",
          collapsed ? "justify-center" : "px-5",
        )}
      >
        <img src={logo} alt="" className="pointer-events-none size-7" draggable={false} />
        {!collapsed && (
          <span data-tauri-drag-region className="text-[16px] font-semibold tracking-tight">
            2VX <span className="font-normal text-muted-foreground">Tweaks</span>
          </span>
        )}
      </div>

      <nav
        ref={navBox}
        className="relative isolate flex flex-1 flex-col gap-3.5 overflow-y-auto px-3 pt-1"
        aria-label="Main"
      >
        <span
          ref={navPill}
          aria-hidden
          className="absolute top-0 left-0 -z-10 h-(--h) w-(--w) translate-x-(--x) translate-y-(--y) rounded-[10px] bg-sidebar-accent transition-[translate,width,height,opacity] duration-220"
        />
        {NAV_GROUPS.map((group, i) => (
          <div key={group.label} className="flex flex-col gap-1">
            {!collapsed && (
              <div className="px-3 pb-0.5 text-[12.5px] text-muted-foreground/60">
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

      <div className="shrink-0 p-3">
        <AccountMenu collapsed={collapsed} />
      </div>
    </aside>
  );
}

function SidebarLink({ item, collapsed }: { item: NavItem; collapsed: boolean }) {
  const Icon = item.icon;
  const end = item.to === "/";
  // Worked out here because the tooltip's Slot can only merge a string className, not NavLink's
  // function form.
  const isActive = useMatch({ path: item.to, end }) !== null;
  const link = (
    <NavLink
      to={item.to}
      end={end}
      aria-label={collapsed ? item.label : undefined}
      className={cn(
        "group flex h-9 items-center gap-3 rounded-[10px] px-3 text-[14px] font-medium transition-colors",
        item.highlight
          ? "text-primary"
          : isActive
            ? "text-foreground"
            : "text-muted-foreground hover:text-foreground",
        !isActive && "hover:bg-sidebar-accent/60",
        collapsed && "justify-center px-0",
      )}
    >
      <Icon
        className="size-5 shrink-0 transition-[scale] duration-200 group-hover:scale-110"
        strokeWidth={1.75}
      />
      {!collapsed && <span className="truncate">{item.label}</span>}
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
