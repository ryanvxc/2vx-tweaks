import { createContext } from "react";

/** Where a page's title goes: the title bar, so it shares a row with the window controls. */
export const HeaderSlot = createContext<HTMLElement | null>(null);
