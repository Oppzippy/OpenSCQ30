use async_trait::async_trait;
use btleplug::{
    api::Characteristic,
    api::{Central, CentralEvent, CharPropFlags, Peripheral as _, WriteType},
    platform::{Adapter, Peripheral},
};
use futures::StreamExt;
use macaddr::MacAddr6;
use tokio::{
    sync::{
        mpsc::{self, error::TrySendError},
        watch,
    },
    task::JoinHandle,
};
use tracing::{instrument, trace, trace_span, warn};

use crate::{
    api::connection::{Connection, ConnectionStatus},
    device_utils,
};

use super::mac_address::IntoMacAddr;

const WRITE_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: device_utils::WRITE_CHARACTERISTIC_UUID,
    service_uuid: device_utils::SERVICE_UUID,
    properties: CharPropFlags::WRITE_WITHOUT_RESPONSE.union(CharPropFlags::WRITE),
};
const NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: device_utils::READ_CHARACTERISTIC_UUID,
    service_uuid: device_utils::SERVICE_UUID,
    properties: CharPropFlags::READ.union(CharPropFlags::NOTIFY),
};

#[derive(Debug)]
pub struct BtlePlugConnection {
    peripheral: Peripheral,
    characteristic: Characteristic,
    connection_status_receiver: watch::Receiver<ConnectionStatus>,
    connection_status_handle: JoinHandle<()>,
}

impl BtlePlugConnection {
    pub async fn new(adapter: Adapter, peripheral: Peripheral) -> crate::Result<Self> {
        peripheral.connect().await?;
        peripheral.discover_services().await?;

        peripheral
            .subscribe(&NOTIFY_CHARACTERISTIC)
            .await
            .map_err(|err| crate::Error::CharacteristicNotFound {
                uuid: NOTIFY_CHARACTERISTIC.uuid,
                source: Some(Box::new(err)),
            })?;

        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);

        let connection_status_handle = {
            let mut events = adapter.events().await?;
            let peripheral = peripheral.to_owned();
            tokio::spawn(async move {
                loop {
                    if let Some(event) = events.next().await {
                        match event {
                            CentralEvent::DeviceConnected(peripheral_id) => {
                                if peripheral_id == peripheral.id() {
                                    connection_status_sender
                                        .send_replace(ConnectionStatus::Connected);
                                }
                            }
                            CentralEvent::DeviceDisconnected(peripheral_id) => {
                                if peripheral_id == peripheral.id() {
                                    connection_status_sender
                                        .send_replace(ConnectionStatus::Disconnected);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            })
        };

        let connection = BtlePlugConnection {
            peripheral,
            characteristic: WRITE_CHARACTERISTIC,
            connection_status_receiver,
            connection_status_handle,
        };

        Ok(connection)
    }
}

#[async_trait]
impl Connection for BtlePlugConnection {
    async fn name(&self) -> crate::Result<String> {
        let maybe_name = self
            .peripheral
            .properties()
            .await?
            .map(|property| property.local_name);

        match maybe_name {
            Some(Some(name)) => Ok(name),
            _ => Err(crate::Error::NameNotFound {
                mac_address: self.peripheral.address().to_string(),
            }),
        }
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver.clone()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.peripheral.address().into_mac_addr())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await?;
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        self.peripheral
            .write(&self.characteristic, data, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        // This queue should always be really small unless something is malfunctioning
        let (sender, receiver) = mpsc::channel(100);

        let mut notifications = self.peripheral.notifications().await?;

        tokio::spawn(async move {
            let span = trace_span!("inbound_packets_channel async task");
            let _enter = span.enter();
            while let Some(data) = notifications.next().await {
                trace!(event = "btleplug notification", data = ?data);
                if data.uuid == NOTIFY_CHARACTERISTIC.uuid {
                    if let Err(err) = sender.try_send(data.value) {
                        if let TrySendError::Closed(_) = err {
                            break;
                        }
                        warn!("error forwarding packet to channel: {err}",)
                    }
                }
            }
        });

        Ok(receiver)
    }
}

impl Drop for BtlePlugConnection {
    fn drop(&mut self) {
        self.connection_status_handle.abort();
    }
}
