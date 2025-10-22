use crate::devices::soundcore::common::packet;

use super::ToPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestSerialNumberAndFirmwareVersion {}

impl RequestSerialNumberAndFirmwareVersion {
    pub const COMMAND: packet::Command = packet::Command([0x01, 0x05]);
}

impl ToPacket for RequestSerialNumberAndFirmwareVersion {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::packet::outbound::{
        RequestSerialNumberAndFirmwareVersion, ToPacket,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0a, 0x00, 0x06];
        assert_eq!(
            expected,
            RequestSerialNumberAndFirmwareVersion::default()
                .to_packet()
                .bytes()
        );
    }
}
