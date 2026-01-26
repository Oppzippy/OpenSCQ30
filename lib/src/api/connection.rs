use std::{collections::HashSet, panic::Location};

use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bluetooth adapter unavailable")]
    BluetoothAdapterUnavailable {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("device not found")]
    DeviceNotFound {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("write error")]
    WriteError {
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        location: &'static Location<'static>,
    },
    #[error("other")]
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

    /// List all devices that are currently connected to the bluetooth adapter. On platforms where this isn't practical,
    /// this may include devices that are not currently connected.
    fn devices(&self) -> impl Future<Output = Result<HashSet<ConnectionDescriptor>>> + Send;

    /// Connect via RFCOMM to the device. It should already be paired.
    ///
    /// The RFCOMM UUID to connect to may depend on what is available, so a `select_uuid` function should be passed that
    /// picks the uuid to connect to from the list of what is available.
    ///
    /// This method is cancel safe. Implementers should make sure that is the case.
    fn connect(
        &self,
        mac_address: MacAddr6,
        service_selection_strategy: RfcommServiceSelectionStrategy,
    ) -> impl Future<Output = Result<Self::ConnectionType>> + Send;
}

pub enum RfcommServiceSelectionStrategy {
    Constant(Uuid),
    Dynamic(fn(HashSet<Uuid>) -> Uuid),
}

pub trait RfcommConnection {
    /// Sends `data` over the RFCOMM connection. This will be sent as a single packet when possible.
    fn write(&self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;

    /// Returns a channel that will receive packets from the RFCOMM connection.
    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>>;

    /// Returns a `tokio::sync::watch::Receiver` for tracking when the connection disconnects.
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
