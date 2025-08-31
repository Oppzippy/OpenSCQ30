mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
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
        a3959::structures::SoundModes,
        common::{modules::ModuleCollection, packet::packet_io_controller::PacketIOController},
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
    WindNoiseDetected,
    AdaptiveNoiseCancelingSensitivityLevel,
    MultiSceneNoiseCanceling,
}

impl From<SoundModeSetting> for SettingId {
    fn from(setting: SoundModeSetting) -> Self {
        match setting {
            SoundModeSetting::AmbientSoundMode => Self::AmbientSoundMode,
            SoundModeSetting::TransparencyMode => Self::TransparencyMode,
            SoundModeSetting::NoiseCancelingMode => Self::NoiseCancelingMode,
            SoundModeSetting::AdaptiveNoiseCanceling => Self::AdaptiveNoiseCanceling,
            SoundModeSetting::ManualNoiseCanceling => Self::ManualNoiseCanceling,
            SoundModeSetting::WindNoiseSuppression => Self::WindNoiseSuppression,
            SoundModeSetting::WindNoiseDetected => Self::WindNoiseDetected,
            SoundModeSetting::AdaptiveNoiseCancelingSensitivityLevel => {
                Self::AdaptiveNoiseCancelingSensitivityLevel
            }
            SoundModeSetting::MultiSceneNoiseCanceling => Self::MultiSceneNoiseCanceling,
        }
    }
}

impl TryFrom<&SettingId> for SoundModeSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::AmbientSoundMode => Ok(Self::AmbientSoundMode),
            SettingId::TransparencyMode => Ok(Self::TransparencyMode),
            SettingId::AdaptiveNoiseCanceling => Ok(Self::AdaptiveNoiseCanceling),
            SettingId::ManualNoiseCanceling => Ok(Self::ManualNoiseCanceling),
            SettingId::NoiseCancelingMode => Ok(Self::NoiseCancelingMode),
            SettingId::WindNoiseSuppression => Ok(Self::WindNoiseSuppression),
            SettingId::WindNoiseDetected => Ok(Self::WindNoiseDetected),
            SettingId::AdaptiveNoiseCancelingSensitivityLevel => {
                Ok(Self::AdaptiveNoiseCancelingSensitivityLevel)
            }
            SettingId::MultiSceneNoiseCanceling => Ok(Self::MultiSceneNoiseCanceling),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Clone + Send + Sync,
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
