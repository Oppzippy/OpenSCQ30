use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    devices::soundcore::{
        a3954,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
    settings::{CategoryId, SettingId},
};

mod setting_handler;
mod state_modifier;

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum CaseFeaturesSetting {
        Atmospheric,
        RemoteCamera,
        FindDevice,
        SpatialAudio,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3954::structures::CaseFeatures> + Clone + Send + Sync,
{
    pub fn add_a3954_case_features(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::Case,
            setting_handler::CaseFeaturesSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::CaseFeaturesStateModifier::new(
                packet_io,
            )));
    }
}
