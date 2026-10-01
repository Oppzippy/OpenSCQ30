use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a6611::structures::{FindDevice, BluetoothMode},
    common::{
        self,
        structures::{DualBatteryLevel, DualFirmwareVersion, SerialNumber, TwsStatus},
    },
};

use super::packets::inbound::A6611StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A6611State {
    tws_status: TwsStatus,
    battery: DualBatteryLevel,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    bluetooth_mode: BluetoothMode,
    find_device: FindDevice,
}

impl A6611State {
    pub fn new(packet: A6611StateUpdatePacket) -> Self {
        Self {
            tws_status: packet.tws_status,
            battery: packet.battery,
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number,
            bluetooth_mode: packet.bluetooth_mode,
            find_device: FindDevice::default(),
        }
    }
}

// Not From<A6611StateUpdatePacket>, since the blanket Update impl would reset find_device on every state update
impl common::state::Update<A6611StateUpdatePacket> for A6611State {
    fn update(&mut self, packet: A6611StateUpdatePacket) {
        self.tws_status = packet.tws_status;
        self.battery = packet.battery;
        self.firmware_version = packet.firmware_version;
        self.serial_number = packet.serial_number;
        self.bluetooth_mode = packet.bluetooth_mode;
    }
}
