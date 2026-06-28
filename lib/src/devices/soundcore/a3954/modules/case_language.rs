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
    enum CaseLanguageSetting {
        CaseLanguage,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3954::structures::CaseLanguage> + Clone + Send + Sync,
{
    pub fn add_a3954_case_language(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::Case,
            setting_handler::CaseLanguageSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::CaseLanguageStateModifier::new(
                packet_io,
            )));
    }
}
