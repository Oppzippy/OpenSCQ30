use nom::{
    bytes::complete::take,
    combinator::{all_consuming, opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
    IResult,
};

use crate::devices::{
    a3933::device_profile::A3933_DEVICE_PROFILE,
    standard::{
        packets::{inbound::state_update_packet::StateUpdatePacket, parsing::take_bool},
        quirks::TwoExtraEqBandsValues,
        structures::{
            AgeRange, AmbientSoundModeCycle, BatteryLevel, CustomButtonModel, CustomHearId,
            DualBattery, EqualizerConfiguration, FirmwareVersion, HearId, HostDevice, SerialNumber,
            SoundModes, StereoEqualizerConfiguration,
        },
    },
};

// A3933 and A3939
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq)]
pub struct A3933StateUpdatePacket {
    pub host_device: HostDevice,
    pub tws_status: bool,
    pub battery: DualBattery,
    pub left_firmware: FirmwareVersion,
    pub right_firmware: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub left_equalizer_configuration: EqualizerConfiguration,
    pub right_equalizer_configuration: EqualizerConfiguration,
    pub extra_band_values: TwoExtraEqBandsValues,
    pub age_range: AgeRange,
    pub hear_id: Option<CustomHearId>, // 10 bands
    pub custom_button_model: CustomButtonModel,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: SoundModes,
    pub touch_tone_switch: bool,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub device_color: u8,
    pub wind_noise_detection: bool,
}

impl From<A3933StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3933StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3933_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.left_equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: packet.hear_id.map(HearId::Custom),
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: Some(packet.left_firmware.min(packet.right_firmware)),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: Some(packet.ambient_sound_mode_cycle),
            sound_modes_type_two: None,
        }
    }
}

impl A3933StateUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3933StateUpdatePacket, E> {
        context(
            "a3933 state update packet",
            all_consuming(|input| {
                let (
                    input,
                    (
                        host_device,
                        tws_status,
                        battery,
                        left_firmware,
                        right_firmware,
                        serial_number,
                        (equalizer_configuration, extra_band_values),
                        age_range,
                    ),
                ) = tuple((
                    HostDevice::take,
                    take_bool::<E>,
                    DualBattery::take,
                    FirmwareVersion::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    StereoEqualizerConfiguration::take_with_two_extra_bands(8),
                    AgeRange::take,
                ))(input)?;

                let (input, hear_id) = if !age_range.supports_hear_id() {
                    let (input, _) = take(48usize)(input)?;
                    (input, None)
                } else {
                    let (input, hear_id) = CustomHearId::take_without_music_type(10)(input)?;
                    (input, Some(hear_id))
                };

                let (
                    input,
                    (custom_button_model, ambient_sound_mode_cycle, sound_modes, _unknown, extra),
                ) = tuple((
                    CustomButtonModel::take,
                    AmbientSoundModeCycle::take,
                    SoundModes::take,
                    // Unsure if these two unknown bytes should be inside or outside the optional
                    take(2usize), // unknown bytes
                    opt(pair(Self::take_optional_extra_data, take(3usize))),
                ))(input)?;

                Ok((
                    input,
                    A3933StateUpdatePacket {
                        host_device,
                        tws_status,
                        battery,
                        left_firmware,
                        right_firmware,
                        serial_number,
                        left_equalizer_configuration: equalizer_configuration.left,
                        right_equalizer_configuration: equalizer_configuration.right,
                        extra_band_values,
                        age_range,
                        hear_id,
                        custom_button_model,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        touch_tone_switch: extra.map(|(e, _)| e.0).unwrap_or_default(),
                        wear_detection_switch: extra.map(|(e, _)| e.1).unwrap_or_default(),
                        game_mode_switch: extra.map(|(e, _)| e.2).unwrap_or_default(),
                        charging_case_battery_level: extra.map(|(e, _)| e.3).unwrap_or_default(),
                        device_color: extra.map(|(e, _)| e.5).unwrap_or_default(),
                        wind_noise_detection: extra.map(|(e, _)| e.6).unwrap_or_default(),
                    },
                ))
            }),
        )(input)
    }

    fn take_optional_extra_data<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], (bool, bool, bool, BatteryLevel, u8, u8, bool), E> {
        context(
            "extra data",
            tuple((
                take_bool, // touch tone
                take_bool, // wear detection
                take_bool, // game mode
                BatteryLevel::take,
                le_u8,     // what is this byte?
                le_u8,     // device color
                take_bool, // wind noise detection
            )),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::{
        a3933::packets::inbound::A3933StateUpdatePacket,
        standard::{
            packets::inbound::take_inbound_packet_header,
            structures::{
                AmbientSoundMode, BatteryLevel, CustomNoiseCanceling, EqualizerConfiguration,
                FirmwareVersion, HostDevice, IsBatteryCharging, PresetEqualizerProfile,
                SingleBattery,
            },
        },
    };

    #[test]
    fn it_parses_packet() {
        // state update
        // length 142
        // host device 1
        // tws status 1
        // both batteries level 4
        // both batteries not charging
        // both firmware version 02.61
        // serial number 39392A7FCC2F12AC
        // soundcore signature
        // no hear id
        let input: &[u8] = &[
            9, 255, 0, 0, 1, 1, 1, 142, 0, 1, 1, 4, 4, 0, 0, 48, 50, 46, 54, 49, 48, 50, 46, 54,
            49, 51, 57, 51, 57, 50, 65, 55, 70, 67, 67, 50, 70, 49, 50, 65, 67, 0, 0, 120, 120,
            120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99,
            1, 82, 1, 102, 1, 84, 1, 1, 1, 0, 7, 0, 0, 0, 10, 255, 255, 0, 255, 0, 0, 0, 51, 255,
            255, 255, 255, 102,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let (_, packet) =
            A3933StateUpdatePacket::take::<VerboseError<_>>(body).expect("should parse packet");

        assert_eq!(HostDevice::Left, packet.host_device);
        assert_eq!(true, packet.tws_status);
        assert_eq!(
            SingleBattery {
                level: BatteryLevel(4),
                is_charging: IsBatteryCharging::No,
            },
            packet.battery.left,
        );
        assert_eq!(FirmwareVersion::new(2, 61), packet.left_firmware);
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature
            ),
            packet.left_equalizer_configuration
        );
        assert_eq!(
            AmbientSoundMode::NoiseCanceling,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            CustomNoiseCanceling::new(10),
            packet.sound_modes.custom_noise_canceling
        );
    }
}
