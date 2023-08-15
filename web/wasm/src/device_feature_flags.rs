use openscq30_lib::packets::structures::DeviceFeatureFlags as LibDeviceFeatureFlags;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct DeviceFeatureFlags;

#[wasm_bindgen]
impl DeviceFeatureFlags {
    #[wasm_bindgen(js_name = hasTransparencyModes)]
    pub fn has_transparency_modes(flags: u32) -> bool {
        LibDeviceFeatureFlags::from(flags).contains(LibDeviceFeatureFlags::TRANSPARENCY_MODES)
    }

    #[wasm_bindgen(js_name = hasNoiseCancelingMode)]
    pub fn noise_canceling_mode(flags: u32) -> bool {
        LibDeviceFeatureFlags::from(flags).contains(LibDeviceFeatureFlags::NOISE_CANCELING_MODE)
    }

    #[wasm_bindgen(js_name = hasCustomNoiseCanceling)]
    pub fn custom_noise_canceling(flags: u32) -> bool {
        LibDeviceFeatureFlags::from(flags).contains(
            LibDeviceFeatureFlags::NOISE_CANCELING_MODE
                | LibDeviceFeatureFlags::CUSTOM_NOISE_CANCELING,
        )
    }
}
