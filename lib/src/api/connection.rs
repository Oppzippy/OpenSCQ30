mod connection;
mod connection_descriptor;
mod connection_registry;
mod connection_status;
mod generic_connection_descriptor;

use std::collections::HashSet;

pub use connection::*;
pub use connection_descriptor::*;
pub use connection_registry::*;
pub use generic_connection_descriptor::*;
use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::connection_backend;

pub trait ConnectionBackends {
    type Rfcomm: RfcommBackend + Send + Sync;

    fn rfcomm(&self) -> impl Future<Output = crate::Result<Self::Rfcomm>> + Send;
}

pub fn default_backends() -> impl ConnectionBackends {
    PlatformConnectionBackends {}
}

#[cfg(target_os = "linux")]
struct PlatformConnectionBackends {}
#[cfg(target_os = "linux")]
impl ConnectionBackends for PlatformConnectionBackends {
    type Rfcomm = connection_backend::rfcomm::BluerRfcommBackend;

    async fn rfcomm(&self) -> crate::Result<Self::Rfcomm> {
        connection_backend::rfcomm::BluerRfcommBackend::new().await
    }
}

pub trait RfcommBackend {
    type ConnectionType: RfcommConnection + Send + Sync;

    fn devices(&self) -> impl Future<Output = crate::Result<HashSet<DeviceDescriptor>>> + Send;
    fn connect(
        &self,
        mac_address: MacAddr6,
        select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send,
    ) -> impl Future<Output = crate::Result<Self::ConnectionType>> + Send;
}

pub trait RfcommConnection {
    fn write(&self, data: &[u8]) -> impl Future<Output = crate::Result<()>> + Send;
    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>;
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
}

impl<T> Connection for T
where
    T: RfcommConnection + Send + Sync,
{
    async fn name(&self) -> crate::Result<String> {
        todo!()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        todo!()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        RfcommConnection::connection_status(self)
    }

    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write(data).await
    }

    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write(data).await
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        Ok(self.read_channel())
    }

    fn service_uuid(&self) -> Uuid {
        todo!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord, Hash)]
pub struct DeviceDescriptor {
    pub name: String,
    pub mac_address: MacAddr6,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}
