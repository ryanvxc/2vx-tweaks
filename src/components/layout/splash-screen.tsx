import { useCallback, useEffect, useRef, useState } from "react";
import logo from "@/assets/logo.svg";
import { useEngineStatus, useSystemSnapshot, useTweakStatuses } from "@/hooks/use-system";
import { FACTS } from "@/lib/splash-facts";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import { SplashCanvas } from "./splash-canvas";
import { WindowControls } from "./window-controls";

const STAGES = ["Starting the engine", "Reading this PC", "Checking your tweaks"];

// Even when loading takes no time the logo forms over this long, then stays up whole
// until the hold ends, so there's something to see and time to read a line.
const FORM_MS = 1600;
const HOLD_MS = 2600;
// The still version has nothing to form, but a screen gone in 300 ms is just a flicker.
const STILL_HOLD_MS = 1200;
// A stuck backend call must never trap anyone here: pages show their own loading states.
const GIVE_UP_MS = 8000;
// Playing with the dots keeps the screen up for a while, but not for ever.
const PLAY_CAP_MS = 7000;
const PLAY_GRACE_MS = 500;
const FADE_MS = 450; // matches .splash-leaving in globals.css

/**
 * Shown once at launch, over the app, which loads underneath it. Progress is the real
 * startup calls settling (a failed one counts: its page reports the error). It can only
 * run slower than they do, never ahead of them.
 */
export function SplashScreen({ onDone }: { onDone: () => void }) {
  const engine = useEngineStatus();
  const pc = useSystemSnapshot();
  const tweaks = useTweakStatuses();
  const settled = [engine, pc, tweaks].filter((q) => !q.loading).length;

  // Only 2VX's own switch counts here, not the Windows animation setting: that is often off
  // for speed on the PCs this app is for, and following it would hide this screen entirely.
  const still = useUiStore((s) => s.reduceMotion);
  const [elapsed, setElapsed] = useState(0);
  const [leaving, setLeaving] = useState(false);
  const [fact, setFact] = useState(() => Math.floor(Math.random() * FACTS.length));

  const progress = Math.min(settled / STAGES.length, still ? 1 : elapsed / FORM_MS);
  const stage = STAGES[Math.floor(progress * STAGES.length)] ?? "Ready";

  const anchor = useRef<HTMLDivElement>(null);
  const shownProgress = useRef(0);
  const settledRef = useRef(0);
  const lastPlay = useRef(0);

  useEffect(() => {
    shownProgress.current = progress;
    settledRef.current = settled;
  }, [progress, settled]);

  useEffect(() => {
    const started = Date.now();
    const timer = setInterval(() => {
      const now = Date.now();
      const ms = now - started;
      setElapsed(ms);
      const ready = settledRef.current === STAGES.length || ms >= GIVE_UP_MS;
      const playing = ms < PLAY_CAP_MS && now - lastPlay.current < PLAY_GRACE_MS;
      if (ready && ms >= (still ? STILL_HOLD_MS : HOLD_MS) && !playing) setLeaving(true);
    }, 100);
    return () => clearInterval(timer);
  }, [still]);

  useEffect(() => {
    if (!leaving) return;
    const timer = setTimeout(onDone, still ? 0 : FADE_MS);
    return () => clearTimeout(timer);
  }, [leaving, onDone, still]);

  const nextFact = useCallback(() => setFact((f) => (f + 1) % FACTS.length), []);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      // Space and Enter already press a focused button.
      const pressing = e.target instanceof Element && e.target.closest("button");
      if (e.key === "Escape" || (e.key === "Enter" && !pressing)) setLeaving(true);
      if (e.key === "ArrowRight" || (e.key === " " && !pressing)) {
        e.preventDefault();
        nextFact();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [nextFact]);

  const onPlay = useCallback(() => {
    lastPlay.current = Date.now();
  }, []);

  return (
    <div
      className={cn(
        "fixed inset-0 z-50 overflow-hidden bg-background",
        leaving && !still && "splash-leaving",
      )}
    >
      {!still && (
        <SplashCanvas progress={shownProgress} leaving={leaving} anchor={anchor} onPlay={onPlay} />
      )}

      {/* Clicks pass through to the canvas, except on the line that takes them. */}
      <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center px-8">
        <div ref={anchor} className="relative size-[168px]">
          <div
            className="absolute -inset-40 -z-10 rounded-full transition-opacity duration-300"
            style={{
              opacity: 0.25 + progress * 0.75,
              background:
                "radial-gradient(closest-side, color-mix(in oklch, var(--brand) 16%, transparent), transparent)",
            }}
          />
          {still && <img src={logo} alt="" className="size-full" />}
        </div>

        <div className="mt-7 font-heading text-[13px] font-bold tracking-[0.34em] text-muted-foreground uppercase">
          2VX Tweaks
        </div>

        <button
          type="button"
          tabIndex={-1}
          title="Another fact"
          onClick={nextFact}
          className="pointer-events-auto mt-5 flex min-h-[3.1em] max-w-xl cursor-default items-center text-center font-heading text-[21px] leading-[1.3] font-bold text-balance outline-none"
        >
          {FACTS[fact]}
        </button>

        <div role="status" className="mt-5 text-[13px] text-muted-foreground">
          {stage}
        </div>
        <div
          role="progressbar"
          aria-label="Loading 2VX"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(progress * 100)}
          className="mt-3 h-1 w-56 overflow-hidden rounded-full bg-muted"
        >
          <div
            className="h-full rounded-full bg-linear-to-r from-brand to-primary transition-[width] duration-200"
            style={{ width: `${progress * 100}%` }}
          />
        </div>
      </div>

      <header
        data-tauri-drag-region
        className="absolute inset-x-0 top-0 flex h-(--titlebar-height) justify-end"
      >
        <WindowControls />
      </header>
    </div>
  );
}
