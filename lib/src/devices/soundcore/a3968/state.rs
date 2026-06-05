use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3959,
    common::structures::{DualBattery, DualFirmwareVersion, SerialNumber},
};

use super::packets::inbound::A3968StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3968State {
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    // The X20 uses the same sound-mode structure as the A3959 (P30i).
    pub sound_modes: a3959::structures::SoundModes,
}

impl From<A3968StateUpdatePacket> for A3968State {
    fn from(value: A3968StateUpdatePacket) -> Self {
        Self {
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            sound_modes: value.sound_modes,
        }
    }
}
