use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::standard::{
        modules::{
            ModuleCollection,
            touch_tone::{
                setting_handler::TouchToneSettingHandler, state_modifier::TouchToneStateModifier,
            },
        },
        packet::packet_io_controller::PacketIOController,
        structures::TouchTone,
    },
};

mod setting_handler;
mod state_modifier;

#[derive(EnumIter, EnumString)]
enum TouchToneSetting {
    TouchTone,
}

impl From<TouchToneSetting> for SettingId {
    fn from(value: TouchToneSetting) -> Self {
        match value {
            TouchToneSetting::TouchTone => Self::TouchTone,
        }
    }
}

impl TryFrom<SettingId> for TouchToneSetting {
    type Error = ();

    fn try_from(setting_id: SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::TouchTone => Ok(Self::TouchTone),
            _ => Err(()),
        }
    }
}

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
