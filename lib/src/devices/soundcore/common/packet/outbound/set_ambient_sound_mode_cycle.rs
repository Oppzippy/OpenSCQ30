use crate::devices::soundcore::common::{
    packet::{self, outbound::IntoPacket},
    structures::AmbientSoundModeCycle,
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetAmbientSoundModeCycle {
    pub cycle: AmbientSoundModeCycle,
}

impl IntoPacket for SetAmbientSoundModeCycle {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x06, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}
