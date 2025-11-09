use crate::devices::soundcore::common::{
    packet::{self, Command},
    structures::GamingMode,
};

pub fn set_gaming_mode(gaming_mode: &GamingMode) -> packet::Outbound {
    packet::Outbound::new(Command([0x10, 0x85]), gaming_mode.bytes().to_vec())
}
