use crate::devices::soundcore::common::{
    packet::{self, outbound::ToPacket},
    structures::{AmbientSoundModeCycle, AmbientSoundModeCycleTws},
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetAmbientSoundModeCycle {
    pub cycle: AmbientSoundModeCycle,
}

impl ToPacket for SetAmbientSoundModeCycle {
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        packet::Command([0x06, 0x82])
    }

    fn body(&self) -> Vec<u8> {
        vec![self.cycle.into()]
    }
}

pub fn set_ambient_sound_mode_cycle_tws(tws_cycle: AmbientSoundModeCycleTws) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0x06, 0x82]), vec![u8::from(tws_cycle)])
}
