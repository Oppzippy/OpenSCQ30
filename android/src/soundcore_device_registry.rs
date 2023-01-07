use openscq30_lib::{
    api,
    soundcore_bluetooth::btleplug::{self, BtlePlugSoundcoreDeviceConnectionRegistry},
};
use rifgen::rifgen_attr::generate_interface;

use crate::{tokio_runtime, SoundcoreDevice};

pub struct SoundcoreDeviceRegistry {
    device_registry: api::SoundcoreDeviceRegistry<BtlePlugSoundcoreDeviceConnectionRegistry>,
}

impl SoundcoreDeviceRegistry {
    #[generate_interface(constructor)]
    pub fn new() -> SoundcoreDeviceRegistry {
        // Lifetime specifiers don't work well with flapigen, so we can't pass down references to a runtime handle.
        // Throwing an Arc at it solves the problem.
        let device_registry = tokio_runtime::get_handle()
            .block_on(async {
                let connection_registry = btleplug::new_connection_registry()
                    .await
                    .unwrap_or_else(|err| panic!("failed to initialize handler: {err}"));
                api::SoundcoreDeviceRegistry::new(connection_registry).await
            })
            .unwrap();

        Self { device_registry }
    }

    #[generate_interface]
    pub fn refresh_devices(&self) -> Result<(), String> {
        tokio_runtime::get_handle()
            .block_on(async { self.device_registry.refresh_devices().await })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn devices(&self) -> Vec<SoundcoreDevice> {
        tokio_runtime::get_handle().block_on(async {
            self.device_registry
                .devices()
                .await
                .into_iter()
                .map(|device| SoundcoreDevice::new(device))
                .collect()
        })
    }

    #[generate_interface]
    pub fn device_by_mac_address(&self, mac_address: &String) -> Option<SoundcoreDevice> {
        tokio_runtime::get_handle().block_on(async {
            self.device_registry
                .device_by_mac_address(mac_address)
                .await
                .map(|device| SoundcoreDevice::new(device))
        })
    }
}
