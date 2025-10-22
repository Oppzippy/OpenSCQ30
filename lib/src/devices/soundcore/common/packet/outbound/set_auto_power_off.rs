use crate::devices::soundcore::common::{
    packet::{self, outbound::IntoPacket},
    structures::AutoPowerOff,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetAutoPowerOff(pub AutoPowerOff);

impl SetAutoPowerOff {
    pub const COMMAND: packet::Command = packet::Command([0x01, 0x86]);
}

impl IntoPacket for SetAutoPowerOff {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        vec![self.0.is_enabled.into(), self.0.duration.0]
    }
}
