use openscq30_lib::devices::standard::{
    state::DeviceState,
    structures::{
        AmbientSoundModeCycle, CustomButtonModel, EqualizerConfiguration, HearId,
        PresetEqualizerProfile, SoundModes,
    },
};
use prost::{DecodeError, Message};

mod conversion;
mod protobuf;

pub fn serialize_device_state(device_state: DeviceState) -> Vec<u8> {
    protobuf::DeviceState::from(device_state).encode_to_vec()
}

pub fn serialize_equalizer_configuration(configuration: EqualizerConfiguration) -> Vec<u8> {
    protobuf::EqualizerConfiguration::from(configuration).encode_to_vec()
}

pub fn serialize_preset_equalizer_profile(profile: PresetEqualizerProfile) -> Vec<u8> {
    let protobuf_profile = protobuf::PresetEqualizerProfile::from(profile);
    protobuf::PresetEqualizerProfileSelection {
        preset_profile: protobuf_profile.into(),
    }
    .encode_to_vec()
}

pub fn serialize_ambient_sound_mode_cycle(cycle: AmbientSoundModeCycle) -> Vec<u8> {
    protobuf::AmbientSoundModeCycle::from(cycle).encode_to_vec()
}

pub fn deserialize_sound_modes(protobuf: &[u8]) -> Result<SoundModes, DecodeError> {
    protobuf::SoundModes::decode(protobuf).map(SoundModes::from)
}

pub fn deserialize_ambient_sound_mode_cycle(
    protobuf: &[u8],
) -> Result<AmbientSoundModeCycle, DecodeError> {
    protobuf::AmbientSoundModeCycle::decode(protobuf).map(Into::into)
}

pub fn deserialize_equalizer_configuration(
    protobuf: &[u8],
) -> Result<EqualizerConfiguration, DecodeError> {
    protobuf::EqualizerConfiguration::decode(protobuf).map(EqualizerConfiguration::from)
}

pub fn deserialize_hear_id(protobuf: &[u8]) -> Result<HearId, DecodeError> {
    protobuf::HearId::decode(protobuf).map(HearId::from)
}

pub fn deserialize_custom_button_model(protobuf: &[u8]) -> Result<CustomButtonModel, DecodeError> {
    protobuf::CustomButtonModel::decode(protobuf).map(CustomButtonModel::from)
}

pub fn deserialize_preset_equalizer_profile(
    protobuf: &[u8],
) -> Result<PresetEqualizerProfile, DecodeError> {
    protobuf::PresetEqualizerProfileSelection::decode(protobuf)
        .map(|request| request.preset_profile().into())
}
