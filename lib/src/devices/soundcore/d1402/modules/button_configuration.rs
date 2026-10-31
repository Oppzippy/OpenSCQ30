mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::ButtonConfigurationSettingHandler;
use state_modifier::ButtonConfigurationStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{modules::ModuleCollection, packet::PacketIOController},
        d1402,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum ButtonConfigurationSetting {
        SinglePress,
        DoublePress,
        MfbLongPress,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1402::structures::ButtonConfiguration>
        + Has<d1402::structures::MfbButtonConfiguration>
        + Clone
        + Send
        + Sync,
{
    pub fn add_d1402_button_configuration(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(packet_io)));
    }
}
