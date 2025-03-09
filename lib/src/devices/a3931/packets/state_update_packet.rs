use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::tuple,
};
use tokio::sync::watch;

use crate::{
    devices::{
        a3931::{device_profile::A3931_DEVICE_PROFILE, state::A3931State},
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
                DualBattery, EqualizerConfiguration, InternalMultiButtonConfiguration, SoundModes,
                StereoEqualizerConfiguration, TwsStatus,
            },
        },
    },
    soundcore_device::device::Packet,
};

// A3931 and A3935 and A3931XR and A3935W
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3931StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub button_configuration: InternalMultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off_on: bool,
    pub auto_power_off_index: u8, // 0 to 3
}

impl From<A3931StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3931StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3931_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: None,
            button_configuration: Some(packet.button_configuration.into()),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3931StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3931StateUpdatePacket, E> {
        context(
            "a3931 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    InternalMultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    le_u8,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    touch_tone,
                    auto_power_off_on,
                    auto_power_off_index,
                )| {
                    A3931StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        touch_tone,
                        auto_power_off_on,
                        auto_power_off_index,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3931StateUpdatePacket {
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
            .chain(self.button_configuration.bytes())
            .chain([
                self.sound_modes.ambient_sound_mode as u8,
                self.sound_modes.noise_canceling_mode as u8,
                self.sound_modes.transparency_mode as u8,
                self.sound_modes.custom_noise_canceling.value(),
            ])
            .chain([
                self.side_tone as u8,
                self.touch_tone as u8,
                self.auto_power_off_on as u8,
                self.auto_power_off_index,
            ])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3931State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3931State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3931StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3931State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            StateUpdatePacket::command(),
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
