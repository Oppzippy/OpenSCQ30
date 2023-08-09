use serde::{Deserialize, Serialize};

use crate::packets::{
    inbound::state_update_packet::StateUpdatePacket,
    structures::{
        AgeRange, Battery, CustomButtonModel, DeviceFeatureFlags, EqualizerConfiguration,
        FirmwareVersion, HearId, SerialNumber, SoundModes,
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
    pub custom_hear_id: Option<HearId>,
    pub custom_button_model: Option<CustomButtonModel>,
    pub firmware_version: Option<FirmwareVersion>,
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
            custom_hear_id: packet.custom_hear_id,
            custom_button_model: packet.custom_button_model,
            firmware_version: packet.firmware_version,
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
        self.feature_flags
            .contains(DeviceFeatureFlags::DYNAMIC_RANGE_COMPRESSION)
            && self.firmware_version.unwrap_or_default()
                >= self
                    .dynamic_range_compression_min_firmware_version
                    .unwrap_or_default()
    }
}
