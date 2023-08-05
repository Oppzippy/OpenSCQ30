use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};
use openscq30_lib::packets::inbound::SoundModeUpdatePacket as LibSoundModeUpdatePacket;
use rifgen::rifgen_attr::generate_interface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundModeUpdatePacket(LibSoundModeUpdatePacket);

impl SoundModeUpdatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<SoundModeUpdatePacket, String> {
        Err("do not construct directly".to_string())
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.0.ambient_sound_mode.into()
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.0.noise_canceling_mode.into()
    }
}

impl From<LibSoundModeUpdatePacket> for SoundModeUpdatePacket {
    fn from(packet: LibSoundModeUpdatePacket) -> Self {
        Self(packet)
    }
}