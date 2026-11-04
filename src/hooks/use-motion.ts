import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { play } from "@/lib/motion";

const STAGGER_STEP_MS = 24;
// Past this the delay stops growing, so a long list isn't still arriving after you've read it.
const STAGGER_CAP = 6;
const STAGGER_LIMIT = 24;

/**
 * Children marked `data-stagger` rise in one after another when `key` changes, and only then.
 * Started from code rather than a CSS class because a search or filter also adds cards, and
 * those must appear at once. `null` waits for the data.
 */
export function useStaggerIn<T extends HTMLElement>(key: string | null) {
  const root = useRef<T>(null);
  useLayoutEffect(() => {
    if (key === null) return;
    root.current?.querySelectorAll("[data-stagger]").forEach((el, i) => {
      if (i >= STAGGER_LIMIT) return;
      play(el, [{ opacity: 0, transform: "translateY(8px)" }, {}], {
        duration: 240,
        delay: Math.min(i, STAGGER_CAP) * STAGGER_STEP_MS,
        fill: "backwards",
      });
    });
  }, [key]);
  return root;
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
  mark.style.setProperty("--x", `${target.offsetLeft}px`);
  mark.style.setProperty("--y", `${target.offsetTop}px`);
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
