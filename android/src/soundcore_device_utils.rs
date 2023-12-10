use std::str::FromStr;

use macaddr::MacAddr;
use openscq30_lib::{
    device_utils,
    devices::standard::structures::{
        EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments,
    },
};
use uuid::Uuid;

#[uniffi::export]
pub fn is_mac_address_soundcore_device(mac_address: String) -> bool {
    match MacAddr::from_str(&mac_address) {
        Ok(MacAddr::V6(parsed_address)) => {
            device_utils::is_mac_address_soundcore_device(parsed_address)
        }
        _ => false,
    }
}

#[uniffi::export]
pub fn is_soundcore_service_uuid(uuid: Uuid) -> bool {
    device_utils::is_soundcore_service_uuid(&uuid)
}

#[uniffi::export]
pub fn read_characteristic_uuid() -> String {
    device_utils::READ_CHARACTERISTIC_UUID.to_string()
}

#[uniffi::export]
pub fn write_characteristic_uuid() -> String {
    device_utils::WRITE_CHARACTERISTIC_UUID.to_string()
}

#[uniffi::export]
pub fn volume_adjustments_min_volume() -> f64 {
    VolumeAdjustments::MIN_VOLUME
}

#[uniffi::export]
pub fn volume_adjustments_max_volume() -> f64 {
    VolumeAdjustments::MAX_VOLUME
}

#[uniffi::export]
pub fn volume_adjustments_step() -> f64 {
    VolumeAdjustments::STEP
}

#[uniffi::export]
pub fn new_equalizer_configuration_from_preset_profile(
    preset_profile: PresetEqualizerProfile,
) -> EqualizerConfiguration {
    EqualizerConfiguration::new_from_preset_profile(preset_profile)
}
