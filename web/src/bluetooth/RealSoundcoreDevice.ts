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
  InboundPacket,
  RequestStatePacket,
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
import { SoundModesState, SoundcoreDeviceState } from "./SoundcoreDeviceState";

export class RealSoundcoreDevice implements SoundcoreDevice {
  private readonly connection: SoundcoreDeviceConnection;
  private readonly incomingPacketsSubscription: Subscription;
  private readonly _state: BehaviorSubject<SoundcoreDeviceState>;

  // Don't expose mutating methods
  public get state(): UnmodifiableBehaviorSubject<SoundcoreDeviceState> {
    return this._state;
  }

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

  public get soundModes() {
    return this._state.value.soundModes;
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
      soundModes: newState.soundModes,
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

  // TODO when incoming packets update the state, that triggers an outgoing packet to set
  // the state to what we just received. Incoming packets should not trigger outgoing packets.
  private onPacketReceived(bytes: Uint8Array) {
    try {
      const packet = new InboundPacket(bytes);
      if (packet.ambientSoundModeUpdate) {
        this._state.next({
          ...this._state.value,
          soundModes: {
            ambientSoundMode: packet.ambientSoundModeUpdate.ambientSoundMode,
            noiseCancelingMode:
              packet.ambientSoundModeUpdate.noiseCancelingMode,
          },
        });
      } else if (packet.stateUpdate) {
        let soundModes: SoundModesState | undefined;
        if (packet.stateUpdate.soundModes) {
          soundModes = {
            ambientSoundMode: packet.stateUpdate.soundModes.ambient_sound_mode,
            noiseCancelingMode:
              packet.stateUpdate.soundModes.noise_canceling_mode,
          };
        }
        this._state.next({
          soundModes,
          equalizerConfiguration: packet.stateUpdate.equalizerConfiguration,
        });
      } else if (!!packet.setEqualizerOk || !!packet.setSoundModeOk) {
        // ok
      } else {
        console.error("unhandled received packet, missing else if statement");
      }
    } catch (err) {
      console.error("error parsing packet", err);
    }
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
    map((value) => {
      try {
        return new InboundPacket(value).stateUpdate;
      } catch (err) {
        console.error("error parsing packet", err);
      }
    }),
    filter((packet): packet is StateUpdatePacket => packet != undefined),
    takeUntil(interval(4000)), // TODO retry on timeout
    first(),
  );
  const [, stateUpdatePacket] = await Promise.all([
    connection.write(new RequestStatePacket().bytes()),
    firstValueFrom(initialStateObservable),
  ]);

  let soundModes: SoundModesState | undefined;
  if (stateUpdatePacket.soundModes) {
    soundModes = {
      ambientSoundMode: stateUpdatePacket.soundModes.ambient_sound_mode,
      noiseCancelingMode: stateUpdatePacket.soundModes.noise_canceling_mode,
    };
  }
  const initialState = {
    soundModes,
    equalizerConfiguration: stateUpdatePacket.equalizerConfiguration,
  };
  return new RealSoundcoreDevice(connection, initialState);
}
