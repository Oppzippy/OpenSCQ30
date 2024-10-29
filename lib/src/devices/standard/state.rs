#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    device_profile::DeviceFeatures,
    devices::standard::{
        packets::inbound::state_update_packet::StateUpdatePacket,
        structures::{
            AgeRange, Battery, CustomButtonModel, EqualizerConfiguration, FirmwareVersion, Gender,
            HearId, SerialNumber, SoundModes,
        },
    },
};

use super::structures::{AmbientSoundModeCycle, SoundModesTypeTwo};

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DeviceState {
    pub device_features: DeviceFeatures,
    pub battery: Battery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub sound_modes: Option<SoundModes>,
    pub sound_modes_type_two: Option<SoundModesTypeTwo>,
    pub age_range: Option<AgeRange>,
    pub gender: Option<Gender>,
    pub hear_id: Option<HearId>,
    pub custom_button_model: Option<CustomButtonModel>,
    pub firmware_version: Option<FirmwareVersion>,
    pub serial_number: Option<SerialNumber>,
    pub ambient_sound_mode_cycle: Option<AmbientSoundModeCycle>,
}

impl From<StateUpdatePacket> for DeviceState {
    fn from(packet: StateUpdatePacket) -> Self {
        Self {
            device_features: packet.device_profile.features,
            battery: packet.battery,
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: packet.sound_modes,
            sound_modes_type_two: packet.sound_modes_type_two,
            age_range: packet.age_range,
            gender: packet.gender,
            hear_id: packet.hear_id,
            custom_button_model: packet.custom_button_model,
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number.clone(),
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
        }
    }
}

impl DeviceState {
    // need drc:                     A3951, A3930, A3931, A3931XR, A3935, A3935W,
    // separate left/right firmware: A3951, A3930, A3931, A3931XR, A3935, A3935W,
    pub fn supports_dynamic_range_compression(&self) -> bool {
        if self.device_features.has_dynamic_range_compression {
            self.does_firmware_version_support_drc(self.firmware_version.unwrap_or_default())
        } else {
            false
        }
    }

    fn does_firmware_version_support_drc(&self, firmware_version: FirmwareVersion) -> bool {
        firmware_version
            >= self
                .device_features
                .dynamic_range_compression_min_firmware_version
                .unwrap_or_default()
    }
}
