mod setting_handler;

use openscq30_lib_has::Has;
use setting_handler::SpatialAudioSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{self, modules::ModuleCollection},
        d1202,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SpatialAudioSetting {
        SpatialAudio,
        SpatialAudioMode,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1202::structures::SpatialAudio>
        + Has<common::structures::CommonEqualizerConfiguration<2, 10>>
        + Has<common::structures::CustomHearId<2, 10>>
        + Clone
        + Send
        + Sync,
{
    pub fn add_d1202_spatial_audio(&mut self) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, SpatialAudioSettingHandler);
        // state modifier is handled by d1202::modules::equalizer::state_modifier
    }
}
