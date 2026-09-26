use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, rest},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    a3953::{self, state::A3953State},
    common::{
        macros::state_update_packet_module,
        packet::{self, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, DualBattery, DualFirmwareVersion, Ldac, LowBatteryPrompt, SerialNumber,
            TwsStatus, WearingDetection, WearingTone, button_configuration::ButtonStatusCollection,
        },
    },
};

// Byte offsets cited inline below are from A3953AnalysisService.R0 (com.oceanwing.soundcore
// v6.4.0-17); R0's own bArr indices are this packet's index + 9.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3953StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub is_hear_id_initialized: a3953::structures::IsHearIdInitialized,
    pub hear_id: CustomHearId<2, 10>,
    pub custom_length: u8,
    pub button_configuration: ButtonStatusCollection<8>,
    pub unknown_gap: Vec<u8>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3953::structures::SoundModes,
    pub wearing_detection: WearingDetection,
    pub case_battery_level: CaseBatteryLevel,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
    pub wearing_tone: WearingTone,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ambient_sound_prompt: a3953::structures::AmbientSoundPrompt,
    pub spatial_audio: a3953::structures::SpatialAudio,
    pub device_colour: Option<u8>,
    pub press_sensitivity: Option<a3953::structures::PressSensitivity>,
    pub unknown_suffix: Vec<u8>,
}

impl Default for A3953StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            is_hear_id_initialized: Default::default(),
            hear_id: Default::default(),
            custom_length: Default::default(),
            button_configuration: a3953::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            unknown_gap: Default::default(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            wearing_detection: Default::default(),
            case_battery_level: Default::default(),
            ldac: Default::default(),
            dual_connections_enabled: Default::default(),
            auto_power_off: Default::default(),
            wearing_tone: Default::default(),
            low_battery_prompt: Default::default(),
            ambient_sound_prompt: Default::default(),
            spatial_audio: Default::default(),
            device_colour: Default::default(),
            press_sensitivity: Default::default(),
            unknown_suffix: Default::default(),
        }
    }
}

impl FromPacketBody for A3953StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        let total_len = input.len();
        context("a3953 state update packet", move |input| {
            let (input, tws_status) = TwsStatus::take(input)?;
            let (input, battery) = DualBattery::take(input)?;
            let (input, dual_firmware_version) = DualFirmwareVersion::take(input)?;
            let (input, serial_number) = SerialNumber::take(input)?;
            // bArr[41..63]: preset id + 2x10 raw band bytes, both channels (right duplicates left)
            let (input, equalizer_configuration) =
                CommonEqualizerConfiguration::<2, 10>::take(input)?;
            let (input, hear_id_status) = le_u8(input)?; // bArr[63]: 255/254 = no hear id data
            let is_hear_id_initialized = a3953::structures::IsHearIdInitialized(
                hear_id_status != 255 && hear_id_status != 254,
            );
            // bArr[64..112], same wire format as a3955 (DRC coefficients in VolumeAdjustments::apply_drc)
            let (input, hear_id) = CustomHearId::<2, 10>::take_with_music_genre_at_end(input)?;
            let (input, custom_length) = le_u8(input)?; // bArr[112]: base offset for fields below
            // bArr[113..129], see a3953::BUTTON_CONFIGURATION_SETTINGS (write confirmed on real hardware)
            let (input, button_configuration) = ButtonStatusCollection::take(
                a3953::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
            )(input)?;
            let gap_len = (custom_length as usize).saturating_sub(18); // usually 0; nonzero if custom_length != 18
            let (input, unknown_gap) = take(gap_len)(input)?;
            let (input, ambient_sound_mode_cycle) = AmbientSoundModeCycle::take(input)?;
            let (input, sound_modes) = a3953::structures::SoundModes::take(input)?;
            let (input, _unknown_personal_anc_test_info) = take(6usize)(input)?; // test time/volume/result, always 255/255 seen so far
            let (input, wearing_detection) = WearingDetection::take(input)?;
            let (input, _unknown_wearing_status) = take(2usize)(input)?; // left/right in-ear status, read-only
            let (input, case_battery_level) = CaseBatteryLevel::take(input)?;
            let (input, _unknown_bass_up) = take(1usize)(input)?; // "bass up" toggle, no write command found
            let (input, ldac) = Ldac::take(input)?;
            let (input, dual_connections_enabled) = take_bool(input)?; // same [0x0B,0x84] toggle as common::modules::dual_connections
            let (input, auto_power_off) = AutoPowerOff::take(input)?;
            let (input, _unknown_hear_id_volume_db) = take(1usize)(input)?; // hear id feature, not implemented
            let (input, wearing_tone) = WearingTone::take(input)?; // app calls this "in ear beep"
            let (input, low_battery_prompt) = LowBatteryPrompt::take(input)?;
            let (input, ambient_sound_prompt) = a3953::structures::AmbientSoundPrompt::take(input)?;
            let (input, spatial_audio) = a3953::structures::SpatialAudio::take(input)?;
            let (input, _unknown_health_and_gap) = take(5usize)(input)?; // daily-care health fields, all-zero on this earbud
            // device_colour/press_sensitivity only present when the body is >154 bytes (R0's own
            // length check). device_colour is an ASCII char, not a numeric id; '2' = Cloud White
            // confirmed on real hardware, other colors unconfirmed. Not exposed: no device in this
            // project exposes case color.
            let (input, (device_colour, press_sensitivity)) = if total_len > 154 {
                let (input, colour_byte) = le_u8(input)?;
                let (input, press_sensitivity) = a3953::structures::PressSensitivity::take(input)?;
                (input, (Some(colour_byte), Some(press_sensitivity)))
            } else {
                (input, (None, None))
            };
            let (input, unknown_suffix) = map(rest, |s: &[u8]| s.to_vec()).parse(input)?; // trailing bytes, 2 in captures seen so far
            Ok((
                input,
                Self {
                    tws_status,
                    battery,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    is_hear_id_initialized,
                    hear_id,
                    custom_length,
                    button_configuration,
                    unknown_gap: unknown_gap.to_vec(),
                    ambient_sound_mode_cycle,
                    sound_modes,
                    wearing_detection,
                    case_battery_level,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    wearing_tone,
                    low_battery_prompt,
                    ambient_sound_prompt,
                    spatial_audio,
                    device_colour,
                    press_sensitivity,
                    unknown_suffix,
                },
            ))
        })
        .parse_complete(input)
    }
}

impl ToPacket for A3953StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.to_string().into_bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(if self.is_hear_id_initialized.0 {
                0
            } else {
                255
            }))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(iter::once(self.custom_length))
            .chain(
                self.button_configuration
                    .bytes(a3953::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.unknown_gap.iter().copied())
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 6]) // unknown personal ANC test info
            .chain(self.wearing_detection.bytes())
            .chain([0; 2]) // unknown wearing status
            .chain(self.case_battery_level.bytes())
            .chain(iter::once(0)) // unknown bass up
            .chain(self.ldac.bytes())
            .chain(iter::once(self.dual_connections_enabled.into()))
            .chain(self.auto_power_off.bytes())
            .chain(iter::once(0)) // unknown hear id volume db
            .chain(self.wearing_tone.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ambient_sound_prompt.bytes())
            .chain(self.spatial_audio.bytes())
            .chain([0; 5]) // unknown health and gap
            .chain(self.device_colour)
            .chain(
                self.press_sensitivity
                    .into_iter()
                    .flat_map(|value| value.bytes()),
            )
            .chain(self.unknown_suffix.iter().copied())
            .collect()
    }
}

state_update_packet_module!(A3953State, A3953StateUpdatePacket);

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3953StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3953StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
