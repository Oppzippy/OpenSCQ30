use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    packet::inbound::SerialNumberAndFirmwareVersion,
    state::Update,
    structures::{
        AgeRange, CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion, Gender,
        SerialNumber, SoundModes, TwsStatus,
    },
};

use super::packets::A3909StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3909State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: CommonEqualizerConfiguration<2, 8>,
    serial_number: SerialNumber,
    dual_firmware_version: DualFirmwareVersion,
    #[has(skip)]
    gender: Gender,
    #[has(skip)]
    age_range: AgeRange,
    #[has(skip)]
    hear_id_is_enabled: bool,
    #[has(skip)]
    hear_id_time: u32,
    #[has(skip)]
    sound_modes: SoundModes,
    #[has(skip)]
    side_tone: bool,
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
            hear_id_is_enabled: state_update_packet.hear_id_is_enabled,
            hear_id_time: state_update_packet.hear_id_time,
            sound_modes: state_update_packet.sound_modes,
            side_tone: state_update_packet.side_tone,
            serial_number: sn_and_firmware.serial_number,
            dual_firmware_version: sn_and_firmware.dual_firmware_version,
        }
    }
}

impl Update<A3909StateUpdatePacket> for A3909State {
    fn update(&mut self, partial: A3909StateUpdatePacket) {
        let A3909StateUpdatePacket {
            tws_status,
            battery,
            equalizer_configuration,
            gender,
            age_range,
            hear_id_is_enabled,
            hear_id_time,
            sound_modes,
            side_tone,
        } = partial;
        self.tws_status = tws_status;
        self.battery = battery;
        self.equalizer_configuration = equalizer_configuration;
        self.gender = gender;
        self.age_range = age_range;
        self.hear_id_is_enabled = hear_id_is_enabled;
        self.hear_id_time = hear_id_time;
        self.sound_modes = sound_modes;
        self.side_tone = side_tone;
    }
}
