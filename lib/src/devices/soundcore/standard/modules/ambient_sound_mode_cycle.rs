use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::AmbientSoundModeCycleSettingHandler;
use state_modifier::AmbientSoundModeCycleStateModifier;
use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::standard::{
        packet::packet_io_controller::PacketIOController, structures::AmbientSoundModeCycle,
    },
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

#[derive(EnumIter, EnumString)]
#[allow(clippy::enum_variant_names)]
enum SoundModeCycleSetting {
    NormalModeInCycle,
    TransparencyModeInCycle,
    NoiseCancelingModeInCycle,
}

impl From<SoundModeCycleSetting> for SettingId {
    fn from(value: SoundModeCycleSetting) -> Self {
        match value {
            SoundModeCycleSetting::NormalModeInCycle => Self::NormalModeInCycle,
            SoundModeCycleSetting::TransparencyModeInCycle => Self::TransparencyModeInCycle,
            SoundModeCycleSetting::NoiseCancelingModeInCycle => Self::NoiseCancelingModeInCycle,
        }
    }
}

impl TryFrom<&SettingId> for SoundModeCycleSetting {
    type Error = ();

    fn try_from(value: &SettingId) -> Result<Self, Self::Error> {
        match value {
            SettingId::NormalModeInCycle => Ok(Self::NormalModeInCycle),
            SettingId::TransparencyModeInCycle => Ok(Self::TransparencyModeInCycle),
            SettingId::NoiseCancelingModeInCycle => Ok(Self::NoiseCancelingModeInCycle),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<AmbientSoundModeCycle> + Clone + Send + Sync,
{
    pub fn add_ambient_sound_mode_cycle<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            AmbientSoundModeCycleSettingHandler::new(),
        );
        self.state_modifiers
            .push(Box::new(AmbientSoundModeCycleStateModifier::new(packet_io)));
    }
}
