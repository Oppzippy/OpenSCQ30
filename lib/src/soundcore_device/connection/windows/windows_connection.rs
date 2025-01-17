use std::{
    collections::HashMap,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

use macaddr::MacAddr6;
use tokio::{
    select,
    sync::{mpsc as tokio_mpsc, watch, Semaphore},
};
use tracing::{debug, debug_span, error, instrument, trace, warn};
use uuid::Uuid;
use windows::{
    core::{AgileReference, HSTRING},
    Devices::{
        Bluetooth::{
            BluetoothConnectionStatus, BluetoothDevice, BluetoothLEDevice,
            GenericAttributeProfile::{GattCharacteristic, GattDeviceService},
            Rfcomm::RfcommDeviceService,
        },
        Enumeration::{DeviceInformation, DeviceInformationKind},
    },
    Foundation::TypedEventHandler,
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
    le_service_uuid: Uuid,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_changed_token: i64,
    handle: std::sync::Mutex<Option<JoinHandle<()>>>,
}

impl WindowsConnection {
    #[instrument()]
    pub async fn new(mac_address: MacAddr6) -> crate::Result<Option<Self>> {
        trace!("getting bluetooth device");
        let device = Self::get_bluetooth_device_from_mac_address(mac_address).await?;

        trace!("getting bluetooth le service uuid");
        let le_service_uuid = match Self::get_le_service_uuid(mac_address).await {
            Ok(uuid) => uuid,
            Err(err) => {
                debug!("failed to get gatt service uuid, but that's okay since we're using rfcomm: {err:?}");
                Uuid::nil()
            }
        };

        trace!("getting rfcomm service");
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
            .ok_or(crate::Error::ServiceNotFound { source: None })?;

        trace!("making socket");
        let socket = StreamSocket::new()?;
        socket
            .ConnectWithProtectionLevelAsync(
                &service.ConnectionHostName()?,
                &service.ConnectionServiceName()?,
                SocketProtectionLevel::BluetoothEncryptionAllowNullAuthentication,
            )?
            .await?;

        trace!("starting connection status changed handler");
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
            le_service_uuid,
            connection_status_receiver: receiver,
            connection_status_changed_token,
            handle: std::sync::Mutex::new(None),
        }))
    }

    #[instrument]
    async fn get_bluetooth_device_from_mac_address(
        mac_address: MacAddr6,
    ) -> crate::Result<BluetoothDevice> {
        let connected_filter = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
            BluetoothConnectionStatus::Connected,
        )?;
        // we can't use GetDeviceSelectorFromBluetoothAddress because that will use System.Devices.Aep.Bluetooth.IssueInquiry
        // which causes a massive delay
        let mac_address_filter = format!(
            "System.DeviceInterface.Bluetooth.DeviceAddress:=\"{}\"",
            hex::encode(mac_address)
        );
        let filter: HSTRING = format!("{connected_filter} AND {mac_address_filter} AND System.Devices.Aep.IsPresent:=System.StructuredQueryType.Boolean#True").into();
        trace!("built filter {filter}");
        let device_information_collection =
            DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
                &filter,
                None,
                DeviceInformationKind::Device,
            )?
            .await?;
        trace!(
            "found {} matching devices",
            device_information_collection.Size()?
        );
        if device_information_collection.Size()? == 0 {
            return Err(crate::Error::DeviceNotFound { source: None });
        }
        let device_information = device_information_collection.First()?.Current()?;

        Ok(BluetoothDevice::FromIdAsync(&device_information.Id()?)?.await?)
    }

    #[instrument]
    async fn get_le_service_uuid(mac_address: MacAddr6) -> crate::Result<Uuid> {
        trace!("getting bluetooth le device");
        let le_device = Self::get_bluetooth_le_device_from_mac_address(mac_address).await?;

        trace!("getting bluetooth le service uuid");
        let guid = Self::service(&le_device)
            .await
            .and_then(|service| service.Uuid().map_err(Into::into))?;
        Ok(Uuid::from_u128(guid.to_u128()))
    }

    #[instrument]
    async fn get_bluetooth_le_device_from_mac_address(
        mac_address: MacAddr6,
    ) -> crate::Result<BluetoothLEDevice> {
        let operation = BluetoothLEDevice::FromBluetoothAddressAsync(mac_address.as_windows_u64())?;
        let timeout = Arc::new(Semaphore::new(0));
        {
            let timeout = timeout.to_owned();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(3));
                timeout.close();
            });
        }

        select! {
            _ = timeout.acquire() => Err(crate::Error::DeviceNotFound{source:None}),
            result = operation => result.map_err(Into::into),
        }
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
            Err(crate::Error::ServiceNotFound { source: None })
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
        self.le_service_uuid
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
            .field("service_uuid", &self.le_service_uuid)
            .field(
                "connection_status_changed_token",
                &self.connection_status_changed_token,
            )
            .finish()
    }
}
