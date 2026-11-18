use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3954::{self, state::A3954State},
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket, parsing::take_bool},
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, DualBattery, DualFirmwareVersion, Ldac, LimitHighVolume,
            LowBatteryPrompt, SerialNumber, SoundLeakCompensation, TwsStatus, WearingDetection,
            button_configuration::ButtonStatusCollection,
        },
    },
};

pub struct A3954StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_firmware_version: a3954::structures::CaseFirmwareVersion,
    pub case_battery_level: CaseBatteryLevel,
    pub case_serial_number: a3954::structures::CaseSerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub hear_id: CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    /// Not included in the state update packet by some firmware versions. None means it was not
    /// reported, not that it is disabled.
    pub slide_button_configuration: Option<ButtonStatusCollection<4>>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3954::structures::SoundModes,
    pub case_features: a3954::structures::CaseFeatures,
    pub air_pressure: a3954::structures::AirPressure,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub spatial_audio: a3954::structures::SpatialAudio,
    pub easy_chat: a3954::structures::EasyChat,
    pub sound_leak_compensation: SoundLeakCompensation,
    pub case_language: a3954::structures::CaseLanguage,
    pub wearing_detection: WearingDetection,
}

impl Default for A3954StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            case_firmware_version: Default::default(),
            case_serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            hear_id: Default::default(),
            button_configuration: a3954::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            slide_button_configuration: Some(
                a3954::SLIDE_BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            case_features: Default::default(),
            air_pressure: Default::default(),
            low_battery_prompt: Default::default(),
            ldac: Default::default(),
            dual_connections_enabled: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            spatial_audio: Default::default(),
            easy_chat: Default::default(),
            sound_leak_compensation: Default::default(),
            case_language: Default::default(),
            wearing_detection: Default::default(),
            case_battery_level: Default::default(),
        }
    }
}

impl FromPacketBody for A3954StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3954 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBattery::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        a3954::structures::CaseFirmwareVersion::take,
                        CaseBatteryLevel::take,
                        a3954::structures::CaseSerialNumber::take,
                        CommonEqualizerConfiguration::take,
                        take(1usize), // unknown
                        CustomHearId::take_with_music_genre_at_end,
                        take(1usize), // unknown
                        // The non-slide buttons and slide buttons are not together, so we have to split up parsing
                        ButtonStatusCollection::<8>::take(
                            a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        AmbientSoundModeCycle::take,
                        a3954::structures::SoundModes::take,
                        take(3usize), // unknown
                        a3954::structures::CaseFeatures::take,
                        a3954::structures::AirPressure::take,
                        take(3usize), // unknown
                        LowBatteryPrompt::take,
                        Ldac::take,
                        take_bool, // dual connections enabled
                    ),
                    (
                        AutoPowerOff::take,
                        LimitHighVolume::take,
                        a3954::structures::SpatialAudio::take,
                        take_bool,    // Easy chat enabled
                        take(1usize), // unknown
                        SoundLeakCompensation::take,
                        take(3usize), // unknown
                        a3954::structures::CaseLanguage::take,
                        a3954::structures::EasyChatWaitTime::take,
                        WearingDetection::take,
                        take(1usize), // unknown
                        take_slide_button_configuration,
                    ),
                ),
                |(
                    (
                        tws_status,
                        battery,
                        firmware_version,
                        serial_number,
                        case_firmware_version,
                        case_battery_level,
                        case_serial_number,
                        equalizer_configuration,
                        _unknown1,
                        hear_id,
                        _unknown2,
                        main_buttons,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        _unknown3,
                        case_features,
                        air_pressure,
                        _unknown4,
                        low_battery_prompt,
                        ldac,
                        dual_connections_enabled,
                    ),
                    (
                        auto_power_off,
                        limit_high_volume,
                        spatial_audio,
                        is_easy_chat_enabled,
                        _unknown5,
                        sound_leak_compensation,
                        _unknown6,
                        case_language,
                        easy_chat_wait_time,
                        wearing_detection,
                        _unknown7,
                        slide_button_configuration,
                    ),
                )| Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    case_firmware_version,
                    case_battery_level,
                    case_serial_number,
                    equalizer_configuration,
                    hear_id,
                    button_configuration: main_buttons,
                    slide_button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    case_features,
                    air_pressure,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                    easy_chat: a3954::structures::EasyChat {
                        is_enabled: is_easy_chat_enabled,
                        wait_time: easy_chat_wait_time,
                    },
                    sound_leak_compensation,
                    case_language,
                    wearing_detection,
                },
            ),
        )
        .parse_complete(input)
    }
}

