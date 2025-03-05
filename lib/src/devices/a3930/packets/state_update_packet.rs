use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u16,
    sequence::tuple,
};
use tokio::sync::watch;

use crate::{
    devices::{
        a3930::{device_profile::A3930_DEVICE_PROFILE, state::A3930State},
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
                AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
                InternalMultiButtonConfiguration, SoundModes, StereoEqualizerConfiguration,
                StereoVolumeAdjustments, TwsStatus, VolumeAdjustments,
            },
        },
    },
    soundcore_device::device::Packet,
};

// A3930
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3930StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
    pub button_configuration: InternalMultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    // length >= 94
    pub hear_id_eq_index: Option<u16>,
}

impl From<A3930StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3930StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3930_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.custom_hear_id.into()),
            button_configuration: Some(packet.button_configuration.into()),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3930StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3930StateUpdatePacket, E> {
        context(
            "a3930 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    InternalMultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    opt(le_u16),
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    custom_hear_id,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    hear_id_eq_index,
                )| {
                    A3930StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        hear_id_eq_index,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3930StateUpdatePacket {
    fn command(&self) -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain([
                self.battery.left.is_charging as u8,
                self.battery.right.is_charging as u8,
                self.battery.left.level.0,
                self.battery.right.level.0,
            ])
            .chain(self.equalizer_configuration.profile_id().to_le_bytes())
            .chain(self.equalizer_configuration.volume_adjustments().bytes())
            .chain(self.equalizer_configuration.volume_adjustments().bytes())
            .chain([self.gender.0, self.age_range.0])
            .chain([self.custom_hear_id.is_enabled as u8])
            .chain(self.custom_hear_id.volume_adjustments.bytes())
            .chain(self.custom_hear_id.time.to_le_bytes())
            .chain([
                self.custom_hear_id.hear_id_type.0,
                self.custom_hear_id.hear_id_music_type.0,
            ])
            .chain(
                self.custom_hear_id
                    .custom_volume_adjustments
                    .as_ref()
                    .unwrap_or(&StereoVolumeAdjustments {
                        left: VolumeAdjustments::new(vec![0f64; 8]).unwrap(),
                        right: VolumeAdjustments::new(vec![0f64; 8]).unwrap(),
                    })
                    .bytes(),
            )
            .chain(self.button_configuration.bytes())
            .chain([
                self.sound_modes.ambient_sound_mode as u8,
                self.sound_modes.noise_canceling_mode as u8,
                self.sound_modes.transparency_mode as u8,
                self.sound_modes.custom_noise_canceling.value(),
            ])
            .chain([self.side_tone as u8])
            .chain(
                self.hear_id_eq_index
                    .map(u16::to_le_bytes)
                    .into_iter()
                    .flatten(),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3930State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3930State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3930StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3930State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            StateUpdatePacket::command(),
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
