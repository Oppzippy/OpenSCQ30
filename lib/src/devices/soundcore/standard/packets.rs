pub mod checksum;
pub mod inbound;
pub mod multi_queue;
pub mod outbound;
pub mod packet_io_controller;
pub mod parsing;

use crate::devices::soundcore::standard::{
    packets::checksum::calculate_checksum, structures::Command,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Packet {
    pub command: Command,
    pub body: Vec<u8>,
}

impl Packet {
    pub fn command(&self) -> Command {
        self.command
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);
        let command = self.command;
        let body = &self.body;

        const PACKET_SIZE_LENGTH: usize = 2;
        const CHECKSUM_LENGTH: usize = 1;
        let length = command.bytes().len() + PACKET_SIZE_LENGTH + body.len() + CHECKSUM_LENGTH;

        bytes.extend(command.bytes());
        bytes.extend((length as u16).to_le_bytes());
        bytes.extend(body);
        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}
