use std::{sync::Arc, time::Duration};

use bluer::{
    Device, DeviceEvent, DeviceProperty,
    gatt::remote::Service,
    rfcomm::{
        Stream,
        stream::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::Handle,
    select,
    sync::{
        Mutex, Semaphore,
        mpsc::{self, error::TrySendError},
        watch,
    },
    task::JoinHandle,
};
use tracing::{Instrument, debug, info_span, trace, trace_span, warn};
use uuid::Uuid;

use crate::{
    api::connection::{Connection, ConnectionStatus},
    device_utils::{self},
};

#[derive(Debug)]
pub struct BluerConnection {
    handle: Handle,
    device: Device,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
    read_stream: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    service_uuid: Uuid,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_handle: JoinHandle<()>,
    quit: Arc<Semaphore>,
}

impl BluerConnection {
    pub async fn new(device: Device, stream: Stream, handle: Handle) -> crate::Result<Self> {
        handle
            .clone()
            .spawn(
                async move {
                    let (connection_status_receiver, connection_status_handle) =
                        Self::spawn_connection_status(&handle, device.to_owned()).await?;
                    let (read_stream, write_stream) = stream.into_split();
                    let quit = Arc::new(Semaphore::new(0));
                    let service_uuid = Self::get_service_uuid_with_retry(&device).await?;

                    let connection = BluerConnection {
                        device,
                        service_uuid,
                        write_stream: Arc::new(Mutex::new(write_stream)),
                        read_stream: Mutex::new(Some(
                            Self::spawn_inbound_packet_channel(
                                read_stream,
                                handle.to_owned(),
                                quit.to_owned(),
                            )
                            .await?,
                        )),
                        connection_status_receiver,
                        connection_status_handle,
                        handle,
                        quit,
                    };
                    Ok(connection)
                }
                .instrument(info_span!("BluerConnection::new")),
            )
            .await
            .unwrap()
    }

    async fn get_service_uuid_with_retry(device: &Device) -> crate::Result<Uuid> {
        match Self::get_service_with_retry(device, Duration::from_secs(2)).await {
            Ok(service) => service.uuid().await.map_err(Into::into),
            Err(err) => {
                debug!("GATT service not found, but that's okay since we're using RFCOMM: {err:?}");
                Ok(Uuid::nil())
            }
        }
    }

    #[tracing::instrument]
    async fn get_service_with_retry(device: &Device, timeout: Duration) -> crate::Result<Service> {
        let service_found = device.events().await?.any(async |event| match event {
            DeviceEvent::PropertyChanged(DeviceProperty::Uuids(uuids)) => {
                uuids.iter().any(device_utils::is_soundcore_service_uuid)
            }
            _ => false,
        });
        match Self::get_service(device).await {
            Ok(service) => {
                debug!("found service on first try");
                return Ok(service);
            }
            Err(err) => {
                if let crate::Error::ServiceNotFound { .. } = err {
                    // keep going and retry later
                } else {
                    return Err(err);
                }
            }
        }
        debug!("service not found, waiting for event");
        select! {
            _ = service_found => debug!("got service found event"),
            _ = tokio::time::sleep(timeout) => debug!("no event, giving it one last try"),
        }
        Self::get_service(device).await
    }

    async fn get_service(device: &Device) -> crate::Result<Service> {
        for service in device.services().await? {
            if device_utils::is_soundcore_service_uuid(&service.uuid().await?) {
                return Ok(service);
            }
        }
        Err(crate::Error::ServiceNotFound { source: None })
    }

    async fn spawn_connection_status(
        handle: &Handle,
        device: Device,
    ) -> crate::Result<(watch::Receiver<ConnectionStatus>, JoinHandle<()>)> {
        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);

        let connection_status_handle = {
            let mut events = device.events().await?;
            handle.spawn(async move {
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
        handle: Handle,
        quit: Arc<Semaphore>,
    ) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);
        handle.spawn(
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

impl Connection for BluerConnection {
    async fn name(&self) -> crate::Result<String> {
        let device = self.device.to_owned();
        self.handle
            .spawn(async move {
                match device.name().await? {
                    Some(name) => Ok(name),
                    None => Err(crate::Error::NameNotFound {
                        mac_address: device.address().into(),
                        source: None,
                    }),
                }
            })
            .await
            .unwrap()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver.clone()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.device.address().0.into())
    }

    fn service_uuid(&self) -> Uuid {
        self.service_uuid
    }

    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_without_response(data).await
    }

    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_stream
            .lock()
            .await
            .write_all(data)
            .await
            .map_err(|err| crate::Error::WriteFailed {
                source: Box::new(err),
            })
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        Ok(self
            .read_stream
            .lock()
            .await
            .take()
            .expect("inbound_packets_channel should only be called once"))
    }
}

impl Drop for BluerConnection {
    fn drop(&mut self) {
        self.connection_status_handle.abort();
        self.quit.close();
    }
}
