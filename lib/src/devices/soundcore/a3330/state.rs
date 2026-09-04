use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    state::Update,
    structures::{
        CaseBatteryLevel, CommonEqualizerConfiguration, DisableAllButtons, DualBatteryLevel,
        DualConnections, DualConnectionsDevice, DualFirmwareVersion, LowBatteryPrompt,
        SerialNumber, SurroundSound, TouchTone, TwsStatus,
        button_configuration::ButtonStatusCollection,
    },
};

use super::packets::inbound::A3330StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3330State {
    tws_status: TwsStatus,
    dual_battery_level: DualBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_battery_level: CaseBatteryLevel,
    button_configuration: ButtonStatusCollection<6>,
    call_button_configuration: ButtonStatusCollection<4>,
    surround_sound: SurroundSound,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    dual_connections: DualConnections,
    disable_all_buttons: DisableAllButtons,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    reset_button_configuration_pending: ResetButtonConfigurationPending,
}

impl A3330State {
    pub fn new(
        state_update_packet: A3330StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            dual_battery_level: state_update_packet.dual_battery_level,
            dual_firmware_version: state_update_packet.dual_firmware_version,
            serial_number: state_update_packet.serial_number,
            case_battery_level: state_update_packet.case_battery_level,
            button_configuration: state_update_packet.button_configuration,
            surround_sound: state_update_packet.surround_sound,
            touch_tone: state_update_packet.touch_tone,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            disable_all_buttons: state_update_packet.disable_all_buttons,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            call_button_configuration: state_update_packet.call_button_configuration,
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3330StateUpdatePacket> for A3330State {
    fn update(&mut self, partial: A3330StateUpdatePacket) {
        let A3330StateUpdatePacket {
            tws_status,
            dual_battery_level,
            dual_firmware_version,
            serial_number,
            case_battery_level,
            button_configuration,
            surround_sound,
            touch_tone,
            low_battery_prompt,
            dual_connections_enabled,
            disable_all_buttons,
            equalizer_configuration,
            call_button_configuration,
            _bass_mode,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery_level = dual_battery_level;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.case_battery_level = case_battery_level;
        self.button_configuration = button_configuration;
        self.surround_sound = surround_sound;
        self.touch_tone = touch_tone;
        self.low_battery_prompt = low_battery_prompt;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.disable_all_buttons = disable_all_buttons;
        self.equalizer_configuration = equalizer_configuration;
        self.call_button_configuration = call_button_configuration;
    }
}
