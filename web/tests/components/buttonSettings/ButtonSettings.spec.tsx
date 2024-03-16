import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ButtonSettings } from "../../../src/components/buttonSettings/ButtonSettings";
import { CustomButtonModel } from "../../../src/libTypes/DeviceState";

const testButtonModel: Readonly<CustomButtonModel> = {
  leftDoubleClick: {
    isEnabled: false,
    twsConnectedAction: "volumeUp",
    twsDisconnectedAction: "volumeUp",
  },
  leftSingleClick: {
    action: "ambientSoundMode",
    isEnabled: false,
  },
  leftLongPress: {
    isEnabled: false,
    twsConnectedAction: "volumeDown",
    twsDisconnectedAction: "volumeDown",
  },
  rightSingleClick: {
    action: "nextSong",
    isEnabled: true,
  },
  rightDoubleClick: {
    isEnabled: true,
    twsConnectedAction: "voiceAssistant",
    twsDisconnectedAction: "voiceAssistant",
  },
  rightLongPress: {
    isEnabled: true,
    twsConnectedAction: "previousSong",
    twsDisconnectedAction: "previousSong",
  },
};

describe("Button Settings", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should change non tws button actions", async () => {
    const setButtonModel = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonModel={{
          ...testButtonModel,
        }}
        setButtonModel={setButtonModel}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.rightSingleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setButtonModel).toHaveBeenCalledWith({
      ...testButtonModel,
      rightSingleClick: { isEnabled: true, action: "previousSong" },
    });
  });

  it("should change both tws button actions", async () => {
    const setButtonModel = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonModel={{
          ...testButtonModel,
        }}
        setButtonModel={setButtonModel}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.rightDoubleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setButtonModel).toHaveBeenCalledWith({
      ...testButtonModel,
      rightDoubleClick: {
        isEnabled: true,
        twsConnectedAction: "previousSong",
        twsDisconnectedAction: "previousSong",
      },
    });
  });

  it("should set isEnabled to true when selecting a value", async () => {
    const setButtonModel = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonModel={{
          ...testButtonModel,
        }}
        setButtonModel={setButtonModel}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.leftSingleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setButtonModel).toHaveBeenCalledWith({
      ...testButtonModel,
      leftSingleClick: {
        isEnabled: true,
        action: "previousSong",
      },
    });
  });
});
