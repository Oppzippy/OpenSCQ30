use nom::{
    bytes::complete::{tag, take},
    combinator::all_consuming,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    IResult,
};

use crate::devices::{
    a3936::{
        device_profile::A3936_DEVICE_PROFILE, structures::A3936InternalMultiButtonConfiguration,
    },
    standard::{
        packets::{inbound::state_update_packet::StateUpdatePacket, parsing::take_bool},
        quirks::TwoExtraEqBandsValues,
        structures::{
            AgeRange, AmbientSoundModeCycle, BatteryLevel, CustomHearId, DualBattery,
            FirmwareVersion, SerialNumber, SoundModesTypeTwo, StereoEqualizerConfiguration,
            TwsStatus,
        },
    },
};

// A3936
#[derive(Debug, Clone, PartialEq)]
pub struct A3936StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub left_firmware: FirmwareVersion,
    pub right_firmware: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: StereoEqualizerConfiguration,
    pub extra_bands: TwoExtraEqBandsValues,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
    pub sound_modes: SoundModesTypeTwo,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub button_configuration: A3936InternalMultiButtonConfiguration,
    pub touch_tone: bool,
    pub charging_case_battery: BatteryLevel,
    pub color: u8,
    pub ldac: bool,
    pub supports_two_cnn_switch: bool,
    pub auto_power_off_switch: bool,
    pub auto_power_off_index: u8,
    pub game_mode_switch: bool,
}

impl From<A3936StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3936StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3936_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration.left,
            sound_modes: None,
            sound_modes_type_two: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: None,
            hear_id: Some(packet.custom_hear_id.into()),
            button_configuration: Some(
                packet
                    .button_configuration
                    .into_custom_button_actions(packet.tws_status.is_connected),
            ),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
        }
    }
}

impl A3936StateUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3936StateUpdatePacket, E> {
        context(
            "a3936 state update packet",
            all_consuming(|input| {
                let (input, tws_status) = TwsStatus::take(input)?;
                let remaining = input.len() + 1; // remaining from between tws host device and is connected
                let (input, battery) = DualBattery::take(input)?;
                let (input, left_firmware) = FirmwareVersion::take(input)?;
                let (input, right_firmware) = FirmwareVersion::take(input)?;
                let (input, serial_number) = SerialNumber::take(input)?;
                let (input, (equalizer_configuration, extra_bands)) =
                    StereoEqualizerConfiguration::take_with_two_extra_bands(8)(input)?;
                let (input, age_range) = AgeRange::take(input)?;
                let (input, custom_hear_id) = CustomHearId::take_without_music_type(10)(input)?;

                // For some reason, an offset value is taken before the custom button model, which refers to how many bytes
                // until the next data to be read. This offset includes the length of the custom button model. Presumably,
                // there are some extra bytes between the button model and the beginning of the next data to be parsed?
                let (input, skip_offset) = le_u8(input)?;
                let remaining_before_button_configuration = input.len();
                let (input, button_configuration) =
                    A3936InternalMultiButtonConfiguration::take(input)?;
                println!("remaining: {}", remaining - input.len() + 10);
                let button_configuration_size = remaining_before_button_configuration - input.len();
                let (input, _) = take(
                    (skip_offset as usize)
                        // subtract an extra 1 since we want the number of bytes to discard, not
                        // the offset to the first byte to read
                        .checked_sub(button_configuration_size + 2)
                        .unwrap_or_default(),
                )(input)?;

                let (input, ambient_sound_mode_cycle) = AmbientSoundModeCycle::take(input)?;
                let (input, sound_modes) = SoundModesTypeTwo::take(input)?;
                let (input, touch_tone) = take_bool(input)?;
                let (input, charging_case_battery) = BatteryLevel::take(input)?;
                let (input, color) = le_u8(input)?;
                let (input, ldac) = take_bool(input)?;
                let (input, supports_two_cnn_switch) = take_bool(input)?;
                let (input, auto_power_off_switch) = take_bool(input)?;
                let (input, auto_power_off_index) = le_u8(input)?;
                let (input, game_mode_switch) = take_bool(input)?;
                let (input, _) = tag([0xFF; 12])(input)?;
                Ok((
                    input,
                    A3936StateUpdatePacket {
                        tws_status,
                        battery,
                        left_firmware,
                        right_firmware,
                        serial_number,
                        equalizer_configuration,
                        extra_bands,
                        age_range,
                        custom_hear_id,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        button_configuration,
                        touch_tone,
                        charging_case_battery,
                        color,
                        ldac,
                        supports_two_cnn_switch,
                        auto_power_off_switch,
                        auto_power_off_index,
                        game_mode_switch,
                    },
                ))
            }),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::{take_inbound_packet_header, InboundPacket};

    use super::*;

    #[test]
    pub fn it_parses_a_known_good_packet() {
        let input = &[
            0x9, 0xff, 0x0, 0x0, 0x1, 0x1, 0x1, 0x99, 0x0, 0x1, 0x1, 0x5, 0x5, 0x1, 0x1, 0x30,
            0x34, 0x2e, 0x31, 0x39, 0x30, 0x34, 0x2e, 0x31, 0x39, 0x33, 0x39, 0x33, 0x36, 0x61,
            0x34, 0x37, 0x37, 0x35, 0x38, 0x34, 0x37, 0x30, 0x33, 0x36, 0x36, 0x0, 0x0, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x3c, 0x1, 0x0, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c,
            0x3c, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x65, 0x26, 0x8d,
            0xaf, 0x2, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x0, 0x0, 0xe, 0x1, 0x11, 0x1, 0x0,
            0x11, 0x63, 0x11, 0x66, 0x11, 0x49, 0x11, 0x44, 0x7, 0x2, 0x32, 0x0, 0x1, 0x0, 0x0,
            0x0, 0x4, 0x31, 0x0, 0x1, 0x1, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xdd,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        A3936StateUpdatePacket::take::<VerboseError<_>>(body)
            .expect("it should parse successfully as a A3936 state update packet");
        StateUpdatePacket::take::<VerboseError<_>>(body)
            .expect("it should parse successfully as a state update packet");
    }
}
