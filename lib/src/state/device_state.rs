use serde::{Deserialize, Serialize};

use crate::packets::{
    inbound::state_update_packet::StateUpdatePacket,
    structures::{
        AgeRange, Battery, CustomButtonModel, DeviceFeatureFlags, EqualizerConfiguration,
        FirmwareVersion, Gender, HearId, SerialNumber, SoundModes,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub feature_flags: DeviceFeatureFlags,
    pub battery: Battery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub sound_modes: Option<SoundModes>,
    pub age_range: Option<AgeRange>,
    pub gender: Option<Gender>,
    pub hear_id: Option<HearId>,
    pub custom_button_model: Option<CustomButtonModel>,
    pub left_firmware_version: Option<FirmwareVersion>,
    pub right_firmware_version: Option<FirmwareVersion>,
    pub serial_number: Option<SerialNumber>,
    pub dynamic_range_compression_min_firmware_version: Option<FirmwareVersion>,
}

impl From<StateUpdatePacket> for DeviceState {
    fn from(packet: StateUpdatePacket) -> Self {
        Self {
            feature_flags: packet.feature_flags,
            battery: packet.battery,
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: packet.sound_modes,
            age_range: packet.age_range,
            gender: packet.gender,
            hear_id: packet.hear_id,
            custom_button_model: packet.custom_button_model,
            left_firmware_version: packet.firmware_version,
            right_firmware_version: None,
            serial_number: packet.serial_number.clone(),
            dynamic_range_compression_min_firmware_version: packet
                .dynamic_range_compression_min_firmware_version,
        }
    }
}

impl DeviceState {
    // need drc:                     A3951, A3930, A3931, A3931XR, A3935, A3935W,
    // separate left/right firmware: A3951, A3930, A3931, A3931XR, A3935, A3935W,
    pub fn supports_dynamic_range_compression(&self) -> bool {
        if self
            .feature_flags
            .contains(DeviceFeatureFlags::DYNAMIC_RANGE_COMPRESSION)
        {
            match (self.left_firmware_version, self.right_firmware_version) {
                (Some(left), Some(right)) => {
                    self.does_firmware_version_support_drc(left)
                        && self.does_firmware_version_support_drc(right)
                }
                (Some(left), None) => self.does_firmware_version_support_drc(left),
                (None, Some(_)) => unreachable!("right firmware version is set but not left"),
                (None, None) => false,
            }
        } else {
            false
        }
    }

    fn does_firmware_version_support_drc(&self, firmware_version: FirmwareVersion) -> bool {
        firmware_version
            >= self
                .dynamic_range_compression_min_firmware_version
                .unwrap_or_default()
    }
}
