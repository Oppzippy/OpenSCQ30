mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{modules::ModuleCollection, packet::PacketIOController},
        d1202,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SoundModeSetting {
        AmbientSoundMode,
        TransparencyMode,
        NoiseCancelingMode,
        AdaptiveNoiseCanceling,
        ManualNoiseCanceling,
        MultiSceneNoiseCanceling,
        WindNoiseSuppression,
        WindNoiseDetected,
        RealTimeAdaptiveNoiseCanceling,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1202::structures::SoundModes> + Send + Sync,
{
    pub fn add_d1202_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_partial_sound_modes_v2(packet_io);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler);
    }
}
