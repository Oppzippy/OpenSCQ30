use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            state::Update,
            structures::{
                AutoPowerOff, CommonEqualizerConfiguration, DisableAllButtons, DualBatteryLevel,
                DualFirmwareVersion, Ldac, LowBatteryPrompt, SerialNumber, TwsStatus,
                button_configuration::ButtonStatusCollection,
            },
        },
        d1101::{self, state::D1101State},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1101StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub low_battery_prompt: LowBatteryPrompt,
    pub dual_connections_enabled: bool,
    pub button_controls_disabled: DisableAllButtons,
    pub auto_power_off: AutoPowerOff,
    pub ldac: Ldac,
}

impl Default for D1101StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: d1101::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            low_battery_prompt: Default::default(),
            dual_connections_enabled: Default::default(),
            button_controls_disabled: Default::default(),
            auto_power_off: Default::default(),
            ldac: Default::default(),
        }
    }
}

impl FromPacketBody for D1101StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1101 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBatteryLevel::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::<1, 10>::take,
                    take(1usize),
                    ButtonStatusCollection::take(
                        d1101::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    take(1usize),
                    LowBatteryPrompt::take,
                    take_bool,
                    DisableAllButtons::take,
                    AutoPowerOff::take,
                    Ldac::take,
                ),
                |(
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown1,
                    button_configuration,
                    _unknown2,
                    low_battery_prompt,
                    dual_connections_enabled,
                    button_controls_disabled,
                    auto_power_off,
                    ldac,
                )| Self {
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration: CommonEqualizerConfiguration::new(
                        equalizer_configuration.preset_id(),
                        [equalizer_configuration
                            .volume_adjustments_channel_1()
                            .copied(); 2],
                    ),
                    button_configuration,
                    low_battery_prompt,
                    dual_connections_enabled,
                    button_controls_disabled,
                    auto_power_off,
                    ldac,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1101StateUpdatePacket {
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
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0))
            .chain(
                self.button_configuration
                    .bytes(d1101::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(std::iter::once(0))
            .chain(self.low_battery_prompt.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .chain(self.button_controls_disabled.bytes())
            .chain(self.auto_power_off.bytes())
            .chain(self.ldac.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<D1101State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<D1101State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: D1101StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<D1101State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
