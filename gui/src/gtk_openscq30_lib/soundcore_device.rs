use openscq30_lib::{
    api::soundcore_device::SoundcoreDevice,
    packets::structures::{
        ambient_sound_mode::AmbientSoundMode, equalizer_configuration::EqualizerConfiguration,
        noise_canceling_mode::NoiseCancelingMode,
    },
    soundcore_bluetooth::traits::soundcore_device_connection_error::SoundcoreDeviceConnectionError,
};
use tokio::runtime::Handle;

pub struct GtkSoundcoreDevice<'a> {
    tokio_runtime: &'a Handle,
    soundcore_device: SoundcoreDevice,
}

impl<'a> GtkSoundcoreDevice<'a> {
    pub fn new(device: SoundcoreDevice, tokio_runtime: &'a Handle) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }

    pub async fn get_ambient_sound_mode(self) -> AmbientSoundMode {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device.get_ambient_sound_mode().await
        )
    }

    pub async fn set_ambient_sound_mode(
        self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device
                .set_ambient_sound_mode(ambient_sound_mode)
                .await
        )
    }

    pub async fn get_noise_canceling_mode(self) -> NoiseCancelingMode {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device.get_noise_canceling_mode().await
        )
    }

    pub async fn set_noise_canceling_mode(
        self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device
                .set_noise_canceling_mode(noise_canceling_mode)
                .await
        )
    }

    pub async fn get_equalizer_configuration(self) -> EqualizerConfiguration {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device.get_equalizer_configuration().await
        )
    }

    pub async fn set_equalizer_configuration(
        self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device
                .set_equalizer_configuration(configuration)
                .await
        )
    }
}
