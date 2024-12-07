use std::sync::Arc;

use bluer::{gatt::remote::Characteristic, Device, DeviceProperty};
use futures::{pin_mut, StreamExt};
use macaddr::MacAddr6;
use tokio::{
    runtime::Handle,
    select,
    sync::{
        mpsc::{self, error::TrySendError},
        watch, Semaphore,
    },
    task::JoinHandle,
};
use tracing::{instrument, trace, trace_span, warn, Instrument};
use uuid::Uuid;

use crate::{
    api::connection::{Connection, ConnectionStatus},
    device_utils::{self, READ_CHARACTERISTIC_UUID, SERVICE_UUID, WRITE_CHARACTERISTIC_UUID},
};

#[derive(Debug)]
pub struct BluerConnection {
    handle: Handle,
    device: Device,
    write_characteristic: Characteristic,
    read_characteristic: Characteristic,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_handle: JoinHandle<()>,
    service_uuid: Uuid,
    quit: Arc<Semaphore>,
}

impl BluerConnection {
    pub async fn new(device: Device, handle: Handle) -> crate::Result<Self> {
        let handle2 = handle.clone();
        handle
            .spawn(async move {
                let handle = handle2;

                device.connect().await?;
                let mut service = None;
                for s in device.services().await? {
                    if device_utils::is_soundcore_service_uuid(&s.uuid().await?) {
                        service = Some(s);
                        break;
                    }
                }
                let service = service.ok_or(crate::Error::ServiceNotFound {
                    uuid: SERVICE_UUID,
                    source: None,
                })?;
                let service_uuid = service.uuid().await?;

                let mut write_characteristic = None;
                let mut read_characteristic = None;
                for c in service.characteristics().await? {
                    match c.uuid().await? {
                        WRITE_CHARACTERISTIC_UUID => write_characteristic = Some(c),
                        READ_CHARACTERISTIC_UUID => read_characteristic = Some(c),
                        _ => (),
                    }
                }

                let read_characteristic =
                    read_characteristic.ok_or(crate::Error::CharacteristicNotFound {
                        uuid: READ_CHARACTERISTIC_UUID,
                        source: None,
                    })?;
                let write_characteristic =
                    write_characteristic.ok_or(crate::Error::CharacteristicNotFound {
                        uuid: WRITE_CHARACTERISTIC_UUID,
                        source: None,
                    })?;

                let (connection_status_sender, connection_status_receiver) =
                    watch::channel(ConnectionStatus::Connected);

                let connection_status_handle = {
                    let mut events = device.events().await?;
                    tokio::spawn(async move {
                        loop {
                            if let Some(event) = events.next().await {
                                match event {
                                    bluer::DeviceEvent::PropertyChanged(
                                        DeviceProperty::Connected(is_connected),
                                    ) => {
                                        connection_status_sender.send_replace(match is_connected {
                                            true => ConnectionStatus::Connected,
                                            false => ConnectionStatus::Disconnected,
                                        });
                                    }
                                    _ => (),
                                }
                            }
                        }
                    })
                };

                let connection = BluerConnection {
                    device,
                    service_uuid,
                    write_characteristic: write_characteristic.to_owned(),
                    read_characteristic: read_characteristic.to_owned(),
                    connection_status_receiver,
                    connection_status_handle,
                    handle: handle.to_owned(),
                    quit: Arc::new(Semaphore::new(0)),
                };
                Ok(connection)
            })
            .await
            .unwrap()
    }
}

impl Connection for BluerConnection {
    async fn name(&self) -> crate::Result<String> {
        let device = self.device.to_owned();
        self.handle
            .spawn(async move {
                match device.name().await.unwrap() {
                    Some(name) => Ok(name),
                    None => Err(crate::Error::NameNotFound {
                        mac_address: device.address().to_string(),
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

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_without_response(data).await
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        let data = data.to_owned();
        let write_characteristic = self.write_characteristic.to_owned();
        self.handle
            .spawn(async move { write_characteristic.write(&data).await })
            .await
            .unwrap()
            .map_err(Into::into)
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);

        let read_characteristic = self.read_characteristic.to_owned();
        let notify = self
            .handle
            .spawn(async move { read_characteristic.notify().await })
            .await
            .unwrap()?;
        let quit = self.quit.to_owned();
        self.handle.spawn(
            async move {
                pin_mut!(notify);

                loop {
                    select! {
                        _ = quit.acquire() => {
                            break;
                        }
                        result = notify.next() => {
                            match result {
                                Some(data) => {
                                    trace!(event = "bluer notification", ?data);
                                    if let Err(err) = sender.try_send(data) {
                                        if let TrySendError::Closed(_) = err {
                                            break;
                                        }
                                        warn!("error forwarding packet to channel: {err}",)
                                    }
                                }
                                None => break,
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

impl Drop for BluerConnection {
    fn drop(&mut self) {
        self.connection_status_handle.abort();
        self.quit.close();
    }
}
