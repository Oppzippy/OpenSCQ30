use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::structures::{BatteryLevel, IsBatteryCharging, SerialNumber, SingleBattery},
    d5103::{
        packets::inbound::D5103StateUpdatePacket,
        structures::{
            BassUp, D5103EqualizerConfiguration, D5103FirmwareVersion, Lights,
            effective_equalizer_configuration,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D5103State {
    battery: SingleBattery,
    firmware_version: D5103FirmwareVersion,
    serial_number: SerialNumber,
    bass_up: BassUp,
    equalizer_configuration: D5103EqualizerConfiguration,
    lights: Lights,
}

impl D5103State {
    pub fn new(packet: D5103StateUpdatePacket) -> Self {
        Self {
            battery: SingleBattery {
                is_charging: IsBatteryCharging::No,
                level: BatteryLevel(packet.battery_level.min(100)),
            },
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number,
            bass_up: packet.bass_up,
            equalizer_configuration: effective_equalizer_configuration(
                packet.equalizer_configuration,
            ),
            lights: packet.lights,
        }
    }
}

impl crate::devices::soundcore::common::state::Update<D5103StateUpdatePacket> for D5103State {
    fn update(&mut self, partial: D5103StateUpdatePacket) {
        let D5103StateUpdatePacket {
            battery_level,
            firmware_version,
            serial_number,
            bass_up,
            equalizer_configuration,
            lights,
            ..
        } = partial;

        self.battery = SingleBattery {
            is_charging: IsBatteryCharging::No,
            level: BatteryLevel(battery_level.min(100)),
        };
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.bass_up = bass_up;
        self.equalizer_configuration = effective_equalizer_configuration(equalizer_configuration);
        self.lights = lights;
    }
}
