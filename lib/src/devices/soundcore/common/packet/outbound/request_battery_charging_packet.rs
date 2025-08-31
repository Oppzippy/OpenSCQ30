use crate::devices::soundcore::common::packet::Command;

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(dead_code, reason = "TODO send periodically if needed")]
pub struct RequestBatteryChargingPacket {}

impl OutboundPacket for RequestBatteryChargingPacket {
    fn command(&self) -> Command {
        Command([0x01, 0x04])
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::packet::outbound::{
        OutboundPacketBytesExt, RequestBatteryChargingPacket,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x04, 0x0a, 0x00, 0x05];
        assert_eq!(expected, RequestBatteryChargingPacket::default().bytes());
    }
}
