use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3134::{
        packets::inbound::A3134StateUpdatePacket,
        structures::{self, A3134EqualizerConfiguration, BassUp, Lights},
    },
    common::structures::{FirmwareVersion, SerialNumber, SingleBattery},
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3134State {
    battery: SingleBattery,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    bass_up: BassUp,
    equalizer_configuration: A3134EqualizerConfiguration,
    lights: Lights,
}

impl From<A3134StateUpdatePacket> for A3134State {
    fn from(packet: A3134StateUpdatePacket) -> Self {
        let A3134StateUpdatePacket {
            battery,
            firmware_version,
            serial_number,
            bass_up,
            equalizer_configuration,
            lights,
        } = packet;
        Self {
            battery,
            firmware_version,
            serial_number,
            bass_up,
            equalizer_configuration: structures::effective_equalizer_configuration(
                equalizer_configuration,
            ),
            lights,
        }
    }
}
