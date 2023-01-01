use std::sync::Arc;

use openscq30_lib::{
    api, soundcore_bluetooth::btleplug::BtlePlugSoundcoreDeviceConnection,
    state::SoundcoreDeviceState,
};
use rifgen::rifgen_attr::generate_interface;
use tokio::{runtime::Runtime, sync::broadcast};

use crate::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode};

pub struct SoundcoreDevice {
    runtime: Option<Arc<Runtime>>,
    soundcore_device: Option<Arc<api::SoundcoreDevice<BtlePlugSoundcoreDeviceConnection>>>,
}

impl SoundcoreDevice {
    #[generate_interface(constructor)]
    pub fn _new() -> SoundcoreDevice {
        Self {
            runtime: None,
            soundcore_device: None,
        }
    }

    pub fn new(
        device: Arc<api::SoundcoreDevice<BtlePlugSoundcoreDeviceConnection>>,
        runtime: Arc<Runtime>,
    ) -> Self {
        Self {
            soundcore_device: Some(device),
            runtime: Some(runtime),
        }
    }

    pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        self.runtime.as_ref().unwrap().block_on(async {
            self.soundcore_device
                .as_ref()
                .unwrap()
                .subscribe_to_state_updates()
        })
    }

    #[generate_interface]
    pub fn mac_address(&self) -> Result<String, String> {
        self.runtime
            .as_ref()
            .unwrap()
            .block_on(async { self.soundcore_device.as_ref().unwrap().mac_address().await })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn name(&self) -> Result<String, String> {
        self.runtime
            .as_ref()
            .unwrap()
            .block_on(async { self.soundcore_device.as_ref().unwrap().name().await })
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.runtime.as_ref().unwrap().block_on(async {
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
        self.runtime
            .as_ref()
            .unwrap()
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
        self.runtime.as_ref().as_ref().unwrap().block_on(async {
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
        self.runtime
            .as_ref()
            .unwrap()
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
        self.runtime.as_ref().unwrap().block_on(async {
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
        self.runtime
            .as_ref()
            .unwrap()
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
