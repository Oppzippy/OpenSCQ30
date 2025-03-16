use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, opt},
    error::{ContextError, ParseError, context},
    number::complete::{le_u8, le_u16},
    sequence::tuple,
};
use tokio::sync::watch;

use crate::{
    devices::{
        a3951::state::A3951State,
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

// A3951
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3951StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
    pub button_configuration: InternalMultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub wear_detection: bool,
    pub touch_tone: bool,
    pub hear_id_eq_preset: Option<u16>,
    pub supports_new_battery: bool, // yes if packet is >98, don't parse
    pub left_new_battery: u8,       // 0 to 9
    pub right_new_battery: u8,      // 0 to 9
}

impl InboundPacket for A3951StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3951StateUpdatePacket, E> {
        context(
            "a3951 state update packet",
            all_consuming(|input| {
                // required fields
                let (
                    input,
                    (
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        wear_detection,
                        touch_tone,
                    ),
                ) = tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    InternalMultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool, // side tone
                    take_bool, // wear detection
                    take_bool, // touch tone
                ))(input)?;

                // >=96 length optional fields
                let (input, hear_id_eq_preset) = opt(le_u16)(input)?;

                // >=98 length optional fields
                let (input, new_battery) = opt(tuple((le_u8, le_u8)))(input)?;

                Ok((
                    input,
                    A3951StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        wear_detection,
                        touch_tone,
                        hear_id_eq_preset,
                        supports_new_battery: new_battery.is_some(),
                        left_new_battery: new_battery.as_ref().map(|b| b.0).unwrap_or_default(),
                        right_new_battery: new_battery.as_ref().map(|b| b.1).unwrap_or_default(),
                    },
                ))
            }),
        )(input)
    }
}

impl OutboundPacket for A3951StateUpdatePacket {
    fn command(&self) -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    // let (
    //     input,
    //     (
    //         tws_status,
    //         battery,
    //         equalizer_configuration,
    //         gender,
    //         age_range,
    //         custom_hear_id,
    //         button_configuration,
    //         sound_modes,
    //         side_tone,
    //         wear_detection,
    //         touch_tone,
    //     ),
    // ) = tuple((
    //     TwsStatus::take,
    //     DualBattery::take,
    //     StereoEqualizerConfiguration::take(8),
    //     Gender::take,
    //     AgeRange::take,
    //     CustomHearId::take_with_all_fields,
    //     InternalMultiButtonConfiguration::take,
    //     SoundModes::take,
    //     take_bool, // side tone
    //     take_bool, // wear detection
    //     take_bool, // touch tone
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
            .chain([
                self.side_tone as u8,
                self.wear_detection as u8,
                self.touch_tone as u8,
            ])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3951State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3951State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3951StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3951State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            StateUpdatePacket::command(),
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
