import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { BehaviorSubject } from "rxjs";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SoundcoreDevice } from "../../src/bluetooth/SoundcoreDevice";
import App from "../../src/components/App";
import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";

describe("App", () => {
  let user: ReturnType<typeof userEvent.setup>;

  function mockDevice() {
    vi.mock("../../src/bluetooth/RealSoundcoreDevice", () => {
      return {
        async selectDevice() {
          const mockDevice = {
            state: new BehaviorSubject<{
              ambientSoundMode: AmbientSoundMode;
              noiseCancelingMode: NoiseCancelingMode;
              equalizerConfiguration: EqualizerConfiguration;
            }>({
              ambientSoundMode: AmbientSoundMode.NoiseCanceling,
              noiseCancelingMode: NoiseCancelingMode.Transport,
              equalizerConfiguration: EqualizerConfiguration.fromPresetProfile(
                PresetEqualizerProfile.SoundcoreSignature
              ),
            }),
            connect: vi.fn<unknown[], unknown>(),
            get ambientSoundMode() {
              return this.state.value.ambientSoundMode;
            },
            get noiseCancelingMode() {
              return this.state.value.noiseCancelingMode;
            },
            get equalizerConfiguration() {
              return this.state.value.equalizerConfiguration;
            },
          };
          return mockDevice as unknown as SoundcoreDevice;
        },
      };
    });
  }

  beforeEach(() => {
    user = userEvent.setup();
  });

  afterEach(() => {
    const nav = navigator as object;
    if ("bluetooth" in nav) {
      delete nav.bluetooth;
    }
  });

  it("should have a link to github", () => {
    const renderResult = render(<App />);
    const link = renderResult.baseElement.querySelector(
      "a[href='https://github.com/oppzippy/OpenSCQ30']"
    );
    expect(link).not.toBeNull();
  });

  it("should have a disconnect button", async () => {
    navigator.bluetooth = {} as Bluetooth;
    mockDevice();
    const renderResult = render(<App />);

    await user.click(renderResult.getByText("device.selectDevice"));
    expect(renderResult.queryByText("device.disconnect")).not.toBeNull();
  });
});