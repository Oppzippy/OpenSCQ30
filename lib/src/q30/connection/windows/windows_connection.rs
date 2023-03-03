use std::{
    sync::{Arc, RwLock},
    thread,
};

use async_trait::async_trait;
use futures::channel::oneshot;
use macaddr::MacAddr6;
use tokio::sync::mpsc as tokio_mpsc;
use uuid::Uuid;
use windows::{
    Devices::Bluetooth::{
        BluetoothLEDevice,
        GenericAttributeProfile::{
            GattCharacteristic, GattClientCharacteristicConfigurationDescriptorValue,
            GattDeviceService, GattValueChangedEventArgs, GattWriteOption,
        },
    },
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Storage::Streams::{DataReader, DataWriter},
};

use crate::{api::connection::Connection, device_utils};

use super::WindowsMacAddress;

#[derive(Debug)]
pub struct WindowsConnection {
    device: BluetoothLEDevice,
    read_characteristic: GattCharacteristic,
    value_changed_token: Arc<RwLock<Option<EventRegistrationToken>>>,
    write_queue_sender: crossbeam::channel::Sender<WriteCommand>,
    cancel_write_queue_sender: crossbeam::channel::Sender<()>,
}

struct WriteCommand {
    data: Vec<u8>,
    write_option: GattWriteOption,
}

impl WindowsConnection {
    pub async fn new(address: u64) -> crate::Result<Option<Self>> {
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            let result = (|| {
                let device = BluetoothLEDevice::FromBluetoothAddressAsync(address)?
                    .get()
                    .map_err(|err| {
                        // If there is no error but the device is not found, an error with code 0 is returned
                        if windows::core::HRESULT::is_ok(err.code()) {
                            crate::Error::DeviceNotFound {
                                source: Box::new(err),
                            }
                        } else {
                            err.into()
                        }
                    })?;
                let service = Self::service(&device, &device_utils::SERVICE_UUID)?;
                let read_characteristic =
                    Self::characteristic(&service, &device_utils::READ_CHARACTERISTIC_UUID)?;
                let write_characteristic =
                    Self::characteristic(&service, &device_utils::WRITE_CHARACTERISTIC_UUID)?;

                let (command_sender, command_receiver) =
                    crossbeam::channel::unbounded::<WriteCommand>();
                let (cancel_sender, cancel_receiver) = crossbeam::channel::unbounded::<()>();
                Self::start_write_queue_thread(
                    write_characteristic,
                    command_receiver,
                    cancel_receiver,
                );

                Ok(Some(Self {
                    device,
                    read_characteristic,
                    value_changed_token: Arc::new(RwLock::new(None)),
                    write_queue_sender: command_sender,
                    cancel_write_queue_sender: cancel_sender,
                }))
            })();
            sender.send(result).unwrap();
        });
        receiver.await.unwrap()
    }

    fn start_write_queue_thread(
        write_characteristic: GattCharacteristic,
        command_receiver: crossbeam::channel::Receiver<WriteCommand>,
        cancel_receiver: crossbeam::channel::Receiver<()>,
    ) {
        thread::spawn(move || loop {
            crossbeam::channel::select! {
                recv(command_receiver) -> write_command => {
                    match write_command {
                        Ok(write_command) => {
                            if let Err(err) =  Self::write_to_characteristic(&write_characteristic, &write_command) {
                                tracing::error!("failed to write to characteristic: {err}");
                            }
                        },
                        Err(err) => tracing::error!("failed to receive from command_receiver on write queue thread: {err}"),
                    };
                }
                recv(cancel_receiver) -> _ => {
                    tracing::debug!("stopping write queue thread");
                    return;
                }
            }
        });
    }

    fn write_to_characteristic(
        characteristic: &GattCharacteristic,
        write_command: &WriteCommand,
    ) -> crate::Result<()> {
        let writer = DataWriter::new()?;
        writer.WriteBytes(&write_command.data)?;
        let buffer = writer.DetachBuffer()?;

        characteristic
            .WriteValueWithOptionAsync(&buffer, write_command.write_option)?
            .get()?;
        Ok(())
    }

    fn characteristic(
        service: &GattDeviceService,
        characteristic_uuid: &Uuid,
    ) -> crate::Result<GattCharacteristic> {
        let characteristics = service
            .GetCharacteristicsAsync()?
            .get()?
            .Characteristics()?;

        let characteristic_uuid_u128 = characteristic_uuid.as_u128();
        characteristics
            .clone()
            .into_iter()
            .find(|characteristic| match characteristic.Uuid() {
                Ok(uuid) => uuid.to_u128() == characteristic_uuid_u128,
                Err(err) => {
                    tracing::warn!("error getting uuid: {err}");
                    false
                }
            })
            .ok_or_else(|| crate::Error::CharacteristicNotFound {
                uuid: characteristic_uuid.to_owned(),
                source: None,
            })
    }

    fn service(
        device: &BluetoothLEDevice,
        service_uuid: &Uuid,
    ) -> crate::Result<GattDeviceService> {
        let services = device.GetGattServicesAsync()?.get()?.Services()?;

        let service_uuid_u128 = service_uuid.as_u128();
        let service = services.into_iter().find(|service| match service.Uuid() {
            Ok(uuid) => uuid.to_u128() == service_uuid_u128,
            Err(err) => {
                tracing::warn!("error getting uuid: {err}");
                false
            }
        });

        if let Some(service) = service {
            Ok(service)
        } else {
            Err(crate::Error::ServiceNotFound {
                uuid: device_utils::SERVICE_UUID,
                source: None,
            })
        }
    }
}

