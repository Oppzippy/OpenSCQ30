use crate::devices::soundcore::standard::{
    packets::outbound::OutboundPacket,
    structures::{AmbientSoundModeCycle, Command},
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetAmbientSoundModeCyclePacket {
    pub cycle: AmbientSoundModeCycle,
}

impl OutboundPacket for SetAmbientSoundModeCyclePacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x06, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}
