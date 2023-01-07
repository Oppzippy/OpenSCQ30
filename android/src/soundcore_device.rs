use std::sync::Arc;

use openscq30_lib::{
    api, soundcore_bluetooth::btleplug::BtlePlugSoundcoreDeviceConnection,
    state::SoundcoreDeviceState,
};
use rifgen::rifgen_attr::generate_interface;
use tokio::sync::broadcast;

use crate::{tokio_runtime, AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode};

pub struct SoundcoreDevice {
    soundcore_device: Option<Arc<api::SoundcoreDevice<BtlePlugSoundcoreDeviceConnection>>>,
}

impl SoundcoreDevice {
    #[generate_interface(constructor)]
    pub fn _new() -> SoundcoreDevice {
        Self {
            soundcore_device: None,
        }
    }

    pub fn new(device: Arc<api::SoundcoreDevice<BtlePlugSoundcoreDeviceConnection>>) -> Self {
        Self {
            soundcore_device: Some(device),
        }
    }

    pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        tokio_runtime::get_handle().block_on(async {
            self.soundcore_device
                .as_ref()
                .unwrap()
                .subscribe_to_state_updates()
        })
    }

    #[generate_interface]
    pub fn mac_address(&self) -> Result<String, String> {
        tokio_runtime::get_handle()
            .block_on(async { self.soundcore_device.as_ref().unwrap().mac_address().await })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn name(&self) -> Result<String, String> {
        tokio_runtime::get_handle()
            .block_on(async { self.soundcore_device.as_ref().unwrap().name().await })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        tokio_runtime::get_handle().block_on(async {
            self.soundcore_device
                .as_ref()
                .unwrap()
                .ambient_sound_mode()
                .await
                .into()
        })
    }

    #[generate_interface]
    pub fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), String> {
        tokio_runtime::get_handle()
            .block_on(async {
                self.soundcore_device
                    .as_ref()
                    .unwrap()
                    .set_ambient_sound_mode(ambient_sound_mode.into())
                    .await
            })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        tokio_runtime::get_handle().block_on(async {
            self.soundcore_device
                .as_ref()
                .unwrap()
                .noise_canceling_mode()
                .await
                .into()
        })
    }

    #[generate_interface]
    pub fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), String> {
        tokio_runtime::get_handle()
            .block_on(async {
                self.soundcore_device
                    .as_ref()
                    .unwrap()
                    .set_noise_canceling_mode(noise_canceling_mode.into())
                    .await
            })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        tokio_runtime::get_handle().block_on(async {
            self.soundcore_device
                .as_ref()
                .unwrap()
                .equalizer_configuration()
                .await
                .into()
        })
    }

    #[generate_interface]
    pub fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), String> {
        tokio_runtime::get_handle()
            .block_on(async {
                self.soundcore_device
                    .as_ref()
                    .unwrap()
                    .set_equalizer_configuration(configuration.into())
                    .await
            })
            .map_err(|err| err.to_string())
    }
}
