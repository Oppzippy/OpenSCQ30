use crate::devices::soundcore::common::packet;

use super::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(dead_code, reason = "TODO send periodically if needed")]
pub struct RequestBatteryLevel {}

impl IntoPacket for RequestBatteryLevel {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x01, 0x03])
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::packet::outbound::{IntoPacket, RequestBatteryLevel};

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x03, 0x0a, 0x00, 0x04];
        assert_eq!(
            expected,
            RequestBatteryLevel::default().into_packet().bytes()
        );
    }
}
