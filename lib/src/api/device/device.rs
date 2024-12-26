use macaddr::MacAddr6;
use tokio::sync::watch;
use uuid::Uuid;

use crate::{
    api::connection::ConnectionStatus,
    devices::standard::{
        state::DeviceState,
        structures::{
            AmbientSoundModeCycle, EqualizerConfiguration, HearId, MultiButtonConfiguration,
            SoundModes, SoundModesTypeTwo,
        },
    },
};

pub trait Device {
    async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState>;

    async fn mac_address(&self) -> crate::Result<MacAddr6>;

    fn service_uuid(&self) -> Uuid;

    async fn name(&self) -> crate::Result<String>;

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;

    async fn state(&self) -> DeviceState;

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()>;
    async fn set_sound_modes_type_two(&self, sound_modes: SoundModesTypeTwo) -> crate::Result<()>;

    async fn set_ambient_sound_mode_cycle(&self, cycle: AmbientSoundModeCycle)
        -> crate::Result<()>;

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> crate::Result<()>;

    async fn set_hear_id(&self, hear_id: HearId) -> crate::Result<()>;
    async fn set_multi_button_configuration(
        &self,
        button_configuration: MultiButtonConfiguration,
    ) -> crate::Result<()>;
}
