use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, rest},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    a3388::{self, state::A3388State},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            CaseBatteryLevel, CommonEqualizerConfiguration, DisableAllButtons, DualBatteryLevel,
            DualFirmwareVersion, LowBatteryPrompt, OptionalVolumeAdjustmentsExt, SerialNumber,
            SurroundSound, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

// The layout matches the A3330 up to and including the equalizer. The two bytes after the
// double and triple press actions are long press slots, but the soundcore app offers no long
// press setting and long pressing does nothing, so they are kept but not exposed. The A3330's
// call button block does not fit in what remains, and the app has no call button settings, so
// the tail is kept as unknown bytes.
//
// The equalizer has a left and a right channel, since the soundcore app sends the same bands twice
// when setting it, but the state update packet holds only one, which applies to both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3388StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_battery_level: CaseBatteryLevel,
    pub button_configuration: ButtonStatusCollection<4>,
    pub surround_sound: SurroundSound,
    pub touch_tone: TouchTone,
    pub low_battery_prompt: LowBatteryPrompt,
    pub dual_connections_enabled: bool,
    pub disable_all_buttons: DisableAllButtons,
    pub _bass_mode: bool,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
}

impl Default for A3388StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_battery_level: Default::default(),
            button_configuration: a3388::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            surround_sound: Default::default(),
            touch_tone: Default::default(),
            low_battery_prompt: Default::default(),
            dual_connections_enabled: Default::default(),
            disable_all_buttons: Default::default(),
            _bass_mode: Default::default(),
            equalizer_configuration: Default::default(),
        }
    }
}

impl FromPacketBody for A3388StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3388 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBatteryLevel::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    take(5usize), // unknown, changes between reads
                    CaseBatteryLevel::take,
                    le_u8, // unknown
                    ButtonStatusCollection::take(
                        a3388::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    take(2usize), // long press slots, unused
                    le_u8,        // unknown
                    SurroundSound::take,
                    TouchTone::take,
                    LowBatteryPrompt::take,
                    take_bool, // dual connections enabled
                    DisableAllButtons::take,
                    take_bool, // TODO bass mode
                    take_equalizer_configuration,
                    rest, // unknown
                ),
                |(
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    _unknown0,
                    case_battery_level,
                    _unknown1,
                    button_configuration,
                    _long_press,
                    _unknown2,
                    surround_sound,
                    touch_tone,
                    low_battery_prompt,
                    dual_connections_enabled,
                    disable_all_buttons,
                    _bass_mode,
                    equalizer_configuration,
                    _unknown3,
                )| Self {
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    case_battery_level,
                    button_configuration,
                    surround_sound,
                    touch_tone,
                    low_battery_prompt,
                    dual_connections_enabled,
                    disable_all_buttons,
                    _bass_mode,
                    equalizer_configuration,
                },
            ),
        )
        .parse_complete(input)
    }
}

fn take_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], CommonEqualizerConfiguration<2, 10>, E> {
    map(
        CommonEqualizerConfiguration::<1, 10>::take,
        |configuration| {
            let [volume_adjustments] = *configuration.volume_adjustments();
            CommonEqualizerConfiguration::new(
                configuration.preset_id(),
                [volume_adjustments, volume_adjustments],
            )
        },
    )
    .parse(input)
}

impl ToPacket for A3388StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.dual_battery_level.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(iter::repeat_n(0, 5))
            .chain(self.case_battery_level.bytes())
            .chain(iter::once(0))
            .chain(
                self.button_configuration
                    .bytes(a3388::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain([0x0F, 0x0F])
            .chain(iter::once(0))
            .chain(self.surround_sound.bytes())
            .chain(self.touch_tone.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(iter::once(self.dual_connections_enabled.into()))
            .chain(self.disable_all_buttons.bytes())
            .chain(iter::once(self._bass_mode.into()))
            .chain(self.equalizer_configuration.preset_id().to_le_bytes())
            .chain(
                self.equalizer_configuration
                    .volume_adjustments_channel_1()
                    .copied()
                    .bytes(),
            )
            .chain([1, 0, 0, 255])
            .collect()
    }
}

state_update_packet_module!(A3388State, A3388StateUpdatePacket);
