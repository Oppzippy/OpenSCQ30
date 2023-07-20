use crate::packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode};
use openscq30_lib::packets::inbound::StateUpdatePacket as LibStateUpdatePacket;
use rifgen::rifgen_attr::generate_interface;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket(LibStateUpdatePacket);

impl StateUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<StateUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.0.ambient_sound_mode().into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.0.noise_canceling_mode().into()
    }

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.0.equalizer_configuration().into()
    }
}

impl From<LibStateUpdatePacket> for StateUpdatePacket {
    fn from(packet: LibStateUpdatePacket) -> Self {
        Self(packet)
    }
}

impl From<StateUpdatePacket> for openscq30_lib::packets::inbound::StateUpdatePacket {
    fn from(value: StateUpdatePacket) -> Self {
        value.0
    }
}
