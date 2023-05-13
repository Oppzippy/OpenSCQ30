import { Button } from "@mui/material";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useContext } from "react";
import * as pwaRegisterReact from "virtual:pwa-register/react";
import { beforeEach, describe, expect, it } from "vitest";
import { ToastQueue, ToastQueueContext } from "../../src/components/ToastQueue";
import { useUpdateAvailableToast } from "../../src/hooks/useUpdateAvailableToast";

function UseUpdateAvailable() {
  useUpdateAvailableToast();
  return <></>;
}

function FailIfToastQueueDoesNotHaveLengthButton({
  length,
}: {
  length: number;
}) {
  const toastQueue = useContext(ToastQueueContext);
  return (
    <Button onClick={() => expect(toastQueue.toasts).toHaveLength(length)}>
      Check Length {length}
    </Button>
  );
}

describe("Device Settings", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should not display any toasts by default", async () => {
    const renderResult = render(
      <ToastQueue>
        <UseUpdateAvailable />
        <FailIfToastQueueDoesNotHaveLengthButton length={0} />
      </ToastQueue>,
    );
    await user.click(
      renderResult.getByRole("button", { name: `Check Length 0` }),
    );
  });

  it("should display an update available toast when a refresh is needed", async () => {
    const renderResult = render(
      <ToastQueue>
        <UseUpdateAvailable />
        <FailIfToastQueueDoesNotHaveLengthButton length={1} />
      </ToastQueue>,
    );
    const mockPwaRegisterReact =
      pwaRegisterReact as unknown as typeof pwaRegisterReact & {
        needRefresh: boolean;
      };
    if (!("needRefresh" in pwaRegisterReact)) {
      throw new Error("missing needRefresh");
    }
    mockPwaRegisterReact.needRefresh = true;
    mockPwaRegisterReact.needRefresh = false;
    mockPwaRegisterReact.needRefresh = true;
    await user.click(
      renderResult.getByRole("button", { name: "Check Length 1" }),
    );
  });
});
