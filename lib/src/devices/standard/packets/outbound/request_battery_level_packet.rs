use crate::devices::standard::structures::Command;

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestBatteryLevelPacket {}

impl RequestBatteryLevelPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutboundPacket for RequestBatteryLevelPacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x03])
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::packets::outbound::{
        OutboundPacketBytesExt, RequestBatteryLevelPacket,
    };

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        let expected: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x03, 0x0a, 0x00, 0x04];
        assert_eq!(expected, RequestBatteryLevelPacket::new().bytes())
    }
}
