use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3909,
    common::{
        self,
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        packet::inbound::SerialNumberAndFirmwareVersion,
        structures::{
            AgeRange, DualBattery, DualFirmwareVersion, Gender, SerialNumber, TwsStatus,
            button_configuration::ButtonStatusCollection,
        },
    },
};

use super::packets::inbound::A3909StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3909State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: a3909::structures::EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    hear_id: a3909::structures::HearId,
    buttons: ButtonStatusCollection<4>,
    button_reset_pending: ResetButtonConfigurationPending,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
}

impl A3909State {
    pub fn new(
        state_update_packet: A3909StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersion,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            gender: state_update_packet.gender,
            age_range: state_update_packet.age_range,
            hear_id: state_update_packet.hear_id,
            buttons: state_update_packet.buttons,
            button_reset_pending: ResetButtonConfigurationPending::default(),
            firmware_version: sn_and_firmware.dual_firmware_version,
            serial_number: sn_and_firmware.serial_number,
        }
    }
}

impl common::state::Update<A3909StateUpdatePacket> for A3909State {
    fn update(&mut self, partial: A3909StateUpdatePacket) {
        let A3909StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            hear_id,
            buttons,
        } = partial;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.gender = gender;
        self.age_range = age_range;
        self.hear_id = hear_id;
        self.buttons = buttons;
    }
}
