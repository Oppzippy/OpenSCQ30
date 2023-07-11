import { debounce } from "lodash-es";
import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { EqualizerConfiguration } from "../../../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDevice } from "../../../bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "../../../bluetooth/SoundcoreDeviceState";
import { useActualState } from "./useActualState";

/**
 * This will set up display state that is backed by actual state. When either the display state
 * or the actual state changes, they will be synchronized with each other.
 * @param device Device containing the actual state that backs the display state
 * @returns displayState and a setter for displayState
 */
export function useDisplayState(
  device: SoundcoreDevice,
  onBluetoothError: (err: Error) => void,
): [SoundcoreDeviceState, Dispatch<SetStateAction<SoundcoreDeviceState>>] {
  const [actualState, setActualState] = useActualState(
    device,
    onBluetoothError,
  );
  const [displayState, setDisplayState] = useState(actualState);

  useUpdateActualFromDisplay(device, setActualState, displayState);
  useUpdateDisplayFromActual(setDisplayState, actualState);

  return [displayState, setDisplayState];
}

function useUpdateDisplayFromActual(
  setDisplayState: Dispatch<SetStateAction<SoundcoreDeviceState>>,
  actualState: SoundcoreDeviceState,
) {
  // Synchronizes the displayed state with the actual state of the headphones. They are
  // different because of the equalizer debouncing.
  useEffect(() => {
    // An equalizer configuration change can never be initiated by the headphones, only us,
    // so we don't need to worry about keeping it in sync. Not updating it here fixes a bug
    // where the displayed equalizer state will revert for a short period of time after being
    // changed if a sound mode is changed before the debounce finishes.
    setDisplayState((state) => ({
      ...actualState,
      equalizerConfiguration: state.equalizerConfiguration,
    }));
  }, [actualState, setDisplayState]);
}

function useUpdateActualFromDisplay(
  device: SoundcoreDevice,
  setActualState: (state: SoundcoreDeviceState) => void,
  displayState: SoundcoreDeviceState,
) {
  // Debounce so we don't spam the headphones with eq update packets
  // Since the function won't be run immediately, the actual state may be out of date if we pass it to this function
  // directly. Instead, pass the device. That way we will have a reference to the current actual state.
  const debouncedSetActualEqualizerConfiguration = useMemo(
    () =>
      debounce((config: EqualizerConfiguration) => {
        setActualState({
          ambientSoundMode: device.state.value.ambientSoundMode,
          noiseCancelingMode: device.state.value.noiseCancelingMode,
          equalizerConfiguration: config,
        });
      }, 500),
    [device, setActualState],
  );

  // Update real equalizer configuration to match displayed with debounce
  useEffect(() => {
    debouncedSetActualEqualizerConfiguration(
      displayState.equalizerConfiguration,
    );
  }, [
    displayState.equalizerConfiguration,
    debouncedSetActualEqualizerConfiguration,
  ]);

  // Update ambient sound mode and noise canceling mode instantly
  useEffect(() => {
    setActualState({
      ambientSoundMode: displayState.ambientSoundMode,
      noiseCancelingMode: displayState.noiseCancelingMode,
      equalizerConfiguration: device.state.value.equalizerConfiguration,
    });
  }, [
    device,
    displayState.ambientSoundMode,
    displayState.noiseCancelingMode,
    setActualState,
  ]);
}
