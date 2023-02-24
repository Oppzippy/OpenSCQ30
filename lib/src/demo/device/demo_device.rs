use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::{broadcast, Mutex};

use crate::{
    api::device::Device,
    packets::structures::{
        AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
    },
    state::DeviceState,
};

#[derive(Debug)]
pub struct DemoDevice {
    name: String,
    mac_address: String,
    state: Mutex<DeviceState>,
    sender: broadcast::Sender<DeviceState>,
}

impl DemoDevice {
    pub async fn new(name: String, mac_address: String) -> Self {
        tokio::time::sleep(Duration::from_millis(500)).await; // it takes some time to connect
        let (sender, _receiver) = broadcast::channel(50);
        Self {
            name,
            mac_address,
            sender,
            state: Mutex::new(DeviceState::new(
                AmbientSoundMode::Normal,
                NoiseCancelingMode::Indoor,
                EqualizerConfiguration::new_from_preset_profile(
                    PresetEqualizerProfile::SoundcoreSignature,
                ),
            )),
        }
    }
}

#[async_trait]
impl Device for DemoDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.sender.subscribe()
    }

    async fn mac_address(&self) -> crate::Result<String> {
        Ok(self.mac_address.to_owned())
    }

    async fn name(&self) -> crate::Result<String> {
        Ok(self.name.to_owned())
    }

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> crate::Result<()> {
        let mut state = self.state.lock().await;
        *state = state.with_ambient_sound_mode(ambient_sound_mode);
        Ok(())
    }

    async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.state.lock().await.ambient_sound_mode()
    }

    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> crate::Result<()> {
        let mut state = self.state.lock().await;
        *state = state.with_noise_canceling_mode(noise_canceling_mode);
        Ok(())
    }

    async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.state.lock().await.noise_canceling_mode()
    }

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        let mut state = self.state.lock().await;
        *state = state.with_equalizer_configuration(configuration);
        Ok(())
    }

    async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.lock().await.equalizer_configuration()
    }
}
