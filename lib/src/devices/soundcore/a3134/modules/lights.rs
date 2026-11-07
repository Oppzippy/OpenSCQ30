use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3134::structures::Lights,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum LightsSetting {
        LightsEnabled,
        LightsBrightness,
        LightsMode,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<Lights> + Clone + Send + Sync,
{
    pub fn add_a3134_lights(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Lights, setting_handler::LightsSettingHandler);
        self.state_modifiers
            .push(Box::new(state_modifier::LightsStateModifier::new(
                packet_io,
            )));
    }
}
