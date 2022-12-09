use std::sync::Arc;

use openscq30_lib::{
    api::SoundcoreDevice,
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};
use tokio::runtime::Handle;

pub struct GtkSoundcoreDevice<'a> {
    tokio_runtime: &'a Handle,
    soundcore_device: Arc<SoundcoreDevice>,
}

impl<'a> GtkSoundcoreDevice<'a> {
    pub fn new(device: Arc<SoundcoreDevice>, tokio_runtime: &'a Handle) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }

    pub async fn get_mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(self.tokio_runtime, soundcore_device.get_mac_address().await)
    }

    pub async fn get_name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(self.tokio_runtime, soundcore_device.get_name().await)
    }

    pub async fn get_ambient_sound_mode(&self) -> AmbientSoundMode {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.get_ambient_sound_mode().await
        )
    }

    pub async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device
                .set_ambient_sound_mode(ambient_sound_mode)
                .await
        )
    }

    pub async fn get_noise_canceling_mode(&self) -> NoiseCancelingMode {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.get_noise_canceling_mode().await
        )
    }

    pub async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device
                .set_noise_canceling_mode(noise_canceling_mode)
                .await
        )
    }

    pub async fn get_equalizer_configuration(&self) -> EqualizerConfiguration {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.get_equalizer_configuration().await
        )
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device
                .set_equalizer_configuration(configuration)
                .await
        )
    }
}
