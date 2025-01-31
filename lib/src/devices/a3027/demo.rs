use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use macaddr::MacAddr6;
use nom::error::VerboseError;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::{
    api::connection::{
        Connection, ConnectionRegistry, ConnectionStatus, GenericConnectionDescriptor,
    },
    devices::standard::{packets::inbound::take_inbound_packet_header, structures::STATE_UPDATE},
    soundcore_device::device::Packet,
};

pub struct DemoConnectionRegistry {
    name: String,
    state_update_packet: Vec<u8>,
}

impl DemoConnectionRegistry {
    pub fn new(name: String, state_update_packet: Vec<u8>) -> Self {
        Self {
            name,
            state_update_packet,
        }
    }
}

impl ConnectionRegistry for DemoConnectionRegistry {
    type ConnectionType = DemoConnection;
    type DescriptorType = GenericConnectionDescriptor;
    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        let mut descriptors = HashSet::new();
        descriptors.insert(GenericConnectionDescriptor::new(
            "Demo A3027",
            MacAddr6::nil(),
        ));
        Ok(descriptors)
    }
    async fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>> {
        Ok(Some(Arc::new(DemoConnection::new(
            self.name.to_owned(),
            mac_address,
            self.state_update_packet.to_owned(),
        ))))
    }
}

pub struct DemoConnection {
    name: String,
    mac_address: MacAddr6,
    _connection_status_sender: watch::Sender<ConnectionStatus>,
    connection_status_receiver: Mutex<Option<watch::Receiver<ConnectionStatus>>>,
    packet_sender: mpsc::Sender<Vec<u8>>,
    packet_receiver: Mutex<Option<mpsc::Receiver<Vec<u8>>>>,
    state_update_packet: Vec<u8>,
}

impl DemoConnection {
    pub fn new(name: String, mac_address: MacAddr6, state_update_packet: Vec<u8>) -> Self {
        let (connection_status_sender, connection_status_receiver) =
            watch::channel(ConnectionStatus::Connected);
        let (packet_sender, packet_receiver) = mpsc::channel(10);
        Self {
            name,
            mac_address,
            _connection_status_sender: connection_status_sender,
            connection_status_receiver: Mutex::new(Some(connection_status_receiver)),
            packet_sender,
            packet_receiver: Mutex::new(Some(packet_receiver)),
            state_update_packet,
        }
    }
}

impl Connection for DemoConnection {
    async fn name(&self) -> crate::Result<String> {
        Ok(self.name.to_owned())
    }
    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.mac_address)
    }
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_receiver
            .lock()
            .unwrap()
            .take()
            .unwrap()
    }
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()> {
        self.write_without_response(data).await
    }
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()> {
        let (_body, command) = take_inbound_packet_header::<VerboseError<_>>(data).unwrap();
        match command.to_inbound() {
            STATE_UPDATE => self
                .packet_sender
                .send(self.state_update_packet.clone())
                .await
                .unwrap(),
            _ => self
                .packet_sender
                .send(
                    Packet {
                        command,
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap(),
        }
        Ok(())
    }
    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        Ok(self.packet_receiver.lock().unwrap().take().unwrap())
    }
    fn service_uuid(&self) -> Uuid {
        Uuid::nil()
    }
}
