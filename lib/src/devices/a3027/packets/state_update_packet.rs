use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
use tokio::sync::watch;

use crate::{
    devices::{
        a3027::state::A3027State,
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
            structures::{
                AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender,
                SerialNumber, SingleBattery, SoundModes,
            },
        },
    },
    soundcore_device::device::Packet,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3027StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wear_detection: bool,
    // if length >= 72
    pub touch_func: Option<bool>,
}

impl InboundPacket for A3027StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3027StateUpdatePacket, E> {
        context(
            "a3027 state update packet",
            all_consuming(map(
                tuple((
                    SingleBattery::take,
                    EqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    SoundModes::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    take_bool,
                    opt(take_bool),
                )),
                |(
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                    wear_detection,
                    touch_func,
                )| {
                    A3027StateUpdatePacket {
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        sound_modes,
                        firmware_version,
                        serial_number,
                        wear_detection,
                        touch_func,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3027StateUpdatePacket {
    fn command(&self) -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.level.0, self.battery.is_charging as u8]
            .into_iter()
            .chain(self.equalizer_configuration.profile_id().to_le_bytes())
            .chain(self.equalizer_configuration.volume_adjustments().bytes())
            .chain([self.gender.0])
            .chain([self.age_range.0])
            .chain([self.hear_id.is_enabled as u8])
            .chain(self.hear_id.volume_adjustments.bytes())
            .chain(self.hear_id.time.to_le_bytes())
            .chain([
                self.sound_modes.ambient_sound_mode.id(),
                self.sound_modes.noise_canceling_mode.id(),
                self.sound_modes.transparency_mode.id(),
                self.sound_modes.custom_noise_canceling.value(),
            ])
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.as_str().as_bytes().iter().cloned())
            .chain([self.wear_detection as u8])
            .chain(self.touch_func.map(|v| v as u8))
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3027State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3027State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3027StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3027State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            StateUpdatePacket::command(),
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::{
        devices::standard::packets::{
            inbound::{TryIntoInboundPacket, take_inbound_packet_header},
            outbound::OutboundPacketBytesExt,
        },
        soundcore_device::device::Packet,
    };

    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let bytes = A3027StateUpdatePacket::default().bytes();
        let (body, command) = take_inbound_packet_header::<VerboseError<_>>(&bytes).unwrap();
        let packet = Packet {
            command,
            body: body.to_vec(),
        };
        let _: A3027StateUpdatePacket = packet.try_into_inbound_packet().unwrap();
    }
}
