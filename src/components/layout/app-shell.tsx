import { useEffect, useLayoutEffect, useRef, useState, type CSSProperties } from "react";
import { Outlet, useLocation } from "react-router";
import { NAV_GROUPS } from "@/app/navigation";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { RestartDialog } from "@/components/common/restart-prompt";
import { useSpotlight } from "@/hooks/use-motion";
import { cn } from "@/lib/utils";
import { CommandPalette } from "./command-palette";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

/** Tab pages that reserve the scrollbar's width, or their cards shift between short and long tabs. */
const STEADY = ["/tweaks", "/visuals", "/cleaner", "/hardware", "/network", "/advanced", "/bios"];

/** Sidebar order, for which way a page comes in. */
const ROUTES = NAV_GROUPS.flatMap((group) => group.items.map((item) => item.to));

/** A tint for the light at the top of each section. Never a status colour. */
const AURORA: Record<string, string> = {
  "/gaming": "var(--chart-5)",
  "/tweaks": "var(--primary)",
  "/visuals": "var(--chart-5)",
  "/cleaner": "var(--chart-2)",
  "/hardware": "var(--chart-2)",
  "/network": "var(--chart-2)",
  "/advanced": "var(--primary)",
};

export function AppShell() {
  const { pathname, search } = useLocation();
  const tab = new URLSearchParams(search).get("tab");
  const main = useRef<HTMLElement>(null);
  useSpotlight(main);
  const order = useRef(0);
  const [searching, setSearching] = useState(false);

  // Each page and tab opens at the top. Details change the rest of the query, so they don't move it.
  useEffect(() => {
    if (main.current) main.current.scrollTop = 0;
  }, [pathname, tab]);

  // Before the page's entrance starts, so it comes from the side you moved towards.
  useLayoutEffect(() => {
    const next = ROUTES.indexOf(pathname);
    if (next < 0) return;
    main.current?.style.setProperty("--rise", next < order.current ? "-10px" : "10px");
    order.current = next;
  }, [pathname]);

  return (
    <div className="flex h-full">
      <Sidebar onSearch={() => setSearching(true)} />
      <div
        className="relative isolate flex min-w-0 flex-1 flex-col"
        style={{ "--aurora": AURORA[pathname] ?? "var(--brand)" } as CSSProperties}
      >
        <div aria-hidden className="aurora" />
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
      <RestartDialog />
    </div>
  );
}
