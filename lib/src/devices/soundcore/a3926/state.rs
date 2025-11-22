use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    packet::inbound::SerialNumberAndFirmwareVersion,
    state::Update,
    structures::{
        AgeRange, BasicHearId, CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion,
        Gender, SerialNumber, TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::A3926StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3926State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: CommonEqualizerConfiguration<2, 8>,
    gender: Gender,
    age_range: AgeRange,
    hear_id: BasicHearId<2, 8>,
    button_configuration: ButtonStatusCollection<6>,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3926State {
    pub fn new(
        state_update_packet: A3926StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersion,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            gender: state_update_packet.gender,
            age_range: state_update_packet.age_range,
            hear_id: state_update_packet.hear_id,
            button_configuration: state_update_packet.button_configuration,
            serial_number: sn_and_firmware.serial_number,
            dual_firmware_version: sn_and_firmware.dual_firmware_version,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3926StateUpdatePacket> for A3926State {
    fn update(&mut self, partial: A3926StateUpdatePacket) {
        let A3926StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            hear_id,
            button_configuration,
        } = partial;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.gender = gender;
        self.age_range = age_range;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
    }
}
