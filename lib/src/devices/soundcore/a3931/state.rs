use crate::{
    devices::soundcore::standard::{
        packets::inbound::SerialNumberAndFirmwareVersionUpdatePacket,
        structures::{
            DualBattery, DualFirmwareVersion, EqualizerConfiguration, MultiButtonConfiguration,
            SerialNumber, SoundModes, TwsStatus,
        },
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3931StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3931State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    button_configuration: MultiButtonConfiguration,
    sound_modes: SoundModes,
    side_tone: bool,
    touch_tone: bool,
    auto_power_off_on: bool,
    auto_power_off_index: u8, // 0 to 3
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
}

impl_as_ref_for_field!(
    struct A3931State {
        tws_status: TwsStatus,
        battery: DualBattery,
        equalizer_configuration: EqualizerConfiguration<2, 8>,
        button_configuration: MultiButtonConfiguration,
        sound_modes: SoundModes,
        serial_number: SerialNumber,
        dual_firmware_version: DualFirmwareVersion,
    }
);

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
            auto_power_off_on: state_update_packet.auto_power_off_on,
            auto_power_off_index: state_update_packet.auto_power_off_index,
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
            auto_power_off_on,
            auto_power_off_index,
        } = packet;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.side_tone = side_tone;
        self.touch_tone = touch_tone;
        self.auto_power_off_on = auto_power_off_on;
        self.auto_power_off_index = auto_power_off_index;
    }
}
