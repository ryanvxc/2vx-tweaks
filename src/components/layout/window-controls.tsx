import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";
import { isDesktop } from "@/services/backend";
import { logger } from "@/services/logger";
import { cn } from "@/lib/utils";

/** The window has one size, so it can only be minimised or closed. */
export function WindowControls() {
  const act = (action: "minimize" | "close") => () => {
    if (!isDesktop) return;
    const win = getCurrentWindow();
    (action === "close" ? win.close() : win.minimize()).catch((e: unknown) =>
      logger.error(`window ${action} failed: ${String(e)}`, "window"),
    );
  };

  return (
    <div className="flex h-full items-stretch">
      <CaptionButton label="Minimise" onClick={act("minimize")}>
        <Minus className="size-4" strokeWidth={1.5} />
      </CaptionButton>
      <CaptionButton label="Close" onClick={act("close")} destructive>
        <X className="size-4" strokeWidth={1.5} />
      </CaptionButton>
    </div>
  );
}

function CaptionButton({
  label,
  onClick,
  destructive,
  children,
}: {
  label: string;
  onClick: () => void;
  destructive?: boolean;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      onClick={onClick}
      className={cn(
        "flex w-11 items-center justify-center text-muted-foreground transition-colors",
        destructive
          ? "hover:bg-[#c42b1c] hover:text-white"
          : "hover:bg-white/6 hover:text-foreground",
      )}
    >
      {children}
    </button>
  );
}
