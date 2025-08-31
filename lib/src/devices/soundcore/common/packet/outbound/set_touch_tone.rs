use crate::devices::soundcore::common::{
    packet::{Command, outbound::OutboundPacket},
    structures::TouchTone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetTouchTone(pub TouchTone);

impl SetTouchTone {
    pub const COMMAND: Command = Command([0x01, 0x83]);
}

impl OutboundPacket for SetTouchTone {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.bytes().to_vec()
    }
}
