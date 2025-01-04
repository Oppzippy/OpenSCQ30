mod activate_quick_preset;
mod create_custom_equalizer_profile;
mod create_quick_preset;
mod delete_custom_equalizer_profile;
mod delete_quick_preset;
mod import_custom_equalizer_profiles;
mod refresh_custom_equalizer_profiles;
mod refresh_devices;
mod refresh_quick_presets;
mod select_custom_equalizer_configuration;
mod set_ambient_sound_mode;
mod set_ambient_sound_mode_cycle;
mod set_ambient_sound_mode_type_two;
mod set_button_configuration;
mod set_custom_noise_canceling;
mod set_device;
mod set_equalizer_configuration;
mod set_hear_id;
mod set_manual_noise_canceling;
mod set_noise_canceling_mode;
mod set_noise_canceling_mode_type_two;
mod set_transparency_mode;
mod set_transparency_mode_type_two;
mod state;

use std::sync::Arc;

pub use activate_quick_preset::*;
pub use create_custom_equalizer_profile::*;
pub use create_quick_preset::*;
pub use delete_custom_equalizer_profile::*;
pub use delete_quick_preset::*;
pub use import_custom_equalizer_profiles::*;
use macaddr::MacAddr6;
use openscq30_lib::devices::standard::{
    state::DeviceState,
    structures::{
        AmbientSoundMode, AmbientSoundModeCycle, CustomNoiseCanceling, EqualizerConfiguration,
        HearId, ManualNoiseCanceling, MultiButtonConfiguration, NoiseCancelingMode,
        NoiseCancelingModeTypeTwo, TransparencyMode,
    },
};
pub use refresh_custom_equalizer_profiles::*;
pub use refresh_devices::*;
pub use refresh_quick_presets::*;
pub use select_custom_equalizer_configuration::*;
pub use set_ambient_sound_mode::*;
pub use set_ambient_sound_mode_cycle::*;
pub use set_ambient_sound_mode_type_two::*;
pub use set_button_configuration::*;
pub use set_custom_noise_canceling::*;
pub use set_device::*;
pub use set_equalizer_configuration::*;
pub use set_hear_id::*;
pub use set_manual_noise_canceling::*;
pub use set_noise_canceling_mode::*;
pub use set_noise_canceling_mode_type_two::*;
pub use set_transparency_mode::*;
pub use set_transparency_mode_type_two::*;
pub use state::*;

use crate::objects::{GlibCustomEqualizerProfile, GlibDevice, GlibNamedQuickPresetValue};

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum StateUpdate {
    SetDevices(Vec<GlibDevice>),
    SetLoading(bool),
    SetDeviceState(DeviceState),
    SetEqualizerConfiguration(EqualizerConfiguration),
    SetSelectedDevice(Option<GlibDevice>),
    SetCustomEqualizerProfiles(Vec<GlibCustomEqualizerProfile>),
    AddToast(String),
    SetQuickPresets(Vec<GlibNamedQuickPresetValue>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    SetAmbientSoundMode(AmbientSoundMode),
    SetNoiseCancelingMode(NoiseCancelingMode),
    Connect(MacAddr6),
    Disconnect,
    SelectCustomEqualizerProfile(GlibCustomEqualizerProfile),
    CreateCustomEqualizerProfile(GlibCustomEqualizerProfile),
    DeleteCustomEqualizerProfile(GlibCustomEqualizerProfile),
    SetEqualizerConfiguration(EqualizerConfiguration),
    SetCustomNoiseCanceling(CustomNoiseCanceling),
    SetTransparencyMode(TransparencyMode),
    CreateQuickPreset(GlibNamedQuickPresetValue),
    ActivateQuickPreset(GlibNamedQuickPresetValue),
    DeleteQuickPreset(Arc<str>),
    SetHearId(HearId),
    SetCustomButtonModel(MultiButtonConfiguration),
    SetAmbientSoundModeCycle(AmbientSoundModeCycle),
    ImportCustomEqualizerProfiles {
        profiles: Vec<GlibCustomEqualizerProfile>,
        overwrite: bool,
    },
    SetNoiseCancelingModeTypeTwo(NoiseCancelingModeTypeTwo),
    SetManualNoiseCanceling(ManualNoiseCanceling),
    SetAmbientSoundModeTypeTwo(AmbientSoundMode),
    SetTransparencyModeTypeTwo(TransparencyMode),
}
