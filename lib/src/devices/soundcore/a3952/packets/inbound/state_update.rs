use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3952::{self, state::A3952State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
                CommonEqualizerConfiguration, CustomHearId, DualBattery, DualFirmwareVersion, Ldac,
                SerialNumber, TouchTone, TwsStatus, WearingDetection, WearingTone,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3952StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub hear_id: CustomHearId<2, 10>,
    pub buttons: ButtonStatusCollection<6>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3952::structures::SoundModes,
    pub touch_tone: TouchTone,
    pub wear_detection: WearingDetection,
    pub case_battery_level: CaseBatteryLevel,
    pub wearing_tone: WearingTone,
    pub auto_power_off: AutoPowerOff,
    pub ldac: Ldac,
}

impl Default for A3952StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            age_range: Default::default(),
            hear_id: Default::default(),
            buttons: a3952::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            wear_detection: Default::default(),
            case_battery_level: Default::default(),
            wearing_tone: Default::default(),
            auto_power_off: Default::default(),
            ldac: Default::default(),
        }
    }
}

impl FromPacketBody for A3952StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3952 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBattery::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        CommonEqualizerConfiguration::take,
                        AgeRange::take,
                        CustomHearId::take_with_music_genre_at_end,
                        take(1usize), // unknown
                        ButtonStatusCollection::take(
                            a3952::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        take(4usize), // unknown
                        AmbientSoundModeCycle::take,
                        a3952::structures::SoundModes::take,
                        TouchTone::take,
                        WearingDetection::take,
                        take(1usize), // unknown
                        CaseBatteryLevel::take,
                        take(2usize), // unknown
                        Ldac::take,
                    ),
                    (
                        take(2usize), // unknown
                        WearingTone::take,
                        take(1usize), // unknown
                        AutoPowerOff::take,
                    ),
                ),
                |(
                    (
                        tws_status,
                        battery,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                        age_range,
                        hear_id,
                        _unknown1,
                        buttons,
                        _unknown2,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        touch_tone,
                        wear_detection,
                        _unknown3,
                        case_battery_level,
                        _unknown4,
                        ldac,
                    ),
                    (_unknown5, wearing_tone, _unknown6, auto_power_off),
                )| Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    age_range,
                    hear_id,
                    buttons,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    touch_tone,
                    wear_detection,
                    case_battery_level,
                    wearing_tone,
                    auto_power_off,
                    ldac,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3952StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(self.age_range.0))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(iter::once(0)) // unknown
            .chain(
                self.buttons
                    .bytes(a3952::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain([0; 4])
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain(self.touch_tone.bytes())
            .chain(self.wear_detection.bytes())
            .chain(iter::once(0))
            .chain(self.case_battery_level.bytes())
            .chain([0; 5])
            .chain(self.wearing_tone.bytes())
            .chain(iter::once(0))
            .chain(self.auto_power_off.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<A3952State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3952State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3952StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3952State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
