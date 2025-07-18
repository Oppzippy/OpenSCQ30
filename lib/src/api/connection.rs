use std::{collections::HashSet, panic::Location};

use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bluetooth adapter unavailable: {}", .source.as_ref().map(|s| s.to_string()).unwrap_or_default())]
    BluetoothAdapterUnavailable {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("device not found: {}", .source.as_ref().map(|s| s.to_string()).unwrap_or_default())]
    DeviceNotFound {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("write error: {}", .source.as_ref().map(|s| s.to_string()).unwrap_or_default())]
    WriteError {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("other: {source}")]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: &'static Location<'static>,
    },

    #[error("{action} timed out")]
    TimedOut { action: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait RfcommBackend {
    type ConnectionType: RfcommConnection + Send + Sync;

    fn devices(&self) -> impl Future<Output = Result<HashSet<ConnectionDescriptor>>> + Send;
    fn connect(
        &self,
        mac_address: MacAddr6,
        select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send + Sync + 'static,
    ) -> impl Future<Output = Result<Self::ConnectionType>> + Send;
}

pub trait RfcommConnection {
    fn write(&self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;
    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>;
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDescriptor {
    pub name: String,
    #[serde(with = "crate::serialization::mac_addr")]
    pub mac_address: MacAddr6,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[cfg(test)]
pub mod test_stub {
    use std::sync::Mutex;

    use super::*;
    pub struct StubRfcommConnection {
        connection_status: watch::Sender<ConnectionStatus>,
        inbound_receiver: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
        outbound_sender: mpsc::Sender<Vec<u8>>,
    }

    impl StubRfcommConnection {
        pub fn new() -> (Self, mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) {
            let (inbound_sender, inbound_receiver) = mpsc::channel(100);
            let (outbound_sender, outbound_receiver) = mpsc::channel(100);
            (
                Self {
                    connection_status: watch::channel(ConnectionStatus::Connected).0,
                    inbound_receiver: Mutex::new(Some(inbound_receiver)),
                    outbound_sender,
                },
                inbound_sender,
                outbound_receiver,
            )
        }
    }

    impl RfcommConnection for StubRfcommConnection {
        async fn write(&self, data: &[u8]) -> Result<()> {
            self.outbound_sender.send(data.to_owned()).await.unwrap();
            Ok(())
        }

        fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
            self.inbound_receiver
                .lock()
                .unwrap()
                .take()
                .expect("StubRfcommConnection::read_channel should only be called once")
        }

        fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
            self.connection_status.subscribe()
        }
    }
}
