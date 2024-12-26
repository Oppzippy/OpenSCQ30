import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ButtonSettings } from "../../../src/components/buttonSettings/ButtonSettings";
import { CustomButtonActions } from "../../../src/libTypes/DeviceState";

const testButtonActions: Readonly<CustomButtonActions> = {
  leftDoubleClick: {
    isEnabled: false,
    action: "volumeUp",
  },
  leftSingleClick: {
    action: "ambientSoundMode",
    isEnabled: false,
  },
  leftLongPress: {
    isEnabled: false,
    action: "volumeDown",
  },
  rightSingleClick: {
    action: "nextSong",
    isEnabled: true,
  },
  rightDoubleClick: {
    isEnabled: true,
    action: "voiceAssistant",
  },
  rightLongPress: {
    isEnabled: true,
    action: "previousSong",
  },
};

describe("Button Settings", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should change button actions", async () => {
    const setButtonActions = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonActions={{
          ...testButtonActions,
        }}
        setButtonActions={setButtonActions}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.rightDoubleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setButtonActions).toHaveBeenCalledWith({
      ...testButtonActions,
      rightDoubleClick: {
        isEnabled: true,
        action: "previousSong",
      },
    });
  });

  it("should set isEnabled to true when selecting a value", async () => {
    const setButtonActions = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonActions={{
          ...testButtonActions,
        }}
        setButtonActions={setButtonActions}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.leftSingleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setButtonActions).toHaveBeenCalledWith({
      ...testButtonActions,
      leftSingleClick: {
        isEnabled: true,
        action: "previousSong",
      },
    });
  });
});
