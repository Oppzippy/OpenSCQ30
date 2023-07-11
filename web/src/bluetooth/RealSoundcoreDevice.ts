import {
  BehaviorSubject,
  Subscription,
  filter,
  first,
  firstValueFrom,
  interval,
  map,
  takeUntil,
} from "rxjs";
import {
  AmbientSoundModeUpdatePacket,
  RequestStatePacket,
  SetAmbientModeOkPacket,
  SetEqualizerOkPacket,
  StateUpdatePacket,
} from "../../wasm/pkg/openscq30_web_wasm";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";
import { transitionEqualizerState } from "./EqualizerConfigurationStateTransition";
import { transitionSoundMode } from "./SoundModeStateTransition";
import { SoundcoreDevice } from "./SoundcoreDevice";
import {
  SoundcoreDeviceConnection,
  selectDeviceConnection,
} from "./SoundcoreDeviceConnection";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

export class RealSoundcoreDevice implements SoundcoreDevice {
  private readonly connection: SoundcoreDeviceConnection;
  private readonly incomingPacketsSubscription: Subscription;
  private readonly _state: BehaviorSubject<SoundcoreDeviceState>;

  // Don't expose mutating methods
  public get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState> {
    return this._state;
  }

  // TODO when incoming packets update the state, that triggers an outgoing packet to set
  // the state to what we just received. Incoming packets should not trigger outgoing packets.
  private packetHandlers: ((bytes: Uint8Array) => boolean)[] = [
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

  public constructor(
    connection: SoundcoreDeviceConnection,
    state: SoundcoreDeviceState,
  ) {
    this.connection = connection;
    this._state = new BehaviorSubject(state);
    this.incomingPacketsSubscription = connection.incomingPackets.subscribe(
      (value) => {
        this.onPacketReceived(value);
      },
    );
  }

  public disconnect() {
    this.incomingPacketsSubscription.unsubscribe();
    this.connection.disconnect();
  }

  public get name() {
    return this.connection.name;
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
    try {
      await this.attemptTransitionState(newState);
    } catch (err) {
      if (err instanceof DOMException) {
        // Disconnected from headphones, unsure if any other errors share the same name
        if (err.name == "NetworkError") {
          await this.reconnect();
          // Only attempt once more, give up if it fails
          await this.attemptTransitionState(newState);
          return;
        }
      }
      throw err;
    }
  }

  private async attemptTransitionState(newState: SoundcoreDeviceState) {
    await transitionSoundMode(this.connection, this.state.value, newState);
    this._state.next({
      ...this.state.value,
      ambientSoundMode: newState.ambientSoundMode,
      noiseCancelingMode: newState.noiseCancelingMode,
    });
    await transitionEqualizerState(this.connection, this.state.value, newState);
    this._state.next({
      ...this.state.value,
      equalizerConfiguration: newState.equalizerConfiguration,
    });
  }

  private async reconnect() {
    if (!this.connection.connected) {
      await this.connection.reconnect();
      await this.connection.write(new RequestStatePacket().bytes());
    }
  }

  private onPacketReceived(bytes: Uint8Array) {
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
  connection: SoundcoreDeviceConnection,
): Promise<RealSoundcoreDevice> {
  const initialStateObservable = connection.incomingPackets.pipe(
    map((value) => StateUpdatePacket.fromBytes(new Uint8Array(value.buffer))),
    filter((packet): packet is StateUpdatePacket => packet != undefined),
    takeUntil(interval(4000)), // TODO retry on timeout
    first(),
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
  return new RealSoundcoreDevice(connection, initialState);
}
