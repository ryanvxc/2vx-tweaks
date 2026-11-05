import { useEffect, useState } from "react";
import { Link } from "react-router";
import {
  Bell,
  Bug,
  BookOpenCheck,
  Database,
  ExternalLink,
  FolderOpen,
  Globe,
  Info,
  Loader,
  LogIn,
  Move,
  PanelLeft,
  RefreshCw,
  ShieldUser,
} from "lucide-react";
import { toast } from "sonner";
import { StatusLabel } from "@/components/common/notice";
import { RecommendedAction } from "@/components/common/recommended-action";
import { SettingsGroup, SettingsRow } from "@/components/common/settings-row";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import type { AutoLaunch } from "@/types/system";
import { useUiStore } from "@/stores/ui-store";
import { RUST_LICENCES, WEB_LICENCES, type Licence } from "./licences";
import { UpdateCentre } from "./update-centre";

/** Everything on one screen: there are too few settings to hide any behind tabs. */
export function SettingsPage() {
  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="Settings"
        subtitle="App behaviour and updates"
        action={<RecommendedAction />}
      />
      <div className="flex animate-rise flex-col gap-1">
        <GeneralGroup />
        <AppearanceGroup />
        <UpdatesGroup />
        <AboutGroup />
      </div>
    </div>
  );
}

function GeneralGroup() {
  const { data: app } = useAppInfo();
  return (
    <SettingsGroup title="General">
      <SettingsRow
        icon={ShieldUser}
        title="Privileges"
        control={
          app && (
            <StatusLabel tone={app.isElevated ? "success" : "neutral"}>
              {app.isElevated ? "Administrator" : "Standard user"}
            </StatusLabel>
          )
        }
      />
      <AutoLaunchRow />
      <InterruptedRow />
    </SettingsGroup>
  );
}

function AppearanceGroup() {
  const ui = useUiStore();
  return (
    <SettingsGroup title="Appearance">
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
  const [licences, setLicences] = useState(false);

  const run = (action: Promise<void>) =>
    action.catch((err: unknown) => toast.error(toBackendError(err).message));

  return (
    <SettingsGroup title="About and diagnostics">
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
          <Button
            size="xs"
            variant="ghost"
            onClick={() => void run(backend.openLogFolder())}
            disabled={!app}
          >
            Open folder
          </Button>
        }
      />
      <SettingsRow
        icon={Bug}
        title="Report an issue"
        control={
          <Button
            size="xs"
            variant="ghost"
            onClick={() => void run(backend.openLink("reportIssue"))}
          >
            <ExternalLink /> Open
          </Button>
        }
      />
      <SettingsRow
        icon={Globe}
        title="Project page"
        control={
          <Button
            size="xs"
            variant="ghost"
            onClick={() => void run(backend.openLink("projectPage"))}
          >
            <ExternalLink /> Open
          </Button>
        }
      />
      <SettingsRow
        icon={BookOpenCheck}
        title="Open-source licences"
        control={
          <Button size="xs" variant="ghost" onClick={() => setLicences(true)}>
            View
          </Button>
        }
      />
      <LicenceDialog open={licences} onOpenChange={setLicences} />
    </SettingsGroup>
  );
}

/**
 * Off unless the user turns it on. In the browser preview there is nothing to set, so it stays
 * disabled, and so does an entry switched off in Task Manager, which is the place to undo that.
 */
function AutoLaunchRow() {
  const [state, setState] = useState<AutoLaunch | null>(null);
  useEffect(() => {
    let live = true;
    backend.getAutoLaunch().then(
      (v) => live && setState(v),
      () => live && setState("off"),
    );
    return () => {
      live = false;
    };
  }, []);

  const change = (next: boolean) => {
    const before = state;
    setState(next ? "on" : "off");
    backend
      .setAutoLaunch(next)
      // Asked again, because a stale Task Manager entry can hold back what was just written.
      .then(() => backend.getAutoLaunch())
      .then(setState, (err: unknown) => {
        setState(before);
        toast.error(toBackendError(err).message);
      });
  };

  const offInTaskManager = state === "offInTaskManager";
  return (
    <SettingsRow
      icon={LogIn}
      title="Open at sign-in"
      description={offInTaskManager ? "Off in Task Manager" : undefined}
      htmlFor="pref-auto-launch"
      control={
        <Switch
          id="pref-auto-launch"
          checked={state === "on"}
          disabled={!isDesktop || state === null || offInTaskManager}
          onCheckedChange={change}
        />
      }
    />
  );
}

function LicenceDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[85vh] gap-3 sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Open-source licences</DialogTitle>
          <DialogDescription>Direct dependencies and the licence each declares</DialogDescription>
        </DialogHeader>
        <div className="flex flex-col gap-4 overflow-y-auto pr-1">
          <LicenceList title="Interface" items={WEB_LICENCES} />
          <LicenceList title="Backend" items={RUST_LICENCES} />
        </div>
      </DialogContent>
    </Dialog>
  );
}

function LicenceList({ title, items }: { title: string; items: Licence[] }) {
  return (
    <section>
      <h3 className="mb-1 text-[12px] text-muted-foreground">{title}</h3>
      <ul className="divide-y">
        {items.map((l) => (
          <li key={l.name} className="flex items-center justify-between gap-3 py-1.5 text-[13px]">
            <span className="truncate">{l.name}</span>
            <span className="shrink-0 text-muted-foreground">{l.licence}</span>
          </li>
        ))}
      </ul>
    </section>
  );
}

/** Only shown when something needs a look: Restore Centre reports the journal's health. */
function InterruptedRow() {
  const { data: status } = useEngineStatus();
  if (!status || status.interruptedSnapshots === 0) return null;
  return (
    <SettingsRow
      icon={Database}
      title="Interrupted changes"
      control={
        <Button asChild size="xs" variant="ghost">
          <Link to="/restore">Review {status.interruptedSnapshots}</Link>
        </Button>
      }
    />
  );
}
