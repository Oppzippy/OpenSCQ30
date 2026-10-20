use crate::devices::soundcore::{
    a6611::structures::{BluetoothMode, FindDevice},
    common::packet,
};

// Body is [1] for Bluetooth mode, [0] for sleep mode
pub const SET_BLUETOOTH_MODE_COMMAND: packet::Command = packet::Command([0x01, 0xa9]);

pub fn set_bluetooth_mode(bluetooth_mode: BluetoothMode) -> packet::Outbound {
    packet::Outbound::new(SET_BLUETOOTH_MODE_COMMAND, bluetooth_mode.bytes().to_vec())
}

pub const SET_FIND_DEVICE_COMMAND: packet::Command = packet::Command([0x10, 0x89]);

// Body is [left, right, 0], taken from a capture of the official app
pub fn set_find_device(find_device: FindDevice) -> packet::Outbound {
    packet::Outbound::new(
        SET_FIND_DEVICE_COMMAND,
        vec![find_device.left as u8, find_device.right as u8, 0],
    )
}
