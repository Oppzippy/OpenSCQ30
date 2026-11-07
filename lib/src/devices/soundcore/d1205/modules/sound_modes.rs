use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{
    api::settings::{self, CategoryId, Setting, SettingId, Value},
    devices::soundcore::{
        common::{
            modules::ModuleCollection,
            packet::PacketIOController,
            settings_manager::{SettingHandler, SettingHandlerResult},
            structures::ManualNoiseCanceling,
        },
        d1205::structures::SoundModes,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SoundModeSetting {
        AmbientSoundMode,
        ManualNoiseCanceling,
        WindNoiseSuppression,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Send + Sync,
{
    pub fn add_d1205_sound_modes(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_partial_sound_modes_v2::<SoundModes>(packet_io);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler);
    }
}

struct SoundModesSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for SoundModesSettingHandler
where
    T: Has<SoundModes> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SoundModeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let sound_modes: &SoundModes = state.get();
        let setting: SoundModeSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            SoundModeSetting::AmbientSoundMode => {
                Setting::select_from_enum_all_variants(sound_modes.ambient_sound_mode)
            }
            SoundModeSetting::ManualNoiseCanceling => Setting::I32Range {
                setting: settings::Range {
                    range: 1..=5,
                    step: 1,
                },
                value: sound_modes.manual_noise_canceling.inner().into(),
            },
            SoundModeSetting::WindNoiseSuppression => Setting::Toggle {
                value: sound_modes.wind_noise.is_suppression_enabled,
            },
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let sound_modes: &mut SoundModes = state.get_mut();
        let setting: SoundModeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            SoundModeSetting::AmbientSoundMode => {
                sound_modes.ambient_sound_mode = value.try_as_enum_variant()?;
            }
            SoundModeSetting::ManualNoiseCanceling => {
                sound_modes.manual_noise_canceling =
                    ManualNoiseCanceling::new(value.try_as_i32()? as u8);
            }
            SoundModeSetting::WindNoiseSuppression => {
                sound_modes.wind_noise.is_suppression_enabled = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
