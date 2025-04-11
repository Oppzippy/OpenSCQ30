use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::tuple,
};
use tokio::sync::watch;

use crate::devices::soundcore::{
    a3945::state::A3945State,
    standard::{
        modules::ModuleCollection,
        packet_manager::PacketHandler,
        packets::{
            Packet,
            inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
            outbound::OutboundPacket,
            parsing::take_bool,
        },
        structures::{
            BatteryLevel, Command, DualBattery, DualFirmwareVersion, EqualizerConfiguration,
            MultiButtonConfiguration, SerialNumber, TwsStatus,
        },
    },
};

// A3945 only
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3945StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub button_configuration: MultiButtonConfiguration,
    pub touch_tone_switch: bool,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub bass_up_switch: bool,
    pub device_color: u8,
}

impl InboundPacket for A3945StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3945StateUpdatePacket, E> {
        context(
            "a3945 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    MultiButtonConfiguration::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    BatteryLevel::take,
                    take_bool,
                    le_u8,
                )),
                |(
                    tws_status,
                    battery,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    button_configuration,
                    touch_tone_switch,
                    wear_detection_switch,
                    game_mode_switch,
                    charging_case_battery_level,
                    bass_up_switch,
                    device_color,
                )| {
                    A3945StateUpdatePacket {
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        touch_tone_switch,
                        wear_detection_switch,
                        game_mode_switch,
                        charging_case_battery_level,
                        bass_up_switch,
                        device_color,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3945StateUpdatePacket {
    fn command(&self) -> Command {
        state_update_packet::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.to_string().into_bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(self.button_configuration.bytes())
            .chain([
                self.touch_tone_switch as u8,
                self.wear_detection_switch as u8,
                self.game_mode_switch as u8,
                self.charging_case_battery_level.0,
                self.bass_up_switch as u8,
                self.device_color,
            ])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3945State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3945State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3945StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3945State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            state_update_packet::COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
