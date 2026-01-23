use std::{
    collections::HashSet,
    panic::Location,
    sync::Arc,
    time::{Duration, Instant},
};

use bluer::{
    Adapter, Device, DeviceProperty, Session,
    rfcomm::{
        Profile, ReqError, Stream,
        stream::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    select,
    sync::{
        Mutex,
        mpsc::{self},
        watch,
    },
};
use tracing::{Instrument, debug, debug_span, instrument, trace, trace_span, warn};
use uuid::Uuid;

use crate::{
    api::connection::{
        self, ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection,
    },
    util::AbortOnDropHandle,
};

pub struct BluerRfcommBackend {
    session: Session,
}

impl BluerRfcommBackend {
    pub async fn new() -> connection::Result<Self> {
        Ok(Self {
            session: Session::new().await?,
        })
    }

    async fn adapters(&self) -> connection::Result<Vec<Adapter>> {
        let adapters = self
            .session
            .adapter_names()
            .await?
            .into_iter()
            .map(|adapter_name| {
                self.session.adapter(&adapter_name).map_err(|err| {
                    connection::Error::BluetoothAdapterUnavailable {
                        source: Some(Box::new(err)),
                        location: Location::caller(),
                    }
                })
            })
            .collect::<Result<Vec<Adapter>, connection::Error>>()?;

        if adapters.is_empty() {
            tracing::warn!("No bluetooth adapters found");
            return Err(connection::Error::BluetoothAdapterUnavailable {
                source: None,
                location: Location::caller(),
            });
        }

        Ok(adapters)
    }

    async fn device(&self, mac_address: MacAddr6) -> connection::Result<Device> {
        for adapter in self.adapters().await? {
            match Self::device_from_adapter(&adapter, mac_address).await {
                Ok(Some(device)) => return Ok(device),
                Ok(None) => (),
                // Keep going on error in case another adapter has the device
                Err(err) => tracing::error!(
                    "failed to find device {mac_address} on bluetoooth adapter {adapter:?}: {err:?}"
                ),
            }
        }
        Err(connection::Error::DeviceNotFound {
            source: None,
            location: Location::caller(),
        })
    }

    async fn device_from_adapter(
        adapter: &Adapter,
        mac_address: MacAddr6,
    ) -> bluer::Result<Option<Device>> {
        match adapter.device(mac_address.into()) {
            Ok(device) => device
                .is_connected()
                .await
                .map(|is_connected| is_connected.then_some(device)),
            Err(err) => Err(err),
        }
    }

    async fn add_devices_from_adapter(
        adapter: &Adapter,
        connection_descriptors: &mut HashSet<ConnectionDescriptor>,
    ) -> connection::Result<()> {
        let device_addresses = adapter.device_addresses().await?;
        for address in device_addresses {
            let device = adapter.device(address)?;
            if device.is_connected().await? {
                connection_descriptors.insert(ConnectionDescriptor {
                    name: device.name().await?.unwrap_or_default(),
                    mac_address: address.into(),
                });
            }
        }
        Ok(())
    }
}

impl RfcommBackend for BluerRfcommBackend {
    type ConnectionType = BluerRfcommConnection;

    #[instrument(skip(self))]
    async fn devices(&self) -> connection::Result<HashSet<ConnectionDescriptor>> {
        let mut connection_descriptors = HashSet::new();
        for adapter in self.adapters().await? {
            Self::add_devices_from_adapter(&adapter, &mut connection_descriptors).await?;
        }

        Ok(connection_descriptors)
    }

    #[instrument(skip(self, select_uuid))]
    async fn connect(
        &self,
        mac_address: MacAddr6,
        select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send,
    ) -> connection::Result<Self::ConnectionType> {
        let device = self.device(mac_address).await?;
        device.connect().await?;
        let uuids = device.uuids().await?.unwrap();
        debug!("found RFCOMM services: {uuids:?}");
        let uuid = select_uuid(uuids);
        debug!("using RFCOMM service {uuid}");

        let mut rfcomm_handle = self
            .session
            .register_profile(Profile {
                uuid,
                ..Default::default()
            })
            .await?;

        let timeout = Instant::now().checked_add(Duration::from_secs(5)).unwrap();
        debug!("connecting");
        let stream = loop {
            select! {
                res = async {
                    trace!("connect_profile");
                    device.connect_profile(&uuid).await
                } => {
                    if let Err(err)=res{
                        warn!("connect profile failed: {err:?}");
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                req = rfcomm_handle.next() => {
                    let req = req.unwrap();
                    if req.device() == device.address() {
                        debug!("accepting request from {}", req.device());
                        break req.accept()?;
                    } else {
                        debug!("rejecting request from {}", req.device());
                        req.reject(ReqError::Rejected);
                    }
                }
                () = tokio::time::sleep_until(timeout.into()) => {
                    return Err(connection::Error::TimedOut { action: "connect" })
                }
            }
        };
        debug!("connected");
        let connection = BluerRfcommConnection::new(device, stream).await?;
        Ok(connection)
    }
}

#[derive(Debug)]
pub struct BluerRfcommConnection {
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
    inbound_packet_stream: std::sync::Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    _inbound_packet_handle: AbortOnDropHandle<()>,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    _connection_status_handle: AbortOnDropHandle<()>,
    // We need to not drop device in order for the device.events() stream in spawn_connection_status to not terminate
    _device: Device,
}

impl BluerRfcommConnection {
    #[instrument(skip(stream))]
    pub async fn new(device: Device, stream: Stream) -> connection::Result<Self> {
        // AbortOnDropHandle used for all join handles to ensure cancel safety
        let (connection_status_receiver, connection_status_handle) =
            Self::spawn_connection_status(device.to_owned()).await?;
        let (read_stream, write_stream) = stream.into_split();
        let (inbound_packet_stream, inbound_packet_handle) =
            Self::spawn_inbound_packet_channel(read_stream).await;

        let connection = Self {
            write_stream: Arc::new(Mutex::new(write_stream)),
            inbound_packet_stream: std::sync::Mutex::new(Some(inbound_packet_stream)),
            _inbound_packet_handle: inbound_packet_handle,
            connection_status_receiver,
            _connection_status_handle: connection_status_handle,
            _device: device,
        };
        Ok(connection)
    }

    async fn spawn_connection_status(
        device: Device,
    ) -> connection::Result<(watch::Receiver<ConnectionStatus>, AbortOnDropHandle<()>)> {
        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);

        let mut events = device.events().await?;
        let connection_status_handle = AbortOnDropHandle::new(tokio::spawn(
            async move {
                while let Some(event) = events.next().await {
                    tracing::debug!("got event {event:?}");
                    if let bluer::DeviceEvent::PropertyChanged(DeviceProperty::Connected(
                        is_connected,
                    )) = event
                    {
                        connection_status_sender.send_replace(match is_connected {
                            true => ConnectionStatus::Connected,
                            false => ConnectionStatus::Disconnected,
                        });
                    }
                }
                tracing::debug!("event stream ended");
            }
            .instrument(debug_span!("spawn_connection_status")),
        ));

        Ok((connection_status_receiver, connection_status_handle))
    }

    async fn spawn_inbound_packet_channel(
        mut read_stream: OwnedReadHalf,
    ) -> (mpsc::Receiver<Vec<u8>>, AbortOnDropHandle<()>) {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);
        let abort_handle = AbortOnDropHandle::new(tokio::spawn(
            async move {
                let mut buffer: Vec<u8> = vec![0; 1024];
                loop {
                    let Ok(permit) = sender.reserve().await else {
                        // sender closed
                        break;
                    };
                    match read_stream.read(&mut buffer).await {
                        Ok(bytes_read) => {
                            let bytes = &buffer[0..bytes_read];
                            trace!(event = "rfcomm read", ?bytes);
                            if bytes_read > 0 {
                                permit.send(bytes.to_vec());
                            }
                        }
                        Err(err) => {
                            debug!("read failed: {err:?}");
                            break;
                        }
                    }
                }
            }
            .instrument(trace_span!(
                "bluer_connection inbound_packets_channel reader"
            )),
        ));

        (receiver, abort_handle)
    }
}

impl RfcommConnection for BluerRfcommConnection {
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver.clone()
    }

    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        self.write_stream
            .lock()
            .await
            .write_all(data)
            .await
            .map_err(|err| connection::Error::WriteError {
                source: Some(Box::new(err)),
                location: Location::caller(),
            })
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.inbound_packet_stream
            .lock()
            .unwrap()
            .take()
            .expect("inbound_packets_channel should only be called once")
    }
}

impl From<bluer::Error> for connection::Error {
    #[track_caller]
    fn from(error: bluer::Error) -> Self {
        Self::Other {
            source: Box::new(error),
            location: Location::caller(),
        }
    }
}
