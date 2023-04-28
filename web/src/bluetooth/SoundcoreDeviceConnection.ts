import { SoundcoreDeviceUtils } from "../../wasm/pkg/openscq30_web_wasm";
import { Observable } from "rxjs";

export class SoundcoreDeviceConnection {
  public readonly incomingPackets: Observable<Uint8Array>;

  private readonly gatt: BluetoothRemoteGATTServer;
  private readonly writeCharacteristic: BluetoothRemoteGATTCharacteristic;
  private readonly readCharacteristic: BluetoothRemoteGATTCharacteristic;

  constructor(
    gatt: BluetoothRemoteGATTServer,
    writeCharacteristic: BluetoothRemoteGATTCharacteristic,
    readCharacteristic: BluetoothRemoteGATTCharacteristic
  ) {
    this.gatt = gatt;
    this.writeCharacteristic = writeCharacteristic;
    this.readCharacteristic = readCharacteristic;

    this.incomingPackets = new Observable((subscriber) => {
      const handler = () => {
        if (readCharacteristic.value) {
          subscriber.next(new Uint8Array(readCharacteristic.value.buffer));
        } else {
          console.error(
            "Read characteristic value changed, but it is undefined?"
          );
        }
      };
      this.readCharacteristic.addEventListener(
        "characteristicvaluechanged",
        handler
      );
      return () =>
        readCharacteristic.removeEventListener(
          "characteristicvaluechanged",
          handler
        );
    });
  }

  public disconnect() {
    this.readCharacteristic.removeEventListener(
      "characteristicvaluechanged",
      null
    );
    this.gatt.disconnect();
  }

  public get name() {
    return this.gatt.device.name;
  }

  public async write(value: BufferSource) {
    await this.writeCharacteristic.writeValueWithoutResponse(value);
  }
}

export async function selectDeviceConnection(): Promise<SoundcoreDeviceConnection> {
  const serviceUuid = SoundcoreDeviceUtils.getServiceUuid();
  const device = await navigator.bluetooth.requestDevice({
    filters: [{ services: [serviceUuid] }],
  });
  if (device.gatt == undefined) {
    throw new Error("Bluetooth device does not support GATT");
  }
  const gatt = await device.gatt.connect();
  const service = await gatt.getPrimaryService(
    SoundcoreDeviceUtils.getServiceUuid()
  );
  const [writeCharacteristic, readCharacteristic] = await Promise.all([
    service.getCharacteristic(
      SoundcoreDeviceUtils.getWriteCharacteristicUuid()
    ),
    service.getCharacteristic(SoundcoreDeviceUtils.getReadCharacteristicUuid()),
  ]);
  await readCharacteristic.startNotifications();

  return new SoundcoreDeviceConnection(
    gatt,
    writeCharacteristic,
    readCharacteristic
  );
}
