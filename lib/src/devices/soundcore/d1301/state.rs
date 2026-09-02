use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            DualBatteryLevel, DualFirmwareVersion, FirmwareVersion, LowBatteryPrompt, SerialNumber,
            TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
    d1301,
};

use super::packets::inbound::D1301StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1301State {
    tws_status: TwsStatus,
    dual_battery_level: DualBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_firmware_version: FirmwareVersion,
    button_configuration: ButtonStatusCollection<4>,
    listening_mode: d1301::structures::ListeningMode,
    default_listening_mode: d1301::structures::DefaultListeningMode,
    low_battery_prompt: LowBatteryPrompt,
    auto_power_off_prompt: d1301::structures::AutoPowerOffPrompt,
    listening_mode_prompt: d1301::structures::ListeningModePrompt,
    noise_canceling: d1301::structures::NoiseCanceling,
    incoming_calls_during_bluetooth_mode: d1301::structures::IncomingCallsDuringBluetoothMode,
    tap_controls_disabled: d1301::structures::TapControlsDisabled,
    noise_canceling_prompt: d1301::structures::NoiseCancelingPrompt,
    auto_switch_once_asleep: d1301::structures::AutoSwitchOnceAsleep,
    button_reset_pending: ResetButtonConfigurationPending,

    auto_stop_timer: d1301::structures::AutoStopTimer,
    alarms: Vec<d1301::structures::Alarm>,
}

impl D1301State {
    pub fn new(
        state_update_packet: D1301StateUpdatePacket,
        auto_stop_timer: d1301::structures::AutoStopTimer,
        alarms: Vec<d1301::structures::Alarm>,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            dual_battery_level: state_update_packet.dual_battery_level,
            dual_firmware_version: state_update_packet.dual_firmware_version,
            serial_number: state_update_packet.serial_number,
            case_firmware_version: state_update_packet.case_firmware_version,
            button_configuration: state_update_packet.button_configuration,
            listening_mode: state_update_packet.listening_mode,
            default_listening_mode: state_update_packet.default_listening_mode,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            auto_power_off_prompt: state_update_packet.auto_power_off_prompt,
            listening_mode_prompt: state_update_packet.listening_mode_prompt,
            noise_canceling: state_update_packet.noise_canceling,
            incoming_calls_during_bluetooth_mode: state_update_packet
                .incoming_calls_during_bluetooth_mode,
            tap_controls_disabled: state_update_packet.tap_controls_disabled,
            noise_canceling_prompt: state_update_packet.noise_canceling_prompt,
            auto_switch_once_asleep: state_update_packet.auto_switch_once_asleep,
            button_reset_pending: ResetButtonConfigurationPending::default(),
            auto_stop_timer,
            alarms,
        }
    }
}

impl Update<D1301StateUpdatePacket> for D1301State {
    fn update(&mut self, partial: D1301StateUpdatePacket) {
        let D1301StateUpdatePacket {
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
            auto_switch_once_asleep,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery_level = dual_battery_level;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.case_firmware_version = case_firmware_version;
        self.button_configuration = button_configuration;
        self.listening_mode = listening_mode;
        self.default_listening_mode = default_listening_mode;
        self.low_battery_prompt = low_battery_prompt;
        self.auto_power_off_prompt = auto_power_off_prompt;
        self.listening_mode_prompt = listening_mode_prompt;
        self.noise_canceling = noise_canceling;
        self.incoming_calls_during_bluetooth_mode = incoming_calls_during_bluetooth_mode;
        self.tap_controls_disabled = tap_controls_disabled;
        self.noise_canceling_prompt = noise_canceling_prompt;
        self.auto_switch_once_asleep = auto_switch_once_asleep;
    }
}