/// Older firmware versions end the packet before the slide button configuration. If there are no
/// bytes left, it was not reported. If there are some bytes left, all of it must be present, since
/// a partial slide button configuration is a truncated packet rather than an older format.
fn take_slide_button_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Option<ButtonStatusCollection<4>>, E> {
    if input.is_empty() {
        return Ok((input, None));
    }
    context(
        "a3954 slide button configuration",
        map(
            ButtonStatusCollection::<4>::take(
                a3954::SLIDE_BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
            ),
            Some,
        ),
    )
    .parse_complete(input)
}

impl ToPacket for A3954StateUpdatePacket {
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
            .chain(self.case_firmware_version.bytes())
            .chain(self.case_battery_level.bytes())
            .chain(self.case_serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(std::iter::once(0)) // unknown
            .chain(
                self.button_configuration
                    .bytes(a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.case_features.bytes())
            .chain(self.air_pressure.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.spatial_audio.bytes())
            .chain(std::iter::once(u8::from(self.easy_chat.is_enabled)))
            .chain(std::iter::once(0)) // unknown
            .chain(self.sound_leak_compensation.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.case_language.bytes())
            .chain(self.easy_chat.wait_time.bytes())
            .chain(self.wearing_detection.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(self.slide_button_configuration.iter().flat_map(|buttons| {
                buttons.bytes(a3954::SLIDE_BUTTON_CONFIGURATION_SETTINGS.parse_settings())
            }))
            .collect()
    }
}

state_update_packet_module!(A3954State, A3954StateUpdatePacket);

#[cfg(test)]
pub mod fixtures {
    // Liberty 4 Pro, firmware 03.23/03.23, case firmware 02.56. No slide button configuration.
    // Source: https://github.com/Oppzippy/OpenSCQ30/issues/246, with the serial numbers (bytes
    // 16..32 and 38..44) replaced.
    pub const ISSUE_246_BODY: [u8; 161] = [
        1, 0, 100, 97, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 52, 48, 48, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 49, 48, 50, 46, 53, 54, 7, 0, 0, 0, 0, 0, 1, 0, 0, 120,
        120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 102, 102, 50, 51, 255,
        255, 68, 68, 51, 2, 6, 0, 0, 0, 0, 255, 0, 0, 0, 0, 91, 1, 49, 1, 1, 0, 1, 1, 2, 0, 90, 0,
        0, 0, 0, 0, 0, 0, 49, 1, 0, 1, 0, 1, 255,
    ];

    // Liberty 4 Pro, firmware 03.23/03.23, case firmware 01.56. No slide button configuration.
    // Source: https://github.com/Oppzippy/OpenSCQ30/issues/284, with the serial numbers (bytes
    // 16..32 and 38..44) replaced.
    pub const ISSUE_284_BODY: [u8; 161] = [
        1, 1, 100, 99, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 52, 48, 48, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 50, 48, 49, 46, 53, 54, 9, 0, 0, 0, 0, 0, 2, 4, 0, 150,
        150, 100, 100, 120, 140, 150, 160, 120, 120, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 0, 0, 254, 254, 254, 254, 254, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 102, 102, 50,
        51, 255, 255, 68, 68, 51, 2, 6, 0, 2, 1, 1, 255, 0, 0, 0, 0, 99, 1, 50, 1, 1, 0, 1, 1, 2,
        0, 90, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 1, 0, 1, 255,
    ];

    // Firmware 03.29/03.29, case firmware 02.58. Includes slide button configuration. Same packet as
    // a3954::tests::parses_known_packet
    pub const FULL_BODY: [u8; 165] = [
        0, 1, 99, 100, 0, 0, 48, 51, 46, 50, 57, 48, 51, 46, 50, 57, 51, 57, 53, 52, 68, 69, 55,
        55, 53, 49, 56, 65, 57, 68, 70, 52, 48, 50, 46, 53, 56, 9, 244, 157, 138, 83, 43, 186, 254,
        254, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 0, 1, 145, 147, 139, 141, 122, 111, 105, 100, 60, 60, 145, 147, 139, 141,
        122, 111, 105, 100, 60, 60, 0, 0, 0, 0, 1, 145, 147, 139, 130, 122, 133, 133, 114, 60, 60,
        145, 147, 139, 130, 122, 133, 133, 114, 60, 60, 0, 0, 10, 102, 102, 50, 51, 255, 255, 68,
        68, 51, 2, 6, 0, 0, 255, 1, 255, 0, 0, 0, 0, 94, 1, 110, 1, 0, 0, 0, 1, 0, 1, 95, 0, 0, 1,
        0, 0, 0, 0, 110, 1, 0, 1, 0, 0, 255, 0, 0, 17, 17,
    ];
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use openscq30_lib_has::MaybeHas;
    use tokio::sync::watch;

    use super::{fixtures::*, *};
    use crate::devices::soundcore::common::{
        packet::inbound::TryToPacket,
        packet_manager::PacketHandler,
        structures::{
            FirmwareVersion,
            button_configuration::{ActionStatus, ButtonStatus},
        },
    };

    fn parse(body: &[u8]) -> Result<A3954StateUpdatePacket, nom::Err<VerboseError<&[u8]>>> {
        A3954StateUpdatePacket::take::<VerboseError<_>>(body).map(|(remaining, packet)| {
            assert!(remaining.is_empty(), "{} bytes remaining", remaining.len());
            packet
        })
    }

    fn try_to_packet(body: &[u8]) -> Result<A3954StateUpdatePacket, ()> {
        packet::Inbound::new(packet::inbound::STATE_COMMAND, body.to_vec())
            .try_to_packet()
            .map_err(|_| ())
    }

    fn main_button_bytes(packet: &A3954StateUpdatePacket) -> Vec<u8> {
        packet
            .button_configuration
            .bytes(a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings())
            .collect()
    }

    fn slide_button_bytes(packet: &A3954StateUpdatePacket) -> Option<Vec<u8>> {
        packet.slide_button_configuration.map(|buttons| {
            buttons
                .bytes(a3954::SLIDE_BUTTON_CONFIGURATION_SETTINGS.parse_settings())
                .collect()
        })
    }

    #[test]
    fn parses_161_byte_packet_without_slide_buttons() {
        for body in [&ISSUE_246_BODY, &ISSUE_284_BODY] {
            let packet = parse(body).unwrap();
            assert_eq!(packet.slide_button_configuration, None);
            assert_eq!(
                main_button_bytes(&packet),
                [0x66, 0x66, 0x32, 0x33, 0xFF, 0xFF, 0x44, 0x44],
            );
            assert_eq!(packet.battery.left.level.0, body[2]);
            assert_eq!(packet.battery.right.level.0, body[3]);
            assert_eq!(
                packet.firmware_version,
                DualFirmwareVersion::Both {
                    left: FirmwareVersion::new(3, 23),
                    right: FirmwareVersion::new(3, 23),
                },
            );

            // Fields in known positions are serialized back to the same place, so this validates
            // that nothing before the slide buttons was shifted.
            let serialized = packet.body();
            assert_eq!(serialized.len(), 161);
            assert_eq!(serialized[..44], body[..44]);
            // main buttons
            assert_eq!(serialized[116..124], body[116..124]);
            // sound modes
            assert_eq!(serialized[125..129], body[125..129]);
            // case language, easy chat wait time, wearing detection
            assert_eq!(serialized[157..160], body[157..160]);
        }
        assert_eq!(
            parse(&ISSUE_284_BODY)
                .unwrap()
                .case_firmware_version
                .0
                .to_string(),
            "01.56",
        );
        assert_eq!(
            parse(&ISSUE_246_BODY)
                .unwrap()
                .case_firmware_version
                .0
                .to_string(),
            "02.56",
        );
    }

    #[test]
    fn parses_165_byte_packet_with_slide_buttons() {
        let packet = parse(&FULL_BODY).unwrap();
        assert_eq!(
            slide_button_bytes(&packet),
            Some(vec![0x00, 0x00, 0x11, 0x11])
        );
        assert_eq!(packet.body().len(), 165);
    }

    #[test]
    fn parses_non_default_slide_buttons() {
        // Synthetic: low nibble is the action when TWS is connected, high nibble when disconnected
        let mut body = FULL_BODY;
        body[161..].copy_from_slice(&[0xF0, 0x0F, 0x01, 0x1F]);
        let packet = parse(&body).unwrap();
        let slide_buttons = packet.slide_button_configuration.unwrap();
        assert_eq!(
            slide_buttons.0.map(|status| status.action),
            [
                ActionStatus::Tws {
                    connected: 0x0,
                    disconnected: 0xF,
                },
                ActionStatus::Tws {
                    connected: 0xF,
                    disconnected: 0x0,
                },
                ActionStatus::Tws {
                    connected: 0x1,
                    disconnected: 0x0,
                },
                ActionStatus::Tws {
                    connected: 0xF,
                    disconnected: 0x1,
                },
            ],
        );
        assert!(
            slide_buttons
                .0
                .iter()
                .all(|status: &ButtonStatus| status.enabled.is_none())
        );
        assert_eq!(packet.body()[161..], body[161..]);
    }

    #[test]
    fn rejects_truncated_packets() {
        for len in 0..161 {
            assert!(parse(&FULL_BODY[..len]).is_err(), "length {len}");
            assert!(try_to_packet(&FULL_BODY[..len]).is_err(), "length {len}");
        }
    }

    #[test]
    fn rejects_partial_slide_buttons() {
        // TryToPacket discards the remaining input, so make sure a partial slide button
        // configuration isn't treated as missing.
        for len in 162..165 {
            assert!(
                A3954StateUpdatePacket::take::<VerboseError<_>>(&FULL_BODY[..len]).is_err(),
                "length {len}",
            );
            assert!(try_to_packet(&FULL_BODY[..len]).is_err(), "length {len}");
        }
    }

    #[test]
    fn slide_buttons_are_only_optional_at_the_end() {
        assert_eq!(
            try_to_packet(&FULL_BODY[..161])
                .unwrap()
                .slide_button_configuration,
            None,
        );
        assert!(
            try_to_packet(&FULL_BODY)
                .unwrap()
                .slide_button_configuration
                .is_some()
        );

        let mut body = FULL_BODY.to_vec();
        body.push(0xAB);
        let (remaining, packet) = A3954StateUpdatePacket::take::<VerboseError<_>>(&body).unwrap();
        assert_eq!(remaining, [0xAB]);
        assert_eq!(
            slide_button_bytes(&packet),
            Some(vec![0x00, 0x00, 0x11, 0x11])
        );
    }

    #[test]
    fn serializes_slide_buttons_only_when_present() {
        let mut packet = A3954StateUpdatePacket::default();
        assert_eq!(packet.body().len(), 165);
        packet.slide_button_configuration = None;
        assert_eq!(packet.body().len(), 161);
        assert_eq!(
            parse(&packet.body()).unwrap().slide_button_configuration,
            None,
        );
    }

    fn slide_buttons_in_state(state: &A3954State) -> Option<ButtonStatusCollection<4>> {
        MaybeHas::<ButtonStatusCollection<4>>::maybe_get(state).copied()
    }

    #[tokio::test]
    async fn state_update_packet_handler_updates_slide_buttons() {
        let (sender, _receiver) =
            watch::channel(A3954State::new(parse(&FULL_BODY).unwrap(), Vec::new()));
        let full_slide_buttons = slide_buttons_in_state(&sender.borrow());
        assert!(full_slide_buttons.is_some());

        let handle = async |body: &[u8]| {
            StateUpdatePacketHandler
                .handle_packet(
                    &sender,
                    &packet::Inbound::new(packet::inbound::STATE_COMMAND, body.to_vec()),
                )
                .await
        };

        // Some -> None
        handle(&ISSUE_284_BODY).await.unwrap();
        assert_eq!(slide_buttons_in_state(&sender.borrow()), None);
        // None -> None
        handle(&ISSUE_246_BODY).await.unwrap();
        assert_eq!(slide_buttons_in_state(&sender.borrow()), None);

        // A rejected packet leaves the state unchanged
        let before = sender.borrow().clone();
        assert!(handle(&FULL_BODY[..163]).await.is_err());
        assert_eq!(*sender.borrow(), before);

        // None -> Some
        handle(&FULL_BODY).await.unwrap();
        assert_eq!(slide_buttons_in_state(&sender.borrow()), full_slide_buttons);
        // Some -> Some
        handle(&FULL_BODY).await.unwrap();
        assert_eq!(slide_buttons_in_state(&sender.borrow()), full_slide_buttons);
    }
}
