use crate::devices::soundcore::common::packet;

use super::outbound_packet::ToPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestState {}

impl RequestState {
    pub const COMMAND: packet::Command = packet::Command([0x01, 0x01]);
}

impl ToPacket for RequestState {
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
    use crate::devices::soundcore::common::packet::outbound::{RequestState, ToPacket};

    #[test]
    fn it_matches_an_example_request_state_packet() {
        const EXPECTED: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0a, 0x00, 0x02];
        assert_eq!(
            EXPECTED,
            RequestState::default().to_packet().bytes_with_checksum()
        );
    }
}
