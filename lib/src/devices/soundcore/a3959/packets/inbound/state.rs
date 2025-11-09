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
        a3959,
        common::{
            self,
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3959State {
    pub tws_status: common::structures::TwsStatus,
    pub dual_battery: common::structures::DualBattery,
    pub dual_firmware_version: common::structures::DualFirmwareVersion,
    pub serial_number: common::structures::SerialNumber,
    pub equalizer_configuration: common::structures::EqualizerConfiguration<1, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: common::structures::AmbientSoundModeCycle,
    pub sound_modes: a3959::structures::SoundModes,
    pub touch_tone: common::structures::TouchTone,
    pub auto_power_off: common::structures::AutoPowerOff,
    pub low_battery_prompt: bool,
    pub gaming_mode: bool,
}

impl Default for A3959State {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3959::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            auto_power_off: Default::default(),
            low_battery_prompt: Default::default(),
            gaming_mode: Default::default(),
        }
    }
}

impl FromPacketBody for A3959State {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3959 state update packet",
            all_consuming(map(
                (
                    common::structures::TwsStatus::take,
                    common::structures::DualBattery::take,
                    common::structures::DualFirmwareVersion::take,
                    common::structures::SerialNumber::take,
                    common::structures::EqualizerConfiguration::take,
                    take(10usize),
                    take(1usize),
                    ButtonStatusCollection::take(
                        a3959::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    common::structures::AmbientSoundModeCycle::take,
                    a3959::structures::SoundModes::take,
                    take(1usize),
                    common::structures::TouchTone::take,
                    take(2usize),
                    common::structures::AutoPowerOff::take,
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
                    _unknown1,
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

impl ToPacket for A3959State {
    type DirectionMarker = packet::InboundMarker;

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
            .chain([0; 10])
            .chain([0])
            .chain(
                self.button_configuration
                    .bytes(a3959::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0])
            .chain([self.touch_tone.0.into()])
            .chain([0, 0])
            .chain(self.auto_power_off.bytes())
            .chain([self.low_battery_prompt as u8, self.gaming_mode as u8])
            .chain([0; 12])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<a3959::state::A3959State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<a3959::state::A3959State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3959State = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<a3959::state::A3959State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
