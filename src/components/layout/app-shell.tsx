import { useEffect, useRef, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { CommandPalette } from "./command-palette";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

export function AppShell() {
  const { pathname } = useLocation();
  const main = useRef<HTMLElement>(null);
  const [searching, setSearching] = useState(false);

  // Each page opens at the top. Tabs and details change the query, so they don't move it.
  useEffect(() => {
    if (main.current) main.current.scrollTop = 0;
  }, [pathname]);

  return (
    <div className="flex h-full">
      <Sidebar onSearch={() => setSearching(true)} />
      <div className="flex min-w-0 flex-1 flex-col">
        <TitleBar />
        <main ref={main} className="flex-1 overflow-y-auto">
          {/* Keyed by route so an error on one page doesn't stick after navigating away. */}
          <ErrorBoundary key={pathname}>
            <div className="mx-auto w-full max-w-[1400px] animate-in px-7 pt-1 pb-7 duration-200 fade-in-0">
              <Outlet />
            </div>
          </ErrorBoundary>
        </main>
      </div>
      <CommandPalette open={searching} onOpenChange={setSearching} />
    </div>
  );
}
