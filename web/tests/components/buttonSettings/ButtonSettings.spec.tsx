import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ButtonSettings } from "../../../src/components/buttonSettings/ButtonSettings";
import { MultiButtonConfiguration } from "../../../src/libTypes/DeviceState";

const testMultiButtonConfiguration: Readonly<MultiButtonConfiguration> = {
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
    const setMultiButtonConfiguration = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonConfiguration={{
          ...testMultiButtonConfiguration,
        }}
        setMultiButtonConfiguration={setMultiButtonConfiguration}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.rightDoubleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setMultiButtonConfiguration).toHaveBeenCalledWith({
      ...testMultiButtonConfiguration,
      rightDoubleClick: {
        isEnabled: true,
        action: "previousSong",
      },
    });
  });

  it("should set isEnabled to true when selecting a value", async () => {
    const setMultiButtonConfiguration = vi.fn();
    const renderResult = render(
      <ButtonSettings
        buttonConfiguration={{
          ...testMultiButtonConfiguration,
        }}
        setMultiButtonConfiguration={setMultiButtonConfiguration}
      />,
    );
    await user.click(renderResult.getByLabelText("buttons.leftSingleClick"));
    await user.click(
      renderResult.getByRole("option", {
        name: "buttonActions.previousSong",
      }),
    );
    expect(setMultiButtonConfiguration).toHaveBeenCalledWith({
      ...testMultiButtonConfiguration,
      leftSingleClick: {
        isEnabled: true,
        action: "previousSong",
      },
    });
  });
});
