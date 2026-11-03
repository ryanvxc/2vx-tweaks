import { useEffect } from "react";
import { HashRouter, Navigate, Route, Routes } from "react-router";
import { AppShell } from "@/components/layout/app-shell";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { CleanerPage } from "@/pages/cleaner/cleaner-page";
import { DashboardPage } from "@/pages/dashboard/dashboard-page";
import { HardwarePage } from "@/pages/hardware/hardware-page";
import { HistoryPage } from "@/pages/history/history-page";
import { GamingPage } from "@/pages/placeholder-pages";
import { SettingsPage } from "@/pages/settings/settings-page";
import { StartupPage } from "@/pages/startup/startup-page";
import { ToolsPage } from "@/pages/tools/tools-page";
import { SystemTweaksPage } from "@/pages/tweaks/system-tweaks-page";
import { useUiStore } from "@/stores/ui-store";

// Hash routing: the app is served from a custom protocol in production, so there is
// no server to rewrite deep links.
export function App() {
  const reduceMotion = useUiStore((s) => s.reduceMotion);
  useEffect(() => {
    document.documentElement.classList.toggle("reduce-motion", reduceMotion);
  }, [reduceMotion]);

  return (
    <TooltipProvider delayDuration={300}>
      <HashRouter>
        <Routes>
          <Route element={<AppShell />}>
            <Route index element={<DashboardPage />} />
            <Route path="hardware" element={<HardwarePage />} />
            {/* Old link target from 0.1. */}
            <Route path="monitoring" element={<Navigate to="/hardware" replace />} />
            <Route path="history" element={<HistoryPage />} />
            <Route path="gaming" element={<GamingPage />} />
            <Route path="system" element={<SystemTweaksPage />} />
            <Route path="startup" element={<StartupPage />} />
            <Route path="cleaner" element={<CleanerPage />} />
            <Route path="tools" element={<ToolsPage />} />
            <Route path="settings" element={<SettingsPage />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Route>
        </Routes>
      </HashRouter>
      <Toaster position="bottom-right" />
    </TooltipProvider>
  );
}
