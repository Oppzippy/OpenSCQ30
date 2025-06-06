use std::{collections::HashSet, panic::Location, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use openscq30_lib::{
    api::connection::{self, ConnectionStatus, RfcommBackend, RfcommConnection},
    connection_backend::ConnectionBackends,
};
use thiserror::Error;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::{AndroidError, serializable};

#[derive(uniffi::Object)]
pub struct ManualConnectionBackends {
    rfcomm: Arc<dyn AndroidRfcommConnectionBackend>,
}

#[uniffi::export]
impl ManualConnectionBackends {
    #[uniffi::constructor]
    pub fn new(rfcomm: Arc<dyn AndroidRfcommConnectionBackend>) -> Self {
        Self { rfcomm }
    }
}

impl ConnectionBackends for ManualConnectionBackends {
    type Rfcomm = ManualRfcommConnectionBackend;

    async fn rfcomm(&self) -> connection::Result<Self::Rfcomm> {
        Ok(ManualRfcommConnectionBackend {
            inner: self.rfcomm.to_owned(),
        })
    }
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait AndroidRfcommConnectionBackend: Send + Sync {
    async fn devices(&self) -> Result<Vec<serializable::ConnectionDescriptor>, AndroidError>;
    async fn connect(
        &self,
        mac_address: serializable::MacAddr6,
        select_uuid: Arc<UuidSelector>,
        output_box: Arc<ManualRfcommConnectionBox>,
    ) -> Result<(), AndroidError>;
}

#[derive(uniffi::Object)]
pub struct UuidSelector {
    selector: Box<dyn Fn(HashSet<Uuid>) -> Uuid + Send + Sync>,
}

#[uniffi::export]
impl UuidSelector {
    pub fn select(&self, uuids: Vec<serializable::Uuid>) -> serializable::Uuid {
        serializable::Uuid((self.selector)(
            uuids.into_iter().map(|uuid| uuid.0).collect(),
        ))
    }
}

#[derive(Default, uniffi::Object)]
/// Having a uniffi foreign trait return another uniffi foreign trait doesn't seem to work,
/// so having an output parameter rather than returning the value directly is used as a workaround.
pub struct ManualRfcommConnectionBox {
    inner: std::sync::Mutex<Option<Arc<ManualRfcommConnection>>>,
}

#[uniffi::export]
impl ManualRfcommConnectionBox {
    pub fn set(&self, inner: Option<Arc<ManualRfcommConnection>>) {
        *self.inner.lock().unwrap() = inner;
    }

    pub fn get(&self) -> Option<Arc<ManualRfcommConnection>> {
        self.inner.lock().unwrap().take()
    }
}

pub struct ManualRfcommConnectionBackend {
    inner: Arc<dyn AndroidRfcommConnectionBackend>,
}

impl RfcommBackend for ManualRfcommConnectionBackend {
    type ConnectionType = WrappedManualRfcommConnection;

    async fn devices(&self) -> connection::Result<HashSet<connection::ConnectionDescriptor>> {
        let descriptors = self
            .inner
            .devices()
            .await
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })?;
        Ok(descriptors
            .into_iter()
            .map(|descriptor| descriptor.0)
            .collect())
    }

    async fn connect(
        &self,
        mac_address: MacAddr6,
        select_uuid: impl Fn(std::collections::HashSet<uuid::Uuid>) -> uuid::Uuid
        + Send
        + Sync
        + 'static,
    ) -> connection::Result<Self::ConnectionType> {
        let output_box = Arc::new(ManualRfcommConnectionBox::default());
        self.inner
            .connect(
                serializable::MacAddr6(mac_address),
                Arc::new(UuidSelector {
                    selector: Box::new(select_uuid),
                }),
                output_box.clone(),
            )
            .await
            .map_err(|err| connection::Error::Other {
                source: Box::new(err),
                location: Location::caller(),
            })?;
        Ok(WrappedManualRfcommConnection(output_box.get().ok_or_else(
            || connection::Error::DeviceNotFound {
                source: None,
                location: Location::caller(),
            },
        )?))
    }
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait AndroidRfcommConnectionWriter: Send + Sync {
    async fn write(&self, data: Vec<u8>);
}

#[derive(Error, Debug, uniffi::Error)]
pub enum ConnectionError {
    #[error("the device connection write queue is full")]
    WriteQueueFullError,
}

// Locks are from std::sync, so make sure to not hold across await points
#[derive(uniffi::Object)]
pub struct ManualRfcommConnection {
    connection_status_sender: watch::Sender<ConnectionStatus>,
    connection_writer: Arc<dyn AndroidRfcommConnectionWriter>,
    inbound_packets_sender: std::sync::RwLock<Option<mpsc::Sender<Vec<u8>>>>,
}

#[uniffi::export]
impl ManualRfcommConnection {
    #[uniffi::constructor]
    pub fn new(
        connection_writer: Arc<dyn AndroidRfcommConnectionWriter>,
    ) -> ManualRfcommConnection {
        let (connection_status_sender, _) = watch::channel(ConnectionStatus::Connected);
        ManualRfcommConnection {
            connection_status_sender,
            connection_writer,
            inbound_packets_sender: std::sync::RwLock::new(None),
        }
    }
}

#[uniffi::export]
impl ManualRfcommConnection {
    pub fn add_inbound_packet(&self, inbound_packet: Vec<u8>) -> Result<(), ConnectionError> {
        tracing::info!("got packet {inbound_packet:?}");
        match &*self.inbound_packets_sender.read().unwrap() {
            Some(sender) => sender
                .blocking_send(inbound_packet)
                .map_err(|_| ConnectionError::WriteQueueFullError),
            None => {
                tracing::warn!(
                    "ManualConnection: add_inbound_packet called while inbound_packet_sender is None with {inbound_packet:?}"
                );
                Ok(())
            }
        }
    }

    pub fn set_connection_status(&self, connection_status: serializable::ConnectionStatus) {
        self.connection_status_sender
            .send_replace(connection_status.0);
    }
}

pub struct WrappedManualRfcommConnection(Arc<ManualRfcommConnection>);

impl RfcommConnection for WrappedManualRfcommConnection {
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.0.connection_status_sender.subscribe()
    }

    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        self.0.connection_writer.write(data.to_owned()).await;
        Ok(())
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        let (sender, receiver) = mpsc::channel(50);
        *self.0.inbound_packets_sender.write().unwrap() = Some(sender);

        receiver
    }
}
