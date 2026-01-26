use std::{
    collections::{HashMap, HashSet},
    panic::Location,
    sync::Mutex,
    thread,
};

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use tracing::{debug, debug_span, error, instrument, trace, warn};
use windows::{
    Devices::{
        Bluetooth::{
            BluetoothConnectionStatus, BluetoothDevice,
            Rfcomm::{RfcommDeviceService, RfcommServiceId},
        },
        Enumeration::DeviceInformation,
    },
    Foundation::TypedEventHandler,
    Networking::Sockets::{SocketProtectionLevel, StreamSocket},
    Storage::Streams::{Buffer, DataReader, DataWriter, InputStreamOptions},
    core::{AgileReference, HSTRING},
};

use crate::{
    api::connection::{self, RfcommBackend, RfcommConnection},
    connection::RfcommServiceSelectionStrategy,
    connection_backend::windows::utils::UuidAsGuidExt,
};

use super::utils::{GuidAsUuidExt, WindowsMacAddressExt};

#[derive(Default)]
pub struct WindowsRfcommBackend {}

impl RfcommBackend for WindowsRfcommBackend {
    type ConnectionType = WindowsRfcommConnection;

    async fn devices(&self) -> connection::Result<HashSet<connection::ConnectionDescriptor>> {
        tokio::task::spawn_blocking(|| {
            let devices = DeviceInformation::FindAllAsyncAqsFilter(
                &format!(
                    "{} AND System.Devices.Aep.IsPresent:=System.StructuredQueryType.Boolean#True",
                    BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
                        BluetoothConnectionStatus::Connected,
                    )?
                )
                .into(),
            )?
            .join()?;
            let mut descriptors = HashSet::with_capacity(devices.Size()? as usize);
            for device in devices {
                let id = device.Id()?;
                let bluetooth_device = BluetoothDevice::FromIdAsync(&id)?.join()?;
                let name = bluetooth_device.Name()?.to_string_lossy();
                let mac_address = MacAddr6::from_windows_u64(bluetooth_device.BluetoothAddress()?);
                descriptors.insert(connection::ConnectionDescriptor { name, mac_address });
            }
            Ok(descriptors)
        })
        .await
        .unwrap()
    }

    async fn connect(
        &self,
        mac_address: MacAddr6,
        service_selection_strategy: RfcommServiceSelectionStrategy,
    ) -> connection::Result<Self::ConnectionType> {
        tokio::task::spawn_blocking(move || {
            let span = debug_span!(
                "RfcommBackend::connect",
                mac_address = tracing::field::display(mac_address),
            );
            let _span_guard = span.enter();

            debug!("finding device with desired mac address");
            let device = Self::get_bluetooth_device_from_mac_address(mac_address)?;

            debug!("selecting RFCOMM service");
            let service = Self::select_service(&device, &service_selection_strategy)?;

            debug!("creating socket");
            let socket = AgileReference::new(&StreamSocket::new()?)?;
            socket
                .resolve()?
                .ConnectWithProtectionLevelAsync(
                    &service.ConnectionHostName()?,
                    &service.ConnectionServiceName()?,
                    SocketProtectionLevel::BluetoothEncryptionAllowNullAuthentication,
                )?
                .join()?;

            WindowsRfcommConnection::new(AgileReference::new(&device)?, socket)
        })
        .await
        .unwrap()
    }
}

impl WindowsRfcommBackend {
    #[instrument]
    fn get_bluetooth_device_from_mac_address(
        mac_address: MacAddr6,
    ) -> connection::Result<BluetoothDevice> {
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
        debug!("built filter: {filter}");
        let device_information_collection =
            DeviceInformation::FindAllAsyncAqsFilter(&filter)?.join()?;
        debug!(
            "found {} matching devices",
            device_information_collection.Size()?
        );
        if device_information_collection.Size()? == 0 {
            return Err(connection::Error::DeviceNotFound {
                source: None,
                location: Location::caller(),
            });
        }
        let device_information = device_information_collection.First()?.Current()?;

        Ok(BluetoothDevice::FromIdAsync(&device_information.Id()?)?.join()?)
    }

    fn select_service(
        device: &BluetoothDevice,
        service_selection_strategy: &RfcommServiceSelectionStrategy,
    ) -> connection::Result<RfcommDeviceService> {
        match service_selection_strategy {
            RfcommServiceSelectionStrategy::Constant(uuid) => {
                let service_id = RfcommServiceId::FromUuid(uuid.as_guid())?;
                let services = device
                    .GetRfcommServicesForIdAsync(&service_id)?
                    .join()?
                    .Services()?;
                let mut services_by_uuid = services
                    .clone()
                    .into_iter()
                    .map(|service| Ok((service.ServiceId()?.Uuid()?.as_uuid(), service)))
                    .collect::<windows::core::Result<HashMap<_, _>>>()?;
                debug!(
                    "looking for specific RFCOMM service {uuid}, found: {:?}",
                    services_by_uuid.keys()
                );
                services_by_uuid
                    .remove(&uuid)
                    .ok_or(connection::Error::DeviceNotFound {
                        source: None,
                        location: Location::caller(),
                    })
            }
            RfcommServiceSelectionStrategy::Dynamic(select_service) => {
                let services = device.GetRfcommServicesAsync()?.join()?.Services()?;
                let mut services_by_uuid = services
                    .clone()
                    .into_iter()
                    .map(|service| Ok((service.ServiceId()?.Uuid()?.as_uuid(), service)))
                    .collect::<windows::core::Result<HashMap<_, _>>>()?;
                debug!("found RFCOMM services: {:?}", services_by_uuid.keys());
                let uuid = select_service(services_by_uuid.keys().copied().collect());
                debug!("using RFCOMM service: {uuid:?}");
                services_by_uuid
                    .remove(&uuid)
                    .ok_or(connection::Error::DeviceNotFound {
                        source: None,
                        location: Location::caller(),
                    })
            }
        }
    }
}

