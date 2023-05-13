import { Button } from "@mui/material";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useContext } from "react";
import { beforeEach, describe, expect, it } from "vitest";
import { ToastQueue, ToastQueueContext } from "../../src/components/ToastQueue";

function TestToastButton({ message }: { message: string }) {
  const toastQueue = useContext(ToastQueueContext);
  return (
    <Button onClick={() => toastQueue.addToast({ message })}>Add Toast</Button>
  );
}

function FailIfToastQueueDoesNotHaveLengthButton({
  length,
}: {
  length: number;
}) {
  const toastQueue = useContext(ToastQueueContext);
  return (
    <Button onClick={() => expect(toastQueue.toasts).toHaveLength(length)}>
      Check Length
    </Button>
  );
}

describe("Toast Queue", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should not display any toasts by default", async () => {
    const renderResult = render(
      <ToastQueue>
        <FailIfToastQueueDoesNotHaveLengthButton length={0} />
      </ToastQueue>,
    );
    const checkLengthButton = renderResult.getByRole("button", {
      name: "Check Length",
    });
    await user.click(checkLengthButton);
    expect(
      renderResult.queryByRole("button", { name: "toast.close" }),
    ).toBeNull();
  });

  it("should queue up multiple toasts and display them in order", async () => {
    const toastMessages = ["toast 1", "toast 2", "toast 3"];
    const renderResult = render(
      <ToastQueue>
        {toastMessages.map((toastMessage, i) => (
          <TestToastButton message={toastMessage} key={i} />
        ))}
        <FailIfToastQueueDoesNotHaveLengthButton length={0} />
      </ToastQueue>,
    );

    const buttons = renderResult.getAllByRole("button", { name: "Add Toast" });
    for (const button of buttons) {
      await user.click(button);
    }

    for (const targetToastMessage of toastMessages) {
      expect(renderResult.queryByText(targetToastMessage)).toBeTruthy();
      toastMessages
        .filter((otherToastMessage) => otherToastMessage != targetToastMessage)
        .forEach((otherToastMessage) =>
          expect(
            renderResult.queryByText(otherToastMessage),
            `"${otherToastMessage}" should not be found when looking for ${targetToastMessage}`,
          ).toBeNull(),
        );

      await user.click(
        renderResult.getByRole("button", { name: "toast.close" }),
      );
    }

    const checkLengthButton = renderResult.getByRole("button", {
      name: "Check Length",
    });
    await user.click(checkLengthButton);
    expect(renderResult.queryByRole("button", { name: "close" })).toBeNull();
  });
});
