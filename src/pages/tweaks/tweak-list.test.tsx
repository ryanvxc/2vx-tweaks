import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { MemoryRouter } from "react-router";
import { EngineBanner } from "@/components/common/engine-banner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { clearQueryCache } from "@/hooks/use-backend-query";
import {
  ADMIN_TWEAK,
  HEALTHY,
  STATUSES,
  USER_TWEAK,
  change,
  outcome,
  plan,
} from "@/test/engine-fixtures";
import type { BackendError } from "@/types/system";
import { TweakList } from "./tweak-list";

const mock = vi.hoisted(() => ({
  getEngineStatus: vi.fn(),
  getTweakStatuses: vi.fn(),
  planTweaks: vi.fn(),
  applyPlan: vi.fn(),
  listSnapshots: vi.fn(),
}));

// The desktop path, with the engine replaced by fixtures.
vi.mock("@/services/backend", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/services/backend")>()),
  isDesktop: true,
  backend: mock,
}));

const reject = (code: BackendError["code"], message: string) => Promise.reject({ code, message });

function renderList() {
  render(
    <MemoryRouter>
      <TooltipProvider>
        <EngineBanner />
        <TweakList categories={["privacy", "performance"]} />
      </TooltipProvider>
    </MemoryRouter>,
  );
}

async function reviewUserTweak() {
  const user = userEvent.setup();
  renderList();
  await user.click(await screen.findByRole("checkbox", { name: `Select ${USER_TWEAK.name}` }));
  await user.click(screen.getByRole("button", { name: /review 1 change/i }));
  return { user, dialog: await screen.findByRole("dialog") };
}

beforeEach(() => {
  clearQueryCache();
  vi.clearAllMocks();
  mock.getEngineStatus.mockResolvedValue(HEALTHY);
  mock.getTweakStatuses.mockResolvedValue(STATUSES);
  mock.planTweaks.mockImplementation((ids: string[]) => Promise.resolve(plan(ids)));
  mock.listSnapshots.mockResolvedValue([]);
});

describe("TweakList", () => {
  it("shows the detected state and only offers what can change", async () => {
    renderList();
    expect(await screen.findByText(USER_TWEAK.name)).toBeInTheDocument();
    expect(screen.getAllByText("Not in effect")).toHaveLength(2);
    expect(screen.getByText("In effect")).toBeInTheDocument();
    // Undo for an applied tweak lives in History, with the recorded before-value.
    expect(screen.getByRole("checkbox", { name: "Select Applied tweak" })).toBeDisabled();
    expect(screen.getByText("Asks for administrator permission")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /review/i })).toBeDisabled();
  });

  it("shows the exact current and desired values in the details", async () => {
    const user = userEvent.setup();
    renderList();
    await user.click(
      await screen.findByRole("button", { name: `Show details for ${USER_TWEAK.name}` }),
    );
    expect(screen.getByText(String.raw`HKCU\Software\Test\User`)).toBeInTheDocument();
    expect(screen.getByText("Now (not set)")).toBeInTheDocument();
    expect(screen.getByText("Wants 0 (REG_DWORD)")).toBeInTheDocument();
    expect(screen.getByText("https://learn.microsoft.com/test")).toBeInTheDocument();
  });
});

