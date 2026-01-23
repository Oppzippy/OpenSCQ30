use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3116::{self, packets::inbound::VoicePromptUpdatePacket},
    common::{
        state::Update,
        structures::{
            EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery, VoicePrompt,
        },
    },
};

use super::packets::inbound::A3116StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3116State {
    battery: SingleBattery,
    volume: a3116::structures::Volume,
    auto_power_off_duration: a3116::structures::AutoPowerOffDuration,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<1, 9, -6, 6, 0>,
    #[has(maybe)]
    voice_prompt: Option<VoicePrompt>,
    power_off_pending: a3116::structures::PowerOffPending,
}

impl A3116State {
    pub fn new(
        state_update_packet: A3116StateUpdatePacket,
        voice_prompt_packet: Option<VoicePromptUpdatePacket>,
    ) -> Self {
        Self {
            battery: state_update_packet.battery,
            volume: state_update_packet.volume,
            auto_power_off_duration: state_update_packet.auto_power_off_duration,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            voice_prompt: voice_prompt_packet.map(|packet| packet.voice_prompt),
            power_off_pending: Default::default(),
        }
    }
}

impl Update<A3116StateUpdatePacket> for A3116State {
    fn update(&mut self, partial: A3116StateUpdatePacket) {
        let A3116StateUpdatePacket {
            battery,
            volume,
            auto_power_off_duration,
            firmware_version,
            serial_number,
            equalizer_configuration,
        } = partial;
        self.battery = battery;
        self.volume = volume;
        self.auto_power_off_duration = auto_power_off_duration;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
    }
}
