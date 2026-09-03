use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3944::{self, state::A3944State},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{
            CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion, SerialNumber,
            TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3944StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub touch_tone: TouchTone,
}

impl Default for A3944StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3944::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            touch_tone: Default::default(),
        }
    }
}

impl FromPacketBody for A3944StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3944 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::take,
                    take(11usize),
                    ButtonStatusCollection::take(
                        a3944::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    take(5usize),
                    TouchTone::take,
                ),
                |(
                    tws_status,
                    dual_battery,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown1,
                    button_configuration,
                    _unknown2,
                    touch_tone,
                )| {
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        touch_tone,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3944StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.dual_battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::repeat_n(0, 11))
            .chain(
                self.button_configuration
                    .bytes(a3944::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(iter::repeat_n(0, 5))
            .chain(self.touch_tone.bytes())
            .collect()
    }
}

state_update_packet_module!(A3944State, A3944StateUpdatePacket);
