use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::all_consuming,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3936::{self, state::A3936State, structures::A3936SoundModes},
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
                AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel, CustomHearId,
                DualBattery, DualFirmwareVersion, EqualizerConfiguration, GamingMode, SerialNumber,
                TouchTone, TwsStatus, VolumeAdjustments,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

// A3936
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3936StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId<2, 10>,
    pub sound_modes: A3936SoundModes,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub button_configuration: ButtonStatusCollection<6>,
    pub touch_tone: TouchTone,
    pub case_battery_level: CaseBatteryLevel,
    pub color: u8,
    pub ldac: bool,
    pub supports_two_cnn_switch: bool,
    pub auto_power_off: AutoPowerOff,
    pub gaming_mode: GamingMode,
}

impl Default for A3936StateUpdatePacket {
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
            custom_hear_id: CustomHearId {
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
            },
            sound_modes: Default::default(),
            ambient_sound_mode_cycle: Default::default(),
            button_configuration: a3936::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            touch_tone: Default::default(),
            case_battery_level: Default::default(),
            color: Default::default(),
            ldac: Default::default(),
            supports_two_cnn_switch: Default::default(),
            auto_power_off: Default::default(),
            gaming_mode: Default::default(),
        }
    }
}

impl FromPacketBody for A3936StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3936 state update packet",
            all_consuming(|input| {
                let (input, tws_status) = TwsStatus::take(input)?;
                let (input, battery) = DualBattery::take(input)?;
                let (input, dual_firmware_version) = DualFirmwareVersion::take(input)?;
                let (input, serial_number) = SerialNumber::take(input)?;
                let (input, equalizer_configuration) = EqualizerConfiguration::take(input)?;
                let (input, age_range) = AgeRange::take(input)?;
                let (input, custom_hear_id) = CustomHearId::take_without_music_type(input)?;

                // For some reason, an offset value is taken before the custom button model, which refers to how many bytes
                // until the next data to be read. This offset includes the length of the custom button model. Presumably,
                // there are some extra bytes between the button model and the beginning of the next data to be parsed?
                let (input, skip_offset) = le_u8(input)?;
                let remaining_before_button_configuration = input.len();
                let (input, button_configuration) = ButtonStatusCollection::take(
                    a3936::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                )(input)?;
                let button_configuration_size = remaining_before_button_configuration - input.len();
                let (input, _) = take(
                    (skip_offset as usize)
                        // subtract an extra 1 since we want the number of bytes to discard, not
                        // the offset to the first byte to read
                        .checked_sub(button_configuration_size + 2)
                        .unwrap_or_default(),
                )(input)?;

                let (input, ambient_sound_mode_cycle) = AmbientSoundModeCycle::take(input)?;
                let (input, sound_modes) = A3936SoundModes::take(input)?;
                let (input, touch_tone) = TouchTone::take(input)?;
                let (input, case_battery_level) = CaseBatteryLevel::take(input)?;
                let (input, color) = le_u8(input)?;
                let (input, ldac) = take_bool(input)?;
                let (input, supports_two_cnn_switch) = take_bool(input)?;
                let (input, auto_power_off) = AutoPowerOff::take(input)?;
                let (input, gaming_mode) = GamingMode::take(input)?;
                let (input, _) = take(12usize)(input)?;
                Ok((
                    input,
                    Self {
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        age_range,
                        custom_hear_id,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        button_configuration,
                        touch_tone,
                        case_battery_level,
                        color,
                        ldac,
                        supports_two_cnn_switch,
                        auto_power_off,
                        gaming_mode,
                    },
                ))
            }),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3936StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.to_string().into_bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([self.age_range.0])
            .chain(
                [self.custom_hear_id.is_enabled as u8]
                    .into_iter()
                    .chain(
                        self.custom_hear_id
                            .volume_adjustments
                            .iter()
                            .flat_map(|v| v.bytes()),
                    )
                    .chain(self.custom_hear_id.time.to_le_bytes())
                    .chain([self.custom_hear_id.hear_id_type.0])
                    .chain(
                        self.custom_hear_id
                            .custom_volume_adjustments
                            .as_ref()
                            .unwrap()
                            .iter()
                            .flat_map(|v| v.bytes()),
                    )
                    .chain([0, 0]),
            )
            .chain([0]) // TODO skip offset
            .chain(
                self.button_configuration
                    .bytes(a3936::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([
                self.touch_tone as u8,
                self.case_battery_level.0.0,
                self.color,
                self.ldac as u8,
                self.supports_two_cnn_switch as u8,
            ])
            .chain(self.auto_power_off.bytes())
            .chain(self.gaming_mode.bytes())
            .chain([0; 12])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3936State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3936State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3936StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3936State> {
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

    use crate::devices::soundcore::common::packet::inbound::FromPacketBody;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3936StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3936StateUpdatePacket = packet.try_to_packet().unwrap();
    }

    #[test]
    pub fn it_parses_a_known_good_packet() {
        let input = &[
            0x9, 0xff, 0x0, 0x0, 0x1, 0x1, 0x1, 0x99, 0x0, 0x1, 0x1, 0x5, 0x5, 0x1, 0x1, 0x30,
            0x34, 0x2e, 0x31, 0x39, 0x30, 0x34, 0x2e, 0x31, 0x39, 0x33, 0x39, 0x33, 0x36, 0x61,
            0x34, 0x37, 0x37, 0x35, 0x38, 0x34, 0x37, 0x30, 0x33, 0x36, 0x36, 0x0, 0x0, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x3c, 0x1, 0x0, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c,
            0x3c, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x65, 0x26, 0x8d,
            0xaf, 0x2, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x3c, 0x3c, 0x0, 0x0, 0xe, 0x1, 0x11, 0x1, 0x0,
            0x11, 0x63, 0x11, 0x66, 0x11, 0x49, 0x11, 0x44, 0x7, 0x2, 0x32, 0x0, 0x1, 0x0, 0x0,
            0x0, 0x4, 0x31, 0x0, 0x1, 0x1, 0x0, 0x0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xdd,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        A3936StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .expect("it should parse successfully as a A3936 state update packet");
    }

    #[test]
    fn it_parses_packet_from_github_issue_157() {
        let input = &[
            0x9, 0xff, 0x0, 0x0, 0x1, 0x1, 0x1, 0x99, 0x0, 1, 1, 5, 5, 1, 1, 48, 53, 46, 51, 51,
            48, 53, 46, 51, 51, 51, 57, 51, 54, 56, 56, 48, 101, 56, 53, 48, 49, 53, 55, 97, 97, 0,
            0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
            120, 120, 120, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0,
            17, 0, 0, 17, 99, 17, 102, 17, 68, 17, 68, 7, 1, 48, 0, 0, 0, 0, 0, 85, 49, 0, 1, 1, 0,
            0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 66,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        A3936StateUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .expect("it should parse successfully as a A3936 state update packet");
    }
}
