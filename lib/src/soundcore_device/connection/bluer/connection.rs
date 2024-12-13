use std::sync::Arc;

use bluer::{
    gatt::remote::{Characteristic, Service},
    Device, DeviceProperty,
};
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
use tracing::{debug, instrument, trace, trace_span, warn, Instrument};
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
        handle
            .clone()
            .spawn(async move {
                device.connect().await?;
                let service = Self::get_service(&device).await?;
                let service_uuid = service.uuid().await?;

                let [read_characteristic, write_characteristic] = Self::get_characteristics(
                    &service,
                    [READ_CHARACTERISTIC_UUID, WRITE_CHARACTERISTIC_UUID],
                )
                .await?;

                let (connection_status_receiver, connection_status_handle) =
                    Self::spawn_connection_status(&handle, device.to_owned()).await?;

                let connection = BluerConnection {
                    device,
                    service_uuid,
                    write_characteristic,
                    read_characteristic,
                    connection_status_receiver,
                    connection_status_handle,
                    handle,
                    quit: Arc::new(Semaphore::new(0)),
                };
                Ok(connection)
            })
            .await
            .unwrap()
    }

    async fn get_service(device: &Device) -> crate::Result<Service> {
        for service in device.services().await? {
            if device_utils::is_soundcore_service_uuid(&service.uuid().await?) {
                return Ok(service);
            }
        }
        Err(crate::Error::ServiceNotFound {
            uuid: SERVICE_UUID,
            source: None,
        })
    }

    async fn get_characteristics<const SIZE: usize>(
        service: &Service,
        uuids: [Uuid; SIZE],
    ) -> crate::Result<[Characteristic; SIZE]> {
        let mut characteristics: [Option<Characteristic>; SIZE] = [const { None }; SIZE];
        for characteristic in service.characteristics().await? {
            let uuid = characteristic.uuid().await?;
            if let Some(index) = uuids.iter().position(|u| *u == uuid) {
                characteristics[index] = Some(characteristic);
            }
        }

        characteristics
            .iter()
            .enumerate()
            .find(|(_, c)| c.is_none())
            .map(|(i, _)| {
                Err(crate::Error::CharacteristicNotFound {
                    uuid: uuids[i],
                    source: None,
                }) as Result<(), crate::Error>
            })
            .transpose()?;
        Ok(characteristics.map(|v| v.expect("we already made sure every element is some")))
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
                    if let Some(event) = events.next().await {
                        match event {
                            bluer::DeviceEvent::PropertyChanged(DeviceProperty::Connected(
                                is_connected,
                            )) => {
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

        Ok((connection_status_receiver, connection_status_handle))
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
                                None => {
                                    debug!("notify channel ended");
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

impl Drop for BluerConnection {
    fn drop(&mut self) {
        self.connection_status_handle.abort();
        self.quit.close();
    }
}
