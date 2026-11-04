mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
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
    enum SoundModeSetting {
        AmbientSoundMode,
        ManualNoiseCanceling,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1402::structures::SoundModes> + Clone + Send + Sync,
{
    pub fn add_d1402_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler);
        self.add_partial_sound_modes_v2_with_migration(packet_io);
    }
}
