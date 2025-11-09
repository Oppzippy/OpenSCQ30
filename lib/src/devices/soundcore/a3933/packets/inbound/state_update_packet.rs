use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{all_consuming, opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::pair,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3933::{self, state::A3933State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, AmbientSoundModeCycle, CaseBatteryLevel, CustomHearId, DualBattery,
                DualFirmwareVersion, EqualizerConfiguration, GamingMode, SerialNumber, SoundModes,
                TouchTone, TwsStatus, VolumeAdjustments, WearingDetection,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

// A3933 and A3939
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3933StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub hear_id: Option<CustomHearId<2, 10>>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: SoundModes,
    pub touch_tone: TouchTone,
    pub wearing_detection: WearingDetection,
    pub gaming_mode: GamingMode,
    pub case_battery_level: CaseBatteryLevel,
    pub device_color: u8,
    pub wind_noise_detection: bool,
}

impl Default for A3933StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: EqualizerConfiguration::new_custom_profile([
                VolumeAdjustments::new([0; 10]),
                VolumeAdjustments::new([0; 10]),
            ]),
            age_range: Default::default(),
            hear_id: Some(CustomHearId {
                is_enabled: Default::default(),
                volume_adjustments: [
                    VolumeAdjustments::new([0; 10]),
                    VolumeAdjustments::new([0; 10]),
                ],
                time: Default::default(),
                hear_id_type: Default::default(),
                hear_id_music_type: Default::default(),
                custom_volume_adjustments: Some([
                    VolumeAdjustments::new([0; 10]),
                    VolumeAdjustments::new([0; 10]),
                ]),
                hear_id_preset_profile_id: Default::default(),
            }),
            button_configuration: a3933::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            wearing_detection: Default::default(),
            gaming_mode: Default::default(),
            case_battery_level: Default::default(),
            device_color: Default::default(),
            wind_noise_detection: Default::default(),
        }
    }
}

impl FromPacketBody for A3933StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3933 state update packet",
            all_consuming(|input| {
                let (
                    input,
                    (
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        age_range,
                    ),
                ) = (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    AgeRange::take,
                )
                    .parse_complete(input)?;

                let (input, hear_id) = if !age_range.supports_hear_id() {
                    let (input, _) = take(48usize)(input)?;
                    (input, None)
                } else {
                    let (input, hear_id) = CustomHearId::take_without_music_type(input)?;
                    (input, Some(hear_id))
                };

                let (
                    input,
                    (button_configuration, ambient_sound_mode_cycle, sound_modes, _unknown, extra),
                ) = (
                    ButtonStatusCollection::take(
                        a3933::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    AmbientSoundModeCycle::take,
                    SoundModes::take,
                    // Unsure if these two unknown bytes should be inside or outside the optional
                    context("unknown bytes", take(2usize)), // unknown bytes
                    opt(pair(Self::take_optional_extra_data, take(3usize))),
                )
                    .parse_complete(input)?;

                Ok((
                    input,
                    Self {
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        age_range,
                        hear_id,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        // TODO make these fields optional?
                        touch_tone: extra.map(|e| e.0.0).unwrap_or_default(),
                        wearing_detection: extra.map(|e| e.0.1).unwrap_or_default(),
                        gaming_mode: extra.map(|e| e.0.2).unwrap_or_default(),
                        case_battery_level: extra.map(|e| e.0.3).unwrap_or_default(),
                        device_color: extra.map(|e| e.0.5).unwrap_or_default(),
                        wind_noise_detection: extra.map(|e| e.0.6).unwrap_or_default(),
                    },
                ))
            }),
        )
        .parse_complete(input)
    }
}

