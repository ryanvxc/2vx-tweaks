import { Link, useLocation, useMatch } from "react-router";
import logo from "@/assets/logo.svg";
import { NAV_GROUPS, type NavItem } from "@/app/navigation";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useSlidingIndicator } from "@/hooks/use-motion";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import { AccountMenu } from "./account-menu";

/** Height of a group's heading, and of the line that stands in for it when compact. */
const GROUP_HEAD_PX = 24;
const GROUP_LINE_PX = 9;
/** Rows stop growing here. On a taller window the spare height goes between the groups. */
const ROW_MAX_PX = 64;

export function Sidebar({ onSearch }: { onSearch: () => void }) {
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
          "flex h-(--titlebar-height) shrink-0 items-center gap-2.5 border-b border-sidebar-border",
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
        className="relative isolate flex min-h-0 flex-1 flex-col justify-between gap-1.5 overflow-y-auto px-3 pt-2 pb-6"
        aria-label="Main"
      >
        <span
          ref={navPill}
          aria-hidden
          className="absolute top-0 left-0 -z-10 h-(--h) w-(--w) translate-x-(--x) translate-y-(--y) rounded-[10px] bg-sidebar-accent transition-[translate,width,height,opacity] duration-220"
        />
        {NAV_GROUPS.map((group, i) => {
          const head = collapsed ? (i > 0 ? GROUP_LINE_PX : 0) : GROUP_HEAD_PX;
          return (
            // Growing by row count, from the heading's height, keeps every row the same height
            // across groups of different sizes, so the list fills the sidebar at any window height.
            <div
              key={group.label}
              className="flex flex-col"
              style={{
                flex: `${group.items.length} 1 ${head}px`,
                maxHeight: head + group.items.length * ROW_MAX_PX,
              }}
            >
              {!collapsed && (
                <div className="flex h-6 shrink-0 items-center px-3 text-[12.5px] text-muted-foreground/60">
                  {group.label}
                </div>
              )}
              {collapsed && i > 0 && <div className="mx-3 mb-2 h-px shrink-0 bg-sidebar-border" />}
              {group.items.map((item) => (
                <div key={item.to} className="flex min-h-8.5 flex-1 flex-col justify-center">
                  <SidebarLink item={item} collapsed={collapsed} />
                </div>
              ))}
            </div>
          );
        })}
      </nav>

      <div className="shrink-0 p-3">
        <AccountMenu collapsed={collapsed} onSearch={onSearch} />
      </div>
    </aside>
  );
}

function SidebarLink({ item, collapsed }: { item: NavItem; collapsed: boolean }) {
  const { icon: Icon, label } = item;
  // A plain Link with this worked out here: the tooltip's Slot can only merge a string className.
  const isActive = useMatch({ path: item.to, end: item.to === "/" }) !== null;
  const link = (
    <Link
      to={item.to}
      aria-current={isActive ? "page" : undefined}
      aria-label={collapsed ? label : undefined}
      className={cn(
        "group flex max-h-10 min-h-0 flex-1 items-center gap-3 rounded-[10px] px-3 text-[14px] font-medium transition-colors",
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
      {!collapsed && <span className="truncate">{label}</span>}
    </Link>
  );

  if (!collapsed) return link;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{link}</TooltipTrigger>
      <TooltipContent side="right">{label}</TooltipContent>
    </Tooltip>
  );
}
