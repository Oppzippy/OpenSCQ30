use crate::devices::soundcore::{a3947, common::packet};

pub fn set_sound_modes(sound_modes: &a3947::structures::SoundModes) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0x06, 0x81]), sound_modes.bytes().to_vec())
}
