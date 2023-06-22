import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { BehaviorSubject } from "rxjs";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SoundcoreDevice } from "../../../src/bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "../../../src/bluetooth/SoundcoreDeviceState";
import { ToastQueue } from "../../../src/components/ToastQueue";
import { DeviceSettings } from "../../../src/components/deviceSettings/DeviceSettings";
import { useCustomEqualizerProfiles } from "../../../src/components/deviceSettings/hooks/useCustomEqualizerProfiles";
import { upsertCustomEqualizerProfile } from "../../../src/storage/customEqualizerProfiles";
import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../../wasm/pkg/openscq30_web_wasm";

vi.mock(
  "../../../src/components/deviceSettings/hooks/useCustomEqualizerProfiles",
  () => {
    return {
      useCustomEqualizerProfiles: vi.fn(() => []),
    };
  },
);

vi.mock("../../../src/storage/customEqualizerProfiles", () => {
  return {
    upsertCustomEqualizerProfile: vi.fn(),
  };
});

describe("Device Settings", () => {
  let device: SoundcoreDevice;
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    vi.useFakeTimers({
      shouldAdvanceTime: true,
    });
    user = userEvent.setup();
    const mockDevice = {
      state: new BehaviorSubject<{
        ambientSoundMode: AmbientSoundMode;
        noiseCancelingMode: NoiseCancelingMode;
        equalizerConfiguration: EqualizerConfiguration;
      }>({
        ambientSoundMode: AmbientSoundMode.NoiseCanceling,
        noiseCancelingMode: NoiseCancelingMode.Transport,
        equalizerConfiguration: EqualizerConfiguration.fromPresetProfile(
          PresetEqualizerProfile.SoundcoreSignature,
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
      async transitionState(newState: SoundcoreDeviceState) {
        this.state.next(newState);
      },
    };
    device = mockDevice as unknown as SoundcoreDevice;
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should change ambient sound mode", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    expect(device.ambientSoundMode).toEqual(AmbientSoundMode.NoiseCanceling);
    await user.click(renderResult.getByText("ambientSoundMode.normal"));

    expect(device.ambientSoundMode).toEqual(AmbientSoundMode.Normal);
  });

  it("should change noise canceling mode", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    expect(device.noiseCancelingMode).toEqual(NoiseCancelingMode.Transport);
    await user.click(renderResult.getByText("noiseCancelingMode.indoor"));
    expect(device.noiseCancelingMode).toEqual(NoiseCancelingMode.Indoor);
  });

  it("should change equalizer configuration", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    expect([
      ...device.equalizerConfiguration.volumeAdjustments.adjustments,
    ]).toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
    await user.click(
      renderResult.getByText("presetEqualizerProfile.soundcoreSignature"),
    );
    await user.click(
      renderResult.getByText("presetEqualizerProfile.classical"),
    );
    vi.advanceTimersByTime(5000);
    expect([
      ...device.equalizerConfiguration.volumeAdjustments.adjustments,
    ]).not.toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
  });

  it("should switch to custom profile when moving a silder", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    const numberInputs = renderResult.baseElement.querySelectorAll(
      "input[type='number']",
    );
    await user.type(numberInputs[0], "1");
    expect(
      renderResult.getByLabelText("equalizer.profile").textContent,
    ).toEqual("equalizer.custom");
  });

  it("should not show custom profile create/delete buttons when a preset is selected", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    expect(
      renderResult.queryByRole("button", { name: "application.create" }),
    ).toBeFalsy();
    expect(
      renderResult.queryByRole("button", { name: "application.delete" }),
    ).toBeFalsy();
  });

  it("should show only one of a custom profile or a preset profile", async () => {
    (useCustomEqualizerProfiles as ReturnType<typeof vi.fn>).mockReturnValue([
      { name: "test", values: [0, 0, 0, 0, 0, 0, 0, 0], id: 1 },
    ]);
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    // Check only preset profile shown
    expect(
      renderResult.getByLabelText("equalizer.profile").textContent,
    ).toEqual("presetEqualizerProfile.soundcoreSignature");
    expect(
      renderResult.getByLabelText("equalizer.customProfile").textContent,
    ).not.toEqual("test");

    // Check only custom profile shown
    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", {
        name: "equalizer.custom",
      }),
    );
    expect(
      renderResult.getByLabelText("equalizer.profile").textContent,
    ).toEqual("equalizer.custom");
    expect(
      renderResult.getByLabelText("equalizer.customProfile").textContent,
    ).toEqual("test");
  });

  it("should synchronize sliders and number input values", async () => {
    const renderResult = render(
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" }),
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
      <DeviceSettings
        device={device as unknown as SoundcoreDevice}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        disconnect={() => {}}
      />,
    );

    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" }),
    );

    expect(device.state.value.equalizerConfiguration.presetProfile).toEqual(
      PresetEqualizerProfile.SoundcoreSignature,
    );
    vi.advanceTimersByTime(500);
    expect(
      device.state.value.equalizerConfiguration.presetProfile,
    ).toBeUndefined();

    const numberInputs: NodeListOf<HTMLInputElement> =
      renderResult.baseElement.querySelectorAll("input[type='number']");
    await user.type(numberInputs[0], "1");

    expect(
      device.state.value.equalizerConfiguration.volumeAdjustments
        .adjustments[0],
    ).toEqual(0);
    vi.advanceTimersByTime(500);
    expect(
      device.state.value.equalizerConfiguration.volumeAdjustments
        .adjustments[0],
    ).toEqual(10);
  });

  it("should display a toast when creating a custom profile fails", async () => {
    (
      upsertCustomEqualizerProfile as ReturnType<typeof vi.fn>
    ).mockRejectedValue(new Error("It should error"));
    const renderResult = render(
      <ToastQueue>
        <DeviceSettings
          device={device as unknown as SoundcoreDevice}
          // eslint-disable-next-line @typescript-eslint/no-empty-function
          disconnect={() => {}}
        />
      </ToastQueue>,
    );

    await user.click(renderResult.getByLabelText("equalizer.profile"));
    await user.click(
      renderResult.getByRole("option", { name: "equalizer.custom" }),
    );
    await user.click(
      renderResult.getByRole("button", {
        name: "equalizer.createCustomProfile",
      }),
    );
    await user.type(
      renderResult.getByLabelText("equalizer.profileName"),
      "test",
    );

    const consoleErrorMock = vi
      .spyOn(console, "error")
      .mockImplementation(() => {
        // do nothing
      });
    await user.click(
      renderResult.getByRole("button", { name: "application.create" }),
    );
    expect(consoleErrorMock).toHaveBeenCalled();
    consoleErrorMock.mockRestore();

    expect(
      renderResult.queryByText("errors.failedToCreateCustomProfile"),
    ).toBeTruthy();
  });
});
