import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { NoiseCancelingModeSelection } from "../../src/components/NoiseCancelingModeSelection";
import { NoiseCancelingMode } from "../../wasm/pkg/openscq30_web_wasm";
import { fireEvent } from "@testing-library/react";

describe("NoiseCancelingModeSelection", () => {
  afterEach(() => {
    cleanup();
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

  it("should fire onValueChanged when clicked", () => {
    const setMode = vi.fn();
    const renderResult = render(
      <NoiseCancelingModeSelection
        value={NoiseCancelingMode.Indoor}
        onValueChanged={setMode}
      />
    );

    fireEvent.click(renderResult.getByText("Outdoor"));

    expect(setMode).toHaveBeenCalledWith(NoiseCancelingMode.Outdoor);
  });
});
