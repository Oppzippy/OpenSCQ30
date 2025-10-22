use crate::devices::soundcore::common::{
    packet::{self, outbound::IntoPacket},
    structures::TouchTone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetTouchTone(pub TouchTone);

impl SetTouchTone {
    pub const COMMAND: packet::Command = packet::Command([0x01, 0x83]);
}

impl IntoPacket for SetTouchTone {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.bytes().to_vec()
    }
}
