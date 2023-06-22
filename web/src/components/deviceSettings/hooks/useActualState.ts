import { useCallback } from "react";
import { SoundcoreDevice } from "../../../bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "../../../bluetooth/SoundcoreDeviceState";
import { useBehaviorSubject } from "../../../hooks/useObservable";

/**
 * @returns Current state of the device, and a setState function with built in error handling
 */
export function useActualState(
  device: SoundcoreDevice,
  onBluetoothError: (err: Error) => void,
): [SoundcoreDeviceState, (newState: SoundcoreDeviceState) => void] {
  const actualState = useBehaviorSubject(device.state);

  const setActualState = useCallback(
    (newState: SoundcoreDeviceState) => {
      device.transitionState(newState).catch(onBluetoothError);
    },
    [device, onBluetoothError],
  );

  return [actualState, setActualState];
}
