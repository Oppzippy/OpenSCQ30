use super::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode};
use openscq30_lib::packets::structures::SoundModes as LibSoundModes;
use rifgen::rifgen_attr::generate_interface;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SoundModes(LibSoundModes);

impl SoundModes {
    #[generate_interface(constructor)]
    pub fn new(
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
        transparency_mode: TransparencyMode,
        custom_noise_canceling: CustomNoiseCanceling,
    ) -> SoundModes {
        Self(LibSoundModes {
            ambient_sound_mode: ambient_sound_mode.into(),
            noise_canceling_mode: noise_canceling_mode.into(),
            transparency_mode: transparency_mode.into(),
            custom_noise_canceling: custom_noise_canceling.into(),
        })
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

    #[generate_interface]
    pub fn inner_equals(&self, other: &SoundModes) -> bool {
        self == other
    }
}

impl From<LibSoundModes> for SoundModes {
    fn from(value: LibSoundModes) -> Self {
        Self(value)
    }
}
