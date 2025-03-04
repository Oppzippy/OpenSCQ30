use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
use tokio::sync::watch;

use crate::{
    devices::{
        a3033::{device_profile::A3033_DEVICE_PROFILE, state::A3033State},
        standard::{
            modules::ModuleCollection,
            packet_manager::PacketHandler,
            packets::{
                inbound::{
                    InboundPacket, TryIntoInboundPacket, state_update_packet::StateUpdatePacket,
                },
                outbound::OutboundPacket,
                parsing::take_bool,
            },
            structures::{EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery},
        },
    },
    soundcore_device::device::Packet,
};

// A3033 and A3033EU
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3033StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wear_detection: bool,
}

impl From<A3033StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3033StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3033_DEVICE_PROFILE,
            tws_status: None,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: None,
            age_range: None,
            gender: None,
            hear_id: None,
            button_configuration: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3033StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3033StateUpdatePacket, E> {
        context(
            "a3033 state update packet",
            all_consuming(map(
                tuple((
                    SingleBattery::take,
                    EqualizerConfiguration::take(8),
                    FirmwareVersion::take,
                    SerialNumber::take,
                    take_bool,
                )),
                |(
                    battery,
                    equalizer_configuration,
                    firmware_version,
                    serial_number,
                    wear_detection,
                )| {
                    A3033StateUpdatePacket {
                        battery,
                        equalizer_configuration,
                        firmware_version,
                        serial_number,
                        wear_detection,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3033StateUpdatePacket {
    fn command(&self) -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.is_charging as u8, self.battery.level.0]
            .into_iter()
            .chain(self.equalizer_configuration.profile_id().to_le_bytes())
            .chain(self.equalizer_configuration.volume_adjustments().bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.0.as_bytes().iter().cloned())
            .chain([self.wear_detection as u8])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3033State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3033State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3033StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3033State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            StateUpdatePacket::command(),
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
