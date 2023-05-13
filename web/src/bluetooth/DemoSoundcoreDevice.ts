import { BehaviorSubject } from "rxjs";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { SoundcoreDevice } from "./SoundcoreDevice";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";
import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";

export class DemoSoundcoreDevice implements SoundcoreDevice {
  public readonly name: string;
  private readonly _state: BehaviorSubject<SoundcoreDeviceState>;

  public constructor(name: string, initialState: SoundcoreDeviceState) {
    this._state = new BehaviorSubject(initialState);
    this.name = name;
  }

  public get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState> {
    return this._state;
  }

  public disconnect() {
    // do nothing
  }

  public get ambientSoundMode() {
    return this._state.value.ambientSoundMode;
  }

  public get noiseCancelingMode() {
    return this._state.value.noiseCancelingMode;
  }

  public get equalizerConfiguration() {
    return this._state.value.equalizerConfiguration;
  }

  public async transitionState(newState: SoundcoreDeviceState) {
    this._state.next(newState);
  }
}

export async function selectDemoDevice() {
  return new DemoSoundcoreDevice("Demo Device", {
    ambientSoundMode: AmbientSoundMode.Normal,
    noiseCancelingMode: NoiseCancelingMode.Indoor,
    equalizerConfiguration: EqualizerConfiguration.fromPresetProfile(
      PresetEqualizerProfile.SoundcoreSignature,
    ),
  });
}
