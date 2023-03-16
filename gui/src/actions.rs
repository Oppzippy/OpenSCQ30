mod refresh_devices;

pub use refresh_devices::*;

use crate::widgets::Device;

#[derive(Debug)]
pub enum StateUpdate {
    SetDevices(Vec<Device>),
}
