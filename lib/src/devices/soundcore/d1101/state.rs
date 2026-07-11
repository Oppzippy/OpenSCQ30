use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    state::Update,
    structures::{
        AutoPowerOff, CommonEqualizerConfiguration, DisableAllButtons, DualBatteryLevel,
        DualConnections, DualConnectionsDevice, DualFirmwareVersion, Ldac, LowBatteryPrompt,
        SerialNumber, TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::inbound::D1101StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1101State {
    tws_status: TwsStatus,
    dual_battery_level: DualBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    button_reset_pending: ResetButtonConfigurationPending,
    low_battery_prompt: LowBatteryPrompt,
    dual_connections: DualConnections,
    button_controls_disabled: DisableAllButtons,
    auto_power_off: AutoPowerOff,
    ldac: Ldac,
}

impl D1101State {
    pub fn new(
        packet: D1101StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery_level: packet.dual_battery_level,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            button_configuration: packet.button_configuration,
            button_reset_pending: ResetButtonConfigurationPending::default(),
            low_battery_prompt: packet.low_battery_prompt,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_controls_disabled: packet.button_controls_disabled,
            auto_power_off: packet.auto_power_off,
            ldac: packet.ldac,
        }
    }
}

impl Update<D1101StateUpdatePacket> for D1101State {
    fn update(&mut self, partial: D1101StateUpdatePacket) {
        let D1101StateUpdatePacket {
            tws_status,
            dual_battery_level,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            low_battery_prompt,
            dual_connections_enabled,
            button_controls_disabled,
            auto_power_off,
            ldac,
        } = partial;

        self.tws_status = tws_status;
        self.dual_battery_level = dual_battery_level;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.low_battery_prompt = low_battery_prompt;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.button_controls_disabled = button_controls_disabled;
        self.auto_power_off = auto_power_off;
        self.ldac = ldac;
    }
}
