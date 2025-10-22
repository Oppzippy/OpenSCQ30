use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3945::{self, state::A3945State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                BatteryLevel, DualBattery, DualFirmwareVersion, EqualizerConfiguration,
                SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
            },
        },
    },
};

// A3945 only
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3945StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub touch_tone: TouchTone,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub bass_up_switch: bool,
    pub device_color: u8,
}

impl Default for A3945StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3945::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            touch_tone: Default::default(),
            wear_detection_switch: Default::default(),
            game_mode_switch: Default::default(),
            charging_case_battery_level: Default::default(),
            bass_up_switch: Default::default(),
            device_color: Default::default(),
        }
    }
}

impl FromPacketBody for A3945StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3945 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    ButtonStatusCollection::take(
                        a3945::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    TouchTone::take,
                    take_bool,
                    take_bool,
                    BatteryLevel::take,
                    take_bool,
                    le_u8,
                ),
                |(
                    tws_status,
                    battery,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    button_configuration,
                    touch_tone,
                    wear_detection_switch,
                    game_mode_switch,
                    charging_case_battery_level,
                    bass_up_switch,
                    device_color,
                )| {
                    Self {
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        touch_tone,
                        wear_detection_switch,
                        game_mode_switch,
                        charging_case_battery_level,
                        bass_up_switch,
                        device_color,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3945StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.to_string().into_bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(
                self.button_configuration
                    .bytes(a3945::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.touch_tone.bytes())
            .chain([
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
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3945StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3945State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3945StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3945StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
