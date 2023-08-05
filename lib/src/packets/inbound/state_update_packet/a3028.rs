use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::packets::{
    parsing::{
        take_age_range, take_basic_hear_id, take_equalizer_configuration, take_firmware_version,
        take_serial_number, take_single_battery, take_sound_modes, ParseResult,
    },
    structures::{
        AgeRange, BasicHearId, DeviceFeatureFlags, EqualizerConfiguration, FirmwareVersion,
        SerialNumber, SingleBattery, SoundModes,
    },
};

use super::StateUpdatePacket;

// A3028 and A3030
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3028StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: u8,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
}

impl From<A3028StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3028StateUpdatePacket) -> Self {
        Self {
            feature_flags: DeviceFeatureFlags::SOUND_MODES
                | DeviceFeatureFlags::NOISE_CANCELING_MODE
                | DeviceFeatureFlags::EQUALIZER,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            custom_hear_id: Some(packet.hear_id.into()),
            custom_button_model: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
        }
    }
}

pub fn take_a3028_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3028StateUpdatePacket, E> {
    context(
        "StateUpdatePacket",
        all_consuming(map(
            tuple((
                take_single_battery,
                take_equalizer_configuration,
                le_u8,
                take_age_range,
                take_basic_hear_id,
                take_sound_modes,
                take_firmware_version,
                take_serial_number,
            )),
            |(
                battery,
                equalizer_configuration,
                gender,
                age_range,
                hear_id,
                sound_modes,
                firmware_version,
                serial_number,
            )| {
                A3028StateUpdatePacket {
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                }
            },
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::packets::{
        inbound::state_update_packet::take_a3028_state_update_packet,
        parsing::{take_checksum, take_packet_header},
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
            PresetEqualizerProfile, SoundModes, VolumeAdjustments,
        },
    };

    fn strip(input: &[u8]) -> &[u8] {
        let input = take_checksum::<VerboseError<&[u8]>>(input).unwrap().0;
        let input = take_packet_header::<VerboseError<&[u8]>>(input).unwrap().0;
        input
    }

    #[test]
    fn it_parses_packet_with_preset_eq() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x35,
        ];
        let input = strip(input);
        let packet = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(
            SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                noise_canceling_mode: NoiseCancelingMode::Transport,
                transparency_mode: Default::default(),
                custom_noise_canceling: CustomNoiseCanceling::new(0),
            },
            packet.sound_modes,
        );
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            packet.equalizer_configuration
        );
    }

    #[test]
    fn it_parses_packet_with_invalid_preset_eq_id_as_a_custom_profile() {
        let input: &[u8] = &[
            //                                                                profile id 0x50 is
            //                                                                invalid
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x50, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x84,
        ];
        let input = strip(input);
        let packet = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration
        );
    }

    #[test]
    fn it_parses_packet_with_custom_eq() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = strip(input);
        let packet = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration
        );
    }

    #[test]
    fn it_parses_packet_with_a_4_at_byte_offset_9() {
        // It's usually a 5 but sometimes a 4 at that byte offset. I don't know why, but it
        // doesn't seem to cause any problems.
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x04, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x2f,
        ];
        let input = strip(input);
        let packet = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration
        );
    }

    #[test]
    fn it_does_not_parse_invalid_ambient_sound_mode() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //          valid range is 0x00 to 0x02
            0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x31,
        ];
        let input = strip(input);
        let result = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_invalid_noise_canceling_mode() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //                valid range is 0x00 to 0x03
            0x00, 0x00, 0x01, 0x04, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x33,
        ];
        let input = strip(input);
        let result = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        let input: &[u8] = &[0x01, 0x02, 0x03];
        let result = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_packet_that_goes_over_expected_length() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            // an extra 0x00 is added on to increase the length without affecting anything else
            // including the checksum
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x00,
            0x35,
        ];
        let input = strip(input);
        let result = take_a3028_state_update_packet::<VerboseError<&[u8]>>(input);
        assert!(result.is_err())
    }
}
