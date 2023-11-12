use crate::devices::standard::packets::checksum::calculate_checksum;

pub trait OutboundPacket {
    fn command(&self) -> [u8; 7];
    fn body(&self) -> Vec<u8>;
}

pub trait OutboundPacketBytes {
    fn bytes(&self) -> Vec<u8>;
}

impl<T> OutboundPacketBytes for T
where
    T: OutboundPacket,
{
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);
        let command = self.command();
        let body = self.body();

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
