use std::{
    collections::{HashMap, HashSet},
    panic::Location,
    sync::Mutex,
};

use macaddr::MacAddr6;
use openscq30_i18n::Translate;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::{
    api::connection::{
        self, ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection,
    },
    devices::{DeviceModel, soundcore::common::packet},
};

use super::packet::Command;

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
        if data.len() < 7 {
            tracing::error!("packet should always contain direction and command: {data:?}");
            return Err(connection::Error::WriteError {
                source: None,
                location: Location::caller(),
            });
        }
        tracing::debug!("writing packet {data:?}");
        // The packet may or may not have a checksum at the end, so rather than actually parsing it,
        // just look at the command
        let command = packet::Command([data[5], data[6]]);
        if let Some(response) = self.packet_responses.get(&command) {
            self.packet_sender.send(response.to_owned()).await.unwrap();
        } else {
            // ACK
            self.packet_sender
                .send(command.ack::<packet::InboundMarker>().bytes())
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
