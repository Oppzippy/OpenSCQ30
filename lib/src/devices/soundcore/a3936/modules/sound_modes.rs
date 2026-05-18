mod setting_handler;

use std::sync::Arc;

use openscq30_lib_has::Has;
use setting_handler::SoundModesSettingHandler;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3936::structures::A3936SoundModes,
        common::{modules::ModuleCollection, packet::PacketIOController},
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
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<A3936SoundModes> + Clone + Send + Sync,
{
    pub fn add_a3936_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_partial_sound_modes_v2_with_sound_mode_update_lock(packet_io);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
    }
}
