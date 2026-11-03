import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { ConfirmDialog } from "./confirm-dialog";

function renderDialog(risk: "low" | "high", onConfirm = vi.fn()) {
  render(
    <ConfirmDialog
      open
      onOpenChange={() => {}}
      title="Apply change"
      description="Test"
      consequences={["Changes a setting"]}
      risk={risk}
      onConfirm={onConfirm}
    />,
  );
  return onConfirm;
}

describe("ConfirmDialog", () => {
  it("lets low-risk actions confirm directly", async () => {
    const onConfirm = renderDialog("low");
    await userEvent.click(screen.getByRole("button", { name: "Apply" }));
    expect(onConfirm).toHaveBeenCalledOnce();
  });

  it("requires acknowledgement for high-risk actions", async () => {
    const onConfirm = renderDialog("high");
    const apply = screen.getByRole("button", { name: "Apply" });
    expect(apply).toBeDisabled();
    await userEvent.click(screen.getByRole("checkbox"));
    expect(apply).toBeEnabled();
    await userEvent.click(apply);
    expect(onConfirm).toHaveBeenCalledOnce();
  });
});
