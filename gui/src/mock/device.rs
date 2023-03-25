use std::time::Duration;

use async_trait::async_trait;
use gtk::glib::timeout_future;
use mockall::mock;
use openscq30_lib::{
    api::device::Device,
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    state::DeviceState,
};
use tokio::sync::broadcast;

mock! {
    #[derive(Debug)]
    pub Device {
        pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState>;
        pub fn mac_address(&self) -> openscq30_lib::Result<String>;
        pub fn name(&self) -> openscq30_lib::Result<String>;
        pub fn set_ambient_sound_mode(
            &self,
            ambient_sound_mode: AmbientSoundMode,
        ) -> openscq30_lib::Result<()>;
        pub fn ambient_sound_mode(&self) -> AmbientSoundMode;
        pub fn set_noise_canceling_mode(
            &self,
            noise_canceling_mode: NoiseCancelingMode,
        ) -> openscq30_lib::Result<()>;
        pub fn noise_canceling_mode(&self) -> NoiseCancelingMode;
        pub fn set_equalizer_configuration(
            &self,
            configuration: EqualizerConfiguration,
        ) -> openscq30_lib::Result<()>;
        pub fn equalizer_configuration(&self) -> EqualizerConfiguration;
    }
}

#[async_trait]
impl Device for MockDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.subscribe_to_state_updates()
    }
    async fn mac_address(&self) -> openscq30_lib::Result<String> {
        timeout_future(Duration::from_millis(10)).await;
        self.mac_address()
    }
    async fn name(&self) -> openscq30_lib::Result<String> {
        timeout_future(Duration::from_millis(10)).await;
        self.name()
    }
    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_ambient_sound_mode(ambient_sound_mode)
    }
    async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        timeout_future(Duration::from_millis(10)).await;
        self.ambient_sound_mode()
    }
    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_noise_canceling_mode(noise_canceling_mode)
    }
    async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        timeout_future(Duration::from_millis(10)).await;
        self.noise_canceling_mode()
    }
    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_equalizer_configuration(configuration)
    }
    async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        timeout_future(Duration::from_millis(10)).await;
        self.equalizer_configuration()
    }
}
