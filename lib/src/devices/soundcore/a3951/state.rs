use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    packet::inbound::SerialNumberAndFirmwareVersion,
    structures::{
        AgeRange, CustomHearId, DualBattery, DualFirmwareVersion, EqualizerConfiguration, Gender,
        MultiButtonConfiguration, SerialNumber, SoundModes, TouchTone, TwsStatus,
    },
};

use super::packets::A3951StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3951State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId<2, 8>,
    button_configuration: MultiButtonConfiguration,
    sound_modes: SoundModes,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
    touch_tone: TouchTone,
    #[has(skip)]
    side_tone: bool,
    #[has(skip)]
    wear_detection: bool,
    #[has(skip)]
    hear_id_eq_preset: Option<u16>,
    #[has(skip)]
    supports_new_battery: bool, // yes if packet is >98, don't parse
    #[has(skip)]
    left_new_battery: u8, // 0 to 9
    #[has(skip)]
    right_new_battery: u8, // 0 to 9
}

impl A3951State {
    pub fn new(
        state_update_packet: A3951StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersion,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            gender: state_update_packet.gender,
            age_range: state_update_packet.age_range,
            custom_hear_id: state_update_packet.custom_hear_id,
            button_configuration: state_update_packet.button_configuration,
            sound_modes: state_update_packet.sound_modes,
            side_tone: state_update_packet.side_tone,
            wear_detection: state_update_packet.wear_detection,
            touch_tone: state_update_packet.touch_tone,
            hear_id_eq_preset: state_update_packet.hear_id_eq_preset,
            supports_new_battery: state_update_packet.supports_new_battery,
            left_new_battery: state_update_packet.left_new_battery,
            right_new_battery: state_update_packet.right_new_battery,
            serial_number: sn_and_firmware.serial_number,
            dual_firmware_version: sn_and_firmware.dual_firmware_version,
        }
    }

    pub fn update_from_state_update_packet(&mut self, packet: A3951StateUpdatePacket) {
        let A3951StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            custom_hear_id,
            button_configuration,
            sound_modes,
            side_tone,
            wear_detection,
            touch_tone,
            hear_id_eq_preset,
            supports_new_battery,
            left_new_battery,
            right_new_battery,
        } = packet;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.gender = gender;
        self.age_range = age_range;
        self.custom_hear_id = custom_hear_id;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.side_tone = side_tone;
        self.wear_detection = wear_detection;
        self.touch_tone = touch_tone;
        self.hear_id_eq_preset = hear_id_eq_preset;
        self.supports_new_battery = supports_new_battery;
        self.left_new_battery = left_new_battery;
        self.right_new_battery = right_new_battery;
    }
}
