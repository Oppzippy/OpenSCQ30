import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
} from "../../wasm/pkg/openscq30_web_wasm";

export interface SoundcoreDeviceState {
  soundModes: SoundModesState | undefined;
  equalizerConfiguration: EqualizerConfiguration;
}

export interface SoundModesState {
  ambientSoundMode: AmbientSoundMode;
  noiseCancelingMode: NoiseCancelingMode;
}
