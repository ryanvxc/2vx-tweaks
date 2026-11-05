import { useEffect, useRef, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { cn } from "@/lib/utils";
import { CommandPalette } from "./command-palette";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

/** Tab pages that reserve the scrollbar's width, or their cards shift between short and long tabs. */
const STEADY = ["/tweaks", "/cleaner", "/hardware", "/network", "/advanced"];

export function AppShell() {
  const { pathname, search } = useLocation();
  const tab = new URLSearchParams(search).get("tab");
  const main = useRef<HTMLElement>(null);
  const [searching, setSearching] = useState(false);

  // Each page and tab opens at the top. Details change the rest of the query, so they don't move it.
  useEffect(() => {
    if (main.current) main.current.scrollTop = 0;
  }, [pathname, tab]);

  return (
    <div className="flex h-full">
      <Sidebar onSearch={() => setSearching(true)} />
      <div className="flex min-w-0 flex-1 flex-col">
        <TitleBar />
        <main
          ref={main}
          className={cn(
            "flex-1 overflow-y-auto",
            STEADY.includes(pathname) && "[scrollbar-gutter:stable]",
          )}
        >
          {/* Keyed by route so an error on one page doesn't stick after navigating away. */}
          <ErrorBoundary key={pathname}>
            {/* A full-height column, so a page's last panel can reach the bottom of the window. */}
            <div className="flex min-h-full w-full animate-rise flex-col px-5 pb-5">
              <Outlet />
            </div>
          </ErrorBoundary>
        </main>
      </div>
      <CommandPalette open={searching} onOpenChange={setSearching} />
    </div>
  );
}
