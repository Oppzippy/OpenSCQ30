mod packet_handler;
mod setting_handler;
mod state_modifier;

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
        a3957::{
            self,
            modules::sound_modes::state_modifier::AncPersonalizedToEarCanalStateModifier,
            structures::{AncPersonalizedToEarCanal, SoundModes},
        },
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
        TransparencyMode,
        NoiseCancelingMode,
        ManualNoiseCanceling,
        AncPersonalizedToEarCanal,
        TransportationMode,
        WindNoiseSuppression,
        WindNoiseDetected,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Clone + Send + Sync,
{
    pub fn add_a3957_sound_modes<ConnectionT>(
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
                SoundModes,
                a3957::structures::SoundModesFields,
                8,
            >::new(packet_io.clone())));
        // This comes after the sound modes state modifier so that when moving to the required
        // state, we can set anc personalized to ear canal, but when moving away from the required
        // state, we can't. This is required to work properly with quick presets.
        self.state_modifiers
            .push(Box::new(AncPersonalizedToEarCanalStateModifier::new(
                packet_io,
            )));
        self.packet_handlers.set_handler(
            SoundModesPacketHandler::COMMAND,
            Box::new(SoundModesPacketHandler::default()),
        );
    }
}
