import { BehaviorSubject } from "rxjs";
import {
  AmbientSoundMode,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { SoundcoreDevice } from "./SoundcoreDevice";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

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

  public get soundModes() {
    return this._state.value.soundModes;
  }

  public get equalizerConfiguration() {
    return this._state.value.equalizerConfiguration;
  }

  public transitionState(newState: SoundcoreDeviceState) {
    console.debug("Transition device state", newState);
    this._state.next(newState);
    return Promise.resolve();
  }
}

export function selectDemoDevice() {
  return Promise.resolve(
    new DemoSoundcoreDevice("Demo Device", {
      soundModes: {
        ambientSoundMode: AmbientSoundMode.Normal,
        noiseCancelingMode: NoiseCancelingMode.Indoor,
      },
      equalizerConfiguration: EqualizerConfiguration.fromPresetProfile(
        PresetEqualizerProfile.SoundcoreSignature,
      ),
    }),
  );
}
