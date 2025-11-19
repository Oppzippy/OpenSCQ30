mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::ButtonConfigurationSettingHandler;
use state_modifier::ButtonConfigurationStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3040,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum ButtonConfigurationSetting {
        SinglePress,
        DoublePress,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3040::structures::ButtonConfiguration> + Clone + Send + Sync,
{
    pub fn add_a3040_button_configuration<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler::default(),
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(packet_io)));
    }
}