pub struct WindowsRfcommConnection {
    device: AgileReference<BluetoothDevice>,
    socket: AgileReference<StreamSocket>,
    read_channel: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    connection_status_receiver: watch::Receiver<connection::ConnectionStatus>,
    connection_status_changed_token: i64,
}

impl WindowsRfcommConnection {
    pub fn new(
        device: AgileReference<BluetoothDevice>,
        socket: AgileReference<StreamSocket>,
    ) -> connection::Result<Self> {
        let read_channel = Self::spawn_read_channel(&socket.resolve()?)?;

        let (connection_status_sender, connection_status_receiver) =
            watch::channel(connection::ConnectionStatus::Connected);
        let connection_status_changed_token =
            device
                .resolve()?
                .ConnectionStatusChanged(&TypedEventHandler::new(
                    move |device: windows::core::Ref<'_, BluetoothDevice>, _| {
                        if let Some(device) = device.as_ref() {
                            let connection_status = if device.ConnectionStatus()?
                                == BluetoothConnectionStatus::Connected
                            {
                                connection::ConnectionStatus::Connected
                            } else {
                                connection::ConnectionStatus::Disconnected
                            };
                            connection_status_sender.send_replace(connection_status);
                        }
                        Ok(())
                    },
                ))?;
        Ok(Self {
            device,
            socket,
            read_channel: Mutex::new(Some(read_channel)),
            connection_status_receiver,
            connection_status_changed_token,
        })
    }

    fn spawn_read_channel(socket: &StreamSocket) -> connection::Result<mpsc::Receiver<Vec<u8>>> {
        let (sender, receiver) = mpsc::channel(100);
        let stream = AgileReference::new(&socket.InputStream()?)?;

        thread::spawn(|| {
            let span = debug_span!("WindowsRfcommConnection::read_channel");
            let _span_guard = span.enter();

            let result = (move || -> windows::core::Result<()> {
                let buffer = Buffer::Create(1000)?;
                let stream = stream.resolve()?;
                loop {
                    trace!("waiting for inbound packet");
                    stream
                        .ReadAsync(&buffer, 1000, InputStreamOptions::Partial)?
                        .join()?;

                    let mut packet = vec![0; buffer.Length()? as usize];
                    let reader = DataReader::FromBuffer(&buffer)?;
                    reader.ReadBytes(&mut packet)?;
                    trace!("received packet: {packet:?}");
                    if !packet.is_empty()
                        && let Err(err) = sender.blocking_send(packet)
                    {
                        debug!("packet failed to send: {err:?}");
                        debug!("receiver is closed, aborting");
                        break;
                    }
                }
                Ok(())
            })();
            match result {
                Ok(()) => debug!("aborted"),
                Err(err) => debug!("error receiving packet: {err:?}"),
            }
        });

        Ok(receiver)
    }
}

impl RfcommConnection for WindowsRfcommConnection {
    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        let socket = self.socket.clone();
        let data = data.to_owned();
        tokio::task::spawn_blocking(move || -> connection::Result<()> {
            let span = debug_span!("RfcommConnection::write");
            let _span_guard = span.enter();

            let writer = DataWriter::new()?;
            writer.WriteBytes(&data)?;
            let buffer = writer.DetachBuffer()?;

            let stream = socket.resolve()?.OutputStream()?;
            stream.WriteAsync(&buffer)?.join()?;
            trace!("wrote packet: {data:?}");
            Ok(())
        })
        .await
        .unwrap()
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.read_channel
            .lock()
            .unwrap()
            .take()
            .expect("read_channel may only be called once per WindowsRfcommConnection")
    }

    fn connection_status(&self) -> watch::Receiver<connection::ConnectionStatus> {
        self.connection_status_receiver.clone()
    }
}

impl Drop for WindowsRfcommConnection {
    #[tracing::instrument(level = "error", skip(self))]
    fn drop(&mut self) {
        if let Err(err) = self.socket.resolve().and_then(|it| it.Close()) {
            error!("failed to close socket: {err:?}");
        }
        if let Err(err) = self.device.resolve().and_then(|device| {
            device.RemoveConnectionStatusChanged(self.connection_status_changed_token)
        }) {
            error!("failed to remove ConnectionStatusChanged event handler: {err:?}");
        }
    }
}
