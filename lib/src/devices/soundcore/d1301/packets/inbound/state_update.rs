use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{
            DualBatteryLevel, DualFirmwareVersion, FirmwareVersion, LowBatteryPrompt, SerialNumber,
            TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
    d1301::{self, state::D1301State},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1301StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery_level: DualBatteryLevel,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_firmware_version: FirmwareVersion,
    pub button_configuration: ButtonStatusCollection<4>,
    pub listening_mode: d1301::structures::ListeningMode,
    pub default_listening_mode: d1301::structures::DefaultListeningMode,
    pub low_battery_prompt: LowBatteryPrompt,
    pub auto_power_off_prompt: d1301::structures::AutoPowerOffPrompt,
    pub listening_mode_prompt: d1301::structures::ListeningModePrompt,
    pub noise_canceling: d1301::structures::NoiseCanceling,
    pub incoming_calls_during_bluetooth_mode: d1301::structures::IncomingCallsDuringBluetoothMode,
    pub tap_controls_disabled: d1301::structures::TapControlsDisabled,
    pub noise_canceling_prompt: d1301::structures::NoiseCancelingPrompt,
    pub post_sleep_audio: d1301::structures::PostSleepAudio,
    pub auto_switch_once_asleep: d1301::structures::AutoSwitchOnceAsleep,
}

impl Default for D1301StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery_level: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_firmware_version: Default::default(),
            button_configuration: d1301::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            listening_mode: Default::default(),
            default_listening_mode: Default::default(),
            low_battery_prompt: Default::default(),
            auto_power_off_prompt: Default::default(),
            listening_mode_prompt: Default::default(),
            noise_canceling: Default::default(),
            incoming_calls_during_bluetooth_mode: Default::default(),
            tap_controls_disabled: Default::default(),
            noise_canceling_prompt: Default::default(),
            post_sleep_audio: Default::default(),
            auto_switch_once_asleep: Default::default(),
        }
    }
}

