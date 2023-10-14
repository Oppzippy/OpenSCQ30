use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::watch;
use uuid::Uuid;

use crate::{
    api::connection::ConnectionStatus,
    packets::structures::{EqualizerConfiguration, SoundModes},
    state::DeviceState,
};

#[async_trait(?Send)]
pub trait Device {
    async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState>;

    async fn mac_address(&self) -> crate::Result<MacAddr6>;

    fn service_uuid(&self) -> Uuid;

    async fn name(&self) -> crate::Result<String>;

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;

    async fn state(&self) -> DeviceState;

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()>;

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> crate::Result<()>;
}
