import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

export interface PageTab<T extends string> {
  value: T;
  label: string;
  icon?: LucideIcon;
  count?: number;
}

/**
 * Section tabs under a page header: small icon + label, a brand underline on the active
 * one. Controlled; the page decides what each tab shows.
 */
export function PageTabs<T extends string>({
  tabs,
  value,
  onChange,
  label,
}: {
  tabs: PageTab<T>[];
  value: T;
  onChange: (value: T) => void;
  label: string;
}) {
  return (
    <div role="tablist" aria-label={label} className="flex items-center gap-1 border-b">
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
              "relative -mb-px flex h-10 items-center gap-2 rounded-t-md px-3 text-[13.5px] font-medium transition-colors outline-none focus-visible:ring-2 focus-visible:ring-ring",
              active ? "text-foreground" : "text-muted-foreground hover:text-foreground",
            )}
          >
            {Icon && <Icon className={cn("size-4", active && "text-brand")} aria-hidden />}
            {t.label}
            {t.count !== undefined && (
              <span className="text-[11px] text-muted-foreground tabular">{t.count}</span>
            )}
            <span
              className={cn(
                "absolute inset-x-2 bottom-0 h-0.5 rounded-full bg-brand transition-opacity",
                active ? "opacity-100" : "opacity-0",
              )}
            />
          </button>
        );
      })}
    </div>
  );
}
