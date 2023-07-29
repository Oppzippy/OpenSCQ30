import { EqualizerConfiguration } from "../../wasm/pkg/openscq30_web_wasm";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { SoundModesState, SoundcoreDeviceState } from "./SoundcoreDeviceState";

export interface SoundcoreDevice {
  get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState>;
  disconnect(): void;
  get name(): string | undefined;
  get soundModes(): SoundModesState | undefined;
  get equalizerConfiguration(): EqualizerConfiguration;
  transitionState(newState: SoundcoreDeviceState): Promise<void>;
}
