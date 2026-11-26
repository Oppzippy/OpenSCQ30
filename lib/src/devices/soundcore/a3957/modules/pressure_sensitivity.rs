mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::PressureSensitivitySettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3957::structures::PressureSensitivity,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum PressureSensitivitySetting {
        PressureSensitivity,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<PressureSensitivity> + Clone + Send + Sync,
{
    pub fn add_a3957_pressure_sensitivity(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, PressureSensitivitySettingHandler);
        self.state_modifiers.push(Box::new(
            state_modifier::PressureSensitivityStateModifier::new(packet_io),
        ));
    }
}
