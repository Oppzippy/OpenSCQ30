use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    connection::RfcommConnection,
    devices::soundcore::common::{
        modules::{
            ModuleCollection, limit_high_volume::state_modifier::LimitHighVolumeStateModifier,
        },
        packet::PacketIOController,
        structures::LimitHighVolume,
    },
    macros::enum_subset,
    settings::{CategoryId, SettingId},
};

mod setting_handler;
mod state_modifier;

enum_subset! {
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum LimitHighVolumeSetting {
        LimitHighVolume,
        LimitHighVolumeDbLimit,
        LimitHighVolumeRefreshRate,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<LimitHighVolume> + Clone + Send + Sync,
{
    pub fn add_limit_high_volume<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::LimitHighVolume,
            setting_handler::LimitHighVolumeSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(LimitHighVolumeStateModifier::new(packet_io)));
    }
}
