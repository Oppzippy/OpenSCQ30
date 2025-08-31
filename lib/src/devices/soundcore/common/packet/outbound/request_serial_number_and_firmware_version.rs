use crate::devices::soundcore::common::packet::Command;

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestSerialNumberAndFirmwareVersion {}

impl RequestSerialNumberAndFirmwareVersion {
    pub const COMMAND: Command = Command([0x01, 0x05]);

    pub fn new() -> Self {
        Self {}
    }
}

impl OutboundPacket for RequestSerialNumberAndFirmwareVersion {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::packet::outbound::{
        OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0a, 0x00, 0x06];
        assert_eq!(
            expected,
            RequestSerialNumberAndFirmwareVersion::new().bytes()
        );
    }
}
