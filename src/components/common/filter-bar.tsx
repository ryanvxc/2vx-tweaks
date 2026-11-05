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
      <div className="relative w-60">
        <Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
        <input
          type="search"
          value={search}
          onChange={(e) => onSearch(e.target.value)}
          placeholder={placeholder}
          aria-label={placeholder}
          className="h-10 w-full panel rounded-lg pr-8 pl-9.5 text-[13.5px] outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring/40 [&::-webkit-search-cancel-button]:hidden"
        />
        {search && (
          <button
            type="button"
            aria-label="Clear search"
            onClick={() => onSearch("")}
            className="absolute top-1/2 right-2 flex size-5 -translate-y-1/2 animate-in items-center justify-center rounded text-muted-foreground duration-120 ease-arrive fade-in-0 zoom-in-75 hover:text-foreground"
          >
            <X className="size-3.5" />
          </button>
        )}
      </div>

      {filters.length > 0 && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className={cn("h-10 px-4", active > 0 && "text-brand")}>
              <ListFilter /> Filter
              {active > 0 && (
                <span key={active} className="animate-pop tabular">
                  {active}
                </span>
              )}
            </Button>
          </DropdownMenuTrigger>
          {/* Two columns, so every group fits on screen without a second scrollbar. */}
          <DropdownMenuContent align="start" className="w-[30rem] p-1.5">
            <div className="columns-2 gap-1.5">
              {filters.map((g) => (
                <div key={g.id} className="break-inside-avoid pb-1">
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
            </div>
            {active > 0 && (
              <button
                type="button"
                className="w-full rounded-sm border-t px-2 py-1.5 text-left text-[13px] text-muted-foreground hover:bg-accent hover:text-foreground"
                onClick={() => filters.forEach((g) => g.onChange([]))}
              >
                Clear filters
              </button>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      )}

      {sort && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="h-10 px-4">
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
