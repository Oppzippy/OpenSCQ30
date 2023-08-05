import { debounce } from "lodash-es";
import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { Device } from "../../../bluetooth/Device";
import { useBehaviorSubject } from "../../../hooks/useObservable";
import {
  DeviceState,
  EqualizerConfiguration,
} from "../../../libTypes/DeviceState";

/**
 * This will set up display state that is backed by actual state. When either the display state
 * or the actual state changes, they will be synchronized with each other.
 * @param device Device containing the actual state that backs the display state
 * @returns displayState and a setter for displayState
 */
export function useDisplayState(
  device: Device,
  onBluetoothError: (err: Error) => void,
): [DeviceState, Dispatch<SetStateAction<DeviceState>>] {
  const actualState = useBehaviorSubject(device.state);
  const [displayState, setDisplayState] = useState(actualState);

  useUpdateActualFromDisplay(device, displayState, onBluetoothError);
  useUpdateDisplayFromActual(setDisplayState, actualState);

  return [displayState, setDisplayState];
}

function useUpdateDisplayFromActual(
  setDisplayState: Dispatch<SetStateAction<DeviceState>>,
  actualState: DeviceState,
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
  device: Device,
  displayState: DeviceState,
  onBluetoothError: (err: Error) => void,
) {
  // Debounce so we don't spam the headphones with eq update packets
  // Since the function won't be run immediately, the actual state may be out of date if we pass it to this function
  // directly. Instead, pass the device. That way we will have a reference to the current actual state.
  const debouncedSetActualEqualizerConfiguration = useMemo(
    () =>
      debounce((equalizerConfiguration: EqualizerConfiguration) => {
        device
          .setEqualizerConfiguration(equalizerConfiguration)
          .catch(onBluetoothError);
      }, 500),
    [device, onBluetoothError],
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
    if (displayState.soundModes) {
      device.setSoundModes(displayState.soundModes).catch(onBluetoothError);
    }
  }, [device, displayState.soundModes, onBluetoothError]);
}
