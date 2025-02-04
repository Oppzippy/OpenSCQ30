use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

use crate::devices::{
    a3027::device_profile::A3027_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            outbound::OutboundPacket,
            parsing::take_bool,
        },
        structures::{
            AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
            SingleBattery, SoundModes,
        },
    },
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

impl From<A3027StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3027StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3027_DEVICE_PROFILE,
            tws_status: None,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
            button_configuration: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
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

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::{
        devices::standard::packets::{
            inbound::{take_inbound_packet_header, TryIntoInboundPacket},
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
