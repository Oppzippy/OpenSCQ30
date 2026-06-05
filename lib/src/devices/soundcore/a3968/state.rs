use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3968,
    common::structures::{DualBattery, DualFirmwareVersion, SerialNumber, TwsStatus},
};

use super::packets::inbound::A3968StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3968State {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub sound_modes: a3968::structures::SoundModes,
}

impl From<A3968StateUpdatePacket> for A3968State {
    fn from(value: A3968StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            sound_modes: value.sound_modes,
        }
    }
}
