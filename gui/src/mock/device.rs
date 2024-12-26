use std::time::Duration;

use gtk::glib::timeout_future;
use macaddr::MacAddr6;
use mockall::mock;
use openscq30_lib::{
    api::{connection::ConnectionStatus, device::Device},
    devices::standard::{
        state::DeviceState,
        structures::{
            AmbientSoundModeCycle, EqualizerConfiguration, HearId, MultiButtonConfiguration,
            SoundModes, SoundModesTypeTwo,
        },
    },
};
use tokio::sync::watch;
use uuid::Uuid;

mock! {
    #[derive(Debug)]
    pub Device {
        pub fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState>;
        pub fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
        pub fn mac_address(&self) -> openscq30_lib::Result<MacAddr6>;
        pub fn name(&self) -> openscq30_lib::Result<String>;
        pub fn service_uuid(&self) -> Uuid;
        pub fn state(&self) -> DeviceState;
        pub fn set_sound_modes(
            &self,
            sound_modes: SoundModes,
        ) -> openscq30_lib::Result<()>;
        pub fn set_sound_modes_type_two(
            &self,
            sound_modes: SoundModesTypeTwo,
        ) -> openscq30_lib::Result<()>;
        pub fn set_ambient_sound_mode_cycle(
            &self,
            cycle: AmbientSoundModeCycle,
        ) -> openscq30_lib::Result<()>;
        pub fn set_equalizer_configuration(
            &self,
            configuration: EqualizerConfiguration,
        ) -> openscq30_lib::Result<()>;
        pub fn set_hear_id(
            &self,
            hear_id: HearId,
        ) -> openscq30_lib::Result<()>;
        pub fn set_multi_button_configuration(
            &self,
            button_configuration: MultiButtonConfiguration,
        ) -> openscq30_lib::Result<()>;
    }
}

impl Device for MockDevice {
    async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState> {
        self.subscribe_to_state_updates()
    }
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status()
    }
    async fn mac_address(&self) -> openscq30_lib::Result<MacAddr6> {
        timeout_future(Duration::from_millis(10)).await;
        self.mac_address()
    }
    fn service_uuid(&self) -> Uuid {
        self.service_uuid()
    }
    async fn name(&self) -> openscq30_lib::Result<String> {
        timeout_future(Duration::from_millis(10)).await;
        self.name()
    }
    async fn state(&self) -> DeviceState {
        timeout_future(Duration::from_millis(10)).await;
        self.state()
    }
    async fn set_sound_modes(&self, sound_modes: SoundModes) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_sound_modes(sound_modes)
    }
    async fn set_sound_modes_type_two(
        &self,
        sound_modes: SoundModesTypeTwo,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_sound_modes_type_two(sound_modes)
    }
    async fn set_ambient_sound_mode_cycle(
        &self,
        cycle: AmbientSoundModeCycle,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_ambient_sound_mode_cycle(cycle)
    }
    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_equalizer_configuration(equalizer_configuration)
    }
    async fn set_hear_id(&self, hear_id: HearId) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_hear_id(hear_id)
    }
    async fn set_multi_button_configuration(
        &self,
        button_configuration: MultiButtonConfiguration,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_multi_button_configuration(button_configuration)
    }
}
