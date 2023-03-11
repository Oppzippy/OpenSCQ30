use rifgen::rifgen_attr::generate_interface;

use crate::{
    packets::structures::{AmbientSoundMode, NoiseCancelingMode},
    type_conversion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmbientSoundModeUpdatePacket {
    packet: Option<openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket>,
}

impl AmbientSoundModeUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<AmbientSoundModeUpdatePacket, String> {
        Err("use from_bytes instead".to_string())
    }

    #[generate_interface]
    pub fn from_bytes(bytes: &[i8]) -> Result<Option<AmbientSoundModeUpdatePacket>, String> {
        let bytes = type_conversion::i8_slice_to_u8_slice(bytes);
        Ok(
            openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket::new(bytes)
                .map(|packet| packet.into()),
        )
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.packet.unwrap().ambient_sound_mode().into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.packet.unwrap().noise_canceling_mode().into()
    }
}

impl From<openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket>
    for AmbientSoundModeUpdatePacket
{
    fn from(packet: openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}
