mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3959::structures::SoundModes,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SoundModeSetting {
        AmbientSoundMode,
        NoiseCancelingMode,
        AdaptiveNoiseCanceling,
        ManualNoiseCanceling,
        WindNoiseSuppression,
        WindNoiseDetected,
        AdaptiveNoiseCancelingSensitivityLevel,
        MultiSceneNoiseCanceling,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Clone + Send + Sync,
{
    pub fn add_a3959_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
        self.add_partial_sound_modes_v2_with_migration(packet_io);
    }
}
