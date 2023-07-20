use openscq30_lib::packets::structures;
use rifgen::rifgen_attr::generate_interface;

#[generate_interface]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseCancelingMode {
    Transport,
    Outdoor,
    Indoor,
    Custom,
}

impl From<structures::NoiseCancelingMode> for NoiseCancelingMode {
    fn from(value: structures::NoiseCancelingMode) -> Self {
        match value {
            structures::NoiseCancelingMode::Transport => NoiseCancelingMode::Transport,
            structures::NoiseCancelingMode::Outdoor => NoiseCancelingMode::Outdoor,
            structures::NoiseCancelingMode::Indoor => NoiseCancelingMode::Indoor,
            structures::NoiseCancelingMode::Custom => NoiseCancelingMode::Custom,
        }
    }
}

impl From<NoiseCancelingMode> for structures::NoiseCancelingMode {
    fn from(value: NoiseCancelingMode) -> Self {
        match value {
            NoiseCancelingMode::Transport => structures::NoiseCancelingMode::Transport,
            NoiseCancelingMode::Outdoor => structures::NoiseCancelingMode::Outdoor,
            NoiseCancelingMode::Indoor => structures::NoiseCancelingMode::Indoor,
            NoiseCancelingMode::Custom => structures::NoiseCancelingMode::Custom,
        }
    }
}
