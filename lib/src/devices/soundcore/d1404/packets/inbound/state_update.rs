use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::{le_f32, le_u8, le_u16},
};

use crate::devices::soundcore::{
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, FirmwareVersion, Ldac, LowBatteryPrompt,
            SerialNumber, SingleBattery, SurroundSound, VolumeAdjustments,
        },
    },
    d1404::{
        self,
        packets::outbound,
        state::D1404State,
        structures::{
            ButtonConfiguration, D1404EqualizerConfiguration, MfbButtonConfiguration, SoundModes,
            decibels_to_adjustment,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1404StateUpdatePacket {
    pub battery: SingleBattery,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: D1404EqualizerConfiguration,
    pub button_configuration: d1404::structures::ButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub mfb_button_configuration: d1404::structures::MfbButtonConfiguration,
    pub sound_modes: d1404::structures::SoundModes,
    /// 3D sound. The device only allows it with LDAC off, see the ldac_and_three_d_sound
    /// module.
    pub surround_sound: SurroundSound,
    pub ambient_sound_mode_voice_prompt: d1404::structures::AmbientSoundModeVoicePrompt,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    /// The device reports LDAC and multipoint (dual connections) as an exclusive pair.
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
}

impl Default for D1404StateUpdatePacket {
    /// Matches the values a real device sends for the parts that aren't stored by this app.
    fn default() -> Self {
        Self {
            battery: SingleBattery::default(),
            firmware_version: FirmwareVersion::default(),
            serial_number: SerialNumber::default(),
            equalizer_configuration: D1404EqualizerConfiguration::default(),
            button_configuration: ButtonConfiguration::default(),
            ambient_sound_mode_cycle: AmbientSoundModeCycle::default(),
            mfb_button_configuration: MfbButtonConfiguration::default(),
            sound_modes: SoundModes::default(),
            surround_sound: SurroundSound::default(),
            ambient_sound_mode_voice_prompt:
                d1404::structures::AmbientSoundModeVoicePrompt::default(),
            low_battery_prompt: LowBatteryPrompt::default(),
            ldac: Ldac::default(),
            dual_connections_enabled: false,
            auto_power_off: AutoPowerOff::default(),
        }
    }
}

/// The filter definitions mirroring the equalizer: a count byte, an unknown byte, then 13
/// bytes per filter. Presets carry their fixed filter set, custom profiles their band
/// filters. The equalizer configuration already carries everything this app needs, so the
/// contents are skipped.
fn take_filter_block<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], E> {
    let (input, filter_count) = le_u8(input)?;
    let (input, _unknown) = take(1usize).parse_complete(input)?;
    take(13usize * filter_count as usize).parse_complete(input)
}

impl FromPacketBody for D1404StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1404 state update packet",
            map(
                (
                    SingleBattery::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    le_u16, // equalizer preset id
                    count(le_f32, 8),
                    take(6usize), // unknown
                    take_filter_block,
                    take(1usize), // unknown
                    d1404::structures::ButtonConfiguration::take,
                    take(1usize), // unknown
                    AmbientSoundModeCycle::take,
                    MfbButtonConfiguration::take,
                    d1404::structures::SoundModes::take,
                    take(1usize), // unknown, 49 in every capture
                    SurroundSound::take,
                    take_bool, // ambient sound mode voice prompt
                    LowBatteryPrompt::take,
                    take_bool, // ldac
                    take_bool, // multipoint
                    AutoPowerOff::take,
                    // active bluetooth profile (0 = headphones, 2 = handsfree) plus the
                    // unknown flag region
                    take(10usize),
                ),
                |(
                    battery,
                    firmware_version,
                    serial_number,
                    preset_id,
                    band_decibels,
                    _unknown1,
                    _filters,
                    _unknown2,
                    button_configuration,
                    _unknown3,
                    ambient_sound_mode_cycle,
                    mfb_button_configuration,
                    sound_modes,
                    _unknown4,
                    surround_sound,
                    ambient_sound_mode_voice_prompt,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    _unknown5,
                )| {
                    let band_decibels: [f32; 8] = band_decibels
                        .try_into()
                        .expect("count guarantees exactly 8 elements");
                    let volume_adjustments = VolumeAdjustments::<8, -60, 60, 1>::new(
                        band_decibels.map(decibels_to_adjustment),
                    );
                    Self {
                        battery,
                        firmware_version,
                        serial_number,
                        equalizer_configuration: D1404EqualizerConfiguration::new_all_bands_present(
                            preset_id,
                            [volume_adjustments],
                        ),
                        button_configuration,
                        ambient_sound_mode_cycle,
                        mfb_button_configuration,
                        sound_modes,
                        surround_sound,
                        ambient_sound_mode_voice_prompt:
                            d1404::structures::AmbientSoundModeVoicePrompt(
                                ambient_sound_mode_voice_prompt,
                            ),
                        low_battery_prompt,
                        ldac: Ldac(ldac),
                        dual_connections_enabled,
                        auto_power_off,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1404StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        let band_decibels: [f32; 8] = self
            .equalizer_configuration
            .volume_adjustments_channel_1()
            .copied()
            .unwrap_or_default()
            .adjustments()
            .map(|adjustment| d1404::structures::adjustment_to_decibels(adjustment));

        self.battery
            .bytes()
            .into_iter()
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.preset_id().to_le_bytes())
            .chain(
                band_decibels
                    .iter()
                    .flat_map(|decibels| decibels.to_le_bytes()),
            )
            .chain([0; 6]) // unknown
            .chain(outbound::filter_section(&self.equalizer_configuration))
            .chain([5]) // unknown
            .chain(self.button_configuration.bytes())
            .chain([255]) // unknown
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.mfb_button_configuration.bytes())
            .chain(self.sound_modes.bytes())
            .chain([49]) // unknown
            .chain(self.surround_sound.bytes())
            .chain(self.ambient_sound_mode_voice_prompt.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain([self.dual_connections_enabled.into()])
            .chain(self.auto_power_off.bytes())
            .chain([0]) // active bluetooth profile: 0 = headphones (a2dp)
            .chain([1, 0, 0, 0, 0, 1, 1, 0, 1]) // unknown
            .collect()
    }
}

state_update_packet_module!(D1404State, D1404StateUpdatePacket);
