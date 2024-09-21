import { BehaviorSubject } from "rxjs";
import {
  Device as LibDevice,
  SoundcoreDeviceUtils,
} from "../../wasm/pkg/openscq30_web_wasm";
import {
  CustomButtonModel,
  DeviceState,
  DeviceStateValidator,
  EqualizerConfiguration,
  SoundModes,
  SoundModesTypeTwo,
} from "../libTypes/DeviceState";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { DeviceStateValidationError } from "./StateValidationError";

export class Device {
  private libDevice: LibDevice;
  public readonly state: UnmodifiableBehaviorSubject<DeviceState>;
  public readonly name: string;

  public constructor(
    libDevice: LibDevice,
    initialState: DeviceState,
    name: string,
  ) {
    this.libDevice = libDevice;
    this.name = name;
    const stateSubject = new BehaviorSubject(initialState);
    this.state = stateSubject;

    // State change listening is infallible and can be set up in the background
    void libDevice.setStateChangeListener((json: string) => {
      try {
        stateSubject.next(Device.parseState(json));
      } catch (err) {
        // TODO alert user
        console.error(err);
      }
    });
  }

  public static async new(libDevice: LibDevice) {
    const initialState = Device.parseState(await libDevice.getState());
    const name = await libDevice.getName();
    return new Device(libDevice, initialState, name);
  }

  public async setSoundModes(soundModes: SoundModes) {
    await this.libDevice.setSoundModes(JSON.stringify(soundModes));
  }

  public async setSoundModesTypeTwo(soundModes: SoundModesTypeTwo) {
    await this.libDevice.setSoundModesTypeTwo(JSON.stringify(soundModes));
  }

  public async setEqualizerConfiguration(
    equalizerConfiguration: EqualizerConfiguration,
  ) {
    await this.libDevice.setEqualizerConfiguration(
      JSON.stringify(equalizerConfiguration),
    );
  }

  public async setCustomButtonModel(customButtonModel: CustomButtonModel) {
    await this.libDevice.setCustomButtonModel(
      JSON.stringify(customButtonModel),
    );
  }

  public destroy() {
    this.libDevice.free();
  }

  private static parseState(json: string) {
    const state: unknown = JSON.parse(json);
    if (DeviceStateValidator.Check(state)) {
      return state;
    } else {
      const errors = [...DeviceStateValidator.Errors(state)];
      throw new DeviceStateValidationError(state, errors);
    }
  }
}

export async function selectDevice(): Promise<Device> {
  // TODO can we set the types in wasm-bindgen?
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
  const serviceUuids: string[] = SoundcoreDeviceUtils.getServiceUuids();

  const macAddressPrefixes = SoundcoreDeviceUtils.getMacAddressPrefixes();
  const device = await navigator.bluetooth.requestDevice({
    // We would filter by available services, but this doesn't seem to work on chromium based browsers on platforms
    // other than Linux without first going to about://bluetooth-internals/#devices, scanning for your device, and
    // then inspecting it.
    // filters: [{ services: [serviceUuid] }],
    filters: macAddressPrefixes.map((prefix) => ({
      manufacturerData: [
        {
          // Non standard manufacturer data format: mac address followed by 0x00 0x00
          // companyIdentifier is picked up as the first two bytes of the mac address
          companyIdentifier: (prefix[1] << 8) | prefix[0],
          // data is everything after those first two bytes. Since we want to filter by the first three bytes of the
          // mac address, that just leaves one more byte.
          dataPrefix: Uint8Array.of(prefix[2]),
        },
      ],
    })),
    optionalServices: serviceUuids,
  });
  const libDevice = await LibDevice.new(device);
  return await Device.new(libDevice);
}

export async function selectDemoDevice(): Promise<Device> {
  const libDevice = await LibDevice.newDemo();
  return await Device.new(libDevice);
}
