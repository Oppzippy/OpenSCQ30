use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3876::{self, state::A3876State},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AutoPowerOff, CommonEqualizerConfiguration, DualBatteryLevel, DualFirmwareVersion,
            GamingMode, SerialNumber, TwsStatus, VoicePrompt,
            button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3876StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub colorful_lights: a3876::structures::ColorfulLights,
    pub voice_prompt: VoicePrompt,
    pub auto_power_off: AutoPowerOff,
    pub gaming_mode: GamingMode,
    pub volume_balance: a3876::structures::VolumeBalance,
    pub dual_connections_enabled: bool,
}

impl Default for A3876StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3876::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            colorful_lights: Default::default(),
            voice_prompt: Default::default(),
            auto_power_off: Default::default(),
            gaming_mode: Default::default(),
            volume_balance: Default::default(),
            dual_connections_enabled: Default::default(),
        }
    }
}

impl FromPacketBody for A3876StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3876 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBatteryLevel::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    take(6usize),
                    CommonEqualizerConfiguration::take,
                    take(1usize),
                    ButtonStatusCollection::take(
                        a3876::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    a3876::structures::ColorfulLights::take,
                    take(1usize),
                    VoicePrompt::take,
                    AutoPowerOff::take,
                    take(1usize),
                    GamingMode::take,
                    a3876::structures::VolumeBalance::take,
                    take_bool,
                ),
                |(
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    _unknown,
                    equalizer_configuration,
                    _unknown2,
                    button_configuration,
                    colorful_lights,
                    _unknown3,
                    voice_prompt,
                    auto_power_off,
                    _unknown4,
                    gaming_mode,
                    volume_balance,
                    dual_connections_enabled,
                )| Self {
                    serial_number,
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    equalizer_configuration,
                    button_configuration,
                    colorful_lights,
                    voice_prompt,
                    auto_power_off,
                    gaming_mode,
                    volume_balance,
                    dual_connections_enabled,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3876StateUpdatePacket {
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
            .chain(std::iter::repeat_n(0, 6))
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0))
            .chain(
                self.button_configuration
                    .bytes(a3876::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.colorful_lights.bytes())
            .chain(std::iter::once(0))
            .chain(self.voice_prompt.bytes())
            .chain(self.auto_power_off.bytes())
            .chain(std::iter::once(0))
            .chain(self.gaming_mode.bytes())
            .chain(self.volume_balance.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .collect()
    }
}

state_update_packet_module!(A3876State, A3876StateUpdatePacket);
