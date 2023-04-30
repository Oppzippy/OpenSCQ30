import { expect, test } from "vitest";
import renderer from "react-test-renderer";
import { NoiseCancelingModeSelection } from "../src/components/NoiseCancelingModeSelection";
import { NoiseCancelingMode } from "../wasm/pkg/openscq30_web_wasm";

test("it renders", () => {
  const component = renderer.create(
    <NoiseCancelingModeSelection
      value={NoiseCancelingMode.Indoor}
      onValueChanged={() => {
        throw new Error("Function not implemented.");
      }}
    />
  );

  const tree = component.toJSON();
  expect(tree).toMatchSnapshot();
});
