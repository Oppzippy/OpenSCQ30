use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3062::{self, state::A3062State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AmbientSoundModeCycle, AutoPowerOff, CommonEqualizerConfiguration, CustomHearId,
                EqualizerConfiguration, FirmwareVersion, Ldac, LimitHighVolume, LowBatteryPrompt,
                SerialNumber, SingleBattery,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3062StateUpdatePacket {
    pub battery: SingleBattery,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub hear_id: CustomHearId<1, 10>,
    pub button_configuration: a3062::structures::ButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3062::structures::SoundModes,
    pub low_battery_prompt: LowBatteryPrompt,
    pub dolby_audio: a3062::structures::DolbyAudio,
    pub ldac: Ldac,
    pub dual_connections: bool,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub side_tone: a3062::structures::SideTone,
    pub ambient_sound_mode_voice_prompt: a3062::structures::AmbientSoundModeVoicePrompt,
}

impl FromPacketBody for A3062StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3062 state update packet",
            map(
                (
                    SingleBattery::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    take(2usize), // unknown
                    CustomHearId::take_with_music_genre_at_end,
                    take(2usize), // unknown
                    a3062::structures::ButtonConfiguration::take,
                    AmbientSoundModeCycle::take,
                    a3062::structures::SoundModes::take,
                    take(1usize), // unknown
                    LowBatteryPrompt::take,
                    a3062::structures::DolbyAudio::take,
                    Ldac::take,
                    take_bool, // dual connections
                    AutoPowerOff::take,
                    LimitHighVolume::take,
                    a3062::structures::SideTone::take,
                    a3062::structures::AmbientSoundModeVoicePrompt::take,
                ),
                |(
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown1,
                    hear_id,
                    _unknown2,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown3,
                    low_battery_prompt,
                    dolby_audio,
                    ldac,
                    dual_connections,
                    auto_power_off,
                    limit_high_volume,
                    side_tone,
                    ambient_sound_mode_voice_prompt,
                )| Self {
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    hear_id,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    low_battery_prompt,
                    dolby_audio,
                    ldac,
                    dual_connections,
                    auto_power_off,
                    limit_high_volume,
                    side_tone,
                    ambient_sound_mode_voice_prompt,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3062StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.battery
            .bytes()
            .into_iter()
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([0; 2]) // unknown
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain([0; 2]) // unknown
            .chain(self.button_configuration.bytes())
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 1]) // unknown
            .chain(self.low_battery_prompt.bytes())
            .chain(self.dolby_audio.bytes())
            .chain(self.ldac.bytes())
            .chain([0]) // dual connections
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.side_tone.bytes())
            .chain(self.ambient_sound_mode_voice_prompt.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<A3062State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3062State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3062StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3062State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
