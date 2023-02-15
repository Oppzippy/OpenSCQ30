use async_trait::async_trait;
use tokio::sync::{broadcast, Mutex};

use crate::{
    api::traits::SoundcoreDevice,
    packets::structures::{
        AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
    },
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
    state::SoundcoreDeviceState,
};

#[derive(Debug)]
pub struct DemoSoundcoreDevice {
    name: String,
    mac_address: String,
    state: Mutex<SoundcoreDeviceState>,
    sender: broadcast::Sender<SoundcoreDeviceState>,
}

impl DemoSoundcoreDevice {
    pub fn new(name: String, mac_address: String) -> Self {
        let (sender, _receiver) = broadcast::channel(50);
        Self {
            name,
            mac_address,
            sender,
            state: Mutex::new(SoundcoreDeviceState::new(
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
impl SoundcoreDevice for DemoSoundcoreDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<SoundcoreDeviceState> {
        self.sender.subscribe()
    }

    async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        Ok(self.mac_address.to_owned())
    }

    async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError> {
        Ok(self.name.to_owned())
    }

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> Result<(), SoundcoreDeviceConnectionError> {
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
    ) -> Result<(), SoundcoreDeviceConnectionError> {
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
    ) -> Result<(), SoundcoreDeviceConnectionError> {
        let mut state = self.state.lock().await;
        *state = state.with_equalizer_configuration(configuration);
        Ok(())
    }

    async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.state.lock().await.equalizer_configuration()
    }
}
