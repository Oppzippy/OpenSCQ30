mod refresh_devices;
mod select_sevice;
mod state;

use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};
pub use refresh_devices::*;
pub use select_sevice::*;
pub use state::*;

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
