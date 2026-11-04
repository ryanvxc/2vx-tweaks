import { Link, Navigate, useLocation } from "react-router";
import {
  Bell,
  BookOpenCheck,
  Database,
  FolderOpen,
  Info,
  Loader,
  Move,
  PanelLeft,
  Palette,
  RefreshCw,
  ShieldUser,
} from "lucide-react";
import { toast } from "sonner";
import { SETTINGS_TABS, type SettingsTab } from "@/app/navigation";
import { StatusPill } from "@/components/common/notice";
import { PageTabs } from "@/components/common/page-tabs";
import { SettingsGroup, SettingsRow } from "@/components/common/settings-row";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import { isDesktop } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { UpdateCentre } from "./update-centre";

// Sections of the old single-page Settings, so `/settings#updates` links keep working.
const OLD_SECTIONS: Record<string, SettingsTab> = {
  general: "general",
  behaviour: "general",
  appearance: "appearance",
  notifications: "updates",
  updates: "updates",
  security: "general",
  about: "about",
};

export function SettingsPage() {
  const [tab, setTab] = useTabParam(SETTINGS_TABS, "general");
  const { hash } = useLocation();
  const moved = OLD_SECTIONS[hash.slice(1)];
  if (moved) return <Navigate to={`/settings?tab=${moved}`} replace />;

  return (
    <>
      <PageHeader title="Settings" />
      <div className="flex flex-col gap-4">
        <PageTabs label="Sections" tabs={SETTINGS_TABS} value={tab} onChange={setTab} />
        <div key={tab} className="max-w-3xl animate-rise">
          {tab === "general" && <GeneralTab />}
          {tab === "appearance" && <AppearanceTab />}
          {tab === "updates" && <UpdatesTab />}
          {tab === "about" && <AboutTab />}
        </div>
      </div>
    </>
  );
}

function GeneralTab() {
  const { data: app } = useAppInfo();

  const copyLogPath = async () => {
    if (!app) return;
    try {
      await navigator.clipboard.writeText(app.logDir);
      toast.success("Path copied");
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };

  return (
    <SettingsGroup>
      <SettingsRow
        icon={ShieldUser}
        title="Privileges"
        control={
          app && (
            <StatusPill tone={app.isElevated ? "warning" : "success"}>
              {app.isElevated ? "Administrator" : "Standard user"}
            </StatusPill>
          )
        }
      />
      <EngineRows />
      <SettingsRow
        icon={FolderOpen}
        title="Log folder"
        description={
          <code data-selectable className="font-mono text-[11.5px] break-all">
            {app?.logDir ?? "…"}
          </code>
        }
        control={
          <Button size="sm" variant="outline" onClick={copyLogPath} disabled={!app}>
            Copy path
          </Button>
        }
      />
    </SettingsGroup>
  );
}

function AppearanceTab() {
  const ui = useUiStore();
  return (
    <SettingsGroup>
      <SettingsRow
        icon={Palette}
        title="Theme"
        control={<StatusPill tone="neutral">Dark</StatusPill>}
      />
      <SettingsRow
        icon={PanelLeft}
        title="Compact sidebar"
        htmlFor="pref-sidebar"
        control={
          <Switch
            id="pref-sidebar"
            checked={ui.sidebarCollapsed}
            onCheckedChange={ui.toggleSidebar}
          />
        }
      />
      <SettingsRow
        icon={Loader}
        title="Loading screen"
        htmlFor="pref-splash"
        control={
          <Switch
            id="pref-splash"
            checked={ui.showSplash}
            onCheckedChange={(v) => ui.set({ showSplash: v })}
          />
        }
      />
      <SettingsRow
        icon={Move}
        title="Reduce motion"
        htmlFor="pref-motion"
        control={
          <Switch
            id="pref-motion"
            checked={ui.reduceMotion}
            onCheckedChange={(v) => ui.set({ reduceMotion: v })}
          />
        }
      />
    </SettingsGroup>
  );
}

function UpdatesTab() {
  const { data: app } = useAppInfo();
  const ui = useUiStore();
  return (
    <SettingsGroup>
      <UpdateCentre currentVersion={app?.version} />
      <SettingsRow
        icon={RefreshCw}
        title="Check automatically"
        htmlFor="pref-auto-update"
        control={
          <Switch
            id="pref-auto-update"
            checked={ui.autoCheckUpdates}
            onCheckedChange={(v) => ui.set({ autoCheckUpdates: v })}
          />
        }
      />
      <SettingsRow
        icon={Bell}
        title="Update notice on Home"
        htmlFor="pref-update-notice"
        control={
          <Switch
            id="pref-update-notice"
            checked={ui.showUpdateNotice}
            onCheckedChange={(v) => ui.set({ showUpdateNotice: v, dismissedUpdateVersion: null })}
          />
        }
      />
    </SettingsGroup>
  );
}

function AboutTab() {
  const { data: app } = useAppInfo();
  return (
    <SettingsGroup>
      <SettingsRow
        icon={Info}
        title="Version"
        description={
          app ? (
            <>
              2VX Tweaks {app.version}
              {app.debugBuild && " (development build)"}
            </>
          ) : (
            <Skeleton className="h-3.5 w-24" />
          )
        }
      />
      <SettingsRow
        icon={BookOpenCheck}
        title="Open-source licences"
        description="Lucide (ISC), Inter and Manrope (OFL 1.1)"
      />
    </SettingsGroup>
  );
}

function EngineRows() {
  const { data: status, error } = useEngineStatus();
  return (
    <>
      <SettingsRow
        icon={Database}
        title="Change journal"
        // Only a problem needs words; the pill covers the normal case.
        description={!isDesktop ? undefined : error ? error.message : status?.journalError}
        control={
          status &&
          (!isDesktop ? (
            <StatusPill tone="info">Preview</StatusPill>
          ) : status.journalAvailable ? (
            <StatusPill tone="success">Available</StatusPill>
          ) : (
            <StatusPill tone="destructive">Changes disabled</StatusPill>
          ))
        }
      />
      {status && status.interruptedSnapshots > 0 && (
        <SettingsRow
          icon={Database}
          title="Interrupted changes"
          control={
            <Button asChild size="sm" variant="outline">
              <Link to="/restore">Review {status.interruptedSnapshots}</Link>
            </Button>
          }
        />
      )}
    </>
  );
}
