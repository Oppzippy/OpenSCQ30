use nom::{
    IResult, Parser,
    bytes::complete::take,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    d1203::{
        modules::{
            D1203AmbientSoundMode, D1203AutoPowerOff, D1203Button, D1203ButtonAction,
            D1203ButtonConfiguration, D1203CustomEqualizer, D1203EasyChat, D1203Ldac,
            D1203ManualNoiseCanceling, D1203NoiseCancelingMode, D1203TouchSensitivity,
            D1203TransportationMode,
        },
        state::D1203State,
    },
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{
            BatteryLevel, CaseBatteryLevel, DualBattery, DualFirmwareVersion, FirmwareVersion,
            IsBatteryCharging, SerialNumber, SingleBattery,
        },
    },
};

/// 6 byte prefix that precedes the TLV fields in a D1203 state packet.
const PREFIX: [u8; 6] = [1, 1, 1, 2, 1, 1];

/// Battery tags. Each value is `[status, battery_level]` where the level
/// is a 0-100 percentage (NOT the 0-10 scale used by the A3957). Confirmed by
/// cross-referencing captures against the real hardware: tag 3 is the LEFT
/// earbud, tag 4 is the RIGHT earbud. The first byte is a status, not a bool:
/// `0x00` out of the case, `0x01` briefly when placed in the case, `0x80`
/// charging. Only `0x80` counts as charging.
const TAG_BATTERY_LEFT: u8 = 3;
const TAG_BATTERY_RIGHT: u8 = 4;
const TAG_FIRMWARE_LEFT: u8 = 5;
const TAG_FIRMWARE_RIGHT: u8 = 6;
const TAG_SERIAL_NUMBER: u8 = 7;
/// Case battery: `[status, percent]`, e.g. `00 64` is 100% (matches the app).
/// The charging flag has not been observed yet (the case was full).
const TAG_CASE_BATTERY: u8 = 8;
/// 8 x f32 LE custom EQ gains, one per band on the fixed grid.
const TAG_EQ_GAINS: u8 = 12;
/// Button actions: `<action> <fixed byte>`, left then right for each gesture
/// (single 13/14, double 15/16, triple 17/18, long 19/20). Tags 21 to 24 are
/// the fixed swipe gestures and are not remappable.
const TAG_BUTTONS_FIRST: u8 = 13;
const TAG_BUTTONS_LAST: u8 = 20;
/// Auto power off: `[enabled, duration_idx]`.
const TAG_AUTO_POWER_OFF: u8 = 25;
/// Byte 0 bit `0x04` is LDAC, byte 1 bit `0x40` is Easy Chat. The tag is 3 bytes
/// on firmware 05.40 and 4 bytes on 04.52, 05.54 and 06.54.
const TAG_FEATURE_FLAGS: u8 = 27;
const LDAC_BIT: u8 = 0x04;
const EASY_CHAT_BIT: u8 = 0x40;
/// `[ambient mode, nc sub-mode, ?]`, same values as bytes 0 and 1 of `06 81`.
const TAG_SOUND_MODE: u8 = 36;
/// `[manual NC level 1-5, 1]`.
const TAG_MANUAL_NC: u8 = 37;
/// Byte 0 is the transport type, `0` airplane and `1` car.
const TAG_TRANSPORTATION: u8 = 38;
/// Touch sensitivity, `0` low to `2` high. Missing on firmware 05.40.
const TAG_TOUCH_SENSITIVITY: u8 = 58;

/// EQ preset tag. Value is `[preset_id, 0]`. The second byte is always 0 for
/// the named presets; the "Custom" preset is encoded as `[0xFE, 0xFE]`.
const TAG_EQ_PRESET: u8 = 11;

/// The currently-selected EQ preset on the D1203, decoded from tag 11 of the
/// state packet.
///
/// This is read-only/informational: the D1203 reports which named preset is
/// active (or `Custom` when a custom parametric EQ has been written). Unknown
/// ids are preserved as `Unknown(id)` so the parser never fails on a preset we
/// have not mapped yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D1203PresetEqualizer {
    /// `0` — Anker Balanced.
    Balanced,
    /// `1` — Clear Voice ("Vozes nítidas" in the pt-BR app).
    ClearVoice,
    /// `2` — Bass Booster.
    BassBooster,
    /// `3` — Calm and Soothing.
    CalmAndSoothing,
    /// `4` — Soundcore / Anker Signature.
    SoundcoreSignature,
    /// `0xFE, 0xFE` — a user-defined parametric EQ (see the outbound `set_eq`).
    Custom,
    /// Any preset id we have not mapped yet.
    Unknown(u8),
}

