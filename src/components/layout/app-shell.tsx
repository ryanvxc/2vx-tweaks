import { useEffect, useRef, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { CommandPalette } from "./command-palette";
import { HeaderSlot } from "./header-slot";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

export function AppShell() {
  const { pathname } = useLocation();
  const main = useRef<HTMLElement>(null);
  const [searching, setSearching] = useState(false);
  const [headerSlot, setHeaderSlot] = useState<HTMLElement | null>(null);

  // Each page opens at the top. Tabs and details change the query, so they don't move it.
  useEffect(() => {
    if (main.current) main.current.scrollTop = 0;
  }, [pathname]);

  return (
    <div className="flex h-full">
      <Sidebar onSearch={() => setSearching(true)} />
      <div className="flex min-w-0 flex-1 flex-col">
        <TitleBar slot={setHeaderSlot} />
        <main ref={main} className="flex-1 overflow-y-auto">
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