impl FromPacketBody for D1301StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1301 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBatteryLevel::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        take(6usize),
                        FirmwareVersion::take, // case firmware version
                        take(4usize),
                        // Not auto switch once asleep, despite appearances:
                        // reads 6 on every device seen so far and never changed
                        // when that setting changed. Still unidentified.
                        le_u8,
                        ButtonStatusCollection::take(
                            d1301::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        take(15usize),
                        d1301::structures::ListeningMode::take,
                        d1301::structures::DefaultListeningMode::take,
                        take(2usize),
                        LowBatteryPrompt::take,
                        take(67usize),
                        d1301::structures::AutoPowerOffPrompt::take,
                        d1301::structures::ListeningModePrompt::take,
                        take(1usize),
                        d1301::structures::NoiseCanceling::take,
                        d1301::structures::IncomingCallsDuringBluetoothMode::take,
                    ),
                    (
                        take(2usize),
                        d1301::structures::PostSleepAudio::take,
                        take(1usize),
                        d1301::structures::TapControlsDisabled::take,
                        d1301::structures::NoiseCancelingPrompt::take,
                        d1301::structures::AutoSwitchOnceAsleep::take,
                    ),
                ),
                |(
                    (
                        tws_status,
                        dual_battery_level,
                        dual_firmware_version,
                        serial_number,
                        _unknown1,
                        case_firmware_version,
                        _unknown2,
                        _auto_switch_once_asleep_maybe,
                        button_configuration,
                        _unknown3,
                        listening_mode,
                        default_listening_mode,
                        _unknown4,
                        low_battery_prompt,
                        _unknown5,
                        auto_power_off_prompt,
                        listening_mode_prompt,
                        _unknown6,
                        noise_canceling,
                        incoming_calls_during_bluetooth_mode,
                    ),
                    (
                        _unknown7,
                        post_sleep_audio,
                        _unknown8,
                        tap_controls_disabled,
                        noise_canceling_prompt,
                        auto_switch_once_asleep,
                    ),
                )| Self {
                    tws_status,
                    dual_battery_level,
                    dual_firmware_version,
                    serial_number,
                    case_firmware_version,
                    button_configuration,
                    listening_mode,
                    default_listening_mode,
                    low_battery_prompt,
                    auto_power_off_prompt,
                    listening_mode_prompt,
                    noise_canceling,
                    incoming_calls_during_bluetooth_mode,
                    tap_controls_disabled,
                    noise_canceling_prompt,
                    post_sleep_audio,
                    auto_switch_once_asleep,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1301StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.dual_battery_level.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(iter::repeat_n(0, 6))
            .chain(self.case_firmware_version.bytes())
            .chain(iter::repeat_n(0, 4))
            .chain(iter::once(6)) // see the matching comment in take()
            .chain(
                self.button_configuration
                    .bytes(d1301::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(iter::repeat_n(0, 15))
            .chain(self.listening_mode.bytes())
            .chain(self.default_listening_mode.bytes())
            .chain(iter::repeat_n(0, 2))
            .chain(self.low_battery_prompt.bytes())
            .chain(iter::repeat_n(0, 67))
            .chain(self.auto_power_off_prompt.bytes())
            .chain(self.listening_mode_prompt.bytes())
            .chain(iter::once(0))
            .chain(self.noise_canceling.bytes())
            .chain(self.incoming_calls_during_bluetooth_mode.bytes())
            .chain(iter::repeat_n(0, 2))
            .chain(self.post_sleep_audio.bytes())
            .chain(iter::once(0))
            .chain(self.tap_controls_disabled.bytes())
            .chain(self.noise_canceling_prompt.bytes())
            .chain(self.auto_switch_once_asleep.bytes())
            .collect()
    }
}

state_update_packet_module!(D1301State, D1301StateUpdatePacket);

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    /// State update bodies captured from a Sleep A30 on firmware 01.91, one for
    /// each of the three values the official app offers. Byte 148 is the enable
    /// flag and byte 144 is the action; Keep Audio only clears the flag, so the
    /// action it retains is whatever was set last.
    const KEEP_AUDIO: &[u8] = &[
        1, 1, 9, 9, 48, 49, 46, 57, 49, 48, 49, 46, 57, 49, 49, 51, 48, 49, 55, 67, 69, 57, 49, 51,
        48, 55, 56, 67, 56, 65, 199, 37, 26, 19, 233, 124, 48, 49, 46, 54, 56, 0, 0, 0, 0, 6, 221,
        136, 17, 0, 255, 0, 1, 104, 1, 124, 0, 50, 0, 255, 255, 1, 255, 255, 255, 0, 0, 1, 0, 0,
        50, 38, 0, 0, 6, 0, 127, 219, 58, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1,
        128, 0, 0, 0, 0, 0, 0, 4, 3, 2, 128, 0, 0, 0, 0, 0, 0, 4, 4, 3, 128, 0, 0, 0, 0, 0, 0, 4,
        5, 4, 128, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0, 1, 9, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0,
    ];
    const PAUSE_AUDIO: &[u8] = &[
        1, 1, 9, 9, 48, 49, 46, 57, 49, 48, 49, 46, 57, 49, 49, 51, 48, 49, 55, 67, 69, 57, 49, 51,
        48, 55, 56, 67, 56, 65, 199, 37, 26, 19, 233, 124, 48, 49, 46, 54, 56, 0, 0, 0, 0, 6, 221,
        136, 17, 0, 255, 0, 1, 104, 1, 124, 0, 50, 0, 255, 255, 0, 255, 255, 255, 0, 0, 1, 0, 0,
        50, 38, 0, 0, 6, 0, 127, 219, 58, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1,
        128, 0, 0, 0, 0, 0, 0, 4, 3, 2, 128, 0, 0, 0, 0, 0, 0, 4, 4, 3, 128, 0, 0, 0, 0, 0, 0, 4,
        5, 4, 128, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0, 1, 9, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0,
    ];
    const PLAY_LOCAL_AUDIO: &[u8] = &[
        1, 1, 9, 9, 48, 49, 46, 57, 49, 48, 49, 46, 57, 49, 49, 51, 48, 49, 55, 67, 69, 57, 49, 51,
        48, 55, 56, 67, 56, 65, 199, 37, 26, 19, 233, 124, 48, 49, 46, 54, 56, 0, 0, 0, 0, 6, 221,
        136, 17, 0, 255, 0, 1, 104, 1, 124, 0, 50, 0, 255, 255, 1, 255, 255, 255, 0, 0, 1, 0, 0,
        50, 38, 0, 0, 6, 0, 127, 219, 58, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1,
        128, 0, 0, 0, 0, 0, 0, 4, 3, 2, 128, 0, 0, 0, 0, 0, 0, 4, 4, 3, 128, 0, 0, 0, 0, 0, 0, 4,
        5, 4, 128, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0, 1, 9, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0,
    ];

    fn parse(body: &[u8]) -> D1301StateUpdatePacket {
        D1301StateUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1
    }

    #[test]
    fn parses_auto_switch_once_asleep_from_captured_state() {
        let keep = parse(KEEP_AUDIO);
        assert!(!keep.auto_switch_once_asleep.0);

        let pause = parse(PAUSE_AUDIO);
        assert!(pause.auto_switch_once_asleep.0);
        assert_eq!(
            d1301::structures::PostSleepAudio::PAUSE,
            pause.post_sleep_audio
        );

        let play_local = parse(PLAY_LOCAL_AUDIO);
        assert!(play_local.auto_switch_once_asleep.0);
        assert_eq!(
            d1301::structures::PostSleepAudio::PLAY_LOCAL,
            play_local.post_sleep_audio
        );
    }

    #[test]
    fn serialize_and_deserialize() {
        let bytes = D1301StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: D1301StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
