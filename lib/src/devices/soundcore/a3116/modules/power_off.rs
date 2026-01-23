use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3116,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum PowerOffSetting {
        PowerOff,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<a3116::structures::PowerOffPending> + Clone + Send + Sync + 'static,
{
    pub fn add_a3116_power_off<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::PowerOffSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::PowerOffStateModifier::new(
                packet_io,
            )));
    }
}
