import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
} from "../../wasm/pkg/openscq30_web_wasm";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

export interface SoundcoreDevice {
  get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState>;
  disconnect(): void;
  get name(): string | undefined;
  get ambientSoundMode(): AmbientSoundMode;
  get noiseCancelingMode(): NoiseCancelingMode;
  get equalizerConfiguration(): EqualizerConfiguration;
  transitionState(newState: SoundcoreDeviceState): Promise<void>;
}
