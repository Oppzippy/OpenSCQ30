use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::common::{
        modules::{
            ModuleCollection,
            touch_tone::{
                setting_handler::TouchToneSettingHandler, state_modifier::TouchToneStateModifier,
            },
        },
        packet::PacketIOController,
        structures::TouchTone,
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum TouchToneSetting {
        TouchTone,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<TouchTone> + Clone + Send + Sync + 'static,
{
    pub fn add_touch_tone<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, TouchToneSettingHandler::new());
        self.state_modifiers
            .push(Box::new(TouchToneStateModifier::new(packet_io)));
    }
}
