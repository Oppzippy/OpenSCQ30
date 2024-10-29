use crate::devices::standard::packets::checksum::calculate_checksum;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Packet {
    pub command: [u8; 7],
    pub body: Vec<u8>,
}

impl Packet {
    pub fn command(&self) -> [u8; 7] {
        self.command
    }

    pub fn ok_command(&self) -> [u8; 7] {
        let mut command = self.command;
        command[0] = 0x09;
        command[1] = 0xff;
        command[4] = 0x01;
        command
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);
        let command = self.command;
        let body = &self.body;

        const PACKET_SIZE_LENGTH: usize = 2;
        const CHECKSUM_LENGTH: usize = 1;
        let length = command.len() + PACKET_SIZE_LENGTH + body.len() + CHECKSUM_LENGTH;

        bytes.extend(command);
        bytes.extend((length as u16).to_le_bytes());
        bytes.extend(body);
        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}
