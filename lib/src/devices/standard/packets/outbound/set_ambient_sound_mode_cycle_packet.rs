use crate::devices::standard::{
    packets::outbound::OutboundPacket, structures::AmbientSoundModeCycle,
};

pub struct SetAmbientSoundModeCyclePacket {
    pub cycle: AmbientSoundModeCycle,
}

impl OutboundPacket for SetAmbientSoundModeCyclePacket {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x06, 0x82]
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}
