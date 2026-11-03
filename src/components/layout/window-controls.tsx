import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Copy, Minus, Square, X } from "lucide-react";
import { isDesktop } from "@/services/backend";
import { logger } from "@/services/logger";
import { cn } from "@/lib/utils";

/** Windows-style caption buttons for the frameless main window. */
export function WindowControls() {
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    if (!isDesktop) return;
    const win = getCurrentWindow();
    let unlisten: (() => void) | undefined;
    win
      .isMaximized()
      .then(setMaximized)
      .catch(() => {});
    win
      .onResized(() => {
        win
          .isMaximized()
          .then(setMaximized)
          .catch(() => {});
      })
      .then((fn) => (unlisten = fn))
      .catch(() => {});
    return () => unlisten?.();
  }, []);

  const act = (action: "minimize" | "toggleMaximize" | "close") => () => {
    if (!isDesktop) return;
    const win = getCurrentWindow();
    const op =
      action === "minimize"
        ? win.minimize()
        : action === "close"
          ? win.close()
          : win.toggleMaximize();
    op.catch((e: unknown) => logger.error(`window ${action} failed: ${String(e)}`, "window"));
  };

  return (
    <div className="flex h-full items-stretch">
      <CaptionButton label="Minimise" onClick={act("minimize")}>
        <Minus className="size-4" strokeWidth={1.5} />
      </CaptionButton>
      <CaptionButton label={maximized ? "Restore" : "Maximise"} onClick={act("toggleMaximize")}>
        {maximized ? (
          <Copy className="size-3.5 -scale-x-100" strokeWidth={1.5} />
        ) : (
          <Square className="size-3.5" strokeWidth={1.5} />
        )}
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
