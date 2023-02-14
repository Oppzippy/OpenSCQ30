use std::sync::Arc;

use openscq30_lib::{
    api::traits::SoundcoreDevice,
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
    state::SoundcoreDeviceState,
};
use tokio::{runtime::Handle, sync::broadcast};

pub struct GtkSoundcoreDevice<'a, SoundcoreDeviceType: 'static>
where
    SoundcoreDeviceType: SoundcoreDevice + Send + Sync,
{
    tokio_runtime: &'a Handle,
    soundcore_device: Arc<SoundcoreDeviceType>,
}

impl<'a, SoundcoreDeviceType> GtkSoundcoreDevice<'a, SoundcoreDeviceType>
where
    SoundcoreDeviceType: SoundcoreDevice + Send + Sync,
{
    pub fn new(device: Arc<SoundcoreDeviceType>, tokio_runtime: &'a Handle) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }

    pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        self.soundcore_device.subscribe_to_state_updates()
    }

    pub async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(self.tokio_runtime, soundcore_device.mac_address().await)
    }

    pub async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(self.tokio_runtime, soundcore_device.name().await)
    }

    pub async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.ambient_sound_mode().await
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

    pub async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.noise_canceling_mode().await
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

    pub async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device.equalizer_configuration().await
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