impl D1203PresetEqualizer {
    /// Decode the raw tag 11 value `[preset_id, second_byte]`.
    pub fn from_tag_value(value: &[u8]) -> Option<Self> {
        match value {
            [0xFE, 0xFE] => Some(Self::Custom),
            [id, _] => Some(match id {
                0 => Self::Balanced,
                1 => Self::ClearVoice,
                2 => Self::BassBooster,
                3 => Self::CalmAndSoothing,
                4 => Self::SoundcoreSignature,
                other => Self::Unknown(*other),
            }),
            _ => None,
        }
    }

    /// Raw 2-byte tag 11 value for this preset.
    pub fn to_tag_value(self) -> [u8; 2] {
        match self {
            Self::Balanced => [0, 0],
            Self::ClearVoice => [1, 0],
            Self::BassBooster => [2, 0],
            Self::CalmAndSoothing => [3, 0],
            Self::SoundcoreSignature => [4, 0],
            Self::Custom => [0xFE, 0xFE],
            Self::Unknown(id) => [id, 0],
        }
    }

    /// Human-readable name of the preset, matching the Soundcore app.
    pub fn name(self) -> String {
        match self {
            Self::Balanced => "Balanced".to_string(),
            Self::ClearVoice => "Clear Voice".to_string(),
            Self::BassBooster => "Bass Booster".to_string(),
            Self::CalmAndSoothing => "Calm and Soothing".to_string(),
            Self::SoundcoreSignature => "Soundcore Signature".to_string(),
            Self::Custom => "Custom".to_string(),
            Self::Unknown(id) => format!("Unknown({id})"),
        }
    }
}

impl std::fmt::Display for D1203PresetEqualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

impl Default for D1203PresetEqualizer {
    fn default() -> Self {
        Self::ClearVoice
    }
}

/// State update packet for the Soundcore Liberty 5 Pro (D1203).
///
/// The D1203 encodes its state as a 6 byte prefix followed by a sequence of
/// TLV fields `[tag(1)][length(1)][value(length)]`. It extracts the earbud
/// battery levels (tags 3/4), firmware versions (tags 5 and 6) and serial
/// number (tag 7). Every other tag is read and ignored, so new/unknown tags
/// never fail the parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1203StateUpdatePacket {
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    /// EQ preset reported by tag 11 (read-only/informational).
    pub preset_equalizer: D1203PresetEqualizer,
    /// Current ambient sound mode (ANC / Transparency / Normal / Adaptive).
    ///
    /// TODO: the D1203 state packet TLV tag carrying the ambient sound mode is
    /// not yet known (a full capture is pending), so this is always `Default`
    /// after a parse. The write path (command `06 81`) is fully functional,
    /// including value 3 = Adaptive; only the read-back of the device's current
    /// mode is outstanding.
    pub ambient_sound_mode: crate::devices::soundcore::d1203::modules::D1203AmbientSoundMode,
    /// Current noise-canceling sub-mode (Manual / Transportation).
    ///
    /// TODO: the TLV tag carrying this is not yet known, so it is always
    /// `Default` (Manual) after a parse. The write path (`06 81`) is fully
    /// functional; only read-back is outstanding.
    pub nc_mode: crate::devices::soundcore::d1203::modules::D1203NoiseCancelingMode,
    /// Current manual ANC level (1..=5).
    ///
    /// TODO: the TLV tag carrying this is not yet known, so it is always
    /// `Default` (level 1) after a parse. The write path is fully functional;
    /// only read-back is outstanding.
    pub manual_nc_level: crate::devices::soundcore::d1203::modules::D1203ManualNoiseCanceling,
    /// Current transport type (Airplane / Car) used inside Transportation.
    ///
    /// TODO: the TLV tag carrying this is not yet known, so it is always
    /// `Default` (Airplane) after a parse. The write path (`06 81`) is fully
    /// functional; only read-back is outstanding.
    pub transportation_mode: crate::devices::soundcore::d1203::modules::D1203TransportationMode,
    /// Current wind noise reduction state.
    ///
    /// TODO: the TLV tag carrying this is not yet known, so it is always
    /// `Default` (OFF) after a parse. The write path (dedicated command
    /// `41 88`) is fully functional; only read-back is outstanding.
    pub wind_noise: crate::devices::soundcore::d1203::modules::D1203WindNoise,
    /// Touch Tone toggle (`41 87` sub-id 0).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub touch_tone: crate::devices::soundcore::d1203::modules::D1203TouchTone,
    /// Low Battery Prompt toggle (`41 87` sub-id 1).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub low_battery_prompt: crate::devices::soundcore::d1203::modules::D1203LowBatteryPrompt,
    /// Wearing Tone toggle (`41 87` sub-id 3).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub wearing_tone: crate::devices::soundcore::d1203::modules::D1203WearingTone,
    /// LDAC codec toggle (`41 88` sub-id 2).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub ldac: crate::devices::soundcore::d1203::modules::D1203Ldac,
    /// Easy Chat toggle (`41 88` sub-id 14).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub easy_chat: crate::devices::soundcore::d1203::modules::D1203EasyChat,
    /// Soundstage / Soundfield rendering toggle (`41 88` sub-id 29).
    ///
    /// TODO: read-back not yet decoded; always `Default` (OFF) after a parse.
    /// The write path is fully functional.
    pub soundstage: crate::devices::soundcore::d1203::modules::D1203Soundstage,
    /// Auto power off selector (`01 86`), read from tag 25.
    pub auto_power_off: crate::devices::soundcore::d1203::modules::D1203AutoPowerOff,
    /// Case battery from tag 8.
    pub case_battery_level: CaseBatteryLevel,
    /// Custom EQ gains from tag 12, rounded to whole dB.
    pub custom_equalizer: D1203CustomEqualizer,
    /// Button actions from tags 13 to 20.
    pub button_configuration: D1203ButtonConfiguration,
    /// Touch sensitivity from tag 58.
    pub touch_sensitivity: D1203TouchSensitivity,
}

