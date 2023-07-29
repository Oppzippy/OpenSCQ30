import {
  AmbientSoundMode,
  SetSoundModePacket,
} from "../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDeviceConnection } from "./SoundcoreDeviceConnection";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

export async function transitionSoundMode(
  connection: SoundcoreDeviceConnection,
  previousState: SoundcoreDeviceState,
  newState: SoundcoreDeviceState,
) {
  const previousSoundModes = previousState.soundModes;
  const newSoundModes = newState.soundModes;
  if (previousSoundModes == null || newSoundModes == null) return;

  if (
    previousSoundModes.ambientSoundMode == newSoundModes.ambientSoundMode &&
    previousSoundModes.noiseCancelingMode == newSoundModes.noiseCancelingMode
  ) {
    return;
  }
  const didNoiseCancelingModeChange =
    previousSoundModes.noiseCancelingMode != newSoundModes.noiseCancelingMode;
  // When changing noise canceling mode with ambient sound mode not set to noise canceling, things get buggy
  // First switch into noise canceling mode if needed
  if (
    didNoiseCancelingModeChange &&
    previousSoundModes.ambientSoundMode != AmbientSoundMode.NoiseCanceling
  ) {
    await connection.write(
      new SetSoundModePacket(
        AmbientSoundMode.NoiseCanceling,
        previousSoundModes.noiseCancelingMode,
      ).bytes(),
    );
  }
  // If we are changing the moise canceling mode, we must be in noise canceling mode at this point
  // Otherwise, set the ambient sound mode to the new state.
  await connection.write(
    new SetSoundModePacket(
      didNoiseCancelingModeChange
        ? AmbientSoundMode.NoiseCanceling
        : newSoundModes.ambientSoundMode,
      newSoundModes.noiseCancelingMode,
    ).bytes(),
  );
  // Set the ambient sound mode to the new state if we didn't already do it in the previous step
  if (
    didNoiseCancelingModeChange &&
    newSoundModes.ambientSoundMode != AmbientSoundMode.NoiseCanceling
  ) {
    await connection.write(
      new SetSoundModePacket(
        newSoundModes.ambientSoundMode,
        newSoundModes.noiseCancelingMode,
      ).bytes(),
    );
  }
}
