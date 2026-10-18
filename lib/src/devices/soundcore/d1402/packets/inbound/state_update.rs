use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff,
            CommonEqualizerConfiguration, CustomHearId, EqualizerConfiguration, FirmwareVersion,
            Ldac, LimitHighVolume, LowBatteryPrompt, SerialNumber, SingleBattery, SurroundSound,
        },
    },
    d1402::{self, packets::outbound, state::D1402State},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct D1402StateUpdatePacket {
    pub battery: SingleBattery,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub equalizer_unknown_byte: d1402::structures::EqualizerUnknownByte,
    /// Index into the official app's list of age ranges (13-19, 20-29, 30-39, 40-49, 50-59,
    /// 60-69, 70+), set by its hear id test. 255 means not set.
    pub age_range: AgeRange,
    pub hear_id: CustomHearId<1, 10>,
    pub button_configuration: d1402::structures::ButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    /// Not configurable in the official app's UI for us to expose, so it's only parsed.
    pub mfb_long_press_action: u8,
    pub sound_modes: d1402::structures::SoundModes,
    pub three_d_sound: SurroundSound,
    pub ambient_sound_mode_voice_prompt: d1402::structures::AmbientSoundModeVoicePrompt,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub auto_play_pause: AutoPlayPause,
    pub hey_anka_enabled: bool,
}

impl FromPacketBody for D1402StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1402 state update packet",
            map(
                (
                    SingleBattery::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    (
                        d1402::structures::EqualizerUnknownByte::take,
                        AgeRange::take,
                    ),
                    CustomHearId::take_with_music_genre_at_end,
                    take(2usize), // unknown
                    d1402::structures::ButtonConfiguration::take,
                    take(1usize), // unknown
                    AmbientSoundModeCycle::take,
                    le_u8, // mfb long press action
                    d1402::structures::SoundModes::take,
                    take(1usize), // unknown
                    SurroundSound::take,
                    take(2usize), // unknown
                    d1402::structures::AmbientSoundModeVoicePrompt::take,
                    LowBatteryPrompt::take,
                    Ldac::take,
                    take_bool, // dual connections
                    AutoPowerOff::take,
                    (
                        LimitHighVolume::take,
                        take(3usize), // audio streaming status and unknown
                        AutoPlayPause::take,
                        take_bool,    // hey anka
                        take(8usize), // unknown
                    ),
                ),
                |(
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    (equalizer_unknown_byte, age_range),
                    hear_id,
                    _unknown1,
                    button_configuration,
                    _unknown2,
                    ambient_sound_mode_cycle,
                    mfb_long_press_action,
                    sound_modes,
                    _unknown3,
                    three_d_sound,
                    _unknown4,
                    ambient_sound_mode_voice_prompt,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    (limit_high_volume, _unknown5, auto_play_pause, hey_anka_enabled, _unknown6),
                )| Self {
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    equalizer_unknown_byte,
                    age_range,
                    hear_id,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    mfb_long_press_action,
                    sound_modes,
                    three_d_sound,
                    ambient_sound_mode_voice_prompt,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    auto_play_pause,
                    hey_anka_enabled,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1402StateUpdatePacket {
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
            .chain([self.equalizer_unknown_byte.0, self.age_range.0])
            .chain(outbound::hear_id_bytes_with_music_genre_at_end(
                &self.hear_id,
            ))
            .chain([0; 2]) // unknown
            .chain(self.button_configuration.bytes())
            .chain([0; 1]) // unknown
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain([self.mfb_long_press_action])
            .chain(self.sound_modes.bytes())
            .chain([0; 1]) // unknown
            .chain(self.three_d_sound.bytes())
            .chain([0; 2]) // unknown
            .chain(self.ambient_sound_mode_voice_prompt.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain([self.dual_connections_enabled.into()])
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain([0; 3]) // audio streaming status and unknown
            .chain(self.auto_play_pause.bytes())
            .chain([self.hey_anka_enabled.into()])
            .chain([0; 8]) // unknown
            .collect()
    }
}

state_update_packet_module!(D1402State, D1402StateUpdatePacket);
