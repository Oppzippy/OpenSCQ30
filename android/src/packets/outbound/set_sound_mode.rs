use openscq30_lib::packets::outbound::OutboundPacket as _;
use rifgen::rifgen_attr::generate_interface;

use crate::{
    packets::structures::{AmbientSoundMode, NoiseCancelingMode},
    type_conversion,
};

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetSoundModePacket {
    packet: openscq30_lib::packets::outbound::SetSoundModePacket,
}

impl SetSoundModePacket {
    #[generate_interface(constructor)]
    pub fn new(
        ambient_sound_mode: &AmbientSoundMode,
        noise_canceling_mode: &NoiseCancelingMode,
    ) -> SetSoundModePacket {
        Self {
            packet: openscq30_lib::packets::outbound::SetSoundModePacket {
                ambient_sound_mode: ambient_sound_mode.to_owned().into(),
                noise_canceling_mode: noise_canceling_mode.to_owned().into(),
                transparency_mode: Default::default(),
                custom_noise_canceling: Default::default(),
            },
        }
    }
}

impl OutboundPacket for SetSoundModePacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
