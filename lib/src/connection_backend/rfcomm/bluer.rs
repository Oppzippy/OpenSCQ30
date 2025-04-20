use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, Instant},
};

use bluer::{
    Adapter, Address, Device, DeviceProperty, Session,
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
        Mutex, Semaphore,
        mpsc::{self, error::TrySendError},
        watch,
    },
    task::JoinHandle,
};
use tracing::{Instrument, debug, instrument, trace, trace_span, warn};
use uuid::Uuid;

use crate::api::connection::{
    self, ConnectionStatus, DeviceDescriptor, RfcommBackend, RfcommConnection,
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

    /// Filters out devices that are not connected and returns descriptors
    async fn address_to_descriptor(
        adapter: &Adapter,
        address: Address,
    ) -> connection::Result<Option<DeviceDescriptor>> {
        let device = adapter.device(address)?;
        if device.is_connected().await? {
            Ok(Some(DeviceDescriptor {
                name: device.name().await?.unwrap_or_default(),
                mac_address: address.into(),
            }))
        } else {
            Ok(None)
        }
    }
}

impl RfcommBackend for BluerRfcommBackend {
    type ConnectionType = BluerRfcommConnection;

    #[instrument(skip(self))]
    async fn devices(&self) -> connection::Result<HashSet<DeviceDescriptor>> {
        let adapter = self.session.default_adapter().await.map_err(|err| {
            if err.kind == bluer::ErrorKind::NotFound {
                connection::Error::BluetoothAdapterUnavailable {
                    source: Some(Box::new(err)),
                }
            } else {
                err.into()
            }
        })?;

        let device_addresses = adapter.device_addresses().await?;
        let mut descriptors = HashSet::new();
        for address in device_addresses {
            if let Some(descriptor) = Self::address_to_descriptor(&adapter, address).await? {
                descriptors.insert(descriptor);
            }
        }
        tracing::debug!("filtered down to {} descriptors", descriptors.len());
        Ok(descriptors)
    }

    #[instrument(skip(self, select_uuid))]
    async fn connect(
        &self,
        mac_address: MacAddr6,
        select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send,
    ) -> connection::Result<Self::ConnectionType> {
        let adapter = self.session.default_adapter().await.map_err(|err| {
            if err.kind == bluer::ErrorKind::NotFound {
                connection::Error::BluetoothAdapterUnavailable {
                    source: Some(Box::new(err)),
                }
            } else {
                err.into()
            }
        })?;

        let device = match adapter.device(mac_address.into_array().into()) {
            Ok(device) => device,
            Err(err) => {
                return match err.kind {
                    bluer::ErrorKind::NotFound => Err(connection::Error::DeviceNotFound {
                        source: Some(Box::new(err)),
                    }),
                    _ => Err(connection::Error::from(err)),
                };
            }
        };
        device.connect().await?;
        let uuids = device.uuids().await?.unwrap();
        let uuid = select_uuid(uuids);
        debug!("using uuid {uuid} for RFCOMM");

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
                        warn!("connect profile failed: {err:?}")
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
                _ = tokio::time::sleep_until(timeout.into()) => {
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
    read_stream: std::sync::Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_handle: JoinHandle<()>,
    quit: Arc<Semaphore>,
}

impl BluerRfcommConnection {
    #[instrument(skip(stream))]
    pub async fn new(device: Device, stream: Stream) -> connection::Result<Self> {
        let (connection_status_receiver, connection_status_handle) =
            Self::spawn_connection_status(device.to_owned()).await?;
        let (read_stream, write_stream) = stream.into_split();
        let quit = Arc::new(Semaphore::new(0));

        let connection = BluerRfcommConnection {
            write_stream: Arc::new(Mutex::new(write_stream)),
            read_stream: std::sync::Mutex::new(Some(
                Self::spawn_inbound_packet_channel(read_stream, quit.to_owned()).await?,
            )),
            connection_status_receiver,
            connection_status_handle,
            quit,
        };
        Ok(connection)
    }

    async fn spawn_connection_status(
        device: Device,
    ) -> connection::Result<(watch::Receiver<ConnectionStatus>, JoinHandle<()>)> {
        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);

        let connection_status_handle = {
            let mut events = device.events().await?;
            tokio::spawn(async move {
                loop {
                    if let Some(bluer::DeviceEvent::PropertyChanged(DeviceProperty::Connected(
                        is_connected,
                    ))) = events.next().await
                    {
                        connection_status_sender.send_replace(match is_connected {
                            true => ConnectionStatus::Connected,
                            false => ConnectionStatus::Disconnected,
                        });
                    }
                }
            })
        };

        Ok((connection_status_receiver, connection_status_handle))
    }

    async fn spawn_inbound_packet_channel(
        mut read_stream: OwnedReadHalf,
        quit: Arc<Semaphore>,
    ) -> connection::Result<mpsc::Receiver<Vec<u8>>> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);
        tokio::spawn(
            async move {
                let mut buffer: Vec<u8> = vec![0; 1024];
                loop {
                    select! {
                        _ = quit.acquire() => {
                            break;
                        }
                        // Does this read one packet at a time?
                        result = read_stream.read(&mut buffer) => {
                            match result {
                                Ok(bytes_read) => {
                                    let bytes = &buffer[0..bytes_read];
                                    trace!(event = "rfcomm read", ?bytes);
                                    if bytes_read > 0 {
                                        if let Err(err) = sender.try_send(bytes.to_vec()) {
                                            if let TrySendError::Closed(_) = err {
                                                break;
                                            }
                                            warn!("error forwarding packet to channel: {err}",)
                                        }
                                    }
                                }
                                Err(err) => {
                                    debug!("read failed: {err:?}");
                                    break;
                                },
                            }
                        },
                    }
                }
            }
            .instrument(trace_span!(
                "bluer_connection inbound_packets_channel reader"
            )),
        );

        Ok(receiver)
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
            })
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.read_stream
            .lock()
            .unwrap()
            .take()
            .expect("inbound_packets_channel should only be called once")
    }
}

impl Drop for BluerRfcommConnection {
    fn drop(&mut self) {
        self.connection_status_handle.abort();
        self.quit.close();
    }
}

impl From<bluer::Error> for connection::Error {
    fn from(error: bluer::Error) -> Self {
        connection::Error::Other(Box::new(error))
    }
}
