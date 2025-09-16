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
        a3959::structures::MultiButtonConfiguration,
        common::{modules::ModuleCollection, packet::PacketIOController, structures::TwsStatus},
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    #[allow(clippy::enum_variant_names)]
    enum ButtonConfigurationSetting {
        LeftSinglePress,
        LeftDoublePress,
        LeftTriplePress,
        LeftLongPress,
        RightSinglePress,
        RightDoublePress,
        RightTriplePress,
        RightLongPress,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<MultiButtonConfiguration> + Has<TwsStatus> + Clone + Send + Sync,
{
    pub fn add_a3959_button_configuration<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler::new(),
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier::new(packet_io)));
    }
}
