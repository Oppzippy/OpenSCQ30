use openscq30_lib_macros::Has;

use crate::{
    devices::soundcore::standard::{
        packet::inbound::SerialNumberAndFirmwareVersionUpdatePacket,
        structures::{
            DualBattery, DualFirmwareVersion, EqualizerConfiguration, MultiButtonConfiguration,
            SerialNumber, SoundModes, TwsStatus,
        },
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3031StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3031State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub serial_number: SerialNumber,
    pub dual_firmware_version: DualFirmwareVersion,
    #[has(skip)]
    pub side_tone: bool,
    #[has(skip)]
    pub touch_tone: bool,
    #[has(skip)]
    pub auto_power_off_on: bool,
    #[has(skip)]
    pub auto_power_off_on_index: u8,
}

impl_as_ref_for_field!(
    struct A3031State {
        tws_status: TwsStatus,
        battery: DualBattery,
        sound_modes: SoundModes,
        equalizer_configuration: EqualizerConfiguration<2, 8>,
        button_configuration: MultiButtonConfiguration,
        serial_number: SerialNumber,
        dual_firmware_version: DualFirmwareVersion,
    }
);

impl A3031State {
    pub fn new(
        state_update_packet: A3031StateUpdatePacket,
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
            auto_power_off_on_index: state_update_packet.auto_power_off_on_index,
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
            auto_power_off_on,
            auto_power_off_on_index,
        } = packet;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.side_tone = side_tone;
        self.touch_tone = touch_tone;
        self.auto_power_off_on = auto_power_off_on;
        self.auto_power_off_on_index = auto_power_off_on_index;
    }
}
