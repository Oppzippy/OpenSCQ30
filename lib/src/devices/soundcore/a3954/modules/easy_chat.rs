mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::EasyChatSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3954,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum EasyChatSetting {
        EasyChat,
        EasyChatWaitTime,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3954::structures::EasyChat> + Clone + Send + Sync,
{
    pub fn add_a3954_easy_chat(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, EasyChatSettingHandler);
        self.state_modifiers
            .push(Box::new(state_modifier::EasyChatStateModifier::new(
                packet_io,
            )));
    }
}
