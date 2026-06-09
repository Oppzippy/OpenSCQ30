use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::AmbientSoundModeCycleTwsSettingHandler;
use state_modifier::AmbientSoundModeCycleTwsStateModifier;
use strum::{EnumIter, EnumString};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        packet::PacketIOController,
        structures::{AmbientSoundModeCycleTws, TwsStatus},
    },
    macros::enum_subset,
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    #[allow(clippy::enum_variant_names)]
    enum SoundModeCycleSetting {
        NormalModeInCycle,
        TransparencyModeInCycle,
        NoiseCancelingModeInCycle,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<TwsStatus>
        + Has<AmbientSoundModeCycleTws>
        + Has<ResetButtonConfigurationPending>
        + Clone
        + Send
        + Sync,
{
    pub fn add_ambient_sound_mode_cycle_tws(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            AmbientSoundModeCycleTwsSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(AmbientSoundModeCycleTwsStateModifier::new(
                packet_io,
            )));
    }
}
