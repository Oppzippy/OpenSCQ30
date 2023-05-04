import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { NoiseCancelingModeSelection } from "../../src/components/soundMode/NoiseCancelingModeSelection";
import { NoiseCancelingMode } from "../../wasm/pkg/openscq30_web_wasm";

describe("NoiseCancelingModeSelection", () => {
  let user: ReturnType<typeof userEvent.setup>;

  beforeEach(() => {
    user = userEvent.setup();
  });

  it("should render", () => {
    const renderResult = render(
      <NoiseCancelingModeSelection
        value={NoiseCancelingMode.Indoor}
        onValueChanged={() => {
          throw new Error("Function not implemented.");
        }}
      />
    );

    expect(renderResult.asFragment()).toMatchSnapshot();
  });

  it("should fire onValueChanged when clicked", async () => {
    const setMode = vi.fn();
    const renderResult = render(
      <NoiseCancelingModeSelection
        value={NoiseCancelingMode.Indoor}
        onValueChanged={setMode}
      />
    );

    await user.click(renderResult.getByText("noiseCancelingMode.outdoor"));

    expect(setMode).toHaveBeenCalledWith(NoiseCancelingMode.Outdoor);
  });
});
