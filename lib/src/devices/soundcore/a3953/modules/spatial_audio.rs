mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SpatialAudioSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3953::structures::SpatialAudio,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SpatialAudioSetting {
        SpatialAudio,
        SpatialAudioMode,
        SpatialAudioMusicMode,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SpatialAudio> + Clone + Send + Sync,
{
    pub fn add_a3953_spatial_audio(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, SpatialAudioSettingHandler);
        self.state_modifiers
            .push(Box::new(state_modifier::SpatialAudioStateModifier::new(
                packet_io,
            )));
    }
}
