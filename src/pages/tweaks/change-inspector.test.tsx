import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TooltipProvider } from "@/components/ui/tooltip";
import { USER_TWEAK, outcome, plan } from "@/test/engine-fixtures";
import type { Plan, SkippedTweak } from "@/types/engine";
import { ChangeInspector } from "./change-inspector";

const mock = vi.hoisted(() => ({
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
}));

vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));

const READ_ERROR = String.raw`elevation required: ADAPTER\Ethernet\*EEE can't be read without administrator rights`;

const UNREADABLE: SkippedTweak = {
  tweakId: "network.unreadable",
  reason: "unreadable",
  detail: READ_ERROR,
};
const MISSING: SkippedTweak = {
  tweakId: "network.missing",
  reason: "notInstalled",
  detail: null,
};

const names = {
  [USER_TWEAK.id]: USER_TWEAK.name,
  [UNREADABLE.tweakId]: "Energy Efficient Ethernet",
};

function inspector(reviewed: Plan) {
  mock.planTweaks.mockResolvedValue(reviewed);
  const onOpenChange = vi.fn();
  const onApplied = vi.fn();
  render(
    <TooltipProvider>
      <ChangeInspector
        open
        onOpenChange={onOpenChange}
        tweakIds={[USER_TWEAK.id]}
        names={names}
        onApplied={onApplied}
      />
    </TooltipProvider>,
  );
  return { onOpenChange, onApplied };
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("ChangeInspector", () => {
  it("puts what was skipped above the changes, with the reason in full", async () => {
    inspector({ ...plan([USER_TWEAK.id]), skipped: [UNREADABLE, MISSING] });

    const notice = await screen.findByText("Skipped 2 tweaks");
    const change = screen.getByRole("heading", { name: USER_TWEAK.name });
    expect(notice.compareDocumentPosition(change)).toBe(Node.DOCUMENT_POSITION_FOLLOWING);

    const list = within(screen.getByRole("alert"));
    expect(list.getByText(/can't be read without administrator rights/)).toBeInTheDocument();
    expect(list.getByText("Energy Efficient Ethernet")).toBeInTheDocument();
    // No name or detail to show: the ID and the reason still say what happened.
    expect(list.getByText("network.missing")).toBeInTheDocument();
    expect(list.getByText(/Not installed on this PC/)).toBeInTheDocument();
  });

  it("doesn't call a plan with nothing left in it nothing to change", async () => {
    inspector({ ...plan([]), skipped: [UNREADABLE] });

    expect(await screen.findByText("Nothing can be applied")).toBeInTheDocument();
    expect(screen.queryByText("Nothing to change")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Nothing to apply" })).toBeDisabled();
  });

  it("says nothing to change when there is nothing to do", async () => {
    inspector({ ...plan([]), alreadyApplied: [USER_TWEAK.id] });
    expect(await screen.findByText("Nothing to change")).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });

  it("stays open after a clean apply that left something out", async () => {
    const user = userEvent.setup();
    mock.applyPlan.mockResolvedValue(outcome({ skipped: [UNREADABLE] }));
    const { onOpenChange, onApplied } = inspector(plan([USER_TWEAK.id]));

    await user.click(await screen.findByRole("button", { name: /apply 1 change/i }));

    expect(await screen.findByText("Skipped 1 tweak")).toBeInTheDocument();
    expect(screen.getByText("Applied")).toBeInTheDocument();
    expect(screen.getByText(/can't be read without administrator rights/)).toBeInTheDocument();
    expect(onApplied).toHaveBeenCalledWith(7);
    expect(onOpenChange).not.toHaveBeenCalled();

    // The footer's, and the dialog's own corner button.
    await user.click(screen.getAllByRole("button", { name: "Close" })[0]!);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("closes itself after a clean apply with nothing left out", async () => {
    const user = userEvent.setup();
    mock.applyPlan.mockResolvedValue(outcome());
    const { onOpenChange, onApplied } = inspector(plan([USER_TWEAK.id]));

    await user.click(await screen.findByRole("button", { name: /apply 1 change/i }));

    await waitFor(() => expect(onOpenChange).toHaveBeenCalledWith(false));
    expect(onApplied).toHaveBeenCalledWith(7);
  });

  it("shows what was skipped when the apply failed too", async () => {
    const user = userEvent.setup();
    mock.applyPlan.mockResolvedValue(
      outcome({ status: "rolledBack", error: "refused", skipped: [UNREADABLE] }),
    );
    const { onOpenChange, onApplied } = inspector(plan([USER_TWEAK.id]));

    await user.click(await screen.findByRole("button", { name: /apply 1 change/i }));

    expect(await screen.findByText("Skipped 1 tweak")).toBeInTheDocument();
    expect(onApplied).not.toHaveBeenCalled();
    expect(onOpenChange).not.toHaveBeenCalled();
  });
});
