use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    packet::inbound::SerialNumberAndFirmwareVersion,
    structures::{
        AutoPowerOff, DualBattery, DualFirmwareVersion, EqualizerConfiguration, SerialNumber,
        SoundModes, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::A3031StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3031State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub sound_modes: SoundModes,
    pub serial_number: SerialNumber,
    pub dual_firmware_version: DualFirmwareVersion,
    pub auto_power_off: AutoPowerOff,
    pub touch_tone: TouchTone,
    #[has(skip)]
    pub side_tone: bool,
}

impl A3031State {
    pub fn new(
        state_update_packet: A3031StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersion,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            button_configuration: state_update_packet.button_configuration,
            sound_modes: state_update_packet.sound_modes,
            side_tone: state_update_packet.side_tone,
            touch_tone: state_update_packet.touch_tone,
            auto_power_off: state_update_packet.auto_power_off,
            serial_number: sn_and_firmware.serial_number,
            dual_firmware_version: sn_and_firmware.dual_firmware_version,
        }
    }

    pub fn update_from_state_update_packet(&mut self, packet: A3031StateUpdatePacket) {
        let A3031StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            button_configuration,
            sound_modes,
            side_tone,
            touch_tone,
            auto_power_off,
        } = packet;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.side_tone = side_tone;
        self.touch_tone = touch_tone;
        self.auto_power_off = auto_power_off;
    }
}
