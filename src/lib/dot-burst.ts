import { useUiStore } from "@/stores/ui-store";

/** Square, like the dots on the loading screen and Home's cards. */
const DOT = 3;
const DRAG = 0.93;
const FALL = 0.035;
const FRAME_MS = 1000 / 60;
/** A big batch applying at once shouldn't cost more than one good burst. */
const MAX_DOTS = 420;

type Tone = "brand" | "primary" | "success";

interface Dot {
  x: number;
  y: number;
  vx: number;
  vy: number;
  age: number;
  life: number;
  colour: string;
}

let canvas: HTMLCanvasElement | null = null;
let frame = 0;
let last = 0;
const dots: Dot[] = [];

function tick(now: number) {
  const ctx = canvas?.getContext("2d");
  if (!canvas || !ctx) return;
  // In 60ths of a second, not frames: this app is used on 144 and 240 Hz screens.
  const step = Math.min(Math.max(now - last, 0) / FRAME_MS, 3);
  const drag = DRAG ** step;
  last = now;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (let i = dots.length - 1; i >= 0; i--) {
    const d = dots[i]!;
    d.age += step;
    if (d.age > d.life) {
      dots.splice(i, 1);
      continue;
    }
    d.vx *= drag;
    d.vy = d.vy * drag + FALL * step;
    d.x += d.vx * step;
    d.y += d.vy * step;
    ctx.globalAlpha = (1 - d.age / d.life) ** 1.5;
    ctx.fillStyle = d.colour;
    ctx.fillRect(d.x, d.y, DOT, DOT);
  }
  if (dots.length > 0) {
    frame = requestAnimationFrame(tick);
    return;
  }
  // Nothing stays on the page, or running, between bursts.
  frame = 0;
  canvas.remove();
  canvas = null;
}

function surface() {
  if (canvas) return canvas;
  canvas = document.createElement("canvas");
  canvas.setAttribute("aria-hidden", "true");
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  canvas.style.cssText = "position:fixed;inset:0;z-index:60;pointer-events:none";
  document.body.append(canvas);
  return canvas;
}

/**
 * A spray of the app's dots from a point on screen, for a moment worth marking. `reach` is roughly
 * how far they travel, in pixels. The canvas exists only while dots are in the air.
 */
export function burstAt(
  x: number,
  y: number,
  { tone = "brand", count = 24, reach = 80 }: { tone?: Tone; count?: number; reach?: number } = {},
) {
  // jsdom has no canvas, and Web Animations are the same test `play` uses.
  if (typeof document.body.animate !== "function") return;
  if (useUiStore.getState().reduceMotion) return;
  surface();
  const colour = getComputedStyle(document.documentElement).getPropertyValue(`--${tone}`);
  // Drag makes the distance travelled about 14 times the starting speed.
  const speed = reach / 14;
  for (let i = 0; i < count && dots.length < MAX_DOTS; i++) {
    const angle = Math.random() * Math.PI * 2;
    const v = speed * (0.35 + Math.random() * 0.65);
    dots.push({
      x,
      y,
      vx: Math.cos(angle) * v,
      vy: Math.sin(angle) * v,
      age: 0,
      life: 26 + Math.random() * 22,
      colour,
    });
  }
  if (!frame) {
    last = performance.now();
    frame = requestAnimationFrame(tick);
  }
}

/** The same, from the middle of an element. */
export function burst(el: Element | null | undefined, options?: Parameters<typeof burstAt>[2]) {
  if (!el) return;
  const box = el.getBoundingClientRect();
  burstAt(box.left + box.width / 2, box.top + box.height / 2, options);
}
