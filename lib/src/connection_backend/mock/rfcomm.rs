use std::{collections::HashSet, sync::Mutex};

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::api::connection::{
    self, ConnectionStatus, ConnectionDescriptor, RfcommBackend, RfcommConnection,
};

pub struct MockRfcommBackend {
    inbound: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    outbound: Mutex<Option<mpsc::Sender<Vec<u8>>>>,
}

impl MockRfcommBackend {
    pub fn new(inbound: mpsc::Receiver<Vec<u8>>, outbound: mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            inbound: Mutex::new(Some(inbound)),
            outbound: Mutex::new(Some(outbound)),
        }
    }
}

impl RfcommBackend for MockRfcommBackend {
    type ConnectionType = MockRfcommConnection;

    async fn devices(&self) -> connection::Result<HashSet<ConnectionDescriptor>> {
        Ok([ConnectionDescriptor {
            name: "Mock Device".to_owned(),
            mac_address: MacAddr6::nil(),
        }]
        .into_iter()
        .collect())
    }

    async fn connect(
        &self,
        _mac_address: MacAddr6,
        _select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send,
    ) -> connection::Result<Self::ConnectionType> {
        Ok(MockRfcommConnection::new(
            self.inbound
                .lock()
                .unwrap()
                .take()
                .expect("connect should only be called once"),
            self.outbound
                .lock()
                .unwrap()
                .take()
                .expect("connect should only be called once"),
        ))
    }
}

pub struct MockRfcommConnection {
    inbound: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    outbound: mpsc::Sender<Vec<u8>>,
    connection_status: watch::Sender<ConnectionStatus>,
}

impl MockRfcommConnection {
    pub fn new(inbound: mpsc::Receiver<Vec<u8>>, outbound: mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            inbound: Mutex::new(Some(inbound)),
            outbound,
            connection_status: watch::channel(ConnectionStatus::Connected).0,
        }
    }
}

impl RfcommConnection for MockRfcommConnection {
    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        self.outbound.send(data.to_vec()).await.unwrap();
        Ok(())
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.inbound
            .lock()
            .unwrap()
            .take()
            .expect("read_channel should only be called once")
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status.subscribe()
    }
}
