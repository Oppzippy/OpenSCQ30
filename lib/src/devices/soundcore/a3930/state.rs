use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    packet::inbound::SerialNumberAndFirmwareVersion,
    structures::{
        AgeRange, CustomHearId, DualBattery, DualFirmwareVersion, EqualizerConfiguration, Gender,
        SerialNumber, SoundModes, TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::A3930StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3930State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId<2, 8>,
    button_configuration: ButtonStatusCollection<6>,
    sound_modes: SoundModes,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
    #[has(skip)]
    side_tone: bool,
    #[has(skip)]
    hear_id_eq_index: Option<u16>,
}

impl A3930State {
    pub fn new(
        state_update_packet: A3930StateUpdatePacket,
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
            hear_id_eq_index: state_update_packet.hear_id_eq_index,
            serial_number: sn_and_firmware.serial_number,
            dual_firmware_version: sn_and_firmware.dual_firmware_version,
        }
    }

    pub fn update_from_state_update_packet(&mut self, packet: A3930StateUpdatePacket) {
        let A3930StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            custom_hear_id,
            button_configuration,
            sound_modes,
            side_tone,
            hear_id_eq_index,
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
        self.hear_id_eq_index = hear_id_eq_index;
    }
}
