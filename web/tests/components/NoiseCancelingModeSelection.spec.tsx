import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { NoiseCancelingModeSelection } from "../../src/components/soundMode/NoiseCancelingModeSelection";

describe("NoiseCancelingModeSelection", () => {
  let user: ReturnType<typeof userEvent.setup>;

  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should fire onValueChanged when clicked", async () => {
    const setMode = vi.fn();
    const renderResult = render(
      <NoiseCancelingModeSelection
        value={"indoor"}
        hasCustomMode={true}
        onValueChanged={setMode}
      />,
    );

    await user.click(renderResult.getByText("noiseCancelingMode.outdoor"));

    expect(setMode).toHaveBeenCalledWith("outdoor");
  });
});
