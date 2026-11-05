import { useCallback, useEffect, useState } from "react";
import { HashRouter, Navigate, Route, Routes, useLocation } from "react-router";
import { AppShell } from "@/components/layout/app-shell";
import { FirstRunDialog } from "@/components/layout/first-run-dialog";
import { SplashScreen } from "@/components/layout/splash-screen";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { CleanerPage } from "@/pages/cleaner/cleaner-page";
import { DashboardPage } from "@/pages/dashboard/dashboard-page";
import { GamingPage } from "@/pages/gaming/gaming-page";
import { HardwarePage } from "@/pages/hardware/hardware-page";
import { NetworkPage } from "@/pages/network/network-page";
import { AdvancedPage } from "@/pages/advanced/advanced-page";
import { BiosPage } from "@/pages/bios/bios-page";
import { RestorePage } from "@/pages/restore/restore-page";
import { SettingsPage } from "@/pages/settings/settings-page";
import { ToolsPage } from "@/pages/tools/tools-page";
import { TweaksPage } from "@/pages/tweaks/tweaks-page";
import { VisualsPage } from "@/pages/visuals/visuals-page";
import { useUiStore } from "@/stores/ui-store";

/** Old addresses keep working, including a tweak named in the query. */
function Moved({ to }: { to: string }) {
  const { search } = useLocation();
  return <Navigate to={`${to}${search}`} replace />;
}

// Hash routing: the app is served from a custom protocol in production, so there is
// no server to rewrite deep links.
export function App() {
  const reduceMotion = useUiStore((s) => s.reduceMotion);
  useEffect(() => {
    document.documentElement.classList.toggle("reduce-motion", reduceMotion);
  }, [reduceMotion]);

  const [splash, setSplash] = useState(() => useUiStore.getState().showSplash);
  const endSplash = useCallback(() => setSplash(false), []);

  return (
    <TooltipProvider delayDuration={300}>
      {/* Inert under the loading screen, so Tab can't reach controls nobody can see. */}
      <div className="h-full" inert={splash}>
        <HashRouter>
          <Routes>
            <Route element={<AppShell />}>
              <Route index element={<DashboardPage />} />
              <Route path="monitoring" element={<Navigate to="/" replace />} />
              <Route path="visuals" element={<VisualsPage />} />
              <Route path="hardware" element={<HardwarePage />} />
              <Route path="network" element={<NetworkPage />} />
              <Route path="advanced" element={<AdvancedPage />} />
              <Route path="bios" element={<BiosPage />} />
              <Route path="tweaks" element={<TweaksPage />} />
              <Route path="library" element={<Moved to="/tweaks" />} />
              <Route path="system" element={<Moved to="/tweaks" />} />
              <Route path="gaming" element={<GamingPage />} />
              <Route path="startup" element={<Navigate to="/cleaner?tab=autoruns" replace />} />
              <Route path="cleaner" element={<CleanerPage />} />
              <Route path="restore" element={<RestorePage />} />
              <Route path="history" element={<Moved to="/restore" />} />
              <Route path="tools" element={<ToolsPage />} />
              <Route path="settings" element={<SettingsPage />} />
              <Route path="*" element={<Navigate to="/" replace />} />
            </Route>
          </Routes>
        </HashRouter>
      </div>
      {splash ? <SplashScreen onDone={endSplash} /> : <FirstRunDialog />}
      <Toaster position="bottom-right" />
    </TooltipProvider>
  );
}
