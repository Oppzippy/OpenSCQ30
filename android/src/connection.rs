use std::sync::Arc;

use macaddr::MacAddr6;
use openscq30_lib::api::connection::{Connection, ConnectionStatus};
use thiserror::Error;
use tokio::sync::{mpsc, watch, RwLock};
use uuid::Uuid;

#[uniffi::export(callback_interface)]
pub trait ConnectionWriter: Send + Sync {
    fn write_with_response(&self, data: Vec<u8>);
    fn write_without_response(&self, data: Vec<u8>);
}

#[derive(Error, Debug, uniffi::Error)]
pub enum ConnectionError {
    #[error("the device connection write queue is full")]
    WriteQueueFullError,
}

#[derive(uniffi::Object)]
pub struct ManualConnection {
    name: RwLock<String>,
    mac_address: RwLock<MacAddr6>,
    connection_status_sender: watch::Sender<ConnectionStatus>,
    connection_writer: Box<dyn ConnectionWriter>,
    service_uuid: Uuid,
    inbound_packets_sender: RwLock<Option<mpsc::Sender<Vec<u8>>>>,
}

#[uniffi::export]
impl ManualConnection {
    #[uniffi::constructor]
    pub fn new(
        name: String,
        mac_address: MacAddr6,
        service_uuid: Uuid,
        connection_writer: Box<dyn ConnectionWriter>,
    ) -> Arc<ManualConnection> {
        let (connection_status_sender, _) = watch::channel(ConnectionStatus::Connected);
        Arc::new(ManualConnection {
            name: RwLock::new(name),
            mac_address: RwLock::new(mac_address),
            connection_status_sender,
            connection_writer,
            service_uuid,
            inbound_packets_sender: RwLock::new(None),
        })
    }
}

#[uniffi::export]
impl ManualConnection {
    pub async fn set_name(&self, name: String) {
        *self.name.write().await = name;
    }

    pub async fn set_mac_address(&self, mac_address: MacAddr6) {
        *self.mac_address.write().await = mac_address;
    }

    pub async fn add_inbound_packet(&self, inbound_packet: Vec<u8>) -> Result<(), ConnectionError> {
        tracing::info!("got packet {inbound_packet:?}");
        match &*self.inbound_packets_sender.read().await {
            Some(sender) => sender
                .send(inbound_packet)
                .await
                .map_err(|_| ConnectionError::WriteQueueFullError),
            None => {
                tracing::warn!(
                    "ManualConnection: add_inbound_packet called while inbound_packet_sender is None with {inbound_packet:?}"
                );
                Ok(())
            }
        }
    }
}

impl Connection for ManualConnection {
    async fn name(&self) -> openscq30_lib::Result<String> {
        Ok(self.name.read().await.to_owned())
    }

    async fn mac_address(&self) -> openscq30_lib::Result<MacAddr6> {
        Ok(*self.mac_address.read().await)
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_sender.subscribe()
    }

    async fn write_with_response(&self, data: &[u8]) -> openscq30_lib::Result<()> {
        self.connection_writer.write_with_response(data.to_owned());
        Ok(())
    }

    async fn write_without_response(&self, data: &[u8]) -> openscq30_lib::Result<()> {
        self.connection_writer.write_with_response(data.to_owned());
        Ok(())
    }

    async fn inbound_packets_channel(&self) -> openscq30_lib::Result<mpsc::Receiver<Vec<u8>>> {
        let (sender, receiver) = mpsc::channel(50);
        *self.inbound_packets_sender.write().await = Some(sender);

        Ok(receiver)
    }

    fn service_uuid(&self) -> Uuid {
        self.service_uuid
    }
}
