import type { ReactNode } from "react";
import { ArrowDownUp, ListFilter, Search, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/lib/utils";

export interface FilterOption<T extends string> {
  value: T;
  label: string;
}

export interface FilterGroup<T extends string = string> {
  id: string;
  label: string;
  options: FilterOption<T>[];
  /** Empty means "no filter" for this group. */
  selected: T[];
  onChange: (selected: T[]) => void;
}

/** Search, filter menu and sort menu in one row, with room for a right-aligned chip. */
export function FilterBar({
  search,
  onSearch,
  placeholder = "Search",
  filters = [],
  sort,
  end,
}: {
  search: string;
  onSearch: (value: string) => void;
  placeholder?: string;
  filters?: FilterGroup[];
  sort?: { value: string; options: FilterOption<string>[]; onChange: (v: string) => void };
  end?: ReactNode;
}) {
  const active = filters.reduce((n, g) => n + g.selected.length, 0);
  return (
    <div className="flex flex-wrap items-center gap-2">
      <div className="relative w-52">
        <Search className="pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground" />
        <input
          type="search"
          value={search}
          onChange={(e) => onSearch(e.target.value)}
          placeholder={placeholder}
          aria-label={placeholder}
          className="h-8 w-full panel rounded-lg pr-7 pl-8 text-[13px] outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring/40 [&::-webkit-search-cancel-button]:hidden"
        />
        {search && (
          <button
            type="button"
            aria-label="Clear search"
            onClick={() => onSearch("")}
            className="absolute top-1/2 right-1.5 flex size-5 -translate-y-1/2 animate-in items-center justify-center rounded text-muted-foreground duration-120 ease-arrive fade-in-0 zoom-in-75 hover:text-foreground"
          >
            <X className="size-3.5" />
          </button>
        )}
      </div>

      {filters.length > 0 && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="sm" className={cn(active > 0 && "text-brand")}>
              <ListFilter /> Filter
              {active > 0 && (
                <span
                  key={active}
                  className="animate-pop rounded-full bg-brand/20 px-1.5 text-[10.5px] text-brand tabular"
                >
                  {active}
                </span>
              )}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-56">
            {filters.map((g, i) => (
              <div key={g.id}>
                {i > 0 && <DropdownMenuSeparator />}
                <DropdownMenuLabel className="text-[11px] text-muted-foreground">
                  {g.label}
                </DropdownMenuLabel>
                {g.options.map((o) => (
                  <DropdownMenuCheckboxItem
                    key={o.value}
                    checked={g.selected.includes(o.value)}
                    onSelect={(e) => e.preventDefault()}
                    onCheckedChange={(on) =>
                      g.onChange(
                        on ? [...g.selected, o.value] : g.selected.filter((v) => v !== o.value),
                      )
                    }
                    className="text-[13px]"
                  >
                    {o.label}
                  </DropdownMenuCheckboxItem>
                ))}
              </div>
            ))}
            {active > 0 && (
              <>
                <DropdownMenuSeparator />
                <button
                  type="button"
                  className="w-full rounded-sm px-2 py-1.5 text-left text-[13px] text-muted-foreground hover:bg-accent hover:text-foreground"
                  onClick={() => filters.forEach((g) => g.onChange([]))}
                >
                  Clear filters
                </button>
              </>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      )}

      {sort && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="sm">
              <ArrowDownUp /> Sort
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-48">
            <DropdownMenuRadioGroup value={sort.value} onValueChange={sort.onChange}>
              {sort.options.map((o) => (
                <DropdownMenuRadioItem key={o.value} value={o.value} className="text-[13px]">
                  {o.label}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      )}

      {end && <div className="ml-auto flex items-center gap-2">{end}</div>}
    </div>
  );
}
