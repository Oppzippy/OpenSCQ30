use std::time::Duration;

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::{broadcast, watch, Mutex};

use crate::{
    api::{connection::ConnectionStatus, device::Device},
    packets::structures::{
        AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
    },
    state::DeviceState,
};

pub struct DemoDevice {
    name: String,
    mac_address: MacAddr6,
    state: Mutex<DeviceState>,
    state_sender: broadcast::Sender<DeviceState>,
    connection_status_sender: watch::Sender<ConnectionStatus>,
}

impl DemoDevice {
    pub async fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        tokio::time::sleep(Duration::from_millis(500)).await; // it takes some time to connect
        let (state_sender, _) = broadcast::channel(50);
        let (connection_status_sender, _) = watch::channel(ConnectionStatus::Connected);
        Self {
            name: name.into(),
            mac_address,
            state_sender,
            connection_status_sender,
            state: Mutex::new(DeviceState {
                ambient_sound_mode: AmbientSoundMode::Normal,
                noise_canceling_mode: NoiseCancelingMode::Indoor,
                transparency_mode: Default::default(),
                custom_noise_canceling: Default::default(),
                equalizer_configuration: EqualizerConfiguration::new_from_preset_profile(
                    PresetEqualizerProfile::SoundcoreSignature,
                ),
            }),
        }
    }
}

#[async_trait]
impl Device for DemoDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.state_sender.subscribe()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.mac_address.to_owned())
    }

    async fn name(&self) -> crate::Result<String> {
        Ok(self.name.to_owned())
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_sender.subscribe()
    }

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> crate::Result<()> {
        tracing::info!("set ambient sound mode to {ambient_sound_mode:?}");
        let mut state = self.state.lock().await;
        *state = DeviceState {
            ambient_sound_mode,
            ..*state
        };
        Ok(())
    }

    async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.lock().await.ambient_sound_mode
    }

    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> crate::Result<()> {
        tracing::info!("set noise canceling mode to {noise_canceling_mode:?}");
        let mut state = self.state.lock().await;
        *state = DeviceState {
            noise_canceling_mode,
            ..*state
        };
        Ok(())
    }

    async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.lock().await.noise_canceling_mode
    }

    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        tracing::info!("set equalizer configuration to {equalizer_configuration:?}");
        let mut state = self.state.lock().await;
        *state = DeviceState {
            equalizer_configuration,
            ..*state
        };
        Ok(())
    }

    async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.lock().await.equalizer_configuration
    }
}

impl core::fmt::Debug for DemoDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemoDevice")
            .field("name", &self.name)
            .field("mac_address", &self.mac_address)
            .field("state", &self.state)
            .field("state_sender", &self.state_sender)
            .finish()
    }
}
