import { useEffect, useRef, useState } from "react";

/** How far past the bottom of the window the next batch is drawn, so it's there on arrival. */
const AHEAD = "800px";

/**
 * Draws a long list a batch at a time, adding one as `end` nears the window. Without a way to watch
 * for that, everything is drawn. `resetKey` starts again from the first batch for a different list.
 */
export function useReveal(batch: number, total: number, resetKey: string) {
  const [limit, setLimit] = useState(batch);
  const [list, setList] = useState(resetKey);
  if (list !== resetKey) {
    setList(resetKey);
    setLimit(batch);
  }

  const end = useRef<HTMLDivElement>(null);
  const watchable = typeof IntersectionObserver !== "undefined";
  useEffect(() => {
    const node = end.current;
    if (!node || !watchable) return;
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((entry) => entry.isIntersecting)) setLimit((n) => n + batch);
      },
      // The page scrolls inside `main`, and a margin only reaches past the root it's set on.
      { root: node.closest("main"), rootMargin: `0px 0px ${AHEAD} 0px` },
    );
    observer.observe(node);
    return () => observer.disconnect();
    // A new observer per batch: one still in range after a batch would never fire again.
  }, [batch, limit, total, watchable]);

  return { limit: watchable ? limit : Number.POSITIVE_INFINITY, end };
}
