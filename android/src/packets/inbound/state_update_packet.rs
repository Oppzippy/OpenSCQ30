use rifgen::rifgen_attr::generate_interface;

use crate::{
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    type_conversion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket {
    packet: Option<openscq30_lib::packets::inbound::StateUpdatePacket>,
}

impl StateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<StateUpdatePacket, String> {
        Err("use from_bytes instead".to_string())
    }

    #[generate_interface]
    pub fn from_bytes(bytes: &[i8]) -> Result<Option<StateUpdatePacket>, String> {
        let bytes = type_conversion::i8_slice_to_u8_slice(bytes);
        Ok(
            openscq30_lib::packets::inbound::StateUpdatePacket::new(&bytes)
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

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.packet.unwrap().equalizer_configuration().into()
    }
}

impl From<openscq30_lib::packets::inbound::StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: openscq30_lib::packets::inbound::StateUpdatePacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}

impl From<StateUpdatePacket> for openscq30_lib::packets::inbound::StateUpdatePacket {
    fn from(value: StateUpdatePacket) -> Self {
        value.packet.unwrap()
    }
}
