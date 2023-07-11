use async_trait::async_trait;
use tokio::sync::{broadcast, watch};

use crate::{
    api::connection::ConnectionStatus,
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    state::DeviceState,
};

#[async_trait]
pub trait Device {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState>;

    async fn mac_address(&self) -> crate::Result<String>;

    async fn name(&self) -> crate::Result<String>;

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> crate::Result<()>;

    async fn ambient_sound_mode(&self) -> AmbientSoundMode;

    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> crate::Result<()>;

    async fn noise_canceling_mode(&self) -> NoiseCancelingMode;

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> crate::Result<()>;

    async fn equalizer_configuration(&self) -> EqualizerConfiguration;
}
