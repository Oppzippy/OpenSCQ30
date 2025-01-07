use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use macaddr::MacAddr6;
use tokio::sync::{mpsc as tokio_mpsc, watch};
use tracing::{debug, debug_span, error, instrument, trace, warn};
use uuid::Uuid;
use windows::{
    core::AgileReference,
    Devices::Bluetooth::{
        BluetoothConnectionStatus, BluetoothDevice, BluetoothLEDevice,
        GenericAttributeProfile::{GattCharacteristic, GattDeviceService},
        Rfcomm::RfcommDeviceService,
    },
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Networking::Sockets::{SocketProtectionLevel, StreamSocket},
    Storage::Streams::{Buffer, DataReader, DataWriter, InputStreamOptions},
};

use crate::{
    api::connection::{Connection, ConnectionStatus},
    device_utils::{self, is_soundcore_service_uuid},
};

use super::WindowsMacAddress;

pub struct WindowsConnection {
    device: BluetoothDevice,
    socket: StreamSocket,
    service_uuid: Uuid,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_changed_token: EventRegistrationToken,
    handle: std::sync::Mutex<Option<JoinHandle<()>>>,
}

impl WindowsConnection {
    #[instrument()]
    pub async fn new(mac_address: MacAddr6) -> crate::Result<Option<Self>> {
        let service_uuid = match Self::get_ble_service_uuid(mac_address).await {
            Ok(service_uuid) => service_uuid,
            Err(err) => {
                debug!("failed to get gatt service uuid, but that's okay since we're using rfcomm: {err:?}");
                Uuid::nil()
            }
        };

        let device = BluetoothDevice::FromBluetoothAddressAsync(mac_address.as_windows_u64())?
            .await
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

        debug!("getting service");
        let services = device.GetRfcommServicesAsync()?.await?.Services()?;
        let services_by_uuid = services
            .into_iter()
            .map(|service| {
                let service_uuid = match service.ServiceId().map(|sid| sid.Uuid()) {
                    Ok(Ok(uuid)) => uuid,
                    Ok(Err(err)) => return Err(err),
                    Err(err) => return Err(err),
                };
                Ok((Uuid::from_u128(service_uuid.to_u128()), service))
            })
            .collect::<windows::core::Result<HashMap<Uuid, RfcommDeviceService>>>()?;
        let service = services_by_uuid
            .iter()
            .filter(|(uuid, _)| device_utils::is_soundcore_vendor_rfcomm_uuid(uuid))
            .map(|(_, service)| service)
            .next()
            .or(services_by_uuid.get(&device_utils::RFCOMM_UUID))
            .ok_or(crate::Error::ServiceNotFound {
                uuid: Uuid::nil(),
                source: None,
            })?;

        debug!("making socket");
        let socket = StreamSocket::new()?;
        socket
            .ConnectWithProtectionLevelAsync(
                &service.ConnectionHostName()?,
                &service.ConnectionServiceName()?,
                SocketProtectionLevel::BluetoothEncryptionAllowNullAuthentication,
            )?
            .await?;
        debug!("starting connection changed");
        let (sender, receiver) = watch::channel(ConnectionStatus::Connected);
        let connection_status_changed_token = device.ConnectionStatusChanged(
            &TypedEventHandler::new(move |device: &Option<BluetoothDevice>, _| {
                if let Some(device) = device {
                    let is_connected =
                        device.ConnectionStatus()? == BluetoothConnectionStatus::Connected;
                    sender.send_replace(if is_connected {
                        ConnectionStatus::Connected
                    } else {
                        ConnectionStatus::Disconnected
                    });
                }
                Ok(())
            }),
        )?;

        Ok(Some(Self {
            device,
            socket,
            service_uuid,
            connection_status_receiver: receiver,
            connection_status_changed_token,
            handle: std::sync::Mutex::new(None),
        }))
    }

    async fn get_ble_service_uuid(mac_address: MacAddr6) -> crate::Result<Uuid> {
        let ble_device =
            BluetoothLEDevice::FromBluetoothAddressAsync(mac_address.as_windows_u64())?
                .await
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
        let service = Self::service(&ble_device).await?;
        Ok(Uuid::from_u128(service.Uuid()?.to_u128()))
    }

