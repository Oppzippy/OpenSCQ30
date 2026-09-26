mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::PressSensitivitySettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3953::structures::PressSensitivity,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum PressSensitivitySetting {
        PressSensitivity,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<PressSensitivity> + Clone + Send + Sync,
{
    pub fn add_a3953_press_sensitivity(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, PressSensitivitySettingHandler);
        self.state_modifiers.push(Box::new(
            state_modifier::PressSensitivityStateModifier::new(packet_io),
        ));
    }
}
