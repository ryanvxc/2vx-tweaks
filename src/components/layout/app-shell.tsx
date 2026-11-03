import { Outlet, useLocation } from "react-router";
import { ErrorBoundary } from "@/components/common/error-boundary";
import { Sidebar } from "./sidebar";
import { TitleBar } from "./title-bar";

export function AppShell() {
  const { pathname } = useLocation();

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="flex min-w-0 flex-1 flex-col">
        <TitleBar />
        <main className="flex-1 overflow-y-auto">
          {/* Keyed by route so an error on one page doesn't stick after navigating away. */}
          <ErrorBoundary key={pathname}>
            <div className="mx-auto w-full max-w-[1400px] animate-in px-8 pt-7 pb-10 duration-200 fade-in-0">
              <Outlet />
            </div>
          </ErrorBoundary>
        </main>
      </div>
    </div>
  );
}
