use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestBatteryChargingPacket {}

impl RequestBatteryChargingPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutboundPacket for RequestBatteryChargingPacket {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x04]
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::packets::outbound::{
        OutboundPacketBytes, RequestBatteryChargingPacket,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x04, 0x0a, 0x00, 0x05];
        assert_eq!(expected, RequestBatteryChargingPacket::new().bytes())
    }
}
