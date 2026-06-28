mod setting_handler;

use openscq30_lib_has::Has;
use setting_handler::SpatialAudioSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3954,
        common::{self, modules::ModuleCollection},
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
    T: Has<a3954::structures::SpatialAudio>
        + Has<common::structures::CommonEqualizerConfiguration<2, 10>>
        + Has<common::structures::CustomHearId<2, 10>>
        + Clone
        + Send
        + Sync,
{
    pub fn add_a3954_spatial_audio(&mut self) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, SpatialAudioSettingHandler);
        // state modifier is handled by a3954::modules::equalizer::state_modifier
    }
}
