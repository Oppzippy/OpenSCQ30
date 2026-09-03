use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    state::Update,
    structures::{
        AutoPowerOff, CommonEqualizerConfiguration, DualConnections, DualConnectionsDevice,
        FirmwareVersion, SerialNumber, SingleBattery,
    },
};

use super::packets::inbound::A3005StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3005State {
    battery: SingleBattery,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
}

impl A3005State {
    pub fn new(
        state_update_packet: A3005StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            battery: state_update_packet.battery,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            auto_power_off: state_update_packet.auto_power_off,
        }
    }
}

impl Update<A3005StateUpdatePacket> for A3005State {
    fn update(&mut self, partial: A3005StateUpdatePacket) {
        let A3005StateUpdatePacket {
            battery,
            firmware_version,
            serial_number,
            equalizer_configuration,
            dual_connections_enabled,
            auto_power_off,
        } = partial;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
    }
}
