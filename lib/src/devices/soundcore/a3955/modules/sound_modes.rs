mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3955::{
            self,
            modules::sound_modes::state_modifier::AncPersonalizedToEarCanalStateModifier,
            structures::{AncPersonalizedToEarCanal, SoundModes},
        },
        common::{modules::ModuleCollection, packet::PacketIOController},
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
        AncPersonalizedToEarCanal,
        MultiSceneNoiseCanceling,
        WindNoiseSuppression,
        WindNoiseDetected,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Clone + Send + Sync,
{
    pub fn add_a3955_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
        self.add_partial_sound_modes_v2_with_migration::<a3955::structures::SoundModes, a3955::structures::SoundModesFields, 8>(
            packet_io.clone(),
        );
        // This comes after the sound modes state modifier so that when moving to the required
        // state, we can set anc personalized to ear canal, but when moving away from the required
        // state, we can't. This is required to work properly with quick presets.
        self.state_modifiers
            .push(Box::new(AncPersonalizedToEarCanalStateModifier::new(
                packet_io,
            )));
    }
}