    #[instrument(level = "trace", skip(service))]
    async fn characteristic(
        service: &GattDeviceService,
        characteristic_uuid: &Uuid,
    ) -> crate::Result<GattCharacteristic> {
        let characteristics = service
            .GetCharacteristicsAsync()?
            .await?
            .Characteristics()?;

        let characteristic_uuid_u128 = characteristic_uuid.as_u128();
        characteristics
            .into_iter()
            .find(|characteristic| match characteristic.Uuid() {
                Ok(uuid) => uuid.to_u128() == characteristic_uuid_u128,
                Err(err) => {
                    tracing::warn!("error getting uuid: {err:?}");
                    false
                }
            })
            .ok_or_else(|| crate::Error::CharacteristicNotFound {
                uuid: characteristic_uuid.to_owned(),
                source: None,
            })
    }

    #[instrument(level = "trace", skip(device))]
    async fn service(device: &BluetoothLEDevice) -> crate::Result<GattDeviceService> {
        let services = device.GetGattServicesAsync()?.await?.Services()?;

        let service = services.into_iter().find(|service| match service.Uuid() {
            Ok(uuid) => is_soundcore_service_uuid(&Uuid::from_u128(uuid.to_u128())),
            Err(err) => {
                tracing::warn!("error getting uuid: {err:?}");
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

impl Connection for WindowsConnection {
    async fn name(&self) -> crate::Result<String> {
        Ok(self.device.Name()?.to_string())
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        let windows_u64_mac_address = self.device.BluetoothAddress()?;
        Ok(MacAddr6::from_windows_u64(windows_u64_mac_address))
    }

    fn service_uuid(&self) -> Uuid {
        self.service_uuid
    }

    #[instrument(level = "trace", skip(self))]
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver.clone()
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_without_response(data).await
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        let writer = DataWriter::new()?;
        writer.WriteBytes(data)?;
        let buffer = writer.DetachBuffer()?;

        let output = self.socket.OutputStream()?;
        output.WriteAsync(&buffer)?.await?;
        trace!("wrote packet: {data:?}");
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    async fn inbound_packets_channel(&self) -> crate::Result<tokio_mpsc::Receiver<Vec<u8>>> {
        let (sender, receiver) = tokio_mpsc::channel(50);
        let input = self.socket.InputStream()?;
        let input = AgileReference::new(&input)?;

        let mut handle = self.handle.lock().unwrap();
        if let Some(_handle) = handle.as_ref() {
            panic!("can't handle multiple inbound packet channels");
        }
        *handle = Some(thread::spawn(|| {
            let span = debug_span!("WindowsConnection::inbound_packets_channel");
            let _span_guard = span.enter();
            let result = (move || {
                let buffer = Buffer::Create(1000)?;
                let input = input.resolve()?;
                loop {
                    trace!("waiting for packets");
                    input
                        .ReadAsync(&buffer, 1000, InputStreamOptions::Partial)?
                        .get()?;
                    let mut packet = vec![0; buffer.Length()? as usize];
                    let reader = DataReader::FromBuffer(&buffer)?;
                    reader.ReadBytes(&mut packet)?;
                    trace!("got packet: {packet:?}");
                    if !packet.is_empty() {
                        match sender.blocking_send(packet) {
                            Ok(_) => (),
                            Err(_err) => {
                                debug!("receiver is closed, aborting");
                                break;
                            }
                        }
                    }
                }
                Ok(()) as crate::Result<()>
            })();
            match result {
                Ok(_) => debug!("inbound_packets_channel: ended"),
                Err(err) => warn!("inbound_packets_channel: error receiving packet: {err:?}"),
            }
        }));

        Ok(receiver)
    }
}

impl Drop for WindowsConnection {
    #[instrument(level = "error", skip(self))]
    fn drop(&mut self) {
        if let Err(err) = self.socket.Close() {
            error!("failed to close socket: {err:?}");
        }
        if let Err(err) = self
            .device
            .RemoveConnectionStatusChanged(self.connection_status_changed_token)
        {
            error!("failed to remove ConnectionStatusChanged event handler: {err:?}");
        }
    }
}

impl core::fmt::Debug for WindowsConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowsConnection")
            .field("device", &self.device)
            .field("socket", &self.socket)
            .field("service_uuid", &self.service_uuid)
            .field(
                "connection_status_changed_token",
                &self.connection_status_changed_token,
            )
            .finish()
    }
}