impl Default for D1203StateUpdatePacket {
    fn default() -> Self {
        Self {
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            preset_equalizer: Default::default(),
            ambient_sound_mode: Default::default(),
            nc_mode: Default::default(),
            manual_nc_level: Default::default(),
            transportation_mode: Default::default(),
            wind_noise: Default::default(),
            touch_tone: Default::default(),
            low_battery_prompt: Default::default(),
            wearing_tone: Default::default(),
            ldac: Default::default(),
            easy_chat: Default::default(),
            soundstage: Default::default(),
            auto_power_off: Default::default(),
            case_battery_level: Default::default(),
            custom_equalizer: Default::default(),
            button_configuration: Default::default(),
            touch_sensitivity: Default::default(),
        }
    }
}

impl FromPacketBody for D1203StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("d1203 state update packet", |input: &'a [u8]| {
            // 6 byte prefix
            let (mut input, _prefix) = take(PREFIX.len())(input)?;

            let mut dual_battery = DualBattery::default();
            let mut firmware_left: Option<FirmwareVersion> = None;
            let mut firmware_right: Option<FirmwareVersion> = None;
            let mut serial_number: Option<SerialNumber> = None;
            let mut preset_equalizer = D1203PresetEqualizer::default();
            let mut case_battery_level = CaseBatteryLevel::default();
            let mut custom_equalizer = D1203CustomEqualizer::default();
            let mut button_configuration = D1203ButtonConfiguration::default();
            let mut auto_power_off = D1203AutoPowerOff::default();
            let mut ldac = D1203Ldac::default();
            let mut easy_chat = D1203EasyChat::default();
            let mut ambient_sound_mode = D1203AmbientSoundMode::default();
            let mut nc_mode = D1203NoiseCancelingMode::default();
            let mut manual_nc_level = D1203ManualNoiseCanceling::default();
            let mut transportation_mode = D1203TransportationMode::default();
            let mut touch_sensitivity = D1203TouchSensitivity::default();

            // Iterate TLV fields until the input is consumed. Be tolerant:
            // unknown tags are skipped, and a value we can't decode (e.g. an
            // unexpected firmware/serial encoding) is ignored rather than
            // failing the whole packet.
            while !input.is_empty() {
                let (rest, tag) = le_u8(input)?;
                let (rest, length) = le_u8(rest)?;
                let (rest, value) = take(length as usize)(rest)?;
                input = rest;

                match tag {
                    TAG_BATTERY_LEFT => {
                        // [charging_flag, battery_level] where level is 0-100.
                        if value.len() >= 2 {
                            dual_battery.left = SingleBattery {
                                is_charging: IsBatteryCharging::from(value[0] & 0x80 != 0),
                                level: BatteryLevel(value[1]),
                            };
                        }
                    }
                    TAG_BATTERY_RIGHT => {
                        if value.len() >= 2 {
                            dual_battery.right = SingleBattery {
                                is_charging: IsBatteryCharging::from(value[0] & 0x80 != 0),
                                level: BatteryLevel(value[1]),
                            };
                        }
                    }
                    TAG_FIRMWARE_LEFT => {
                        if let Ok((_, firmware)) = FirmwareVersion::take::<E>(value) {
                            firmware_left = Some(firmware);
                        }
                    }
                    TAG_FIRMWARE_RIGHT => {
                        if let Ok((_, firmware)) = FirmwareVersion::take::<E>(value) {
                            firmware_right = Some(firmware);
                        }
                    }
                    TAG_SERIAL_NUMBER => {
                        // Value is 16 chars + a trailing null terminator.
                        // SerialNumber::take reads exactly 16 bytes.
                        if value.len() >= 16 {
                            if let Ok((_, serial)) = SerialNumber::take::<E>(&value[..16]) {
                                serial_number = Some(serial);
                            }
                        }
                    }
                    TAG_EQ_PRESET => {
                        // [preset_id, second_byte]; Custom is [0xFE, 0xFE].
                        if let Some(preset) = D1203PresetEqualizer::from_tag_value(value) {
                            preset_equalizer = preset;
                        }
                    }
                    TAG_CASE_BATTERY => {
                        if value.len() >= 2 {
                            case_battery_level = CaseBatteryLevel(BatteryLevel(value[1]));
                        }
                    }
                    TAG_EQ_GAINS => {
                        if value.len() >= 32 {
                            let mut gains = [0_i16; 8];
                            for (gain, chunk) in gains.iter_mut().zip(value.chunks_exact(4)) {
                                let db = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                                // NaN casts to 0; clamp keeps the value inside the UI range.
                                *gain = db.round().clamp(-6.0, 6.0) as i16;
                            }
                            custom_equalizer = D1203CustomEqualizer(gains);
                        }
                    }
                    TAG_BUTTONS_FIRST..=TAG_BUTTONS_LAST => {
                        let index = usize::from(tag - TAG_BUTTONS_FIRST);
                        // Tags alternate left, right per gesture, which is the
                        // order of `D1203Button::ALL`.
                        if let (Some(button), Some(action)) = (
                            D1203Button::ALL.get(index),
                            value.first().and_then(|id| D1203ButtonAction::from_repr(*id)),
                        ) {
                            button_configuration.set_action(*button, action);
                        }
                    }
                    TAG_AUTO_POWER_OFF => {
                        if value.len() >= 2 {
                            if let Some(setting) =
                                D1203AutoPowerOff::from_wire(value[0] != 0, value[1])
                            {
                                auto_power_off = setting;
                            }
                        }
                    }
                    TAG_FEATURE_FLAGS => {
                        if let Some(byte) = value.first() {
                            ldac = D1203Ldac(byte & LDAC_BIT != 0);
                        }
                        if let Some(byte) = value.get(1) {
                            easy_chat = D1203EasyChat(byte & EASY_CHAT_BIT != 0);
                        }
                    }
                    TAG_SOUND_MODE => {
                        if value.len() >= 2 {
                            if let Some(mode) = D1203AmbientSoundMode::from_repr(value[0]) {
                                ambient_sound_mode = mode;
                            }
                            if let Some(mode) = D1203NoiseCancelingMode::from_repr(value[1]) {
                                nc_mode = mode;
                            }
                        }
                    }
                    TAG_MANUAL_NC => {
                        if let Some(level) = value.first() {
                            manual_nc_level = D1203ManualNoiseCanceling::new(*level);
                        }
                    }
                    TAG_TRANSPORTATION => {
                        if let Some(mode) = value.first().and_then(|b| D1203TransportationMode::from_repr(*b)) {
                            transportation_mode = mode;
                        }
                    }
                    TAG_TOUCH_SENSITIVITY => {
                        if let Some(level) = value.first() {
                            touch_sensitivity = D1203TouchSensitivity::from_wire(*level);
                        }
                    }
                    _ => {
                        // Unknown tag: already skipped by consuming its value.
                    }
                }
            }

            let dual_firmware_version = match (firmware_left, firmware_right) {
                (Some(left), Some(right)) => DualFirmwareVersion::Both { left, right },
                (Some(left), None) => DualFirmwareVersion::LeftOnly(left),
                (None, Some(right)) => DualFirmwareVersion::RightOnly(right),
                (None, None) => DualFirmwareVersion::default(),
            };

            Ok((
                input,
                Self {
                    dual_battery,
                    dual_firmware_version,
                    serial_number: serial_number.unwrap_or_default(),
                    preset_equalizer,
                    ambient_sound_mode,
                    nc_mode,
                    manual_nc_level,
                    transportation_mode,
                    // Tags for these are not in the state packet; they are read
                    // with `41 07` / `41 08` after connecting (see `d1203.rs`).
                    wind_noise: Default::default(),
                    touch_tone: Default::default(),
                    low_battery_prompt: Default::default(),
                    wearing_tone: Default::default(),
                    ldac,
                    easy_chat,
                    soundstage: Default::default(),
                    auto_power_off,
                    case_battery_level,
                    custom_equalizer,
                    button_configuration,
                    touch_sensitivity,
                },
            ))
        })
        .parse_complete(input)
    }
}

