import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { BehaviorSubject } from "rxjs";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SoundcoreDevice } from "../../src/bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "../../src/bluetooth/SoundcoreDeviceState";
import { DeviceSettings } from "../../src/components/DeviceSettings";
import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";

interface HasState {
  state: BehaviorSubject<{
    ambientSoundMode: AmbientSoundMode;
    noiseCancelingMode: NoiseCancelingMode;
    equalizerConfiguration: EqualizerConfiguration;
  }>;
}

function decorateWithGettersAndSetters<T extends HasState>(device: T) {
  return {
    ...device,
    get ambientSoundMode() {
      return this.state.value.ambientSoundMode;
    },
    get noiseCancelingMode() {
      return this.state.value.noiseCancelingMode;
    },
    get equalizerConfiguration() {
      return this.state.value.equalizerConfiguration;
    },
    transitionState(newState: SoundcoreDeviceState) {
      this.state.next(newState);
    },
  };
}

describe("Device Settings", () => {
  let device: SoundcoreDevice;
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    vi.useFakeTimers({
      shouldAdvanceTime: true,
    });
    user = userEvent.setup();
    const mockDevice = decorateWithGettersAndSetters({
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
    });
    device = mockDevice as unknown as SoundcoreDevice;
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should change ambient sound mode", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );

    expect(device.ambientSoundMode).toEqual(AmbientSoundMode.NoiseCanceling);
    await user.click(renderResult.getByText("ambientSoundMode.normal"));

    expect(device.ambientSoundMode).toEqual(AmbientSoundMode.Normal);
  });

  it("should change noise canceling mode", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );

    expect(device.noiseCancelingMode).toEqual(NoiseCancelingMode.Transport);
    await user.click(renderResult.getByText("noiseCancelingMode.indoor"));
    expect(device.noiseCancelingMode).toEqual(NoiseCancelingMode.Indoor);
  });

  it("should change equalizer configuration", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );

    expect([
      ...device.equalizerConfiguration.bandOffsets.volumeOffsets,
    ]).toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
    await user.click(
      renderResult.getByText("equalizerPresetProfile.soundcoreSignature")
    );
    await user.click(
      renderResult.getByText("equalizerPresetProfile.classical")
    );
    vi.advanceTimersByTime(5000);
    expect([
      ...device.equalizerConfiguration.bandOffsets.volumeOffsets,
    ]).not.toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
  });

  it("should enable sliders when a custom profile is selected", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );
    function areSlidersDisabled() {
      const sliders: NodeListOf<HTMLInputElement> =
        renderResult.baseElement.querySelectorAll("input[type='range']");
      return [...sliders].every((slider) => slider.disabled);
    }

    expect(areSlidersDisabled()).toEqual(true);
    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" })
    );
    expect(areSlidersDisabled()).toEqual(false);
  });

  it("should synchronize sliders and number input values", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );

    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" })
    );

    const numberInputs: NodeListOf<HTMLInputElement> =
      renderResult.baseElement.querySelectorAll("input[type='number']");
    await user.type(numberInputs[0], "12");
    const sliders: NodeListOf<HTMLInputElement> =
      renderResult.baseElement.querySelectorAll("input[type='range']");
    expect(Number(sliders[0].value)).toEqual(12);
  });

  it("should debounce equalizer updates", async () => {
    const renderResult = render(
      <DeviceSettings device={device as unknown as SoundcoreDevice} />
    );

    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" })
    );

    expect(device.state.value.equalizerConfiguration.presetProfile).toEqual(
      PresetEqualizerProfile.SoundcoreSignature
    );
    vi.advanceTimersByTime(500);
    expect(
      device.state.value.equalizerConfiguration.presetProfile
    ).toBeUndefined();

    const numberInputs: NodeListOf<HTMLInputElement> =
      renderResult.baseElement.querySelectorAll("input[type='number']");
    await user.type(numberInputs[0], "1");

    expect(
      device.state.value.equalizerConfiguration.bandOffsets.volumeOffsets[0]
    ).toEqual(0);
    vi.advanceTimersByTime(500);
    expect(
      device.state.value.equalizerConfiguration.bandOffsets.volumeOffsets[0]
    ).toEqual(10);
  });
});
