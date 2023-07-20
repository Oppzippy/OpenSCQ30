mod create_custom_equalizer_profile;
mod delete_custom_equalizer_profile;
mod refresh_devices;
mod select_custom_equalizer_configuration;
mod set_ambient_sound_mode;
mod set_device;
mod set_equalizer_configuration;
mod set_noise_canceling_mode;
mod state;

pub use create_custom_equalizer_profile::*;
pub use delete_custom_equalizer_profile::*;
use macaddr::MacAddr6;
use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};
pub use refresh_devices::*;
pub use select_custom_equalizer_configuration::*;
pub use set_ambient_sound_mode::*;
pub use set_device::*;
pub use set_equalizer_configuration::*;
pub use set_noise_canceling_mode::*;
pub use state::*;

use crate::objects::{CustomEqualizerProfileObject, DeviceObject};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum StateUpdate {
    SetDevices(Vec<DeviceObject>),
    SetLoading(bool),
    SetAmbientSoundMode(AmbientSoundMode),
    SetNoiseCancelingMode(NoiseCancelingMode),
    SetEqualizerConfiguration(EqualizerConfiguration),
    SetSelectedDevice(Option<DeviceObject>),
    SetCustomEqualizerProfiles(Vec<CustomEqualizerProfileObject>),
    AddToast(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Action {
    SetAmbientSoundMode(AmbientSoundMode),
    SetNoiseCancelingMode(NoiseCancelingMode),
    Connect(MacAddr6),
    Disconnect,
    SelectCustomEqualizerProfile(CustomEqualizerProfileObject),
    CreateCustomEqualizerProfile(CustomEqualizerProfileObject),
    DeleteCustomEqualizerProfile(CustomEqualizerProfileObject),
    SetEqualizerConfiguration(EqualizerConfiguration),
}
