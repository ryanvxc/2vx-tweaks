import { useEffect, useRef, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { cn } from "@/lib/utils";
import { CommandPalette } from "./command-palette";
import { HeaderSlot } from "./header-slot";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

/** Pages that carry their own title (`PageHero`), so the title bar stays thin. */
const OWN_TITLE = ["/", "/gaming", "/cleaner", "/tweaks"];

export function AppShell() {
  const { pathname, search } = useLocation();
  const tab = new URLSearchParams(search).get("tab");
  const main = useRef<HTMLElement>(null);
  const [searching, setSearching] = useState(false);
  const [headerSlot, setHeaderSlot] = useState<HTMLElement | null>(null);

  // Each page and tab opens at the top: a tab can be a sidebar row, clicked from anywhere down the
  // page. Details change the rest of the query, so they don't move it.
  useEffect(() => {
    if (main.current) main.current.scrollTop = 0;
  }, [pathname, tab]);

  return (
    <div className="flex h-full">
      <Sidebar onSearch={() => setSearching(true)} />
      <div className="flex min-w-0 flex-1 flex-col">
        <TitleBar slot={setHeaderSlot} thin={OWN_TITLE.includes(pathname)} />
        {/* Tweaks reserves the scrollbar's width, or its cards shift between short and long tabs. */}
        <main
          ref={main}
          className={cn(
            "flex-1 overflow-y-auto",
            pathname === "/tweaks" && "[scrollbar-gutter:stable]",
          )}
        >
          {/* Keyed by route so an error on one page doesn't stick after navigating away. */}
          <ErrorBoundary key={pathname}>
            <HeaderSlot.Provider value={headerSlot}>
              {/* A full-height column, so a page's last panel can reach the bottom of the window. */}
              <div className="flex min-h-full w-full animate-rise flex-col px-5 pb-5">
                <Outlet />
              </div>
            </HeaderSlot.Provider>
          </ErrorBoundary>
        </main>
      </div>
      <CommandPalette open={searching} onOpenChange={setSearching} />
    </div>
  );
}
