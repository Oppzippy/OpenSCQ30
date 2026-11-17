use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, rest},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AutoPowerOff, CaseBatteryLevel, DualBattery, DualFirmwareVersion, GamingMode, Ldac,
            LimitHighVolume, LowBatteryPrompt, SerialNumber, TouchTone, TwsStatus,
            WearingDetection, WearingTone, button_configuration::ButtonStatusCollection,
        },
    },
    d1205::{
        self,
        state::D1205State,
        structures::{EqualizerBlock, SoundModes, SpatialAudio},
    },
};

/// State update packet of the soundcore P42i (D1205).
///
/// Apart from the equalizer block, the layout follows the D1202. Bytes without a known meaning are kept so the packet
/// can be serialized again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1205StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    /// Most likely the case firmware version, e.g. "06.72"
    pub unknown_case_firmware: [u8; 5],
    pub case_battery_level: CaseBatteryLevel,
    pub equalizer: EqualizerBlock,
    pub unknown_before_buttons: u8,
    pub button_configuration: ButtonStatusCollection<8>,
    pub unknown_after_buttons: u8,
    pub sound_modes: SoundModes,
    pub unknown_after_sound_modes: u8,
    pub touch_tone: TouchTone,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
    /// Same position as on the D1202, but there is no setting for it in the Soundcore app
    pub limit_high_volume: LimitHighVolume,
    pub spatial_audio: SpatialAudio,
    pub unknown_after_spatial_audio: [u8; 2],
    pub wearing_detection: WearingDetection,
    pub wearing_tone: WearingTone,
    pub unknown_after_wearing_tone: [u8; 7],
    pub gaming_mode: GamingMode,
    pub unknown_tail: Vec<u8>,
}

impl Default for D1205StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            unknown_case_firmware: Default::default(),
            case_battery_level: Default::default(),
            equalizer: Default::default(),
            unknown_before_buttons: Default::default(),
            button_configuration: d1205::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            unknown_after_buttons: Default::default(),
            sound_modes: Default::default(),
            unknown_after_sound_modes: Default::default(),
            touch_tone: Default::default(),
            low_battery_prompt: Default::default(),
            ldac: Default::default(),
            dual_connections_enabled: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            spatial_audio: Default::default(),
            unknown_after_spatial_audio: Default::default(),
            wearing_detection: Default::default(),
            wearing_tone: Default::default(),
            unknown_after_wearing_tone: Default::default(),
            gaming_mode: Default::default(),
            unknown_tail: Default::default(),
        }
    }
}

fn take_array<'a, const N: usize, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], [u8; N], E> {
    map(take(N), |bytes: &[u8]| {
        bytes.try_into().expect("take(N) returns N bytes")
    })
    .parse_complete(input)
}

impl FromPacketBody for D1205StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("d1205 state update packet", |input| {
            let (input, (tws_status, battery, firmware_version, serial_number)) = (
                TwsStatus::take,
                DualBattery::take,
                DualFirmwareVersion::take,
                SerialNumber::take,
            )
                .parse_complete(input)?;
            let (input, (unknown_case_firmware, case_battery_level, equalizer)) = (
                take_array::<5, _>,
                CaseBatteryLevel::take,
                EqualizerBlock::take,
            )
                .parse_complete(input)?;
            let (
                input,
                (unknown_before_buttons, button_configuration, unknown_after_buttons, sound_modes),
            ) = (
                le_u8,
                ButtonStatusCollection::take(d1205::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
                le_u8,
                SoundModes::take,
            )
                .parse_complete(input)?;
            let (
                input,
                (
                    unknown_after_sound_modes,
                    touch_tone,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                ),
            ) = (
                le_u8,
                TouchTone::take,
                LowBatteryPrompt::take,
                Ldac::take,
                take_bool,
                AutoPowerOff::take,
                LimitHighVolume::take,
                SpatialAudio::take,
            )
                .parse_complete(input)?;
            let (
                input,
                (
                    unknown_after_spatial_audio,
                    wearing_detection,
                    wearing_tone,
                    unknown_after_wearing_tone,
                    gaming_mode,
                    unknown_tail,
                ),
            ) = (
                take_array::<2, _>,
                WearingDetection::take,
                WearingTone::take,
                take_array::<7, _>,
                GamingMode::take,
                rest,
            )
                .parse_complete(input)?;

            Ok((
                input,
                Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    unknown_case_firmware,
                    case_battery_level,
                    equalizer,
                    unknown_before_buttons,
                    button_configuration,
                    unknown_after_buttons,
                    sound_modes,
                    unknown_after_sound_modes,
                    touch_tone,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                    unknown_after_spatial_audio,
                    wearing_detection,
                    wearing_tone,
                    unknown_after_wearing_tone,
                    gaming_mode,
                    unknown_tail: unknown_tail.to_vec(),
                },
            ))
        })
        .parse_complete(input)
    }
}

impl ToPacket for D1205StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.unknown_case_firmware)
            .chain(self.case_battery_level.bytes())
            .chain(self.equalizer.bytes())
            .chain(std::iter::once(self.unknown_before_buttons))
            .chain(
                self.button_configuration
                    .bytes(d1205::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(std::iter::once(self.unknown_after_buttons))
            .chain(self.sound_modes.bytes())
            .chain(std::iter::once(self.unknown_after_sound_modes))
            .chain(self.touch_tone.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.spatial_audio.bytes())
            .chain(self.unknown_after_spatial_audio)
            .chain(self.wearing_detection.bytes())
            .chain(self.wearing_tone.bytes())
            .chain(self.unknown_after_wearing_tone)
            .chain(self.gaming_mode.bytes())
            .chain(self.unknown_tail.iter().copied())
            .collect()
    }
}

state_update_packet_module!(D1205State, D1205StateUpdatePacket);