fn charging_status(is_charging: IsBatteryCharging) -> u8 {
    match is_charging {
        IsBatteryCharging::Yes => 0x80,
        IsBatteryCharging::No => 0,
    }
}

impl ToPacket for D1203StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    /// Best-effort TLV serialization. Reconstructs a minimal but
    /// parser-compatible body: the 6 byte prefix, then the firmware (tags 5/6)
    /// and serial number (tag 7) TLV fields. This is enough for the demo device
    /// to round-trip the fields Phase 1 decodes.
    fn body(&self) -> Vec<u8> {
        let mut body: Vec<u8> = PREFIX.to_vec();

        // Earbud battery (tags 3/4): value is [charging_flag, level].
        body.push(TAG_BATTERY_LEFT);
        body.push(2);
        body.push(charging_status(self.dual_battery.left.is_charging));
        body.push(self.dual_battery.left.level.0);

        body.push(TAG_BATTERY_RIGHT);
        body.push(2);
        body.push(charging_status(self.dual_battery.right.is_charging));
        body.push(self.dual_battery.right.level.0);

        if let Some(left) = self.dual_firmware_version.left() {
            body.push(TAG_FIRMWARE_LEFT);
            body.push(5);
            body.extend(left.bytes());
        }
        if let Some(right) = self.dual_firmware_version.right() {
            body.push(TAG_FIRMWARE_RIGHT);
            body.push(5);
            body.extend(right.bytes());
        }

        // Serial number: 16 chars + null terminator => length 17.
        let serial_bytes: Vec<u8> = self.serial_number.bytes().collect();
        if serial_bytes.len() == 16 {
            body.push(TAG_SERIAL_NUMBER);
            body.push(17);
            body.extend(serial_bytes);
            body.push(0); // null terminator
        }

        // EQ preset (tag 11): value is [preset_id, second_byte].
        body.push(TAG_EQ_PRESET);
        body.push(2);
        body.extend(self.preset_equalizer.to_tag_value());

        body
    }
}

