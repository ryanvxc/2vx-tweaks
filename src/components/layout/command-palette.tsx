import { createElement, useEffect, useMemo, useState, type KeyboardEvent } from "react";
import { useNavigate } from "react-router";
import { CornerDownLeft, Search, type LucideIcon } from "lucide-react";
import { ALL_NAV_ITEMS } from "@/app/navigation";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/components/ui/dialog";
import { useTweakStatuses } from "@/hooks/use-system";
import { tweakHref, tweakIcon } from "@/lib/tweak-presentation";
import { cn } from "@/lib/utils";

interface Entry {
  id: string;
  label: string;
  hint: string;
  icon: LucideIcon;
  to: string;
  search: string;
}

const LIMIT = 8;

const PAGES: Entry[] = ALL_NAV_ITEMS.flatMap((item) => [
  {
    id: item.to,
    label: item.label,
    hint: "Page",
    icon: item.icon,
    to: item.to,
    search: `${item.label} ${item.description}`,
  },
  ...(item.tabs ?? []).map((tab) => ({
    id: `${item.to}?tab=${tab.value}`,
    label: `${item.label} › ${tab.label}`,
    hint: "Section",
    icon: tab.icon,
    to: `${item.to}?tab=${tab.value}`,
    search: `${item.label} ${tab.label}`,
  })),
]);

/**
 * Ctrl K: jump to any page, section or tweak. Tweaks sit in topic tabs, so a name alone
 * should be enough to find one. It only navigates; nothing here changes the system.
 */
export function CommandPalette({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const navigate = useNavigate();
  const { data } = useTweakStatuses();
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);

  useEffect(() => {
    const onKey = (e: globalThis.KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        onOpenChange(true);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onOpenChange]);

  const entries = useMemo(() => {
    const terms = query.trim().toLowerCase().split(/\s+/).filter(Boolean);
    if (terms.length === 0) return PAGES.filter((p) => p.hint === "Page");
    const tweaks: Entry[] = (data ?? []).map(({ meta }) => ({
      id: meta.id,
      label: meta.name,
      hint: "Tweak",
      icon: tweakIcon(meta),
      to: tweakHref(meta),
      search: `${meta.name} ${meta.description}`,
    }));
    return [...PAGES, ...tweaks]
      .filter((e) => terms.every((t) => e.search.toLowerCase().includes(t)))
      .slice(0, LIMIT);
  }, [data, query]);

  const close = () => {
    onOpenChange(false);
    setQuery("");
    setActive(0);
  };

  const go = (entry: Entry | undefined) => {
    if (!entry) return;
    close();
    void navigate(entry.to);
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      const count = Math.max(entries.length, 1);
      setActive((i) => (i + (e.key === "ArrowDown" ? 1 : -1) + count) % count);
    } else if (e.key === "Enter") {
      e.preventDefault();
      go(entries[active]);
    }
  };

  return (
    <Dialog open={open} onOpenChange={(o) => (o ? onOpenChange(true) : close())}>
      <DialogContent
        showCloseButton={false}
        className="top-[16%] translate-y-0 gap-0 overflow-hidden p-0 sm:max-w-lg"
        onKeyDown={onKeyDown}
      >
        <DialogTitle className="sr-only">Search 2VX Tweaks</DialogTitle>
        <DialogDescription className="sr-only">
          Find a page, section or tweak. Arrow keys move, Enter opens.
        </DialogDescription>
        <div className="flex items-center gap-2.5 border-b px-4">
          <Search className="size-4 shrink-0 text-muted-foreground" aria-hidden />
          <input
            autoFocus
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setActive(0);
            }}
            placeholder="Find a page or tweak"
            aria-label="Find a page or tweak"
            className="h-12 w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
          />
        </div>
        <ul aria-label="Results" className="max-h-80 overflow-y-auto p-1.5">
          {entries.length === 0 ? (
            <li className="px-3 py-6 text-center text-[13px] text-muted-foreground">
              Nothing matches
            </li>
          ) : (
            entries.map((e, i) => (
              <li key={e.id}>
                <button
                  type="button"
                  tabIndex={-1}
                  onClick={() => go(e)}
                  onMouseMove={() => setActive(i)}
                  className={cn(
                    "flex h-9 w-full items-center gap-2.5 rounded-md px-2.5 text-left text-[13px]",
                    i === active && "bg-accent",
                  )}
                >
                  {createElement(e.icon, { className: "size-4 shrink-0 text-brand" })}
                  <span className="min-w-0 flex-1 truncate">{e.label}</span>
                  <span className="text-[11.5px] text-muted-foreground">{e.hint}</span>
                  {i === active && (
                    <CornerDownLeft className="size-3 text-muted-foreground" aria-hidden />
                  )}
                </button>
              </li>
            ))
          )}
        </ul>
      </DialogContent>
    </Dialog>
  );
}
