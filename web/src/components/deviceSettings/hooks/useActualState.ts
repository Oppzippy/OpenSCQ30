import { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { SoundcoreDevice } from "../../../bluetooth/SoundcoreDevice";
import { SoundcoreDeviceState } from "../../../bluetooth/SoundcoreDeviceState";
import { useBehaviorSubject } from "../../../hooks/useObservable";
import { useToastErrorHandler } from "../../../hooks/useToastErrorHandler";

/**
 * @returns Current state of the device, and a setState function with built in error handling
 */
export function useActualState(
  device: SoundcoreDevice
): [SoundcoreDeviceState, (newState: SoundcoreDeviceState) => void] {
  const actualState = useBehaviorSubject(device.state);

  const { t } = useTranslation();
  const errorHandler = useToastErrorHandler(
    t("errors.failedToCommunicateStateChangeToDevice")
  );
  const setActualState = useCallback(
    (newState: SoundcoreDeviceState) => {
      device.transitionState(newState).catch(errorHandler);
    },
    [device, errorHandler]
  );

  return [actualState, setActualState];
}
