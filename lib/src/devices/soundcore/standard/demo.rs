use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use macaddr::MacAddr6;
use nom::error::VerboseError;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::{
    api::connection::{self, ConnectionStatus, DeviceDescriptor, RfcommBackend, RfcommConnection},
    devices::soundcore::standard::packets::inbound::take_inbound_packet_header,
};

use super::{packets::Packet, structures::Command};

pub struct DemoConnectionRegistry {
    name: String,
    packet_responses: HashMap<Command, Vec<u8>>,
}

impl DemoConnectionRegistry {
    pub fn new(name: String, packet_responses: HashMap<Command, Vec<u8>>) -> Self {
        Self {
            name,
            packet_responses,
        }
    }
}

impl RfcommBackend for DemoConnectionRegistry {
    type ConnectionType = DemoConnection;

    async fn devices(&self) -> connection::Result<HashSet<DeviceDescriptor>> {
        Ok(HashSet::from([DeviceDescriptor {
            name: self.name.clone(),
            mac_address: MacAddr6::nil(),
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
        let (_body, command) = take_inbound_packet_header::<VerboseError<_>>(data).unwrap();
        if let Some(response) = self.packet_responses.get(&command) {
            self.packet_sender.send(response.to_owned()).await.unwrap();
        } else {
            // ACK
            self.packet_sender
                .send(
                    Packet {
                        command: command.to_inbound(),
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap()
        }
        Ok(())
    }

    fn read_channel(&self) -> mpsc::Receiver<Vec<u8>> {
        self.packet_receiver.lock().unwrap().take().unwrap()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver
            .lock()
            .unwrap()
            .take()
            .unwrap()
    }
}
