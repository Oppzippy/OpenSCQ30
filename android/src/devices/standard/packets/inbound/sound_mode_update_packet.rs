use crate::{
    devices::standard::structures::{AmbientSoundMode, NoiseCancelingMode},
    CustomNoiseCanceling, TransparencyMode,
};
use openscq30_lib::devices::standard::packets::inbound::SoundModeUpdatePacket as LibSoundModeUpdatePacket;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

#[generate_interface_doc]
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

    #[generate_interface]
    pub fn transparency_mode(&self) -> TransparencyMode {
        self.0.transparency_mode.into()
    }

    #[generate_interface]
    pub fn custom_noise_canceling(&self) -> CustomNoiseCanceling {
        self.0.custom_noise_canceling.into()
    }
}

impl From<LibSoundModeUpdatePacket> for SoundModeUpdatePacket {
    fn from(packet: LibSoundModeUpdatePacket) -> Self {
        Self(packet)
    }
}
