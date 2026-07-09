use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3876,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum ColorfulLightsSetting {
        ColorfulLightsEnabled,
        ColorfulLightsBrightness,
        AutoLightsOffMinutes,
        ColorfulLightsColor,
        ColorfulLightsMode,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<a3876::structures::ColorfulLights> + Clone + Send + Sync,
{
    pub fn add_a3876_colorful_lights(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::ColorfulLights,
            setting_handler::ColorfulLightsSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::ColorfulLightsStateModifier::new(
                packet_io,
            )));
    }
}
