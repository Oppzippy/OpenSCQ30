use crate::devices::soundcore::common::packet::Command;

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestState {}

impl RequestState {
    pub const COMMAND: Command = Command([0x01, 0x01]);

    pub fn new() -> Self {
        Self {}
    }
}

impl OutboundPacket for RequestState {
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
        OutboundPacketBytesExt, RequestState,
    };

    #[test]
    fn it_matches_an_example_request_state_packet() {
        const EXPECTED: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0a, 0x00, 0x02];
        let packet = RequestState::new();
        assert_eq!(EXPECTED, packet.bytes());
    }
}
