use nom::{combinator::map, error::context, number::complete::le_u8, sequence::tuple};

use crate::packets::{
    parsing::{
        take_age_range, take_battery_level, take_equalizer_configuration, take_firmware_version,
        take_hear_id, take_is_battery_charging, take_serial_number, take_sound_modes, ParseResult,
    },
    structures::{
        AgeRange, AmbientSoundMode, BatteryLevel, CustomNoiseCanceling, EqualizerConfiguration,
        FirmwareVersion, HearId, IsBatteryCharging, NoiseCancelingMode, SerialNumber, SoundModes,
        TransparencyMode, VolumeAdjustments,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket {
    battery_level: BatteryLevel,
    is_battery_charging: IsBatteryCharging,
    equalizer_configuration: EqualizerConfiguration,
    gender: u8,
    age_range: AgeRange,
    hear_id: HearId,
    sound_modes: SoundModes,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
}

pub fn take_state_update_packet(input: &[u8]) -> ParseResult<StateUpdatePacket> {
    context(
        "StateUpdatePacket",
        map(
            tuple((
                // offset 9
                take_battery_level,
                // offset 10
                take_is_battery_charging,
                // offset 11
                take_equalizer_configuration,
                // offset 21
                le_u8, // gender
                // offset 22
                take_age_range, // age range
                // offset 23
                take_hear_id,
                // offset 44
                take_sound_modes,
                // offset 48
                take_firmware_version,
                // offset 53
                take_serial_number,
                // offset 69
            )),
            |(
                battery_level,
                is_battery_charging,
                equalizer_configuration,
                gender,
                age_range,
                hear_id,
                sound_modes,
                firmware_version,
                serial_number,
            )| {
                StateUpdatePacket {
                    battery_level,
                    is_battery_charging,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                }
            },
        ),
    )(input)
}

impl StateUpdatePacket {
    pub fn battery_level(&self) -> BatteryLevel {
        self.battery_level
    }
    pub fn is_battery_charging(&self) -> IsBatteryCharging {
        self.is_battery_charging
    }
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.equalizer_configuration
    }
    pub fn gender(&self) -> u8 {
        self.gender
    }
    pub fn age_range(&self) -> AgeRange {
        self.age_range
    }
    pub fn hear_id_switch(&self) -> bool {
        self.hear_id.is_enabled
    }
    pub fn left_hear_id_volume_adjustments(&self) -> VolumeAdjustments {
        self.hear_id.left
    }
    pub fn right_hear_id_volume_adjustments(&self) -> VolumeAdjustments {
        self.hear_id.right
    }
    pub fn hear_id_time(&self) -> i32 {
        self.hear_id.time
    }
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.sound_modes.ambient_sound_mode
    }
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.sound_modes.noise_canceling_mode
    }
    pub fn transparency_mode(&self) -> TransparencyMode {
        self.sound_modes.transparency_mode
    }
    pub fn custom_noise_canceling(&self) -> CustomNoiseCanceling {
        self.sound_modes.custom_noise_canceling
    }
    pub fn firmware(&self) -> &str {
        &self.firmware_version.0
    }
    pub fn serial_number(&self) -> &str {
        &self.serial_number.0
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        inbound::take_state_update_packet,
        parsing::take_packet_header,
        structures::{
            AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
            PresetEqualizerProfile, TransparencyMode, VolumeAdjustments,
        },
    };

    #[test]
    fn it_parses_packet_with_preset_eq() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let packet = take_state_update_packet(input).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(TransparencyMode::default(), packet.transparency_mode());
        assert_eq!(
            CustomNoiseCanceling::new(0),
            packet.custom_noise_canceling()
        );
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_invalid_preset_eq_id_as_a_custom_profile() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id 0x50 is
            //                                                                invalid
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x50, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let packet = take_state_update_packet(input).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_custom_eq() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let packet = take_state_update_packet(input).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_a_4_at_byte_offset_9() {
        // It's usually a 5 but sometimes a 4 at that byte offset. I don't know why, but it
        // doesn't seem to cause any problems.
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x04, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let packet = take_state_update_packet(input).unwrap().1;
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_does_not_parse_invalid_ambient_sound_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //          valid range is 0x00 to 0x02
            0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let result = take_state_update_packet(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_invalid_noise_canceling_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //                valid range is 0x00 to 0x03
            0x00, 0x00, 0x01, 0x04, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        let result = take_state_update_packet(input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        const PACKET_BYTES: &[u8] = &[0x01, 0x02, 0x03];
        let result = take_state_update_packet(PACKET_BYTES);
        assert_eq!(true, result.is_err());
    }
}
