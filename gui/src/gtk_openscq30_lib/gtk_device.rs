use std::sync::Arc;

use openscq30_lib::{
    api::device::Device,
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    state::DeviceState,
};
use tokio::{runtime::Runtime, sync::broadcast};

pub struct GtkDevice<SoundcoreDeviceType: 'static>
where
    SoundcoreDeviceType: Device + Send + Sync,
{
    tokio_runtime: Arc<Runtime>,
    soundcore_device: Arc<SoundcoreDeviceType>,
}

#[allow(dead_code)]
impl<SoundcoreDeviceType> GtkDevice<SoundcoreDeviceType>
where
    SoundcoreDeviceType: Device + Send + Sync,
{
    pub fn new(device: Arc<SoundcoreDeviceType>, tokio_runtime: Arc<Runtime>) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }

    pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.soundcore_device.subscribe_to_state_updates()
    }

    pub async fn mac_address(&self) -> openscq30_lib::Result<String> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(self.tokio_runtime, soundcore_device.mac_address().await)
    }

    pub async fn name(&self) -> openscq30_lib::Result<String> {
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
    ) -> openscq30_lib::Result<()> {
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
    ) -> openscq30_lib::Result<()> {
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
    ) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            soundcore_device
                .set_equalizer_configuration(configuration)
                .await
        )
    }
}
