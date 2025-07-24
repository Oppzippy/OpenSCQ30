mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use packet_handler::SoundModesPacketHandler;
use setting_handler::SoundModesSettingHandler;
use state_modifier::SoundModesStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3959::structures::A3959SoundModes,
        standard::{modules::ModuleCollection, packet::packet_io_controller::PacketIOController},
    },
};

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum SoundModeSetting {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    AdaptiveNoiseCanceling,
    ManualNoiseCanceling,
    WindNoiseSuppression,
    AdaptiveNoiseCancelingSensitivityLevel,
    MultiSceneNoiseCanceling,
}

impl From<SoundModeSetting> for SettingId {
    fn from(setting: SoundModeSetting) -> Self {
        match setting {
            SoundModeSetting::AmbientSoundMode => SettingId::AmbientSoundMode,
            SoundModeSetting::TransparencyMode => SettingId::TransparencyMode,
            SoundModeSetting::NoiseCancelingMode => SettingId::NoiseCancelingMode,
            SoundModeSetting::AdaptiveNoiseCanceling => SettingId::AdaptiveNoiseCanceling,
            SoundModeSetting::ManualNoiseCanceling => SettingId::ManualNoiseCanceling,
            SoundModeSetting::WindNoiseSuppression => SettingId::WindNoiseSuppression,
            SoundModeSetting::AdaptiveNoiseCancelingSensitivityLevel => {
                SettingId::AdaptiveNoiseCancelingSensitivityLevel
            }
            SoundModeSetting::MultiSceneNoiseCanceling => SettingId::MultiSceneNoiseCanceling,
        }
    }
}

impl TryFrom<&SettingId> for SoundModeSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::AmbientSoundMode => Ok(SoundModeSetting::AmbientSoundMode),
            SettingId::TransparencyMode => Ok(SoundModeSetting::TransparencyMode),
            SettingId::AdaptiveNoiseCanceling => Ok(SoundModeSetting::AdaptiveNoiseCanceling),
            SettingId::ManualNoiseCanceling => Ok(SoundModeSetting::ManualNoiseCanceling),
            SettingId::NoiseCancelingMode => Ok(SoundModeSetting::NoiseCancelingMode),
            SettingId::WindNoiseSuppression => Ok(SoundModeSetting::WindNoiseSuppression),
            SettingId::AdaptiveNoiseCancelingSensitivityLevel => {
                Ok(SoundModeSetting::AdaptiveNoiseCancelingSensitivityLevel)
            }
            SettingId::MultiSceneNoiseCanceling => Ok(SoundModeSetting::MultiSceneNoiseCanceling),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<A3959SoundModes> + AsRef<A3959SoundModes> + Clone + Send + Sync,
{
    pub fn add_a3959_sound_modes<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
        self.state_modifiers
            .push(Box::new(SoundModesStateModifier::new(packet_io)));
        self.packet_handlers.set_handler(
            SoundModesPacketHandler::COMMAND,
            Box::new(SoundModesPacketHandler::default()),
        );
    }
}
