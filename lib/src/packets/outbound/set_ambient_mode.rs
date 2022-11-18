use crate::packets::structures::{
    ambient_sound_mode::AmbientSoundMode, noise_canceling_mode::NoiseCancelingMode,
};

use super::{outbound_packet::OutboundPacket, utils::calculate_checksum};

pub struct SetAmbientSoundModePacket {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
}

impl SetAmbientSoundModePacket {
    pub fn new(
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Self {
        SetAmbientSoundModePacket {
            ambient_sound_mode,
            noise_canceling_mode,
        }
    }
}

impl OutboundPacket for SetAmbientSoundModePacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![
            0x08,
            0xee,
            0x00,
            0x00,
            0x00,
            0x06,
            0x81,
            0x0e,
            0x00,
            self.ambient_sound_mode.id(),
            self.noise_canceling_mode.id(),
            0x01,
            0x00,
        ];
        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}
