import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router";
import { describe, expect, it } from "vitest";
import { AccountMenu } from "./account-menu";

const openMenu = async (user: ReturnType<typeof userEvent.setup>) => {
  screen.getByRole("button", { name: "Account" }).focus();
  await user.keyboard("{Enter}");
};

describe("AccountMenu", () => {
  it("logs out, refuses a bad address and sets the account up again", async () => {
    const user = userEvent.setup();
    render(
      <MemoryRouter>
        <AccountMenu collapsed={false} onSearch={() => {}} />
      </MemoryRouter>,
    );

    expect(await screen.findByText("preview")).toBeInTheDocument();
    await openMenu(user);
    expect(await screen.findByText("preview@example.com")).toBeInTheDocument();
    await user.click(screen.getByRole("menuitem", { name: "Log out" }));
    expect(await screen.findByText("Set up account")).toBeInTheDocument();

    await openMenu(user);
    expect(screen.queryByRole("menuitem", { name: "Log out" })).not.toBeInTheDocument();
    await user.click(await screen.findByRole("menuitem", { name: "Set up account" }));

    const email = await screen.findByRole("textbox", { name: "Email" });
    await user.type(email, "nope{Enter}");
    expect(await screen.findByText("Enter a valid email address.")).toBeInTheDocument();

    await user.clear(email);
    await user.type(email, "someone@example.com{Enter}");
    expect(await screen.findByText("someone")).toBeInTheDocument();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });
});
