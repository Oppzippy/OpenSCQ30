mod custom_equalizer_profile_object;
mod device_object;
mod equalizer_profile_object;

pub use custom_equalizer_profile_object::*;
pub use device_object::*;
pub use equalizer_profile_object::*;

use gtk::glib;
use openscq30_lib::packets::structures::{
    AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode,
};

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