#[async_trait]
impl Connection for WindowsConnection {
    async fn name(&self) -> crate::Result<String> {
        Ok(self.device.Name()?.to_string())
    }

    async fn mac_address(&self) -> crate::Result<String> {
        let mac_address = MacAddr6::from_windows_u64(self.device.BluetoothAddress()?);
        Ok(mac_address.to_string())
    }

    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_queue_sender
            .send(WriteCommand {
                data: data.into(),
                write_option: GattWriteOption::WriteWithResponse,
            })
            .map_err(|err| crate::Error::Other {
                source: Box::new(err),
            })
    }

    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_queue_sender
            .send(WriteCommand {
                data: data.into(),
                write_option: GattWriteOption::WriteWithoutResponse,
            })
            .map_err(|err| crate::Error::Other {
                source: Box::new(err),
            })
    }

    async fn inbound_packets_channel(&self) -> crate::Result<tokio_mpsc::Receiver<Vec<u8>>> {
        self.read_characteristic
            .WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Notify,
            )?
            .await?;

        let (sender, receiver) = tokio_mpsc::channel(50);
        let value_changed_token = self.value_changed_token.to_owned();
        let token = self
            .read_characteristic
            .ValueChanged(&TypedEventHandler::new(
                move |characteristic: &Option<GattCharacteristic>,
                      args: &Option<GattValueChangedEventArgs>| {
                    if let Some(characteristic) = characteristic {
                        if let Some(args) = args {
                            let value = args.CharacteristicValue()?;
                            let reader = DataReader::FromBuffer(&value)?;
                            let mut buffer =
                                vec![0 as u8; reader.UnconsumedBufferLength()? as usize];
                            reader.ReadBytes(&mut buffer)?;

                            match sender.try_send(buffer) {
                                Ok(()) => {}
                                Err(tokio_mpsc::error::TrySendError::Closed(_)) => {
                                    if let Some(token) = *value_changed_token.read().unwrap() {
                                        characteristic.RemoveValueChanged(token)?;
                                    }
                                }
                                Err(_err) => {}
                            }
                        }
                    }

                    Ok(())
                },
            ))?;
        let mut token_lock = self.value_changed_token.write().unwrap();
        if let Some(token) = *token_lock {
            self.read_characteristic.RemoveValueChanged(token)?;
        }
        *token_lock = Some(token);
        Ok(receiver)
    }
}

impl Drop for WindowsConnection {
    fn drop(&mut self) {
        if let Err(err) = self.cancel_write_queue_sender.send(()) {
            tracing::error!("failed to cancel write queue thread: {err}");
        }
        if let Some(token) = *self.value_changed_token.read().unwrap() {
            if let Err(err) = self.read_characteristic.RemoveValueChanged(token) {
                tracing::error!("failed to remove ValueChanged event handler: {err}");
            }
        }
    }
}
