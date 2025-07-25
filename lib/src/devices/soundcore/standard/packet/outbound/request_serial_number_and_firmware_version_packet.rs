use crate::devices::soundcore::standard::packet::Command;

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestSerialNumberAndFirmwareVersionPacket {}

impl RequestSerialNumberAndFirmwareVersionPacket {
    pub const COMMAND: Command = Command([0x01, 0x05]);

    pub fn new() -> Self {
        Self {}
    }
}

impl OutboundPacket for RequestSerialNumberAndFirmwareVersionPacket {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::standard::packet::outbound::{
        OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0a, 0x00, 0x06];
        assert_eq!(
            expected,
            RequestSerialNumberAndFirmwareVersionPacket::new().bytes()
        );
    }
}
