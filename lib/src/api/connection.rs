use std::collections::HashSet;

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

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
        async fn write(&self, data: &[u8]) -> crate::Result<()> {
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
