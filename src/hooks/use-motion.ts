import { useEffect, useLayoutEffect, useRef, useState, type RefObject } from "react";
import { play, takeTabStep } from "@/lib/motion";
import { useUiStore } from "@/stores/ui-store";

const STAGGER_STEP_MS = 30;
// Past this the delay stops growing, so a long list isn't still arriving after you've read it.
const STAGGER_CAP = 6;
const STAGGER_LIMIT = 24;
const TAB_SLIDE_PX = 22;
/** How close the pointer gets before a card's edge starts to light up. */
const SPOT_REACH_PX = 120;

/**
 * Children marked `data-stagger` come in as a wave from the top-left when `key` changes, and only
 * then, sliding from the side of the tab you moved to. Started from code rather than a CSS class
 * because a search or filter also adds cards, and those must appear at once. `null` waits for the data.
 */
export function useStaggerIn<T extends HTMLElement>(key: string | null) {
  const root = useRef<T>(null);
  useLayoutEffect(() => {
    if (key === null) return;
    const step = takeTabStep();
    const items = [...(root.current?.querySelectorAll("[data-stagger]") ?? [])];
    const first = items[0]?.getBoundingClientRect();
    if (!first) return;
    const from = step ? `translateX(${step * TAB_SLIDE_PX}px)` : "translateY(10px)";
    items.slice(0, STAGGER_LIMIT).forEach((el) => {
      const box = el.getBoundingClientRect();
      // Rows and columns from the first card, so a grid fills corner to corner.
      const wave =
        (box.left - first.left) / (first.width || 1) + (box.top - first.top) / (first.height || 1);
      play(el, [{ opacity: 0, transform: `${from} scale(0.975)` }, {}], {
        duration: 280,
        delay: Math.min(Math.max(Math.round(wave), 0), STAGGER_CAP) * STAGGER_STEP_MS,
        fill: "backwards",
      });
    });
  }, [key]);
  return root;
}

/**
 * Lights the `data-spot` cards around the pointer. One listener on the page, one frame's work at
 * most, and it writes only to the cards in reach, so no card re-renders and a still pointer costs
 * nothing. With Reduce motion on, no card is marked lit and the light never shows.
 */
export function useSpotlight(root: RefObject<HTMLElement | null>) {
  useEffect(() => {
    const el = root.current;
    if (!el) return;
    let frame = 0;
    let x = 0;
    let y = 0;
    const paint = () => {
      frame = 0;
      for (const card of el.querySelectorAll<HTMLElement>("[data-spot]")) {
        const box = card.getBoundingClientRect();
        const near =
          x > box.left - SPOT_REACH_PX &&
          x < box.right + SPOT_REACH_PX &&
          y > box.top - SPOT_REACH_PX &&
          y < box.bottom + SPOT_REACH_PX;
        if (near) {
          card.style.setProperty("--mx", `${x - box.left}px`);
          card.style.setProperty("--my", `${y - box.top}px`);
          card.toggleAttribute("data-lit", true);
        } else {
          card.toggleAttribute("data-lit", false);
        }
      }
    };
    const move = (e: PointerEvent) => {
      x = e.clientX;
      y = e.clientY;
      if (!frame && !useUiStore.getState().reduceMotion) frame = requestAnimationFrame(paint);
    };
    const leave = () => {
      cancelAnimationFrame(frame);
      frame = 0;
      el.querySelectorAll("[data-lit]").forEach((card) => card.removeAttribute("data-lit"));
    };
    el.addEventListener("pointermove", move, { passive: true });
    el.addEventListener("pointerleave", leave);
    return () => {
      cancelAnimationFrame(frame);
      el.removeEventListener("pointermove", move);
      el.removeEventListener("pointerleave", leave);
    };
  }, [root]);
}

/** Moves the highlight onto the active item. With none it hides, and reappears in place. */
function place(root: HTMLElement, mark: HTMLElement, selector: string) {
  const target = root.querySelector<HTMLElement>(selector);
  if (!target) {
    mark.style.opacity = "0";
    delete mark.dataset.placed;
    return;
  }
  // The first placement has nowhere to slide from.
  const arriving = mark.dataset.placed === undefined;
  if (arriving) mark.style.transition = "none";
  const x = `${target.offsetLeft}px`;
  const y = `${target.offsetTop}px`;
  // Stretches along its way as it slides, so it feels thrown rather than moved.
  const stretch =
    mark.style.getPropertyValue("--y") !== y
      ? "1 1.22"
      : mark.style.getPropertyValue("--x") !== x
        ? "1.3 1"
        : null;
  if (stretch && !arriving) {
    play(mark, [{ scale: "1 1" }, { scale: stretch, offset: 0.35 }, { scale: "1 1" }], {
      duration: 320,
    });
  }
  mark.style.setProperty("--x", x);
  mark.style.setProperty("--y", y);
  mark.style.setProperty("--w", `${target.offsetWidth}px`);
  mark.style.setProperty("--h", `${target.offsetHeight}px`);
  mark.style.opacity = "1";
  if (arriving) {
    void mark.offsetWidth; // commit the jump before transitions come back
    mark.style.transition = "";
    mark.dataset.placed = "true";
  }
}

/**
 * One highlight that slides between the items of a group, instead of each item drawing its
 * own: a tab row, a segmented control, a set of selectable cards.
 *
 * Give `container` to a positioned element and `indicator` to an absolute child, and size and
 * move the child with the --x, --y, --w and --h variables set here. `selector` finds the
 * active item. Elements are held in state because a group can mount after its data arrives.
 */
export function useSlidingIndicator<C extends HTMLElement>(selector: string) {
  const [container, setContainer] = useState<C | null>(null);
  const [indicator, setIndicator] = useState<HTMLSpanElement | null>(null);

  // After every render: the active item, or its width (a count in a tab), may have changed.
  useLayoutEffect(() => {
    if (container && indicator) place(container, indicator, selector);
  });

  useEffect(() => {
    if (!container || !indicator) return;
    const observer = new ResizeObserver(() => place(container, indicator, selector));
    observer.observe(container);
    return () => observer.disconnect();
  }, [container, indicator, selector]);

  return { container: setContainer, indicator: setIndicator };
}