impl A3933StateUpdatePacket {
    fn take_optional_extra_data<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<
        &'a [u8],
        (
            TouchTone,
            WearingDetection,
            GamingMode,
            CaseBatteryLevel,
            u8,
            u8,
            bool,
        ),
        E,
    > {
        context(
            "extra data",
            (
                TouchTone::take,
                WearingDetection::take,
                GamingMode::take,
                CaseBatteryLevel::take,
                le_u8,     // what is this byte?
                le_u8,     // device color
                take_bool, // wind noise detection
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3933StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
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
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.0.as_bytes().iter().copied())
            .chain(self.equalizer_configuration.bytes())
            .chain([self.age_range.0])
            .chain(self.hear_id.as_ref().map_or_else(
                || vec![0; 48],
                |hear_id| {
                    [hear_id.is_enabled as u8]
                        .into_iter()
                        .chain(hear_id.volume_adjustments.iter().flat_map(|v| v.bytes()))
                        .chain(hear_id.time.to_le_bytes())
                        .chain([hear_id.hear_id_type.0])
                        .chain(
                            hear_id
                                .custom_volume_adjustments
                                .as_ref()
                                .unwrap()
                                .iter()
                                .flat_map(|v| v.bytes()),
                        )
                        .chain([0, 0])
                        .collect()
                },
            ))
            .chain(
                self.button_configuration
                    .bytes(a3933::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain([self.ambient_sound_mode_cycle.into()])
            .chain(self.sound_modes.bytes())
            .chain([0, 0])
            .chain(self.touch_tone.bytes())
            .chain(self.wearing_detection.bytes())
            .chain(self.gaming_mode.bytes())
            .chain([
                self.case_battery_level.0.0,
                0,
                self.device_color,
                self.wind_noise_detection as u8,
            ])
            .chain([0, 0, 0])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3933State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3933State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3933StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3933State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::{
        a3933::packets::inbound::A3933StateUpdatePacket,
        common::{
            packet::{
                self,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            structures::{
                AmbientSoundMode, BatteryLevel, CustomNoiseCanceling, EqualizerConfiguration,
                FirmwareVersion, HostDevice, IsBatteryCharging, PresetEqualizerProfile,
                SingleBattery, TwsStatus,
            },
        },
    };

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3933StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3933StateUpdatePacket = packet.try_to_packet().unwrap();
    }

    #[test]
    fn it_parses_packet() {
        // state update
        // length 142
        // host device 1
        // tws status 1
        // both batteries level 4
        // both batteries not charging
        // both firmware version 02.61
        // serial number 39392A7FCC2F12AC
        // soundcore signature
        // no hear id
        let input: &[u8] = &[
            9, 255, 0, 0, 1, 1, 1, 142, 0, 1, 1, 4, 4, 0, 0, 48, 50, 46, 54, 49, 48, 50, 46, 54,
            49, 51, 57, 51, 57, 50, 65, 55, 70, 67, 67, 50, 70, 49, 50, 65, 67, 0, 0, 120, 120,
            120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99,
            1, 82, 1, 102, 1, 84, 1, 1, 1, 0, 7, 0, 0, 0, 10, 255, 255, 0, 255, 0, 0, 0, 51, 255,
            255, 255, 255, 102,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let (_, packet) = A3933StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .expect("should parse packet");

        assert_eq!(
            TwsStatus {
                is_connected: true,
                host_device: HostDevice::Right,
            },
            packet.tws_status
        );
        assert_eq!(
            SingleBattery {
                level: BatteryLevel(4),
                is_charging: IsBatteryCharging::No,
            },
            packet.battery.left,
        );
        assert_eq!(
            FirmwareVersion::new(2, 61),
            packet.dual_firmware_version.left().unwrap()
        );
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
                [vec![0, 0], vec![255 - 120, 255 - 120]] // subtract 120 to convert from byte value to volume adjustment
            ),
            packet.equalizer_configuration
        );
        assert_eq!(
            AmbientSoundMode::NoiseCanceling,
            packet.sound_modes.ambient_sound_mode
        );
        assert_eq!(
            CustomNoiseCanceling::new(10),
            packet.sound_modes.custom_noise_canceling
        );
    }
}
