mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3035,
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
        ManualTransparency,
        WindNoiseSuppression,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3035::structures::SoundModes> + Clone + Send + Sync,
{
    pub fn add_a3035_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_partial_sound_modes_v2(packet_io);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
    }
}
