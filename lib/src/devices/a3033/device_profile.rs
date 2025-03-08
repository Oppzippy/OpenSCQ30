use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{implementation::StandardImplementation, macros::soundcore_device},
    soundcore_device::device_model::DeviceModel,
};

use super::{packets::A3033StateUpdatePacket, state::A3033State};

pub(crate) const A3033_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: None,
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: false,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3033],
    implementation: || StandardImplementation::new::<A3033StateUpdatePacket>(),
};

soundcore_device!(A3033State, A3033StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.mono_equalizer().await;
});
