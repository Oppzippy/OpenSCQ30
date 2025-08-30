use crate::devices::soundcore::standard::{
    packet::{Command, outbound::OutboundPacket},
    structures::TouchTone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetTouchTonePacket(pub TouchTone);

impl SetTouchTonePacket {
    pub const COMMAND: Command = Command([0x01, 0x83]);
}

impl OutboundPacket for SetTouchTonePacket {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.bytes().to_vec()
    }
}
