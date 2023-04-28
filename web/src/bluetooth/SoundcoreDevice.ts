import {
  BehaviorSubject,
  Subscription,
  debounceTime,
  distinct,
  filter,
  first,
  firstValueFrom,
  interval,
  map,
  pairwise,
  startWith,
  takeUntil,
} from "rxjs";
import { SetEqualizerOkPacket } from "../../wasm/pkg/openscq30_web_wasm";
import { SetAmbientModeOkPacket } from "../../wasm/pkg/openscq30_web_wasm";
import { AmbientSoundModeUpdatePacket } from "../../wasm/pkg/openscq30_web_wasm";
import { EqualizerConfiguration } from "../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";
import {
  AmbientSoundMode,
  NoiseCancelingMode,
  RequestStatePacket,
  StateUpdatePacket,
} from "../../wasm/pkg/openscq30_web_wasm";
import {
  SoundcoreDeviceConnection,
  selectDeviceConnection,
} from "./SoundcoreDeviceConnection";
import { transitionSoundMode } from "./SoundModeStateTransition";
import { transitionEqualizerState } from "./EqualizerConfigurationStateTransition";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";

export class SoundcoreDevice {
  private readonly connection: SoundcoreDeviceConnection;
  private readonly incomingPacketsSubscription: Subscription;
  private readonly stateChangeSubscription: Subscription;
  private readonly _state: BehaviorSubject<SoundcoreDeviceState>;

  // Don't expose mutating methods
  public get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState> {
    return this._state;
  }

  private packetHandlers: Array<(bytes: Uint8Array) => boolean> = [
    (bytes) => {
      const packet = AmbientSoundModeUpdatePacket.fromBytes(bytes);
      if (packet) {
        this._state.next({
          ...this._state.value,
          ambientSoundMode: packet.ambientSoundMode,
          noiseCancelingMode: packet.noiseCancelingMode,
        });
        return true;
      }
      return false;
    },
    (bytes) => {
      const packet = StateUpdatePacket.fromBytes(bytes);
      if (packet) {
        this._state.next({
          ambientSoundMode: packet.ambientSoundMode,
          noiseCancelingMode: packet.noiseCancelingMode,
          equalizerConfiguration: packet.equalizerConfiguration,
        });
        return true;
      }
      return false;
    },
    (bytes) => {
      const packet = SetAmbientModeOkPacket.fromBytes(bytes);
      return !!packet;
    },
    (bytes) => {
      const packet = SetEqualizerOkPacket.fromBytes(bytes);
      return !!packet;
    },
  ];

  constructor(
    connection: SoundcoreDeviceConnection,
    state: SoundcoreDeviceState
  ) {
    this.connection = connection;
    this._state = new BehaviorSubject(state);
    this.incomingPacketsSubscription = connection.incomingPackets.subscribe(
      (value) => {
        this.onPacketReceived(value);
      }
    );
    this.stateChangeSubscription = this._state
      .pipe(startWith(state), distinct(), debounceTime(250), pairwise())
      .subscribe(([previousState, newState]) => {
        transitionSoundMode(connection, previousState, newState);
        transitionEqualizerState(connection, previousState, newState);
      });
  }

  public disconnect() {
    this.stateChangeSubscription.unsubscribe();
    this.incomingPacketsSubscription.unsubscribe();
    this.connection.disconnect();
  }

  public get name() {
    return this.connection.name;
  }

  public get ambientSoundMode() {
    return this._state.value.ambientSoundMode;
  }

  public set ambientSoundMode(ambientSoundMode: AmbientSoundMode) {
    this._state.next({
      ...this._state.value,
      ambientSoundMode,
    });
  }

  public get noiseCancelingMode() {
    return this._state.value.noiseCancelingMode;
  }

  public set noiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
    this._state.next({
      ...this._state.value,
      noiseCancelingMode,
    });
  }

  public get equalizerConfiguration() {
    return this._state.value.equalizerConfiguration;
  }

  public set equalizerConfiguration(
    equalizerConfiguration: EqualizerConfiguration
  ) {
    this._state.next({
      ...this._state.value,
      equalizerConfiguration,
    });
  }

  public async write(value: BufferSource) {
    await this.connection.write(value);
  }

  public async onPacketReceived(bytes: Uint8Array) {
    for (const handler of this.packetHandlers) {
      if (handler(bytes)) {
        return;
      }
    }
    console.error("No handler found for packet", bytes);
  }
}

export async function selectDevice(): Promise<SoundcoreDevice> {
  const device = await selectDeviceConnection();
  return await createSoundcoreDevice(device);
}

async function createSoundcoreDevice(
  connection: SoundcoreDeviceConnection
): Promise<SoundcoreDevice> {
  const initialStateObservable = connection.incomingPackets.pipe(
    map((value) => StateUpdatePacket.fromBytes(new Uint8Array(value.buffer))),
    filter((packet): packet is StateUpdatePacket => packet != undefined),
    takeUntil(interval(4000)), // TODO retry on timeout
    first()
  );
  const [, stateUpdatePacket] = await Promise.all([
    connection.write(new RequestStatePacket().bytes()),
    firstValueFrom(initialStateObservable),
  ]);
  const initialState = {
    ambientSoundMode: stateUpdatePacket.ambientSoundMode,
    noiseCancelingMode: stateUpdatePacket.noiseCancelingMode,
    equalizerConfiguration: stateUpdatePacket.equalizerConfiguration,
  };
  return new SoundcoreDevice(connection, initialState);
}
