use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::mpsc as tokio_mpsc;
use tracing::instrument;
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
    write_characteristic: GattCharacteristic,
    value_changed_token: Arc<RwLock<Option<EventRegistrationToken>>>,
}

impl WindowsConnection {
    #[instrument()]
    pub async fn new(address: u64) -> crate::Result<Option<Self>> {
        tokio::task::spawn_blocking(move || {
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

            Ok(Some(Self {
                device,
                read_characteristic,
                write_characteristic,
                value_changed_token: Arc::new(RwLock::new(None)),
            }))
        })
        .await
        .map_err(|err| crate::Error::Other {
            source: Box::new(err),
        })?
    }

    fn write_to_characteristic(
        characteristic: &GattCharacteristic,
        data: &[u8],
        write_option: GattWriteOption,
    ) -> crate::Result<()> {
        let writer = DataWriter::new()?;
        writer.WriteBytes(data)?;
        let buffer = writer.DetachBuffer()?;

        characteristic
            .WriteValueWithOptionAsync(&buffer, write_option)?
            .get()?;
        Ok(())
    }

    #[instrument(level = "trace", skip(service))]
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

    #[instrument(level = "trace", skip(device))]
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

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        let characteristic = self.write_characteristic.to_owned();
        let data = data.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::write_to_characteristic(
                &characteristic,
                &data,
                GattWriteOption::WriteWithResponse,
            )
        })
        .await
        .map_err(|err| crate::Error::Other {
            source: Box::new(err),
        })?
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        let characteristic = self.write_characteristic.to_owned();
        let data = data.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::write_to_characteristic(
                &characteristic,
                &data,
                GattWriteOption::WriteWithResponse,
            )
        })
        .await
        .map_err(|err| crate::Error::Other {
            source: Box::new(err),
        })?
    }

    #[instrument(level = "trace", skip(self))]
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
                    let span = tracing::trace_span!("WindowsConnection ValueChanged");
                    let _enter = span.enter();
                    if let Some(characteristic) = characteristic {
                        if let Some(args) = args {
                            let value = args.CharacteristicValue()?;
                            let reader = DataReader::FromBuffer(&value)?;
                            let mut buffer = vec![0_u8; reader.UnconsumedBufferLength()? as usize];
                            reader.ReadBytes(&mut buffer)?;

                            match sender.try_send(buffer) {
                                Ok(()) => {}
                                Err(tokio_mpsc::error::TrySendError::Closed(_)) => {
                                    let lock = match value_changed_token.read() {
                                        Ok(lock) => lock,
                                        Err(err) => {
                                            tracing::warn!("lock is poisoned: {err:?}");
                                            err.into_inner()
                                        }
                                    };
                                    if let Some(token) = *lock {
                                        characteristic.RemoveValueChanged(token)?;
                                    }
                                }
                                Err(err) => {
                                    tracing::error!("error sending: {err:?}")
                                }
                            }
                        }
                    }

                    Ok(())
                },
            ))?;
        let mut token_lock = match self.value_changed_token.write() {
            Ok(lock) => lock,
            Err(err) => {
                tracing::warn!("lock is poisoned: {err:?}");
                err.into_inner()
            }
        };
        if let Some(token) = *token_lock {
            self.read_characteristic.RemoveValueChanged(token)?;
        }
        *token_lock = Some(token);
        Ok(receiver)
    }
}

impl Drop for WindowsConnection {
    #[instrument(level = "trace", skip(self))]
    fn drop(&mut self) {
        let lock = match self.value_changed_token.read() {
            Ok(lock) => lock,
            Err(err) => {
                tracing::warn!("lock is poisoned: {err:?}");
                err.into_inner()
            }
        };
        if let Some(token) = *lock {
            if let Err(err) = self.read_characteristic.RemoveValueChanged(token) {
                tracing::error!("failed to remove ValueChanged event handler: {err}");
            }
        }
    }
}
