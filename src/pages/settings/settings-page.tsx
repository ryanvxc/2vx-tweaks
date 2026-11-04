import { Link } from "react-router";
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
import { StatusLabel } from "@/components/common/notice";
import { SettingsGroup, SettingsRow } from "@/components/common/settings-row";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { isDesktop } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { UpdateCentre } from "./update-centre";

/** Everything on one screen: there are too few settings to hide any behind tabs. */
export function SettingsPage() {
  return (
    <>
      <PageHeader title="Settings" />
      <div className="grid flex-1 animate-rise grid-cols-2 gap-3">
        <div className="divide-y overflow-hidden panel">
          <GeneralGroup />
          <AppearanceGroup />
        </div>
        <div className="divide-y overflow-hidden panel">
          <UpdatesGroup />
          <AboutGroup />
        </div>
      </div>
    </>
  );
}

function GeneralGroup() {
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
    <SettingsGroup title="General">
      <SettingsRow
        icon={ShieldUser}
        title="Privileges"
        control={
          app && (
            <StatusLabel tone={app.isElevated ? "warning" : "success"}>
              {app.isElevated ? "Administrator" : "Standard user"}
            </StatusLabel>
          )
        }
      />
      <EngineRows />
      <SettingsRow
        icon={FolderOpen}
        title="Log folder"
        description={
          <code
            data-selectable
            className="block truncate font-mono text-[11.5px]"
            title={app?.logDir}
          >
            {app?.logDir ?? "…"}
          </code>
        }
        control={
          <Button size="xs" variant="ghost" onClick={copyLogPath} disabled={!app}>
            Copy path
          </Button>
        }
      />
    </SettingsGroup>
  );
}

function AppearanceGroup() {
  const ui = useUiStore();
  return (
    <SettingsGroup title="Appearance">
      <SettingsRow
        icon={Palette}
        title="Theme"
        control={<span className="text-[12.5px] text-muted-foreground">Dark</span>}
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

function UpdatesGroup() {
  const { data: app } = useAppInfo();
  const ui = useUiStore();
  return (
    <SettingsGroup title="Updates">
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

function AboutGroup() {
  const { data: app } = useAppInfo();
  return (
    <SettingsGroup title="About">
      <SettingsRow
        icon={Info}
        title="Version"
        control={
          app ? (
            <span className="text-[12.5px] text-muted-foreground tabular">
              {app.version}
              {app.debugBuild && " (development)"}
            </span>
          ) : (
            <Skeleton className="h-3.5 w-16" />
          )
        }
      />
      <SettingsRow
        icon={BookOpenCheck}
        title="Open-source licences"
        control={
          <span className="text-[12.5px] text-muted-foreground">
            Lucide (ISC), Geist and Poppins (OFL 1.1)
          </span>
        }
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
        // Only a problem needs words; the label covers the normal case.
        description={!isDesktop ? undefined : error ? error.message : status?.journalError}
        control={
          status &&
          (!isDesktop ? (
            <StatusLabel tone="info">Preview</StatusLabel>
          ) : status.journalAvailable ? (
            <StatusLabel tone="success">Available</StatusLabel>
          ) : (
            <StatusLabel tone="destructive">Changes disabled</StatusLabel>
          ))
        }
      />
      {status && status.interruptedSnapshots > 0 && (
        <SettingsRow
          icon={Database}
          title="Interrupted changes"
          control={
            <Button asChild size="xs" variant="ghost">
              <Link to="/restore">Review {status.interruptedSnapshots}</Link>
            </Button>
          }
        />
      )}
    </>
  );
}
