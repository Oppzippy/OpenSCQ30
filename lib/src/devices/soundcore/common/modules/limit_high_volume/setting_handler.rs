use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        modules::limit_high_volume::LimitHighVolumeSetting,
        settings_manager::{SettingHandler, SettingHandlerResult},
        structures::LimitHighVolume,
    },
    settings,
};

pub struct LimitHighVolumeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for LimitHighVolumeSettingHandler
where
    T: Has<LimitHighVolume> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        LimitHighVolumeSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let limit_high_volume = state.get();
        let setting: LimitHighVolumeSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            LimitHighVolumeSetting::LimitHighVolume => Setting::Toggle {
                value: limit_high_volume.enabled,
            },
            LimitHighVolumeSetting::LimitHighVolumeDbLimit => Setting::I32Range {
                setting: settings::Range {
                    range: 75..=100,
                    step: 5,
                },
                value: limit_high_volume.db_limit.into(),
            },
            LimitHighVolumeSetting::LimitHighVolumeRefreshRate => {
                Setting::select_from_enum_all_variants(limit_high_volume.refresh_rate)
            }
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let limit_high_volume = state.get_mut();
        let setting: LimitHighVolumeSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match setting {
            LimitHighVolumeSetting::LimitHighVolume => {
                limit_high_volume.enabled = value.try_as_bool()?;
            }
            LimitHighVolumeSetting::LimitHighVolumeDbLimit => {
                let db_limit = value.try_as_i32()?;
                // Only multiples of 5 are allowed
                let clamped_db_limit = (db_limit - (db_limit % 5)).clamp(75, 100);
                limit_high_volume.db_limit = clamped_db_limit as u8;
            }
            LimitHighVolumeSetting::LimitHighVolumeRefreshRate => {
                limit_high_volume.refresh_rate = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
