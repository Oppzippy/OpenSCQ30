use crate::devices::soundcore::common::structures::flag;

// Raw value from the device: true in Bluetooth mode, false in sleep mode
flag!(BluetoothMode);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FindDevice {
    pub left: bool,
    pub right: bool,
}
