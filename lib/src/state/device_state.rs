use std::borrow::Borrow;

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
}

impl<T> From<T> for DeviceState
where
    T: Borrow<StateUpdatePacket>,
{
    fn from(packet: T) -> Self {
        let packet: &StateUpdatePacket = packet.borrow();
        Self {
            feature_flags: packet.feature_flags,
            battery: packet.battery,
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: packet.sound_modes,
            age_range: packet.age_range,
            custom_hear_id: packet.custom_hear_id,
            custom_button_model: packet.custom_button_model,
            firmware_version: packet.firmware_version.clone(),
            serial_number: packet.serial_number.clone(),
        }
    }
}
