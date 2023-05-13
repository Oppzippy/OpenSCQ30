import {
  AmbientSoundMode,
  SetAmbientSoundModePacket,
} from "../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDeviceConnection } from "./SoundcoreDeviceConnection";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

export async function transitionSoundMode(
  connection: SoundcoreDeviceConnection,
  previousState: SoundcoreDeviceState,
  newState: SoundcoreDeviceState,
) {
  if (
    previousState.ambientSoundMode == newState.ambientSoundMode &&
    previousState.noiseCancelingMode == newState.noiseCancelingMode
  ) {
    return;
  }
  const didNoiseCancelingModeChange =
    previousState.noiseCancelingMode != newState.noiseCancelingMode;
  // When changing noise canceling mode with ambient sound mode not set to noise canceling, things get buggy
  // First switch into noise canceling mode if needed
  if (
    didNoiseCancelingModeChange &&
    previousState.ambientSoundMode != AmbientSoundMode.NoiseCanceling
  ) {
    await connection.write(
      new SetAmbientSoundModePacket(
        AmbientSoundMode.NoiseCanceling,
        previousState.noiseCancelingMode,
      ).bytes(),
    );
  }
  // If we are changing the moise canceling mode, we must be in noise canceling mode at this point
  // Otherwise, set the ambient sound mode to the new state.
  await connection.write(
    new SetAmbientSoundModePacket(
      didNoiseCancelingModeChange
        ? AmbientSoundMode.NoiseCanceling
        : newState.ambientSoundMode,
      newState.noiseCancelingMode,
    ).bytes(),
  );
  // Set the ambient sound mode to the new state if we didn't already do it in the previous step
  if (
    didNoiseCancelingModeChange &&
    newState.ambientSoundMode != AmbientSoundMode.NoiseCanceling
  ) {
    await connection.write(
      new SetAmbientSoundModePacket(
        newState.ambientSoundMode,
        newState.noiseCancelingMode,
      ).bytes(),
    );
  }
}
