mod refresh_devices;
mod select_sevice;

use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};
pub use refresh_devices::*;
pub use select_sevice::*;

use crate::{objects::DeviceObject, widgets::Device};

#[derive(Debug)]
pub enum StateUpdate {
    SetDevices(Vec<Device>),
    SetLoading(bool),
    SetAmbientSoundMode(AmbientSoundMode),
    SetNoiseCancelingMode(NoiseCancelingMode),
    SetEqualizerConfiguration(EqualizerConfiguration),
    SetSelectedDevice(Option<DeviceObject>),
}
