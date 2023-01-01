use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

#[generate_interface]
pub enum NoiseCancelingMode {
    Transport,
    Outdoor,
    Indoor,
}

impl From<structures::NoiseCancelingMode> for NoiseCancelingMode {
    fn from(value: structures::NoiseCancelingMode) -> Self {
        match value {
            structures::NoiseCancelingMode::Transport => NoiseCancelingMode::Transport,
            structures::NoiseCancelingMode::Outdoor => NoiseCancelingMode::Outdoor,
            structures::NoiseCancelingMode::Indoor => NoiseCancelingMode::Indoor,
        }
    }
}

impl From<NoiseCancelingMode> for structures::NoiseCancelingMode {
    fn from(value: NoiseCancelingMode) -> Self {
        match value {
            NoiseCancelingMode::Transport => structures::NoiseCancelingMode::Transport,
            NoiseCancelingMode::Outdoor => structures::NoiseCancelingMode::Outdoor,
            NoiseCancelingMode::Indoor => structures::NoiseCancelingMode::Indoor,
        }
    }
}
