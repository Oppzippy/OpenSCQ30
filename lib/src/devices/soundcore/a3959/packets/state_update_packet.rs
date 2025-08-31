use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3959::{
            state::A3959State,
            structures::{A3959MultiButtonConfiguration, A3959SoundModes},
        },
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket},
                outbound::OutboundPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AmbientSoundModeCycle, AutoPowerOff, DualBattery, DualFirmwareVersion,
                EqualizerConfiguration, SerialNumber, TouchTone, TwsStatus,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3959StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub button_configuration: A3959MultiButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: A3959SoundModes,
    pub touch_tone: TouchTone,
    pub auto_power_off: AutoPowerOff,
    pub low_battery_prompt: bool,
    pub gaming_mode: bool,
}

impl InboundPacket for A3959StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3959 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    take(1usize),
                    A3959MultiButtonConfiguration::take,
                    AmbientSoundModeCycle::take,
                    A3959SoundModes::take,
                    take(1usize),
                    TouchTone::take,
                    take(2usize),
                    AutoPowerOff::take,
                    take_bool,
                    take_bool,
                    take(12usize),
                ),
                |(
                    tws_status,
                    dual_battery,
                    dual_firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown2,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown3,
                    touch_tone,
                    _unknown4,
                    auto_power_off,
                    low_battery_prompt,
                    gaming_mode,
                    _unknown5,
                )| {
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        touch_tone,
                        auto_power_off,
                        low_battery_prompt,
                        gaming_mode,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3959StateUpdatePacket {
    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.dual_battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([0])
            .chain(self.button_configuration.bytes())
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0])
            .chain([self.touch_tone as u8])
            .chain([0, 0])
            .chain(self.auto_power_off.bytes())
            .chain([self.low_battery_prompt as u8, self.gaming_mode as u8])
            .chain([0; 12])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3959State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3959State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3959StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3959State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
