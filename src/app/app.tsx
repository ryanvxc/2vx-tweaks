import { HashRouter, Navigate, Route, Routes } from "react-router";
import { AppShell } from "@/components/layout/app-shell";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { DashboardPage } from "@/pages/dashboard/dashboard-page";
import {
  CleanerPage,
  GamingPage,
  MonitoringPage,
  StartupPage,
  SystemTweaksPage,
  ToolsPage,
} from "@/pages/placeholder-pages";
import { SettingsPage } from "@/pages/settings-page";

// Hash routing: the app is served from a custom protocol in production, so there is
// no server to rewrite deep links.
export function App() {
  return (
    <TooltipProvider delayDuration={300}>
      <HashRouter>
        <Routes>
          <Route element={<AppShell />}>
            <Route index element={<DashboardPage />} />
            <Route path="monitoring" element={<MonitoringPage />} />
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
