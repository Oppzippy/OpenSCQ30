use crate::devices::soundcore::{a3116, common::packet};

pub fn set_auto_power_off(duration: &a3116::structures::AutoPowerOffDuration) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0, 0]), duration.bytes().collect())
}

pub fn set_volume(volume: &a3116::structures::Volume) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0, 0]), volume.bytes().collect())
}