state_update_packet_module!(D1203State, D1203StateUpdatePacket);

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = D1203StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let parsed: D1203StateUpdatePacket = packet.try_to_packet().unwrap();
        assert_eq!(parsed, D1203StateUpdatePacket::default());
    }

    #[test]
    fn preset_tag_value_mapping() {
        use D1203PresetEqualizer::*;
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[1, 0]), Some(ClearVoice));
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[2, 0]), Some(BassBooster));
        assert_eq!(
            D1203PresetEqualizer::from_tag_value(&[4, 0]),
            Some(SoundcoreSignature)
        );
        assert_eq!(
            D1203PresetEqualizer::from_tag_value(&[0xFE, 0xFE]),
            Some(Custom)
        );
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[0, 0]), Some(Balanced));
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[3, 0]), Some(CalmAndSoothing));
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[7, 0]), Some(Unknown(7)));
    }

    #[test]
    fn preset_names() {
        assert_eq!(D1203PresetEqualizer::ClearVoice.name(), "Clear Voice");
        assert_eq!(D1203PresetEqualizer::BassBooster.name(), "Bass Booster");
        assert_eq!(
            D1203PresetEqualizer::SoundcoreSignature.name(),
            "Soundcore Signature"
        );
        assert_eq!(D1203PresetEqualizer::Custom.name(), "Custom");
        assert_eq!(D1203PresetEqualizer::CalmAndSoothing.name(), "Calm and Soothing");
        assert_eq!(D1203PresetEqualizer::Unknown(9).name(), "Unknown(9)");
    }

    #[test]
    fn parses_preset_from_state_packet() {
        // Minimal TLV body: 6 byte prefix + tag 11 = Bass Booster.
        let mut body: Vec<u8> = PREFIX.to_vec();
        body.extend_from_slice(&[TAG_EQ_PRESET, 2, 2, 0]);
        let packet = packet::Inbound::new(packet::inbound::STATE_COMMAND, body);
        let bytes = packet.bytes_with_checksum();
        let (_, inbound) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let parsed: D1203StateUpdatePacket = inbound.try_to_packet().unwrap();
        assert_eq!(parsed.preset_equalizer, D1203PresetEqualizer::BassBooster);
    }
}
