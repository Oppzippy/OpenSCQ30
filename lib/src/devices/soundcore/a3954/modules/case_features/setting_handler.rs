use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3954::structures::CaseFeatures,
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
};

use super::CaseFeaturesSetting;

pub struct CaseFeaturesSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for CaseFeaturesSettingHandler
where
    T: Has<CaseFeatures> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        CaseFeaturesSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let features: &CaseFeatures = state.get();
        let features_setting: CaseFeaturesSetting = (*setting_id).try_into().ok()?;
        match features_setting {
            CaseFeaturesSetting::Atmospheric => Some(Setting::Toggle {
                value: features.is_atmospheric_enabled,
            }),
            CaseFeaturesSetting::RemoteCamera => Some(Setting::Toggle {
                value: features.is_remote_camera_enabled,
            }),
            CaseFeaturesSetting::FindDevice => Some(Setting::Toggle {
                value: features.is_find_device_enabled,
            }),
            CaseFeaturesSetting::SpatialAudio => Some(Setting::Toggle {
                value: features.is_spatial_audio_enabled,
            }),
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let features_setting: CaseFeaturesSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match features_setting {
            CaseFeaturesSetting::Atmospheric => {
                let features = state.get_mut();
                features.is_atmospheric_enabled = value.try_as_bool()?;
            }
            CaseFeaturesSetting::RemoteCamera => {
                let features = state.get_mut();
                features.is_remote_camera_enabled = value.try_as_bool()?;
            }
            CaseFeaturesSetting::FindDevice => {
                let features = state.get_mut();
                features.is_find_device_enabled = value.try_as_bool()?;
            }
            CaseFeaturesSetting::SpatialAudio => {
                let features = state.get_mut();
                features.is_spatial_audio_enabled = value.try_as_bool()?;
            }
        }
        Ok(())
    }
}
