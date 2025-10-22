use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{map, map_opt, opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3028::state::A3028State,
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryIntoPacket},
                outbound::IntoPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, AutoPowerOff, BasicHearId, EqualizerConfiguration, FirmwareVersion,
                Gender, SerialNumber, SingleBattery, SoundModes,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3028StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId<2, 8>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub extra_fields: Option<ExtraFields>,
}

impl Default for A3028StateUpdatePacket {
    fn default() -> Self {
        Self {
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            gender: Default::default(),
            age_range: Default::default(),
            hear_id: Default::default(),
            sound_modes: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            extra_fields: Some(Default::default()),
        }
    }
}

impl FromPacketBody for A3028StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3028 state update packet",
            map(
                (
                    SingleBattery::take,
                    EqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    SoundModes::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    opt(ExtraFields::take),
                ),
                |(
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                    extra_fields,
                )| {
                    Self {
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        sound_modes,
                        firmware_version,
                        serial_number,
                        extra_fields,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl IntoPacket for A3028StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.level.0, self.battery.is_charging as u8]
            .into_iter()
            .chain(self.equalizer_configuration.bytes())
            .chain([self.gender.0])
            .chain([self.age_range.0])
            .chain(self.hear_id.bytes())
            .chain(self.sound_modes.bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.as_str().as_bytes().iter().copied())
            .chain(
                self.extra_fields
                    .as_ref()
                    .map(|extra_fields| extra_fields.bytes().into_iter())
                    .into_iter()
                    .flatten(),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExtraFields {
    pub unknown1: u8,
    pub touch_control: bool,
    pub dual_connections: bool,
    pub auto_power_off: AutoPowerOff,
    pub ambient_sound_prompt_tone: bool,
    pub battery_alert_prompt_tone: bool,
}

impl ExtraFields {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map_opt(
            (
                le_u8,
                take_bool,
                take_bool,
                AutoPowerOff::take,
                take_bool,
                take_bool,
            ),
            |(
                unknown1,
                touch_control,
                dual_connections,
                auto_power_off,
                ambient_sound_prompt_tone,
                battery_alert_prompt_tone,
            )| {
                Some(Self {
                    unknown1,
                    touch_control,
                    dual_connections,
                    auto_power_off,
                    ambient_sound_prompt_tone,
                    battery_alert_prompt_tone,
                })
            },
        )
        .parse_complete(input)
    }

    fn bytes(&self) -> impl IntoIterator<Item = u8> {
        [
            self.unknown1,
            self.touch_control.into(),
            self.dual_connections.into(),
        ]
        .into_iter()
        .chain(self.auto_power_off.bytes())
        .chain([
            self.ambient_sound_prompt_tone.into(),
            self.battery_alert_prompt_tone.into(),
        ])
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3028State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3028State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3028StateUpdatePacket = packet.try_into_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3028State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::structures::{
        AmbientSoundMode, CustomNoiseCanceling, EqualizerConfiguration, NoiseCancelingMode,
        PresetEqualizerProfile, SoundModes, VolumeAdjustments,
    };

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3028StateUpdatePacket::default().into_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3028StateUpdatePacket = packet.try_into_packet().unwrap();
    }

    #[test]
    fn it_parses_packet_with_preset_eq() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x35,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(
            SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                noise_canceling_mode: NoiseCancelingMode::Transport,
                transparency_mode: Default::default(),
                custom_noise_canceling: CustomNoiseCanceling::new(0),
            },
            packet.sound_modes,
        );
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::Acoustic,
                [Vec::new()]
            ),
            packet.equalizer_configuration
        );
    }

    #[test]
    fn it_parses_packet_with_invalid_preset_eq_id_as_a_custom_profile() {
        let input: &[u8] = &[
            //                                                                profile id 0x50 is
            //                                                                invalid
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x50, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x84,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-60, 60, 23, 40, 22, 60, -4, 16]),
            packet
                .equalizer_configuration
                .volume_adjustments_channel_1(),
        );
    }

    #[test]
    fn it_parses_packet_with_custom_eq() {
        let input: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-60, 60, 23, 40, 22, 60, -4, 16]),
            packet
                .equalizer_configuration
                .volume_adjustments_channel_1(),
        );
    }

    #[test]
    fn it_parses_packet_with_a_4_at_byte_offset_9() {
        // It's usually a 5 but sometimes a 4 at that byte offset. I don't know why, but it
        // doesn't seem to cause any problems.
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x04, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x2f,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(
            AmbientSoundMode::Normal,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            packet.sound_modes.noise_canceling_mode
        );
        assert!(packet.equalizer_configuration.preset_profile().is_none());
        assert_eq!(
            &VolumeAdjustments::new([-60, 60, 23, 40, 22, 60, -4, 16]),
            packet
                .equalizer_configuration
                .volume_adjustments_channel_1(),
        );
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_ambient_sound_mode() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //          valid range is 0x00 to 0x02
            0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x31,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let (_, packet) = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body).unwrap();
        assert_eq!(
            AmbientSoundMode::default(),
            packet.sound_modes.ambient_sound_mode
        );
    }

    #[test]
    fn it_falls_back_to_default_with_invalid_noise_canceling_mode() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //                valid range is 0x00 to 0x03
            0x00, 0x00, 0x01, 0x04, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x33,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let (_, packet) = A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body).unwrap();
        assert_eq!(
            NoiseCancelingMode::default(),
            packet.sound_modes.noise_canceling_mode
        );
    }

    #[test]
    fn it_parses_packet_from_github_issue_141() {
        let input: &[u8] = &[
            9, 255, 0, 0, 1, 1, 1, // command
            77, 0, // length
            5, 0, // battery
            14, 0, 120, 150, 150, 140, 160, 170, 150, 160, // equalizer configuration
            0,   //gender
            0,   //age range
            0,   // hear id is enabled
            0, 0, 0, 0, 0, 0, 0, 0, // hear id left
            0, 0, 0, 0, 0, 0, 0, 0, // hear id right
            0, 0, 0, 0, // hear id time
            0, 0, 0, 0, // sound modes
            48, 52, 46, 51, 51, // firmware version
            51, 48, 50, 56, 54, 68, 67, 56, 57, 51, 52, 52, 52, 55, 57, 56, // serial number
            0, 0, 1, 1, 1, 1, 1,   // 7 optional unknown bools ???
            138, // checksum
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        A3028StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .expect("it parses successfully");
    }

    // https://github.com/flathub/com.oppzippy.OpenSCQ30/issues/34
    #[test]
    fn it_parses_packet_from_flathub_issue_34() {
        let input = &[
            5, 0, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 48, 53, 46, 49, 57, 51, 48, 50, 56, 66,
            51, 66, 67, 50, 52, 56, 53, 48, 69, 56, 56, 0, 1, 1, 1, 0, 1, 1, 3, 4, 3,
        ];
        A3028StateUpdatePacket::take::<VerboseError<_>>(input).expect("it parses successfully");
    }
}
