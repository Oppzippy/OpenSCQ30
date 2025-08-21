use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use macaddr::MacAddr6;
use nom_language::error::VerboseError;
use openscq30_i18n::Translate;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::{
    api::connection::{
        self, ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection,
    },
    devices::DeviceModel,
};

use super::packet::{Command, Direction, Packet};

pub struct DemoConnectionRegistry {
    model: DeviceModel,
    packet_responses: HashMap<Command, Vec<u8>>,
}

impl DemoConnectionRegistry {
    pub fn new(model: DeviceModel, packet_responses: HashMap<Command, Vec<u8>>) -> Self {
        Self {
            model,
            packet_responses,
        }
    }
}

impl RfcommBackend for DemoConnectionRegistry {
    type ConnectionType = DemoConnection;

    async fn devices(&self) -> connection::Result<HashSet<ConnectionDescriptor>> {
        Ok(HashSet::from([ConnectionDescriptor {
            name: self.model.translate().clone(),
            mac_address: self.model.demo_mac_address(),
        }]))
    }

    async fn connect(
        &self,
        _mac_address: MacAddr6,
        _select_uuid: impl Fn(HashSet<Uuid>) -> Uuid + Send,
    ) -> connection::Result<Self::ConnectionType> {
        Ok(DemoConnection::new(self.packet_responses.to_owned()))
    }
}

pub struct DemoConnection {
    _connection_status_sender: watch::Sender<ConnectionStatus>,
    connection_status_receiver: Mutex<Option<watch::Receiver<ConnectionStatus>>>,
    packet_sender: mpsc::Sender<Vec<u8>>,
    packet_receiver: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    packet_responses: HashMap<Command, Vec<u8>>,
}

impl DemoConnection {
    pub fn new(packet_responses: HashMap<Command, Vec<u8>>) -> Self {
        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);
        let (packet_sender, packet_receiver) = mpsc::channel(10);
        Self {
            _connection_status_sender: connection_status_sender,
            connection_status_receiver: Mutex::new(Some(connection_status_receiver)),
            packet_sender,
            packet_receiver: Mutex::new(Some(packet_receiver)),
            packet_responses,
        }
    }
}

impl RfcommConnection for DemoConnection {
    async fn write(&self, data: &[u8]) -> connection::Result<()> {
        tracing::debug!("writing packet {data:?}");
        let (_remainder, packet) = Packet::take::<VerboseError<_>>(data).unwrap();
        if let Some(response) = self.packet_responses.get(&packet.command) {
            self.packet_sender.send(response.to_owned()).await.unwrap();
        } else {
            // ACK
            self.packet_sender
                .send(
                    Packet {
                        direction: Direction::Inbound,
                        command: packet.command,
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap();
        }
        Ok(())
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        let (sender, receiver) = mpsc::channel(100);
        let mut inner_receiver = self.packet_receiver.lock().unwrap().take().unwrap();
        tokio::spawn(async move {
            while let Some(data) = inner_receiver.recv().await {
                tracing::debug!("received packet {data:?}");
                if sender.send(data).await.is_err() {
                    break;
                }
            }
        });
        receiver
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver
            .lock()
            .unwrap()
            .take()
            .unwrap()
    }
}
