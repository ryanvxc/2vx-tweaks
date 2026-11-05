import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { useSlidingIndicator } from "@/hooks/use-motion";
import { cn } from "@/lib/utils";

export interface PageTab<T extends string> {
  value: T;
  label: string;
  icon?: LucideIcon;
  count?: number;
}

/**
 * Section tabs: small icon + label, and a brand underline that slides to the active one, with
 * the page's search and filters sharing the row. Controlled; the page decides what each tab shows.
 */
export function PageTabs<T extends string>({
  tabs,
  value,
  onChange,
  label,
  end,
  filled = false,
  dense = false,
}: {
  tabs: PageTab<T>[];
  value: T;
  onChange: (value: T) => void;
  label: string;
  end?: ReactNode;
  /** A full-width bar with a raised active tab, for tabs that split a whole page. */
  filled?: boolean;
  /** Words only, for a page with too many tabs for a row of icons. */
  dense?: boolean;
}) {
  const { container, indicator } = useSlidingIndicator<HTMLDivElement>('[aria-selected="true"]');
  return (
    <div
      className={cn(
        "flex flex-wrap items-center justify-between gap-x-4 gap-y-2",
        // Past the page's gutter, so its lines run the width of the window.
        filled && "-mx-5 border-y border-hairline px-5",
      )}
    >
      <div
        ref={container}
        role="tablist"
        aria-label={label}
        className={cn("relative flex items-center", filled && "gap-1 py-3")}
      >
        {tabs.map((t) => {
          const active = t.value === value;
          const Icon = t.icon;
          return (
            <button
              key={t.value}
              type="button"
              role="tab"
              aria-selected={active}
              onClick={() => onChange(t.value)}
              className={cn(
                "relative flex h-9 items-center gap-2 rounded-t-md px-3 text-[13px] font-medium transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring",
                filled && "rounded-b-md px-4",
                dense && "px-3",
                filled && active && "border border-hairline bg-raised",
                active
                  ? "text-foreground"
                  : filled
                    ? "text-muted-foreground/70 hover:text-foreground"
                    : "text-muted-foreground hover:text-foreground",
              )}
            >
              {Icon && !dense && (
                <Icon className={cn("size-4", active && !filled && "text-brand")} aria-hidden />
              )}
              {t.label}
              {t.count !== undefined && (
                <span className="text-[11px] text-muted-foreground tabular">{t.count}</span>
              )}
            </button>
          );
        })}
        <span
          ref={indicator}
          aria-hidden
          className={cn(
            "absolute left-0 h-0.5 rounded-full transition-[translate,width,opacity] duration-220 ease-arrive",
            filled
              ? "-bottom-px w-(--w) translate-x-(--x) bg-foreground"
              : "bottom-0 w-[calc(var(--w)-1rem)] translate-x-[calc(var(--x)+0.5rem)] bg-brand",
          )}
        />
      </div>
      {end}
    </div>
  );
}
