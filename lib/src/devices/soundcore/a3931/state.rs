use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    packet::inbound::SerialNumberAndFirmwareVersionUpdatePacket,
    structures::{
        AutoPowerOff, DualBattery, DualFirmwareVersion, EqualizerConfiguration,
        MultiButtonConfiguration, SerialNumber, SoundModes, TouchTone, TwsStatus,
    },
};

use super::packets::A3931StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3931State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    button_configuration: MultiButtonConfiguration,
    sound_modes: SoundModes,
    auto_power_off: AutoPowerOff,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
    touch_tone: TouchTone,
    #[has(skip)]
    side_tone: bool,
}

impl A3931State {
    pub fn new(
        state_update_packet: A3931StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket,
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

    pub fn update_from_state_update_packet(&mut self, packet: A3931StateUpdatePacket) {
        let A3931StateUpdatePacket {
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
