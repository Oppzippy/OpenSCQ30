use std::thread;

use async_trait::async_trait;
use futures::channel::oneshot;
use macaddr::MacAddr6;
use tokio::sync::mpsc;
use windows::{
    Devices::Bluetooth::{
        BluetoothLEDevice,
        GenericAttributeProfile::{
            GattCharacteristic,
            GattClientCharacteristicConfigurationDescriptorValue, GattValueChangedEventArgs,
            GattWriteOption,
        },
    },
    Foundation::TypedEventHandler,
    Storage::Streams::{DataReader, DataWriter},
};

use crate::{api::connection::Connection, device_utils};

#[derive(Debug)]
pub struct WindowsConnection {
    device: BluetoothLEDevice,
    write_characteristic: GattCharacteristic,
    read_characteristic: GattCharacteristic,
}

impl WindowsConnection {
    pub async fn new(address: u64) -> crate::Result<Option<Self>> {
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            let result = (|| {
                let device = BluetoothLEDevice::FromBluetoothAddressAsync(address)?.get()?;
                let services = device.GetGattServicesAsync()?.get()?.Services()?;
                let service = services.into_iter().find(|service| match service.Uuid() {
                    Ok(uuid) => uuid.to_u128() == device_utils::SERVICE_UUID.as_u128(),
                    Err(err) => {
                        tracing::warn!("error getting uuid: {err}");
                        false
                    }
                });
                let Some(service) = service else {
                    return Ok(None)
                };
                let characteristics = service
                    .GetCharacteristicsAsync()?
                    .get()?
                    .Characteristics()?;
                let read_characteristic = characteristics
                    .clone()
                    .into_iter()
                    .find(|characteristic| match characteristic.Uuid() {
                        Ok(uuid) => {
                            uuid.to_u128() == device_utils::READ_CHARACTERISTIC_UUID.as_u128()
                        }
                        Err(err) => {
                            tracing::warn!("error getting uuid: {err}");
                            false
                        }
                    })
                    .unwrap();
                let write_characteristic = characteristics
                    .into_iter()
                    .find(|characteristic| match characteristic.Uuid() {
                        Ok(uuid) => {
                            uuid.to_u128() == device_utils::WRITE_CHARACTERISTIC_UUID.as_u128()
                        }
                        Err(err) => {
                            tracing::warn!("error getting uuid: {err}");
                            false
                        }
                    })
                    .unwrap();

                Ok(Some(Self {
                    device,
                    write_characteristic,
                    read_characteristic,
                }))
            })();
            sender.send(result).unwrap();
        });
        receiver.await.unwrap()
    }
}

#[async_trait]
impl Connection for WindowsConnection {
    async fn name(&self) -> crate::Result<String> {
        Ok(self.device.Name()?.to_string())
    }

    async fn mac_address(&self) -> crate::Result<String> {
        let addr = self.device.BluetoothAddress()?;
        let bytes: [u8; 8] = addr.to_be_bytes();
        let mac_address = MacAddr6::new(bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]);
        Ok(mac_address.to_string())
    }

    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        let writer = DataWriter::new().unwrap();
        writer.WriteBytes(data).unwrap();
        let buffer = writer.DetachBuffer().unwrap();

        self.write_characteristic
            .WriteValueWithOptionAsync(&buffer, GattWriteOption::WriteWithResponse)?
            .get()?;
        Ok(())
    }

    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        let writer = DataWriter::new().unwrap();
        writer.WriteBytes(data).unwrap();
        let buffer = writer.DetachBuffer().unwrap();

        self.write_characteristic
            .WriteValueWithOptionAsync(&buffer, GattWriteOption::WriteWithoutResponse)?
            .get()?;
        Ok(())
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        self.read_characteristic
            .WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Notify,
            )?
            .await?;

        let (sender, receiver) = mpsc::channel(50);
        self.read_characteristic
            .ValueChanged(&TypedEventHandler::new(
                move |_characteristic: &Option<GattCharacteristic>,
                      args: &Option<GattValueChangedEventArgs>| {
                    if let Some(args) = args {
                        let value = args.CharacteristicValue()?;
                        let reader = DataReader::FromBuffer(&value)?;
                        let mut buffer = vec![0 as u8; reader.UnconsumedBufferLength()? as usize];
                        reader.ReadBytes(&mut buffer)?;

                        sender.try_send(buffer).unwrap();
                    }

                    Ok(())
                },
            ))?;
        Ok(receiver)
    }
}
