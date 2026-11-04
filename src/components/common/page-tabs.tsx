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
}: {
  tabs: PageTab<T>[];
  value: T;
  onChange: (value: T) => void;
  label: string;
  end?: ReactNode;
  /** Also gives the active tab a background, for tabs that split a whole page. */
  filled?: boolean;
}) {
  const { container, indicator } = useSlidingIndicator<HTMLDivElement>('[aria-selected="true"]');
  return (
    <div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-2">
      <div ref={container} role="tablist" aria-label={label} className="relative flex items-center">
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
                filled && "mr-1 rounded-b-md px-4",
                filled && active && "bg-raised",
                active ? "text-foreground" : "text-muted-foreground hover:text-foreground",
              )}
            >
              {Icon && <Icon className={cn("size-4", active && "text-brand")} aria-hidden />}
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
          className="absolute bottom-0 left-0 h-0.5 w-[calc(var(--w)-1rem)] translate-x-[calc(var(--x)+0.5rem)] rounded-full bg-brand transition-[translate,width,opacity] duration-220 ease-arrive"
        />
      </div>
      {end}
    </div>
  );
}
