mod custom_equalizer_profile_object;
mod device_object;
mod equalizer_profile_object;

use std::sync::Arc;

pub use custom_equalizer_profile_object::*;
pub use device_object::*;
pub use equalizer_profile_object::*;

use gtk::glib;
use openscq30_lib::packets::structures::{
    AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, PresetEqualizerProfile,
    TransparencyMode,
};

use crate::settings::QuickPreset;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedAmbientSoundMode")]
pub struct BoxedAmbientSoundMode(pub AmbientSoundMode);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedTransparencyMode")]
pub struct BoxedTransparencyMode(pub TransparencyMode);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedNoiseCancelingMode")]
pub struct BoxedNoiseCancelingMode(pub NoiseCancelingMode);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedCustomNoiseCanceling")]
pub struct BoxedCustomNoiseCanceling(pub CustomNoiseCanceling);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedPresetEqualizerProfile")]
pub struct BoxedPresetEqualizerProfile(pub PresetEqualizerProfile);

#[derive(Clone, PartialEq, Eq, Debug, Hash, glib::Boxed)]
#[boxed_type(name = "OpenSCQ30BoxedQuickPreset")]
pub struct NamedQuickPreset {
    pub name: Arc<str>,
    pub quick_preset: QuickPreset,
}

impl Default for NamedQuickPreset {
    fn default() -> Self {
        Self {
            name: "".into(),
            quick_preset: Default::default(),
        }
    }
}
