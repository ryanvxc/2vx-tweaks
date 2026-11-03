import { useSearchParams } from "react-router";

/** A page's tab, kept in `?tab=` so the back button, search and deep links can reach it. */
export function useTabParam<T extends string>(tabs: readonly { value: T }[], fallback: T) {
  const [params, setParams] = useSearchParams();
  const raw = params.get("tab");
  const tab = tabs.find((t) => t.value === raw)?.value ?? fallback;
  const setTab = (next: T) =>
    setParams(
      (p) => {
        const updated = new URLSearchParams(p);
        updated.set("tab", next);
        return updated;
      },
      { replace: true },
    );
  return [tab, setTab] as const;
}