describe("Change inspector", () => {
  it("shows the plan diff, applies it and refreshes the detected state", async () => {
    mock.applyPlan.mockResolvedValue(outcome());
    const { user, dialog } = await reviewUserTweak();

    expect(mock.planTweaks).toHaveBeenCalledWith([USER_TWEAK.id]);
    expect(within(dialog).getByText(String.raw`HKCU\Software\Test\User`)).toBeInTheDocument();
    expect(within(dialog).getByText("(not set)")).toBeInTheDocument();
    expect(within(dialog).queryByText(/administrator permission once/)).not.toBeInTheDocument();

    const statusCalls = mock.getTweakStatuses.mock.calls.length;
    await user.click(within(dialog).getByRole("button", { name: "Apply 1 change" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(mock.applyPlan).toHaveBeenCalledWith("a".repeat(32));
    expect(mock.getTweakStatuses.mock.calls.length).toBeGreaterThan(statusCalls);
  });

  it("warns once about UAC for machine-wide values", async () => {
    const user = userEvent.setup();
    renderList();
    await user.click(await screen.findByRole("checkbox", { name: `Select ${ADMIN_TWEAK.name}` }));
    await user.click(screen.getByRole("button", { name: /review 1 change/i }));
    const dialog = await screen.findByRole("dialog");
    expect(
      await within(dialog).findByText("Windows will ask for administrator permission once"),
    ).toBeInTheDocument();
  });

  it("reports a rolled-back batch as not applied", async () => {
    mock.applyPlan.mockResolvedValue(
      outcome({
        status: "rolledBack",
        error: "`HKCU\\Software\\Test\\User` reads back 1 instead of 0",
        changes: [change({ status: "rolledBack" })],
      }),
    );
    const { user, dialog } = await reviewUserTweak();
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Not applied")).toBeInTheDocument();
    expect(within(dialog).getByText(/reads back 1 instead of 0/)).toBeInTheDocument();
  });

  it("points to the Restore Centre when the rollback itself failed", async () => {
    mock.applyPlan.mockResolvedValue(
      outcome({ status: "failed", error: "write failed", changes: [change({ status: "failed" })] }),
    );
    const { user, dialog } = await reviewUserTweak();
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(
      await within(dialog).findByText("Failed, and the rollback was incomplete"),
    ).toBeInTheDocument();
    expect(within(dialog).getByText(/Undo it in the Restore Centre/)).toBeInTheDocument();
  });

  it("offers a fresh review after a conflict", async () => {
    mock.applyPlan.mockImplementation(() => reject("conflict", "a value changed since planning"));
    const { user, dialog } = await reviewUserTweak();
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Changed since you reviewed this")).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Review again" }));
    expect(await within(dialog).findByRole("button", { name: "Apply 1 change" })).toBeEnabled();
    expect(mock.planTweaks).toHaveBeenCalledTimes(2);
  });

  it("explains a declined UAC prompt", async () => {
    mock.applyPlan.mockImplementation(() =>
      reject("elevation_required", "the administrator prompt was declined"),
    );
    const { user, dialog } = await reviewUserTweak();
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Admin permission wasn't granted")).toBeInTheDocument();
    expect(within(dialog).getByText(/prompt was declined/)).toBeInTheDocument();
  });

  it("reports a journal failure during apply", async () => {
    mock.applyPlan.mockImplementation(() => reject("storage", "disk full"));
    const { user, dialog } = await reviewUserTweak();
    await user.click(await within(dialog).findByRole("button", { name: "Apply 1 change" }));
    expect(await within(dialog).findByText("Change journal unavailable")).toBeInTheDocument();
  });

  it("requires an acknowledgement for high-risk plans", async () => {
    mock.planTweaks.mockImplementation((ids: string[]) =>
      Promise.resolve({ ...plan(ids), highestRisk: "high" }),
    );
    const { user, dialog } = await reviewUserTweak();
    const apply = await within(dialog).findByRole("button", { name: "Apply 1 change" });
    expect(apply).toBeDisabled();
    await user.click(within(dialog).getByRole("checkbox"));
    expect(apply).toBeEnabled();
  });
});

describe("Engine banner", () => {
  it("disables changes when the journal is unavailable", async () => {
    mock.getEngineStatus.mockResolvedValue({
      ...HEALTHY,
      journalAvailable: false,
      journalError: "database is locked",
    });
    const user = userEvent.setup();
    renderList();
    expect(await screen.findByText("Changes are disabled")).toBeInTheDocument();
    expect(screen.getByText(/database is locked/)).toBeInTheDocument();
    await user.click(await screen.findByRole("checkbox", { name: `Select ${USER_TWEAK.name}` }));
    expect(screen.getByRole("button", { name: /review 1 change/i })).toBeDisabled();
  });

  it("points to the Restore Centre after an interrupted batch", async () => {
    mock.getEngineStatus.mockResolvedValue({ ...HEALTHY, interruptedSnapshots: 1 });
    renderList();
    expect(await screen.findByText("1 change didn't finish")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Open Restore Centre" })).toHaveAttribute(
      "href",
      "/restore",
    );
  });
});
