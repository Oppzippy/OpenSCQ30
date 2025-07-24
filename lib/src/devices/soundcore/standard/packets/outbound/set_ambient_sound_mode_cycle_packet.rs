use crate::devices::soundcore::standard::{
    packets::{Command, outbound::OutboundPacket},
    structures::AmbientSoundModeCycle,
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetAmbientSoundModeCyclePacket {
    pub cycle: AmbientSoundModeCycle,
}

impl OutboundPacket for SetAmbientSoundModeCyclePacket {
    fn command(&self) -> Command {
        Command([0x06, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}
