mod packet_handler;
mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use packet_handler::SoundModesPacketHandler;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3959::{self, structures::SoundModes},
        common::{
            modules::{ModuleCollection, sound_modes_v2},
            packet::PacketIOController,
        },
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
    pub fn add_a3959_sound_modes<ConnectionT>(
        &mut self,
        packet_io: Arc<PacketIOController<ConnectionT>>,
    ) where
        ConnectionT: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
        self.state_modifiers
            .push(Box::new(sound_modes_v2::SoundModesStateModifier::<
                ConnectionT,
                a3959::structures::SoundModes,
                a3959::structures::SoundModesFields,
                7,
            >::new(packet_io)));
        self.packet_handlers.set_handler(
            SoundModesPacketHandler::COMMAND,
            Box::new(SoundModesPacketHandler::default()),
        );
    }
}
