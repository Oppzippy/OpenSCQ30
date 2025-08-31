use crate::devices::soundcore::common::{
    packet::{Command, outbound::OutboundPacket},
    structures::AmbientSoundModeCycle,
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetAmbientSoundModeCycle {
    pub cycle: AmbientSoundModeCycle,
}

impl OutboundPacket for SetAmbientSoundModeCycle {
    fn command(&self) -> Command {
        Command([0x06, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}
