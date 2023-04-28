import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
} from "../../wasm/pkg/openscq30_web_wasm";

export type SoundcoreDeviceState = {
  ambientSoundMode: AmbientSoundMode;
  noiseCancelingMode: NoiseCancelingMode;
  equalizerConfiguration: EqualizerConfiguration;
};
