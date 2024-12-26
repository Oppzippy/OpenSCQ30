#![allow(clippy::inherent_to_string)]

pub mod connection;
mod device;
mod soundcore_device_utils;
use std::str::FromStr;

use log::LevelFilter;
use macaddr::MacAddr6;
use openscq30_lib::devices::standard::{
    state::DeviceState,
    structures::{
        AmbientSoundModeCycle, CustomButtonActions, EqualizerConfiguration, HearId,
        PresetEqualizerProfile, SoundModes, SoundModesTypeTwo,
    },
};
use openscq30_lib_protobuf::{
    deserialize_ambient_sound_mode_cycle, deserialize_custom_button_model,
    deserialize_equalizer_configuration, deserialize_hear_id, deserialize_preset_equalizer_profile,
    deserialize_sound_modes, deserialize_sound_modes_type_two, serialize_ambient_sound_mode_cycle,
    serialize_device_state, serialize_equalizer_configuration, serialize_preset_equalizer_profile,
};
use uuid::Uuid;

pub use crate::soundcore_device_utils::*;

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn init_native_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("openscq30-lib"),
    )
}

uniffi::custom_type!(MacAddr6, String);
impl UniffiCustomTypeConverter for MacAddr6 {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(MacAddr6::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

uniffi::custom_type!(Uuid, String);
impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(Uuid::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

uniffi::custom_type!(DeviceState, Vec<u8>);
impl UniffiCustomTypeConverter for DeviceState {
    type Builtin = Vec<u8>;

    fn into_custom(_val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serialize_device_state(obj)
    }
}

uniffi::custom_type!(SoundModes, Vec<u8>);
impl UniffiCustomTypeConverter for SoundModes {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_sound_modes(&val)?)
    }

    fn from_custom(_obj: Self) -> Self::Builtin {
        unimplemented!()
    }
}

uniffi::custom_type!(SoundModesTypeTwo, Vec<u8>);
impl UniffiCustomTypeConverter for SoundModesTypeTwo {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_sound_modes_type_two(&val)?)
    }

    fn from_custom(_obj: Self) -> Self::Builtin {
        unimplemented!()
    }
}

uniffi::custom_type!(AmbientSoundModeCycle, Vec<u8>);
impl UniffiCustomTypeConverter for AmbientSoundModeCycle {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_ambient_sound_mode_cycle(&val)?)
    }

    fn from_custom(protobuf: Self) -> Self::Builtin {
        serialize_ambient_sound_mode_cycle(protobuf)
    }
}

uniffi::custom_type!(EqualizerConfiguration, Vec<u8>);
impl UniffiCustomTypeConverter for EqualizerConfiguration {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_equalizer_configuration(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serialize_equalizer_configuration(obj)
    }
}

uniffi::custom_type!(HearId, Vec<u8>);
impl UniffiCustomTypeConverter for HearId {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_hear_id(&val)?)
    }

    fn from_custom(_obj: Self) -> Self::Builtin {
        unimplemented!()
    }
}

uniffi::custom_type!(CustomButtonActions, Vec<u8>);
impl UniffiCustomTypeConverter for CustomButtonActions {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_custom_button_model(&val)?)
    }

    fn from_custom(_obj: Self) -> Self::Builtin {
        unimplemented!()
    }
}

uniffi::custom_type!(PresetEqualizerProfile, Vec<u8>);
impl UniffiCustomTypeConverter for PresetEqualizerProfile {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self>
    where
        Self: Sized,
    {
        Ok(deserialize_preset_equalizer_profile(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serialize_preset_equalizer_profile(obj)
    }
}
