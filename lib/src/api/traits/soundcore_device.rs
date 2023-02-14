use async_trait::async_trait;
use tokio::sync::broadcast;

use crate::{
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
    state::SoundcoreDeviceState,
};

#[async_trait]
pub trait SoundcoreDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState>;

    async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError>;

    async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError>;

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError>;

    async fn ambient_sound_mode(&self) -> AmbientSoundMode;

    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> Result<(), SoundcoreDeviceConnectionError>;

    async fn noise_canceling_mode(&self) -> NoiseCancelingMode;

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> Result<(), SoundcoreDeviceConnectionError>;

    async fn equalizer_configuration(&self) -> EqualizerConfiguration;
}
