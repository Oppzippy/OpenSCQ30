import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { VolumeSlider } from "../../../src/components/equalizer/VolumeSlider";

describe("App", () => {
  it("should only display one decimal place for value in spinbutton", () => {
    const renderResult = render(
      <VolumeSlider
        value={3.800001}
        hz={100}
        index={0}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onValueChange={() => {}}
      />,
    );
    const spinButton = renderResult.getByRole("spinbutton");
    expect(spinButton).instanceOf(HTMLInputElement);
    expect((spinButton as HTMLInputElement).value).toEqual("3.8");
  });

  it("should only display one decimal place for value in spinbutton", () => {
    const renderResult = render(
      <VolumeSlider
        value={3.799999}
        hz={100}
        index={0}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onValueChange={() => {}}
      />,
    );
    const spinButton = renderResult.getByRole("spinbutton");
    expect(spinButton).instanceOf(HTMLInputElement);
    expect((spinButton as HTMLInputElement).value).toEqual("3.8");
  });
});
