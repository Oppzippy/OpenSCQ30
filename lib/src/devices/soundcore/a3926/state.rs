use crate::{
    devices::soundcore::standard::{
        packets::inbound::SerialNumberAndFirmwareVersionUpdatePacket,
        structures::{
            AgeRange, BasicHearId, DualBattery, DualFirmwareVersion, EqualizerConfiguration,
            Gender, MultiButtonConfiguration, SerialNumber, TwsStatus,
        },
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3926StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3926State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration<2, 8>,
    gender: Gender,
    age_range: AgeRange,
    hear_id: BasicHearId<2, 8>,
    button_configuration: MultiButtonConfiguration,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
}

impl_as_ref_for_field!(
    struct A3926State {
        tws_status: TwsStatus,
        battery: DualBattery,
        equalizer_configuration: EqualizerConfiguration<2, 8>,
        gender: Gender,
        age_range: AgeRange,
        hear_id: BasicHearId<2, 8>,
        button_configuration: MultiButtonConfiguration,
        serial_number: SerialNumber,
        dual_firmware_version: DualFirmwareVersion,
    }
);

impl A3926State {
    pub fn new(
        state_update_packet: A3926StateUpdatePacket,
        sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket,
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
        }
    }

    pub fn update_from_state_update_packet(&mut self, packet: A3926StateUpdatePacket) {
        let A3926StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            hear_id,
            button_configuration,
        } = packet;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.gender = gender;
        self.age_range = age_range;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
    }
}
