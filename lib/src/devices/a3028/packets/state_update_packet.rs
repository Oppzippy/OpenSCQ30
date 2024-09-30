use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::{
    a3028::device_profile::A3028_DEVICE_PROFILE,
    standard::{
        packets::{inbound::state_update_packet::StateUpdatePacket, parsing::ParseResult},
        structures::{
            AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
            SingleBattery, SoundModes,
        },
    },
};

// A3028 and A3030
#[derive(Debug, Clone, PartialEq)]
pub struct A3028StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
}

impl From<A3028StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3028StateUpdatePacket) -> Self {
        Self {
            device_profile: A3028_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
            custom_button_model: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl A3028StateUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3028StateUpdatePacket, E> {
        context(
            "a3028 state update packet",
            all_consuming(map(
                tuple((
                    SingleBattery::take,
                    EqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    SoundModes::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
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
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{state_update_packet::StateUpdatePacket, take_inbound_packet_header},
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
            PresetEqualizerProfile, SoundModes, VolumeAdjustments,
        },
    };

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
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap().1;
        assert_eq!(
            Some(SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                noise_canceling_mode: NoiseCancelingMode::Transport,
                transparency_mode: Default::default(),
                custom_noise_canceling: CustomNoiseCanceling::new(0),
            }),
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
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap().1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.unwrap().ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.unwrap().noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-6.0, 6.0, 2.3, 4.0, 2.2, 6.0, -0.4, 1.6]).unwrap(),
            packet.equalizer_configuration.volume_adjustments(),
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
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap().1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.unwrap().ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.unwrap().noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-6.0, 6.0, 2.3, 4.0, 2.2, 6.0, -0.4, 1.6]).unwrap(),
            packet.equalizer_configuration.volume_adjustments(),
        );
    }

    #[test]
    fn it_parses_packet_with_a_4_at_byte_offset_9() {
        // It's usually a 5 but sometimes a 4 at that byte offset. I don't know why, but it
        // doesn't seem to cause any problems.
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x04, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x2f,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap().1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.unwrap().ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.unwrap().noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-6.0, 6.0, 2.3, 4.0, 2.2, 6.0, -0.4, 1.6]).unwrap(),
            packet.equalizer_configuration.volume_adjustments(),
        );
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_ambient_sound_mode() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //          valid range is 0x00 to 0x02
            0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x31,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let (_, packet) = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap();
        assert_eq!(
            AmbientSoundMode::default(),
            packet.sound_modes.unwrap().ambient_sound_mode
        );
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_noise_canceling_mode() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //                valid range is 0x00 to 0x03
            0x00, 0x00, 0x01, 0x04, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x33,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let (_, packet) = StateUpdatePacket::take::<VerboseError<_>>(body).unwrap();
        assert_eq!(
            NoiseCancelingMode::default(),
            packet.sound_modes.unwrap().noise_canceling_mode
        );
    }

    #[test]
    fn it_does_not_parse_packet_that_goes_over_expected_length() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x49, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            // Some extra 0x00s are added on to increase the length without affecting anything
            // the checksum. The checksum is affected by the 8th byte (packet length) though.
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x00,
            0x00, 0x00, 0x38,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let result = StateUpdatePacket::take::<VerboseError<_>>(body);
        assert!(result.is_err())
    }
}
