use nom::{
    IResult, Parser,
    bytes::complete::take,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    d1203::state::D1203State,
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{
            BatteryLevel, DualBattery, DualFirmwareVersion, FirmwareVersion, IsBatteryCharging,
            SerialNumber, SingleBattery,
        },
    },
};

/// 6 byte prefix that precedes the TLV fields in a D1203 state packet.
const PREFIX: [u8; 6] = [1, 1, 1, 2, 1, 1];

/// Battery tags. Each value is `[charging_flag, battery_level]` where the level
/// is a 0-100 percentage (NOT the 0-10 scale used by the A3957). Confirmed by
/// cross-referencing captures against the real hardware: tag 3 is the LEFT
/// earbud, tag 4 is the RIGHT earbud. The first byte is a charging flag
/// (0 = not charging, non-zero = charging).
const TAG_BATTERY_LEFT: u8 = 3;
const TAG_BATTERY_RIGHT: u8 = 4;
const TAG_FIRMWARE_LEFT: u8 = 5;
const TAG_FIRMWARE_RIGHT: u8 = 6;
const TAG_SERIAL_NUMBER: u8 = 7;
// TODO: tag 8 is believed to be the charging-case battery, but its encoding
// stayed ambiguous across captures, so case battery is intentionally not
// decoded yet.

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
    /// `1` — Clear Voice ("Vozes nítidas" in the pt-BR app).
    ClearVoice,
    /// `2` — Bass Booster.
    BassBooster,
    /// `4` — Soundcore / Anker Signature.
    SoundcoreSignature,
    /// `0xFE, 0xFE` — a user-defined parametric EQ (see the outbound `set_eq`).
    Custom,
    /// Any preset id we have not mapped yet (e.g. `3`).
    Unknown(u8),
}

impl D1203PresetEqualizer {
    /// Decode the raw tag 11 value `[preset_id, second_byte]`.
    pub fn from_tag_value(value: &[u8]) -> Option<Self> {
        match value {
            [0xFE, 0xFE] => Some(Self::Custom),
            [id, _] => Some(match id {
                1 => Self::ClearVoice,
                2 => Self::BassBooster,
                4 => Self::SoundcoreSignature,
                other => Self::Unknown(*other),
            }),
            _ => None,
        }
    }

    /// Raw 2-byte tag 11 value for this preset.
    pub fn to_tag_value(self) -> [u8; 2] {
        match self {
            Self::ClearVoice => [1, 0],
            Self::BassBooster => [2, 0],
            Self::SoundcoreSignature => [4, 0],
            Self::Custom => [0xFE, 0xFE],
            Self::Unknown(id) => [id, 0],
        }
    }

    /// Human-readable name of the preset, matching the Soundcore app.
    pub fn name(self) -> String {
        match self {
            Self::ClearVoice => "Clear Voice".to_string(),
            Self::BassBooster => "Bass Booster".to_string(),
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
                                is_charging: IsBatteryCharging::from(value[0] != 0),
                                level: BatteryLevel(value[1]),
                            };
                        }
                    }
                    TAG_BATTERY_RIGHT => {
                        if value.len() >= 2 {
                            dual_battery.right = SingleBattery {
                                is_charging: IsBatteryCharging::from(value[0] != 0),
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
                    // TODO: decode the real ambient sound mode once its TLV
                    // tag is known. Defaulting here means the state reads as
                    // the default mode; writing is unaffected.
                    ambient_sound_mode: Default::default(),
                    // TODO: decode the NC sub-mode and manual level once their
                    // TLV tags are known. Defaulting here; writing is
                    // unaffected.
                    nc_mode: Default::default(),
                    manual_nc_level: Default::default(),
                    // TODO: decode the transport type once its TLV tag is
                    // known. Defaulting here (Airplane); writing is unaffected.
                    transportation_mode: Default::default(),
                    // TODO: decode the wind-noise state once its TLV tag is
                    // known. Defaulting here (OFF); writing is unaffected.
                    wind_noise: Default::default(),
                },
            ))
        })
        .parse_complete(input)
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
        body.push(self.dual_battery.left.is_charging as u8);
        body.push(self.dual_battery.left.level.0);

        body.push(TAG_BATTERY_RIGHT);
        body.push(2);
        body.push(self.dual_battery.right.is_charging as u8);
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
        assert_eq!(D1203PresetEqualizer::from_tag_value(&[3, 0]), Some(Unknown(3)));
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
        assert_eq!(D1203PresetEqualizer::Unknown(3).name(), "Unknown(3)");
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
