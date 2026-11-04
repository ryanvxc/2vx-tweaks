import { useState, type FormEvent } from "react";
import { useNavigate } from "react-router";
import {
  ChevronsUpDown,
  LogOut,
  MonitorCog,
  Settings,
  User,
  UserPlus,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { refreshAccount, useAccount, useAppInfo } from "@/hooks/use-system";
import { updateNeedsUser, useUpdater } from "@/hooks/use-updater";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import { UpdateMenuItem } from "./update-menu-item";

const ROW = "h-10 gap-3 rounded-lg px-2.5 text-[13.5px] focus:bg-sidebar-accent";

/** The sidebar's account card and the menu above it. The account lives on this PC only. */
export function AccountMenu({ collapsed }: { collapsed: boolean }) {
  const navigate = useNavigate();
  const { data: app } = useAppInfo();
  const { data: account } = useAccount();
  // Always mounted, unlike the menu's contents, so the updater starts with the app.
  const { state: update } = useUpdater();
  const [settingUp, setSettingUp] = useState(false);
  const name = account?.email.split("@")[0];

  const logOut = () => {
    backend
      .clearAccount()
      .then(refreshAccount, (err: unknown) => toast.error(toBackendError(err).message));
  };

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            type="button"
            aria-label="Account"
            title={collapsed ? (name ?? "Account") : undefined}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl text-left transition-colors outline-none hover:bg-sidebar-accent focus-visible:ring-2 focus-visible:ring-ring data-[state=open]:bg-sidebar-accent",
              collapsed ? "justify-center" : "bg-sidebar-accent/60 p-2",
            )}
          >
            <Avatar name={name} attention={updateNeedsUser(update)} />
            {!collapsed && (
              <>
                <span className="min-w-0 flex-1 leading-snug">
                  <span className="block truncate text-[14px] font-medium text-foreground">
                    {name ?? "Set up account"}
                  </span>
                  {app && (
                    <span className="block truncate text-[12px] text-info">
                      {app.isElevated ? "Administrator" : "Standard user"}
                    </span>
                  )}
                </span>
                <ChevronsUpDown className="size-3.5 shrink-0 text-muted-foreground" aria-hidden />
              </>
            )}
          </button>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          side={collapsed ? "right" : "top"}
          align={collapsed ? "end" : "start"}
          sideOffset={8}
          className="w-64 rounded-xl p-0 font-nav"
        >
          <div className="flex items-center gap-3 p-3">
            <Avatar name={name} />
            <div className="min-w-0 flex-1 leading-snug">
              <div className="truncate text-[14px] font-medium">{name ?? "No account"}</div>
              {account && (
                <div className="truncate text-[12px] text-muted-foreground" title={account.email}>
                  {account.email}
                </div>
              )}
              <div className="truncate text-[12px] text-info tabular">
                {[account && "Local account", app && `v${app.version}`].filter(Boolean).join(" · ")}
              </div>
            </div>
          </div>
          <DropdownMenuSeparator className="m-0" />
          <div className="p-1.5">
            {!account && (
              <MenuRow icon={UserPlus} onSelect={() => setSettingUp(true)}>
                Set up account
              </MenuRow>
            )}
            <MenuRow icon={Settings} onSelect={() => void navigate("/settings")}>
              Settings
            </MenuRow>
            <MenuRow icon={MonitorCog} onSelect={() => void navigate("/hardware")}>
              System detection
            </MenuRow>
            <UpdateMenuItem className={ROW} />
          </div>
          {account && (
            <div className="px-3 pb-3">
              <DropdownMenuItem
                onSelect={logOut}
                className="h-10 justify-between rounded-lg border px-3 text-[13.5px] focus:bg-sidebar-accent"
              >
                Log out
                <LogOut className="size-[18px]" strokeWidth={1.75} aria-hidden />
              </DropdownMenuItem>
            </div>
          )}
        </DropdownMenuContent>
      </DropdownMenu>

      <AccountDialog open={settingUp} onOpenChange={setSettingUp} />
    </>
  );
}

function Avatar({ name, attention = false }: { name?: string; attention?: boolean }) {
  return (
    <span className="relative flex size-10 shrink-0 items-center justify-center rounded-[10px] bg-foreground/8 text-[14px] font-medium text-foreground">
      {name?.[0]?.toUpperCase() ?? (
        <User className="size-5 text-muted-foreground" strokeWidth={1.75} aria-hidden />
      )}
      {/* The update row is out of sight until the menu opens, so the card says one is waiting. */}
      {attention && (
        <span className="absolute -top-0.5 -right-0.5 size-2 rounded-full bg-primary">
          <span className="absolute inset-0 animate-ping-soft rounded-full bg-primary" />
        </span>
      )}
    </span>
  );
}

function MenuRow({
  icon: Icon,
  onSelect,
  children,
}: {
  icon: LucideIcon;
  onSelect: () => void;
  children: string;
}) {
  return (
    <DropdownMenuItem onSelect={onSelect} className={ROW}>
      <Icon className="size-5 shrink-0" strokeWidth={1.75} aria-hidden />
      {children}
    </DropdownMenuItem>
  );
}

function AccountDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [email, setEmail] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  const save = async (e: FormEvent) => {
    e.preventDefault();
    setSaving(true);
    setError(null);
    try {
      await backend.setAccount(email);
      refreshAccount();
      setEmail("");
      onOpenChange(false);
    } catch (err) {
      const failure = toBackendError(err);
      setError(failure.code === "invalid_input" ? "Enter a valid email address." : failure.message);
    } finally {
      setSaving(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <form noValidate onSubmit={(e) => void save(e)} className="flex flex-col gap-4">
          <DialogHeader>
            <DialogTitle>Set up account</DialogTitle>
            <DialogDescription>Saved on this PC only.</DialogDescription>
          </DialogHeader>
          <div className="flex flex-col gap-1.5">
            <Input
              type="email"
              aria-label="Email"
              placeholder="Email"
              autoComplete="off"
              spellCheck={false}
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              aria-invalid={error !== null}
            />
            {error && <p className="text-[12.5px] text-destructive">{error}</p>}
          </div>
          <DialogFooter>
            <Button type="submit" size="sm" disabled={saving || email.trim() === ""}>
              Save
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
