use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

#[generate_interface]
pub enum AmbientSoundMode {
    NoiseCanceling,
    Transparency,
    Normal,
}

impl From<structures::AmbientSoundMode> for AmbientSoundMode {
    fn from(value: structures::AmbientSoundMode) -> Self {
        match value {
            structures::AmbientSoundMode::NoiseCanceling => AmbientSoundMode::NoiseCanceling,
            structures::AmbientSoundMode::Transparency => AmbientSoundMode::Transparency,
            structures::AmbientSoundMode::Normal => AmbientSoundMode::Normal,
        }
    }
}

impl From<AmbientSoundMode> for structures::AmbientSoundMode {
    fn from(value: AmbientSoundMode) -> Self {
        match value {
            AmbientSoundMode::NoiseCanceling => structures::AmbientSoundMode::NoiseCanceling,
            AmbientSoundMode::Transparency => structures::AmbientSoundMode::Transparency,
            AmbientSoundMode::Normal => structures::AmbientSoundMode::Normal,
        }
    }
}
