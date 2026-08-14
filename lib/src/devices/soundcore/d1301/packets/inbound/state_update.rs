use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
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
            },
            packet_manager::PacketHandler,
            state::Update,
            structures::{
                DualBatteryLevel, DualFirmwareVersion, FirmwareVersion, LowBatteryPrompt,
                SerialNumber, TwsStatus, button_configuration::ButtonStatusCollection,
            },
        },
        d1301::{self, state::D1301State},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1301StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_firmware_version: FirmwareVersion,
    pub button_configuration: ButtonStatusCollection<4>,
    pub listening_mode: d1301::structures::ListeningMode,
    pub default_listening_mode: d1301::structures::DefaultListeningMode,
    pub low_battery_prompt: LowBatteryPrompt,
    pub auto_power_off_prompt: d1301::structures::AutoPowerOffPrompt,
    pub listening_mode_prompt: d1301::structures::ListeningModePrompt,
    pub noise_canceling: d1301::structures::NoiseCanceling,
    pub incoming_calls_during_bluetooth_mode: d1301::structures::IncomingCallsDuringBluetoothMode,
    pub tap_controls_disabled: d1301::structures::TapControlsDisabled,
    pub noise_canceling_prompt: d1301::structures::NoiseCancelingPrompt,
}

impl Default for D1301StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_firmware_version: Default::default(),
            button_configuration: d1301::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            listening_mode: Default::default(),
            default_listening_mode: Default::default(),
            low_battery_prompt: Default::default(),
            auto_power_off_prompt: Default::default(),
            listening_mode_prompt: Default::default(),
            noise_canceling: Default::default(),
            incoming_calls_during_bluetooth_mode: Default::default(),
            tap_controls_disabled: Default::default(),
            noise_canceling_prompt: Default::default(),
        }
    }
}

impl FromPacketBody for D1301StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1301 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBatteryLevel::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        take(6usize),
                        FirmwareVersion::take, // case firmware version
                        take(4usize),
                        le_u8, // auto switch once asleep, but unknown how it works
                        ButtonStatusCollection::take(
                            d1301::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        take(15usize),
                        d1301::structures::ListeningMode::take,
                        d1301::structures::DefaultListeningMode::take,
                        take(2usize),
                        LowBatteryPrompt::take,
                        take(67usize),
                        d1301::structures::AutoPowerOffPrompt::take,
                        d1301::structures::ListeningModePrompt::take,
                        take(1usize),
                        d1301::structures::NoiseCanceling::take,
                        d1301::structures::IncomingCallsDuringBluetoothMode::take,
                    ),
                    (
                        take(4usize),
                        d1301::structures::TapControlsDisabled::take,
                        d1301::structures::NoiseCancelingPrompt::take,
                    ),
                ),
                |(
                    (
                        tws_status,
                        dual_battery_level,
                        dual_firmware_version,
                        serial_number,
                        _unknown1,
                        case_firmware_version,
                        _unknown2,
                        _auto_switch_once_asleep_maybe,
                        button_configuration,
                        _unknown3,
                        listening_mode,
                        default_listening_mode,
                        _unknown4,
                        low_battery_prompt,
                        _unknown5,
                        auto_power_off_prompt,
                        listening_mode_prompt,
                        _unknown6,
                        noise_canceling,
                        incoming_calls_during_bluetooth_mode,
                    ),
                    (_unknown7, tap_controls_disabled, noise_canceling_prompt),
                )| Self {
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    case_firmware_version,
                    button_configuration,
                    listening_mode,
                    default_listening_mode,
                    low_battery_prompt,
                    auto_power_off_prompt,
                    listening_mode_prompt,
                    noise_canceling,
                    incoming_calls_during_bluetooth_mode,
                    tap_controls_disabled,
                    noise_canceling_prompt,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1301StateUpdatePacket {
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
            .chain(iter::repeat_n(0, 6))
            .chain(self.case_firmware_version.bytes())
            .chain(iter::repeat_n(0, 4))
            .chain(iter::once(6)) // auto switch once asleep but unknown how it works
            .chain(
                self.button_configuration
                    .bytes(d1301::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(iter::repeat_n(0, 15))
            .chain(self.listening_mode.bytes())
            .chain(self.default_listening_mode.bytes())
            .chain(iter::repeat_n(0, 2))
            .chain(self.low_battery_prompt.bytes())
            .chain(iter::repeat_n(0, 67))
            .chain(self.auto_power_off_prompt.bytes())
            .chain(self.listening_mode_prompt.bytes())
            .chain(iter::once(0))
            .chain(self.noise_canceling.bytes())
            .chain(self.incoming_calls_during_bluetooth_mode.bytes())
            .chain(iter::repeat_n(0, 4))
            .chain(self.tap_controls_disabled.bytes())
            .chain(self.noise_canceling_prompt.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<D1301State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<D1301State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: D1301StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<D1301State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
