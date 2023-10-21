import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { BehaviorSubject } from "rxjs";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Device } from "../../src/bluetooth/Device";
import App from "../../src/components/App";
import {
  DeviceState,
  EqualizerConfiguration,
  SoundModes,
} from "../../src/libTypes/DeviceState";
import { EqualizerHelper } from "../../wasm/pkg/openscq30_web_wasm";

describe("App", () => {
  let user: ReturnType<typeof userEvent.setup>;

  function mockDevice() {
    vi.mock("../../src/bluetooth/Device", () => {
      return {
        async selectDevice() {
          const mockDevice = {
            state: new BehaviorSubject<DeviceState>({
              featureFlags: 0,
              battery: {
                type: "singleBattery",
                isCharging: true,
                level: 5,
              },
              soundModes: {
                ambientSoundMode: "noiseCanceling",
                noiseCancelingMode: "transport",
                transparencyMode: "fullyTransparent",
                customNoiseCanceling: 0,
              },
              equalizerConfiguration: {
                presetProfile: "SoundcoreSignature",
                volumeAdjustments: [
                  ...EqualizerHelper.getPresetProfileVolumeAdjustments(
                    "SoundcoreSignature",
                  ),
                ],
              },
              ageRange: null,
              gender: null,
              customButtonModel: null,
              hearId: null,
              dynamicRangeCompressionMinFirmwareVersion: null,
              leftFirmwareVersion: null,
              rightFirmwareVersion: null,
              serialNumber: null,
            }),
            connect: vi.fn<unknown[], unknown>(),
            async setSoundModes(soundModes: SoundModes) {
              this.state.next({
                ...this.state.value,
                soundModes,
              });
            },
            async setEqualizerConfiguration(
              equalizerConfiguration: EqualizerConfiguration,
            ) {
              this.state.next({
                ...this.state.value,
                equalizerConfiguration,
              });
            },
          };
          // eslint-disable-next-line @typescript-eslint/no-unsafe-return
          return mockDevice as unknown as Device;
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
      "a[href='https://github.com/oppzippy/OpenSCQ30']",
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
