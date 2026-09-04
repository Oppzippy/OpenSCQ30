use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    a3330::{self, state::A3330State},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            CaseBatteryLevel, CommonEqualizerConfiguration, DisableAllButtons, DualBatteryLevel,
            DualFirmwareVersion, LowBatteryPrompt, SerialNumber, SurroundSound, TouchTone,
            TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3330StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_battery_level: CaseBatteryLevel,
    pub button_configuration: ButtonStatusCollection<6>,
    pub call_button_configuration: ButtonStatusCollection<4>,
    pub surround_sound: SurroundSound,
    pub touch_tone: TouchTone,
    pub low_battery_prompt: LowBatteryPrompt,
    pub dual_connections_enabled: bool,
    pub disable_all_buttons: DisableAllButtons,
    pub _bass_mode: bool,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
}

impl Default for A3330StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_battery_level: Default::default(),
            button_configuration: a3330::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            call_button_configuration: a3330::CALL_BUTTON_CONFIGURATION_SETTINGS
                .default_status_collection(),
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

impl FromPacketBody for A3330StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3330 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBatteryLevel::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    take(5usize), // unknown, maybe case firmware version?
                    CaseBatteryLevel::take,
                    le_u8, // unknown
                    ButtonStatusCollection::take(
                        a3330::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    le_u8, // unknown
                    SurroundSound::take,
                    TouchTone::take,
                    LowBatteryPrompt::take,
                    take_bool, // dual connections enabled
                    DisableAllButtons::take,
                    take_bool, // TODO bass mode
                    CommonEqualizerConfiguration::take,
                    le_u8, // unknown
                    ButtonStatusCollection::take(
                        a3330::CALL_BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                ),
                |(
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    _unknown_maybe_case_firmware_version,
                    case_battery_level,
                    _unknown1,
                    button_configuration,
                    _unknown2,
                    surround_sound,
                    touch_tone,
                    low_battery_prompt,
                    dual_connections_enabled,
                    disable_all_buttons,
                    _bass_mode,
                    equalizer_configuration,
                    _unknown3,
                    call_button_configuration,
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
                    call_button_configuration,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3330StateUpdatePacket {
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
                    .bytes(a3330::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(iter::once(0))
            .chain(self.surround_sound.bytes())
            .chain(self.touch_tone.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(iter::once(self.dual_connections_enabled.into()))
            .chain(self.disable_all_buttons.bytes())
            .chain(iter::once(self._bass_mode.into()))
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(0))
            .chain(
                self.call_button_configuration
                    .bytes(a3330::CALL_BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .collect()
    }
}

state_update_packet_module!(A3330State, A3330StateUpdatePacket);
