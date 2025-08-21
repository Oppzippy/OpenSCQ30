use crate::devices::soundcore::{
    a3028::packets::AutoPowerOff,
    standard::packet::{Command, outbound::OutboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetAutoPowerOffPacket(pub AutoPowerOff);

impl SetAutoPowerOffPacket {
    pub const COMMAND: Command = Command([0x01, 0x86]);
}

impl OutboundPacket for SetAutoPowerOffPacket {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        vec![self.0.enabled.into(), self.0.duration as u8]
    }
}
